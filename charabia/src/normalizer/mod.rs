use std::borrow::Cow;

use once_cell::sync::Lazy;

pub use self::arabic::ArabicNormalizer;
#[cfg(feature = "chinese-normalization")]
pub use self::chinese::ChineseNormalizer;
pub use self::classify::{Classifier, ClassifierOption};
pub use self::compatibility_decomposition::CompatibilityDecompositionNormalizer;
pub use self::control_char::ControlCharNormalizer;
#[cfg(feature = "greek")]
use self::greek::GreekNormalizer;
#[cfg(feature = "japanese-transliteration")]
pub use self::japanese::JapaneseNormalizer;
pub use self::lowercase::LowercaseNormalizer;
use self::nonspacing_mark::NonspacingMarkNormalizer;
use self::quote::QuoteNormalizer;
#[cfg(feature = "swedish-recomposition")]
use self::swedish_recomposition::SwedishRecompositionNormalizer;
#[cfg(feature = "turkish")]
pub use self::turkish::TurkishNormalizer;
#[cfg(feature = "vietnamese")]
pub use self::vietnamese::VietnameseNormalizer;
use crate::normalizer::character_converter::CharacterConverterNormalizer;
use crate::segmenter::SegmentedTokenIter;
use crate::Token;

pub use self::ae_oe_normalizer::AeOeNormalizer;

mod arabic;
#[cfg(feature = "chinese-normalization")]
mod chinese;
mod classify;
mod compatibility_decomposition;
mod control_char;
#[cfg(feature = "greek")]
mod greek;
#[cfg(feature = "japanese-transliteration")]
mod japanese;
mod lowercase;
mod nonspacing_mark;
mod quote;
#[cfg(feature = "swedish-recomposition")]
mod swedish_recomposition;
#[cfg(feature = "turkish")]
mod turkish;
#[cfg(feature = "vietnamese")]
mod vietnamese;

mod ae_oe_normalizer;
mod character_converter;

/// List of [`Normalizer`]s used by [`Normalize::normalize`] that are not considered lossy.
pub static NORMALIZERS: Lazy<Vec<Box<dyn Normalizer>>> = Lazy::new(|| {
    vec![
        Box::new(CompatibilityDecompositionNormalizer),
        #[cfg(feature = "swedish-recomposition")]
        Box::new(SwedishRecompositionNormalizer),
        Box::new(ControlCharNormalizer),
        Box::new(Classifier),
    ]
});

/// List of [`Normalizer`]s used by [`Normalize::normalize`] that are considered lossy.
pub static LOSSY_NORMALIZERS: Lazy<Vec<Box<dyn Normalizer>>> = Lazy::new(|| {
    vec![
        // Box::new(LowercaseNormalizer),
        // Box::new(QuoteNormalizer),
        // Box::new(AeOeNormalizer),
        Box::new(CharacterConverterNormalizer),
        #[cfg(feature = "chinese-normalization")]
        Box::new(ChineseNormalizer),
        #[cfg(feature = "japanese-transliteration")]
        Box::new(JapaneseNormalizer),
        #[cfg(feature = "greek")]
        Box::new(GreekNormalizer),
        // Box::new(ArabicNormalizer),
        Box::new(NonspacingMarkNormalizer),
        // #[cfg(feature = "vietnamese")]
        // Box::new(VietnameseNormalizer),
        // #[cfg(feature = "turkish")]
        // Box::new(TurkishNormalizer),
    ]
});

pub(crate) const DEFAULT_NORMALIZER_OPTION: NormalizerOption = NormalizerOption {
    create_char_map: false,
    lossy: true,
    classifier: ClassifierOption { stop_words: None, separators: None },
};

/// Iterator over Normalized [`Token`]s.
pub struct NormalizedTokenIter<'o, 'aho, 'lang, 'tb> {
    token_iter: SegmentedTokenIter<'o, 'aho, 'lang>,
    options: &'tb NormalizerOption<'tb>,
}

