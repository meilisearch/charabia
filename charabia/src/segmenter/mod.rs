use std::borrow::Cow;
use std::collections::HashMap;

use aho_corasick::{AhoCorasick, FindIter, MatchKind};
pub use arabic::ArabicSegmenter;
#[cfg(feature = "chinese-segmentation")]
pub use chinese::ChineseSegmenter;
use either::Either;
#[cfg(feature = "german-segmentation")]
pub use german::GermanSegmenter;
#[cfg(feature = "japanese")]
pub use japanese::JapaneseSegmenter;
#[cfg(feature = "khmer")]
pub use khmer::KhmerSegmenter;
#[cfg(feature = "korean")]
pub use korean::KoreanSegmenter;
pub use latin::LatinSegmenter;
use once_cell::sync::Lazy;
use slice_group_by::StrGroupBy;
#[cfg(feature = "thai")]
pub use thai::ThaiSegmenter;

use crate::detection::{Detect, Language, Script, StrDetection};
use crate::separators::DEFAULT_SEPARATORS;
use crate::token::Token;

mod arabic;
#[cfg(feature = "chinese-segmentation")]
mod chinese;
#[cfg(feature = "german-segmentation")]
mod german;
#[cfg(feature = "japanese")]
mod japanese;
#[cfg(feature = "khmer")]
mod khmer;
#[cfg(feature = "korean")]
mod korean;
mod latin;
#[cfg(feature = "thai")]
mod thai;
#[cfg(any(feature = "thai", feature = "khmer"))]
mod utils;

pub type SegmenterMap = HashMap<(Script, Option<Language>), Box<dyn Segmenter>>;

/// List of used [`Segmenter`]s linked to their corresponding [`Script`] and [`Language`].
///
/// This list is used after `Script` and `Language` detection to pick the specialized [`Segmenter`].
/// If no segmenter corresponds to the `Language`,
/// then the segmenter corresponding to the `Script` is picked.
/// If no segmenter corresponds to both `Script` and `Language`,
/// then the [`DEFAULT_SEGMENTER`] is picked.
///
/// A segmenter assigned to `Language::Other` is considered as the default `Segmenter` for any `Language` that uses the assigned `Script`.
/// For example, [`LatinSegmenter`] is assigned to `(Script::Latin, Language::Other)`,
/// meaning that `LatinSegmenter` is the default `Segmenter` for any `Language` that uses `Latin` `Script`.
pub static SEGMENTERS: Lazy<SegmenterMap> = Lazy::new(|| {
    vec![
        // latin segmenter
        ((Script::Latin, None), Box::new(LatinSegmenter) as Box<dyn Segmenter>),
        #[cfg(feature = "swedish-recomposition")]
        ((Script::Latin, Some(Language::Swe)), Box::new(LatinSegmenter) as Box<dyn Segmenter>),
        // chinese segmenter
        #[cfg(feature = "chinese-segmentation")]
        ((Script::Cj, Some(Language::Cmn)), Box::new(ChineseSegmenter) as Box<dyn Segmenter>),
        #[cfg(feature = "chinese-segmentation")]
        ((Script::Cj, Some(Language::Zho)), Box::new(ChineseSegmenter) as Box<dyn Segmenter>),
        // japanese segmenter
        #[cfg(feature = "japanese")]
        ((Script::Cj, Some(Language::Jpn)), Box::new(JapaneseSegmenter) as Box<dyn Segmenter>),
        // korean segmenter
        #[cfg(feature = "korean")]
        ((Script::Hangul, Some(Language::Kor)), Box::new(KoreanSegmenter) as Box<dyn Segmenter>),
        // thai segmenter
        #[cfg(feature = "thai")]
        ((Script::Thai, Some(Language::Tha)), Box::new(ThaiSegmenter) as Box<dyn Segmenter>),
        #[cfg(feature = "khmer")]
        ((Script::Khmer, Some(Language::Khm)), Box::new(KhmerSegmenter) as Box<dyn Segmenter>),
        // arabic segmenter
        ((Script::Arabic, Some(Language::Ara)), Box::new(ArabicSegmenter) as Box<dyn Segmenter>),
        // german segmenter
        #[cfg(feature = "german-segmentation")]
        ((Script::Latin, Some(Language::Deu)), Box::new(GermanSegmenter) as Box<dyn Segmenter>),
    ]
    .into_iter()
    .collect()
});

