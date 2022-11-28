use std::borrow::Cow;

use once_cell::sync::Lazy;

#[cfg(feature = "chinese")]
pub use self::chinese::ChineseNormalizer;
pub use self::compatibility_decomposition::CompatibilityDecompositionNormalizer;
pub use self::control_char::ControlCharNormalizer;
#[cfg(feature = "japanese-transliteration")]
pub use self::japanese::JapaneseNormalizer;
pub use self::lowercase::LowercaseNormalizer;
use crate::normalizer::nonspacing_mark::NonspacingMarkNormalizer;
use crate::Token;

#[cfg(feature = "chinese")]
mod chinese;
mod compatibility_decomposition;
mod control_char;
#[cfg(feature = "japanese-transliteration")]
mod japanese;
mod lowercase;
mod nonspacing_mark;

/// List of [`Normalizer`]s used by [`Normalize::normalize`].
pub static NORMALIZERS: Lazy<Vec<Box<dyn Normalizer>>> = Lazy::new(|| {
    vec![
        Box::new(CompatibilityDecompositionNormalizer),
        Box::new(LowercaseNormalizer),
        #[cfg(feature = "chinese")]
        Box::new(ChineseNormalizer),
        #[cfg(feature = "japanese-transliteration")]
        Box::new(JapaneseNormalizer),
        Box::new(ControlCharNormalizer),
        Box::new(NonspacingMarkNormalizer),
    ]
});

/// Iterator over Normalized [`Token`]s.
pub struct NormalizedTokenIter<'o> {
    token_iter: Box<dyn Iterator<Item = Token<'o>> + 'o>,
    normalizer: &'static Box<dyn Normalizer>,
    options: NormalizerOption,
}

impl<'o> Iterator for NormalizedTokenIter<'o> {
    type Item = Token<'o>;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.token_iter.next()?;
        if self.normalizer.should_normalize(&token) {
            Some(self.normalizer.normalize(token, self.options))
        } else {
            Some(token)
        }
    }
}

/// Structure for providing options to a normalizer.
#[derive(Clone, Copy)]
pub struct NormalizerOption {
    pub create_char_map: bool,
}

impl Default for NormalizerOption {
    fn default() -> Self {
        NormalizerOption { create_char_map: false }
    }
}

/// Trait defining a normalizer.
pub trait Normalizer: Sync + Send {
    /// Normalize the provided [`Token`].
    /// Options can be set using the provided [`NormalizerOption`].
    ///
    fn normalize<'o>(&self, token: Token<'o>, options: NormalizerOption) -> Token<'o>;

    /// Return true if the normalizer can process Token of a specific [`Script`] and [`Language`].
    ///
    /// Some normalizer are specialized for a `Script` or/and a `Language` and shouldn't be called on every `Token`s.
    fn should_normalize(&self, token: &Token) -> bool;
}

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
    fn normalize<'o>(&self, mut token: Token<'o>, options: NormalizerOption) -> Token<'o> {
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

/// Trait defining methods to normalize [`Token`]s.
pub trait Normalize<'o>: Iterator
where
    Self: Sized,
    Self: Iterator<Item = Token<'o>> + 'o,
{
    /// Normalize [`Token`]s using all the compatible Normalizers.
    ///
    /// A Latin `Token` would not be normalized the same as a Chinese `Token`.
    fn normalize(self, options: NormalizerOption) -> NormalizedTokenIter<'o> {
        let first = NormalizedTokenIter {
            token_iter: Box::new(self),
            normalizer: NORMALIZERS.first().unwrap(),
            options: options,
        };

        NORMALIZERS.iter().skip(1).fold(first, |token_iter, normalizer| NormalizedTokenIter {
            token_iter: Box::new(token_iter),
            normalizer,
            options: options,
        })
    }
}

impl<'o, T> Normalize<'o> for T where T: Iterator<Item = Token<'o>> + 'o {}

#[cfg(test)]
mod test {
    macro_rules! test_normalizer {
        ($normalizer:expr, $tokens:expr, $normalizer_result:expr, $global_result:expr) => {
            use super::*;
            use crate::normalizer::Normalize;
            use crate::{Script, Token};

            #[test]
            fn normalizer_normalize() {
                let normalized_tokens: Vec<_> = $tokens
                    .into_iter()
                    .map(|token| if Normalizer::should_normalize(&$normalizer, &token) {
                        $normalizer.normalize(token, NormalizerOption { create_char_map: true })
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
                let normalized_tokens: Vec<_> = $tokens.into_iter().normalize(NormalizerOption { create_char_map: true }).collect();
                assert_eq!(
                    &normalized_tokens[..],
                    $global_result,
                    r#"
Global normalization pipeline didn't normalize tokens as expected.

help: The `global_result` provided to `test_normalizer!` does not corresponds to the output of the normalizer pipeline, it's probably because the normalizer is missing from `NORMALIZERS` list or because an other normalizer has alterated the token.
Check if the `NORMALIZERS` list in `src/normalizer/mod.rs` contains the tested Normalizer. 
Make sure that normalized tokens are valid or change the trigger condition of the noisy normalizers by updating `should_normalize`.
"#
                );
            }
        };
    }
    pub(crate) use test_normalizer;
}