impl<'o> Iterator for NormalizedTokenIter<'o, '_, '_, '_> {
    type Item = Token<'o>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.token_iter.next()?.normalize(self.options))
    }
}

/// Structure for providing options to a normalizer.
#[derive(Debug, Clone, Default)]
pub struct NormalizerOption<'tb> {
    pub create_char_map: bool,
    pub classifier: ClassifierOption<'tb>,
    pub lossy: bool,
}

/// Trait defining a normalizer.
pub trait Normalizer: Sync + Send {
    /// Normalize the provided [`Token`].
    /// Options can be set using the provided [`NormalizerOption`].
    ///
    fn normalize<'o>(&self, token: Token<'o>, options: &NormalizerOption) -> Token<'o>;

    /// Return true if the normalizer can process Token of a specific [`Script`] and [`Language`].
    ///
    /// Some normalizer are specialized for a `Script` or/and a `Language` and shouldn't be called on every `Token`s.
    fn should_normalize(&self, token: &Token) -> bool;
}

// Allow taking &Cow as argument to spare the allocation if it is already borrowed (and thus ~Copy)
#[allow(clippy::ptr_arg)]
fn shrink_cow<'o>(s: &Cow<'o, str>, new_size: usize) -> Cow<'o, str> {
    match s {
        Cow::Borrowed(s) => Cow::Borrowed(&s[..new_size]),
        Cow::Owned(s) => Cow::Owned(s[..new_size].to_string()),
    }
}

pub trait CharNormalizer: Sync + Send {
    fn normalize_char(&self, c: char) -> Option<CharOrStr>;

    fn normalize_cow_str<'o>(&self, s: Cow<'o, str>) -> Cow<'o, str> {
        let mut new: Option<Cow<str>> = None;

        for (i, c) in s.char_indices() {
            new = match self.normalize_char(c) {
                Some(CharOrStr::Char(normalized)) if normalized == c => {
                    new.take().map(|mut new| {
                        new.to_mut().push(normalized);
                        new
                    })
                }
                Some(CharOrStr::Char(normalized)) => {
                    new.take().or_else(|| Some(shrink_cow(&s, i))).map(|mut new| {
                        new.to_mut().push(normalized);
                        new
                    })
                }
                Some(CharOrStr::Str(normalized)) => {
                    new.take().or_else(|| Some(shrink_cow(&s, i))).map(|mut new| {
                        new.to_mut().push_str(&normalized);
                        new
                    })
                }
                None => new.take().or_else(|| Some(shrink_cow(&s, i))),
            }
        }

        new.unwrap_or(s)
    }

    fn normalize_str<'o>(&self, s: &'o str) -> Cow<'o, str> {
        self.normalize_cow_str(Cow::Borrowed(s))
    }

    /// Return true if the normalizer can process Token of a specific [`Script`] and [`Language`].
    ///
    /// Some normalizer are specialized for a `Script` or/and a `Language` and shouldn't be called on every `Token`s.
    fn should_normalize(&self, token: &Token) -> bool;
}

impl<T> Normalizer for T
where
    T: CharNormalizer,
{
    fn normalize<'o>(&self, mut token: Token<'o>, options: &NormalizerOption) -> Token<'o> {
        if options.create_char_map {
            match token.char_map.take() {
                Some(mut char_map) => {
                    let mut lemma = String::new();
                    let mut tail = token.lemma.as_ref();
                    for (_, normalized_len) in char_map.iter_mut() {
                        let (head, t) = tail.split_at(*normalized_len as usize);
                        tail = t;
                        let normalized = self.normalize_str(head);
                        *normalized_len = normalized.len() as u8;
                        lemma.push_str(normalized.as_ref());
                    }

                    token.lemma = Cow::Owned(lemma);
                    token.char_map = Some(char_map);
                }
                None => {
                    let mut buffer = [0; 4];
                    let mut char_map = Vec::new();
                    let mut lemma = String::new();
                    for c in token.lemma().chars() {
                        let char_str = c.encode_utf8(&mut buffer);
                        let normalized = self.normalize_str(char_str);
                        char_map.push((char_str.len() as u8, normalized.len() as u8));
                        lemma.push_str(normalized.as_ref());
                    }

                    token.lemma = Cow::Owned(lemma);
                    token.char_map = Some(char_map);
                }
            }
        } else {
            token.lemma = self.normalize_cow_str(token.lemma);
        }

        token
    }

    fn should_normalize(&self, token: &Token) -> bool {
        CharNormalizer::should_normalize(self, token)
    }
}