/// Picked [`Segmenter`] when no segmenter is specialized to the detected [`Script`].
pub static DEFAULT_SEGMENTER: Lazy<Box<dyn Segmenter>> = Lazy::new(|| Box::new(LatinSegmenter));

pub static DEFAULT_SEPARATOR_AHO: Lazy<AhoCorasick> = Lazy::new(|| {
    AhoCorasick::builder().match_kind(MatchKind::LeftmostLongest).build(DEFAULT_SEPARATORS).unwrap()
});

/// Iterator over segmented [`Token`]s.
pub struct SegmentedTokenIter<'o, 'aho, 'lang> {
    inner: SegmentedStrIter<'o, 'aho, 'lang>,
    char_index: usize,
    byte_index: usize,
}

impl<'o> Iterator for SegmentedTokenIter<'o, '_, '_> {
    type Item = Token<'o>;

    fn next(&mut self) -> Option<Self::Item> {
        let lemma = self.inner.next()?;
        let char_start = self.char_index;
        let byte_start = self.byte_index;

        self.char_index += lemma.chars().count();
        self.byte_index += lemma.len();

        Some(Token {
            lemma: Cow::Borrowed(lemma),
            script: self.inner.script,
            language: self.inner.language,
            char_start,
            char_end: self.char_index,
            byte_start,
            byte_end: self.byte_index,
            ..Default::default()
        })
    }
}

impl<'o, 'aho, 'lang> From<SegmentedStrIter<'o, 'aho, 'lang>>
    for SegmentedTokenIter<'o, 'aho, 'lang>
{
    fn from(segmented_str_iter: SegmentedStrIter<'o, 'aho, 'lang>) -> Self {
        Self { inner: segmented_str_iter, char_index: 0, byte_index: 0 }
    }
}

pub struct SegmentedStrIter<'o, 'aho, 'lang> {
    inner: Box<dyn Iterator<Item = &'o str> + 'o>,
    current: Box<dyn Iterator<Item = &'o str> + 'o>,
    aho_iter: Option<AhoSegmentedStrIter<'o, 'aho>>,
    segmenter: &'static dyn Segmenter,
    aho: Option<&'aho AhoCorasick>,
    allow_list: Option<&'lang [Language]>,
    script: Script,
    language: Option<Language>,
}

impl<'o, 'aho, 'lang> SegmentedStrIter<'o, 'aho, 'lang> {
    pub fn new(
        original: &'o str,
        aho: Option<&'aho AhoCorasick>,
        allow_list: Option<&'lang [Language]>,
    ) -> Self {
        let mut current_script = Script::Other;
        let mut group_id = 0;
        let inner = original.linear_group_by_key(move |c| {
            let script = Script::from(c);
            if script != Script::Other && script != current_script {
                // if both previous and current scripts are differents than Script::Other,
                // split into a new script group.
                if current_script != Script::Other {
                    group_id += 1;
                }
                current_script = script
            }
            group_id
        });

        Self {
            inner: Box::new(inner),
            current: Box::new(None.into_iter()),
            aho_iter: None,
            segmenter: &*DEFAULT_SEGMENTER,
            aho,
            allow_list,
            script: Script::Other,
            language: None,
        }
    }
}

impl<'o> Iterator for SegmentedStrIter<'o, '_, '_> {
    type Item = &'o str;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current.next() {
            Some(s) => Some(s),
            None => match self.aho_iter.as_mut().and_then(|aho_iter| aho_iter.next()) {
                Some((s, MatchType::Match)) => Some(s),
                Some((s, MatchType::Interleave)) => {
                    self.current = self.segmenter.segment_str(s);

                    self.next()
                }
                None => {
                    let text = self.inner.next()?;
                    let mut detector = text.detect(self.allow_list);
                    self.segmenter = segmenter(&mut detector);
                    self.script = detector.script();
                    self.language = detector.language;
                    self.aho_iter = Some(AhoSegmentedStrIter::new(
                        text,
                        self.aho.unwrap_or(&DEFAULT_SEPARATOR_AHO),
                    ));

                    self.next()
                }
            },
        }
    }
}

struct AhoSegmentedStrIter<'o, 'aho> {
    aho_iter: FindIter<'aho, 'o>,
    prev: Either<usize, aho_corasick::Match>,
    text: &'o str,
}

impl<'o, 'aho> AhoSegmentedStrIter<'o, 'aho> {
    fn new(text: &'o str, aho: &'aho AhoCorasick) -> Self {
        Self { aho_iter: aho.find_iter(text), prev: Either::Left(0), text }
    }
}

impl<'o> Iterator for AhoSegmentedStrIter<'o, '_> {
    type Item = (&'o str, MatchType);

    fn next(&mut self) -> Option<Self::Item> {
        let mut match_type = MatchType::Interleave;
        let (start, end) = match self.prev {
            Either::Left(left) => match self.aho_iter.next() {
                Some(m) => {
                    let range = (left, m.start());
                    self.prev = Either::Right(m);
                    range
                }
                None => {
                    self.prev = Either::Left(self.text.len());
                    (left, self.text.len())
                }
            },
            Either::Right(m) => {
                self.prev = Either::Left(m.end());
                match_type = MatchType::Match;
                (m.start(), m.end())
            }
        };

        if start < end {
            let text = &self.text[start..end];
            if maybe_number(text) {
                Some((text, MatchType::Match))
            } else {
                Some((text, match_type))
            }
        } else if end < self.text.len() {
            self.next()
        } else {
            None
        }
    }
}

fn maybe_number(text: &str) -> bool {
    text.chars().all(|c| c.is_numeric() || c.is_ascii_punctuation())
}

enum MatchType {
    Interleave,
    Match,
}

/// Try to Detect Language and Script and return the corresponding segmenter,
/// if no Language is detected or no segmenter corresponds to the Language
/// the function try to get a segmenter corresponding to the script;
/// if no Script is detected or no segmenter corresponds to the Script,
/// the function try to get the default segmenter in the map;
/// if no default segmenter exists in the map return the library DEFAULT_SEGMENTER.
fn segmenter<'b>(detector: &mut StrDetection) -> &'b dyn Segmenter {
    let detected_script = detector.script();
    let mut filtered_segmenters =
        SEGMENTERS.iter().filter(|((script, _), _)| *script == detected_script);
    match (filtered_segmenters.next(), filtered_segmenters.next()) {
        // no specialized segmenter found for this script,
        // choose the default one.
        (None, None) => &*DEFAULT_SEGMENTER,
        // Only one specialized segmenter found,
        // we don't need to detect the Language.
        (Some((_, segmenter)), None) => segmenter,
        // several segmenters found,
        // we have to detect the language to get the good one.
        _ => {
            let detected_language = detector.language();
            SEGMENTERS
                .get(&(detected_script, detected_language))
                .or_else(|| SEGMENTERS.get(&(detected_script, None)))
                .unwrap_or(&DEFAULT_SEGMENTER)
        }
    }
}

/// Structure for providing options to a normalizer.
#[derive(Debug, Clone, Default)]
pub struct SegmenterOption<'tb> {
    pub aho: Option<AhoCorasick>,
    pub allow_list: Option<&'tb [Language]>,
}

/// Trait defining a segmenter.
///
/// A segmenter should be at least a script specialized segmenter.
pub trait Segmenter: Sync + Send {
    /// Segments the provided text creating an Iterator over `&str`.
    fn segment_str<'o>(&self, s: &'o str) -> Box<dyn Iterator<Item = &'o str> + 'o>;
}

impl Segmenter for Box<dyn Segmenter> {
    fn segment_str<'o>(&self, s: &'o str) -> Box<dyn Iterator<Item = &'o str> + 'o> {
        (**self).segment_str(s)
    }
}