pub enum CharOrStr {
    Char(char),
    Str(String),
}

impl CharOrStr {
    pub fn merge(&self, other: &Self) -> Self {
        let mut result = String::new();
        match self {
            Self::Char(c) => result.push(*c),
            Self::Str(s) => result.push_str(s),
        }
        match other {
            Self::Char(c) => result.push(*c),
            Self::Str(s) => result.push_str(s),
        }
        Self::Str(result)
    }
}

impl From<char> for CharOrStr {
    fn from(c: char) -> Self {
        Self::Char(c)
    }
}

impl From<String> for CharOrStr {
    fn from(s: String) -> Self {
        Self::Str(s)
    }
}

impl<'o, 'aho, 'lang> SegmentedTokenIter<'o, 'aho, 'lang> {
    /// Normalize [`Token`]s using all the compatible Normalizers.
    ///
    /// A Latin `Token` would not be normalized the same as a Chinese `Token`.
    pub fn normalize<'tb>(
        self,
        options: &'tb NormalizerOption<'tb>,
    ) -> NormalizedTokenIter<'o, 'aho, 'lang, 'tb> {
        NormalizedTokenIter { token_iter: self, options }
    }
}

pub trait Normalize {
    type Item;
    fn normalize(self, options: &NormalizerOption) -> Self::Item;
}

impl Normalize for Token<'_> {
    type Item = Self;

    /// Normalize [`Token`] using all the compatible Normalizers.
    ///
    /// A Latin `Token` would not be normalized the same as a Chinese `Token`.
    fn normalize(mut self, options: &NormalizerOption) -> Self::Item {
        for normalizer in NORMALIZERS.iter() {
            if normalizer.should_normalize(&self) {
                self = normalizer.normalize(self, options);
            }
        }

        if options.lossy {
            for normalizer in LOSSY_NORMALIZERS.iter() {
                if normalizer.should_normalize(&self) {
                    self = normalizer.normalize(self, options);
                }
            }
        }

        self
    }
}

impl<'o> Normalize for &'o str {
    type Item = Cow<'o, str>;

    /// Normalize an str.
    fn normalize(self, options: &NormalizerOption) -> Self::Item {
        let mut normalized = Token { lemma: Cow::Borrowed(self), ..Default::default() };
        for normalizer in NORMALIZERS.iter() {
            normalized = normalizer.normalize(normalized, options);
        }

        if options.lossy {
            for normalizer in LOSSY_NORMALIZERS.iter() {
                normalized = normalizer.normalize(normalized, options);
            }
        }

        normalized.lemma
    }
}

#[cfg(test)]
mod test {
    use std::borrow::Cow;

    use crate::normalizer::quote::QuoteNormalizer;
    use crate::normalizer::{
        CompatibilityDecompositionNormalizer, LowercaseNormalizer, Normalizer,
    };
    use crate::Token;