/// Trait defining methods to segment a text.
pub trait Segment<'o> {
    /// Segments the provided text creating an Iterator over Tokens.
    /// Created Tokens are not normalized nether classified,
    /// otherwise, better use the [`tokenize`] method.
    ///
    /// [`tokenize`]: crate::tokenizer::Tokenize#tymethod.tokenize
    ///
    /// # Example
    ///
    /// ```
    /// use charabia::{Token, TokenKind, Segment};
    ///
    /// let orig = "The quick (\"brown\") fox can't jump 32.3 feet, right? Brr, it's 29.3°F!";
    ///
    /// let mut tokens = orig.segment();
    ///
    /// let Token { lemma, kind, .. } = tokens.next().unwrap();
    /// // the token isn't normalized.
    /// assert_eq!(lemma, "The");
    /// // the token isn't classified and defaultly set to Unknown.
    /// assert_eq!(kind, TokenKind::Unknown);
    ///
    /// let Token { lemma, kind, .. } = tokens.next().unwrap();
    /// assert_eq!(lemma, " ");
    /// assert_eq!(kind, TokenKind::Unknown);
    ///
    /// let Token { lemma, kind, .. } = tokens.next().unwrap();
    /// assert_eq!(lemma, "quick");
    /// assert_eq!(kind, TokenKind::Unknown);
    /// ```
    fn segment(&self) -> SegmentedTokenIter<'o, 'o, 'o> {
        self.segment_str().into()
    }

    /// Segments the provided text creating an Iterator over Tokens where you can specify an allowed list of languages to be used with a script.
    fn segment_with_option<'aho, 'lang>(
        &self,
        aho: Option<&'aho AhoCorasick>,
        allow_list: Option<&'lang [Language]>,
    ) -> SegmentedTokenIter<'o, 'aho, 'lang> {
        self.segment_str_with_option(aho, allow_list).into()
    }

    /// Segments the provided text creating an Iterator over `&str`.
    ///
    /// # Example
    ///
    /// ```
    /// use charabia::Segment;
    ///
    /// let orig = "The quick (\"brown\") fox can't jump 32.3 feet, right? Brr, it's 29.3°F!";
    ///
    /// let mut segments = orig.segment_str();
    ///
    /// assert_eq!(segments.next(), Some("The"));
    /// assert_eq!(segments.next(), Some(" "));
    /// assert_eq!(segments.next(), Some("quick"));
    /// ```
    fn segment_str(&self) -> SegmentedStrIter<'o, 'o, 'o> {
        self.segment_str_with_option(None, None)
    }

    /// Segments the provided text creating an Iterator over `&str` where you can specify an allowed list of languages to be used with a script.
    ///
    fn segment_str_with_option<'aho, 'lang>(
        &self,
        aho: Option<&'aho AhoCorasick>,
        allow_list: Option<&'lang [Language]>,
    ) -> SegmentedStrIter<'o, 'aho, 'lang>;
}

impl<'o> Segment<'o> for &'o str {
    fn segment_str_with_option<'aho, 'lang>(
        &self,
        aho: Option<&'aho AhoCorasick>,
        allow_list: Option<&'lang [Language]>,
    ) -> SegmentedStrIter<'o, 'aho, 'lang> {
        SegmentedStrIter::new(self, aho, allow_list)
    }
}

#[cfg(test)]
mod test {
    macro_rules! test_segmenter {
    ($segmenter:expr, $text:expr, $segmented:expr, $tokenized:expr, $script:expr, $language:expr) => {
            use aho_corasick::{AhoCorasick, MatchKind};
            use once_cell::sync::Lazy;
            use crate::{Token, Language, Script};
            use crate::segmenter::{Segment, AhoSegmentedStrIter, MatchType, DEFAULT_SEPARATOR_AHO};
            use super::*;

            const NUMBER_SEPARATOR: &[&str] = &[" "];
            const TEXT_NUMBER: &str = "123 -123 +123 12.3 -12.3 +12.3";
            const SEGMENTED_NUMBER: &[&str] =
                &["123", " ", "-123", " ", "+123", " ", "12.3", " ", "-12.3", " ", "+12.3"];
            const TOKENIZED_NUMBER: &[&str] =
                &["123", " ", "-123", " ", "+123", " ", "12.3", " ", "-12.3", " ", "+12.3"];
            static NUMBER_SEPARATOR_AHO: Lazy<AhoCorasick> = Lazy::new(|| {
                AhoCorasick::builder().match_kind(MatchKind::LeftmostLongest).build(NUMBER_SEPARATOR).unwrap()
            });

            #[test]
            fn segmenter_segment_str() {

                let segmented_text: Vec<_> = AhoSegmentedStrIter::new($text, &DEFAULT_SEPARATOR_AHO).flat_map(|m| match m {
                    (text, MatchType::Match) => Box::new(Some(text).into_iter()),
                    (text, MatchType::Interleave) => $segmenter.segment_str(text),
                }).collect();
                assert_eq!(&segmented_text[..], $segmented, r#"
Segmenter {} didn't segment the text as expected.

help: the `segmented` text provided to `test_segmenter!` does not corresponds to the output of the tested segmenter, it's probably due to a bug in the segmenter or a mistake in the provided segmented text.
"#, stringify!($segmenter));
            }

            #[test]
            fn text_lang_script_assignment() {
                let Token {script, language, ..} = $text.segment().next().unwrap();
                assert_eq!((script, language.unwrap_or($language)), ($script, $language), r#"
Provided text is not detected as the expected Script or Language to be segmented by {}.

help: The tokenizer Script/Language detector detected the wrong Script/Language for the `segmented` text, the provided text will probably be segmented by an other segmenter.
Check if the expected Script/Language corresponds to the detected Script/Language.
"#, stringify!($segmenter));
            }

            #[test]
            fn segment() {
                let segmented_text: Vec<_> = $text.segment_str_with_option(None, Some(&[$language])).collect();
                assert_eq!(&segmented_text[..], $segmented, r#"
Segmenter chosen by global segment() function, didn't segment the text as expected.

help: The selected segmenter is probably the wrong one.
Check if the tested segmenter is assigned to the good Script/Language in `SEGMENTERS` global in `charabia/src/segmenter/mod.rs`.
"#);
            }

            #[test]
            fn tokenize() {
                let tokenizer = crate::TokenizerBuilder::default().into_tokenizer();
                let tokens: Vec<_> = tokenizer.tokenize_with_allow_list($text, Some(&[$language])).collect();
                let tokenized_text: Vec<_> = tokens.iter().map(|t| t.lemma()).collect();

                assert_eq!(&tokenized_text[..], $tokenized, r#"
Global tokenize() function didn't tokenize the text as expected.

help: The normalized version of the segmented text is probably wrong, the used normalizers make unexpeted changes to the provided text.
Make sure that normalized text is valid or change the trigger condition of the noisy normalizers by updating `should_normalize`.
"#);
            }

            #[quickcheck]
            fn segmentor_not_panic_for_random_input(text: String) {
                let _ = $segmenter.segment_str(&text).collect::<Vec<_>>();
            }

            #[test]
            fn segmenter_segment_number() {

                let segmented_text: Vec<_> = AhoSegmentedStrIter::new(TEXT_NUMBER, &NUMBER_SEPARATOR_AHO).flat_map(|m| match m {
                    (text, MatchType::Match) => Box::new(Some(text).into_iter()),
                    (text, MatchType::Interleave) => $segmenter.segment_str(text),
                }).collect();
                assert_eq!(&segmented_text[..], SEGMENTED_NUMBER, r#"
Segmenter {} didn't segment the text as expected.

help: the `segmented` text provided to `test_segmenter!` does not corresponds to the output of the tested segmenter, it's probably due to a bug in the segmenter or a mistake in the provided segmented text.
"#, stringify!($segmenter));
            }

            #[test]
            fn tokenize_number() {

                let mut builder = crate::TokenizerBuilder::default();
                builder.separators(NUMBER_SEPARATOR);
                let tokenizer = builder.build();
                let tokens: Vec<_> = tokenizer.tokenize_with_allow_list(TEXT_NUMBER, Some(&[$language])).collect();
                let tokenized_text: Vec<_> = tokens.iter().map(|t| t.lemma()).collect();

                assert_eq!(&tokenized_text[..], TOKENIZED_NUMBER, r#"
Global tokenize() function didn't tokenize the text as expected.

help: The normalized version of the segmented text is probably wrong, the used normalizers make unexpeted changes to the provided text.
Make sure that normalized text is valid or change the trigger condition of the noisy normalizers by updating `should_normalize`.
"#);
            }

        }
    }
    pub(crate) use test_segmenter;
}