    macro_rules! test_normalizer {
        ($normalizer:expr, $tokens:expr, $normalizer_result:expr, $global_result:expr) => {
            use super::*;
            use crate::{Token, Normalize, StaticToken};
            use fst::Set;

            const TEST_NORMALIZER_OPTIONS: NormalizerOption = NormalizerOption {
                create_char_map: true,
                lossy: true,
                classifier: crate::normalizer::ClassifierOption { stop_words: None, separators: None },
            };

            #[test]
            fn normalizer_normalize() {
                let normalized_tokens: Vec<_> = $tokens
                    .into_iter()
                    .map(|token| if Normalizer::should_normalize(&$normalizer, &token) {
                        $normalizer.normalize(token, &TEST_NORMALIZER_OPTIONS)
                    } else {
                        token
                    })
                    .collect();
                assert_eq!(
                    &normalized_tokens[..],
                    $normalizer_result,
                    r#"
Normalizer {} didn't normalize tokens as expected.

help: The `normalizer_result` provided to `test_normalizer!` does not corresponds to the output of the tested normalizer,
it's probably due to a bug in the normalizer or a mistake in the provided normalized tokens.
"#,
                    stringify!($normalizer)
                );
            }

            #[test]
            fn global_normalize() {
                let normalized_tokens: Vec<_> = $tokens.into_iter().map(|t| t.normalize(&TEST_NORMALIZER_OPTIONS)).collect();
                assert_eq!(
                    &normalized_tokens[..],
                    $global_result,
                    r#"
Global normalization pipeline didn't normalize tokens as expected.

help: The `global_result` provided to `test_normalizer!` does not corresponds to the output of the normalizer pipeline, it's probably because the normalizer is missing from `NORMALIZERS` list or because an other normalizer has alterated the token.
Check if the `NORMALIZERS` list in `charabia/src/normalizer/mod.rs` contains the tested Normalizer.
Make sure that normalized tokens are valid or change the trigger condition of the noisy normalizers by updating `should_normalize`.
"#
                );
            }

            #[quickcheck]
            fn normalizer_not_panic_for_random_option(token: StaticToken, create_char_map: bool, lossy: bool, mut stop_words: Vec<String>, separators: Vec<String>, original_lengths_arg: usize) {
                stop_words.sort();
                let stop_words = Set::from_iter(stop_words.iter()).unwrap();
                let stop_words = Set::new(stop_words.as_fst().as_bytes()).unwrap();
                let separators: Vec<&str> = separators.iter().map(|s| s.as_str()).collect();
                let normalizer_option = NormalizerOption {
                    create_char_map,
                    lossy,
                    classifier:  crate::normalizer::ClassifierOption {
                        stop_words: Some(stop_words),
                        separators: Some(separators.as_slice()),
                    }
                };

                let normalized_token = token.normalize(&normalizer_option);
                let _ = normalized_token.original_lengths(original_lengths_arg);
            }
        };
    }
    pub(crate) use test_normalizer;

    #[test]
    fn split_at() {
        fn display_token<N>(token: &Token) {
            println!("{} with {}", token.lemma(), std::any::type_name::<N>());
            if let Some(char_map) = token.char_map.as_ref() {
                let mut s = &token.lemma[..];
                for (start, len) in char_map {
                    match s.get((*len as usize)..) {
                        Some(n) => {
                            println!("{} - {:?}", &s[..(*len as usize)], (start, len));
                            s = n;
                        }
                        None => println!("⚠ - {:?}", (start, len)),
                    }
                }
            }
        }

        let options = crate::normalizer::NormalizerOption {
            create_char_map: true,
            lossy: true,
            ..Default::default()
        };

        let string = "0÷IÖꞪz";
        let mut normalized = Token { lemma: Cow::Borrowed(string), ..Default::default() };
        display_token::<()>(&normalized);
        normalized = CompatibilityDecompositionNormalizer.normalize(normalized, &options);
        display_token::<CompatibilityDecompositionNormalizer>(&normalized);
        normalized = LowercaseNormalizer.normalize(normalized, &options);
        display_token::<LowercaseNormalizer>(&normalized);
        normalized = QuoteNormalizer.normalize(normalized, &options);
        display_token::<QuoteNormalizer>(&normalized);
        let _ = normalized;
    }
}
