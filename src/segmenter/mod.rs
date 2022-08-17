use std::borrow::Cow;
use std::collections::HashMap;

#[cfg(feature = "chinese")]
pub use chinese::ChineseSegmenter;
#[cfg(feature = "hebrew")]
pub use hebrew::HebrewSegmenter;
#[cfg(feature = "japanese")]
pub use japanese::JapaneseSegmenter;
pub use latin::LatinSegmenter;
use once_cell::sync::Lazy;
use slice_group_by::StrGroupBy;
#[cfg(feature = "thai")]
pub use thai::ThaiSegmenter;

use crate::detection::{Detect, Language, Script, StrDetection};
use crate::token::Token;

#[cfg(feature = "chinese")]
mod chinese;
#[cfg(feature = "hebrew")]
mod hebrew;
#[cfg(feature = "japanese")]
mod japanese;
mod latin;
#[cfg(feature = "thai")]
mod thai;

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
pub static SEGMENTERS: Lazy<HashMap<(Script, Language), Box<dyn Segmenter>>> = Lazy::new(|| {
    vec![
        // latin segmenter
        ((Script::Latin, Language::Other), Box::new(LatinSegmenter) as Box<dyn Segmenter>),
        // chinese segmenter
        #[cfg(feature = "chinese")]
        ((Script::Cj, Language::Cmn), Box::new(ChineseSegmenter) as Box<dyn Segmenter>),
        // hebrew segmenter
        #[cfg(feature = "hebrew")]
        ((Script::Hebrew, Language::Heb), Box::new(HebrewSegmenter) as Box<dyn Segmenter>),
        // japanese segmenter
        #[cfg(feature = "japanese")]
        ((Script::Cj, Language::Jpn), Box::new(JapaneseSegmenter) as Box<dyn Segmenter>),
        // thai segmenter
        #[cfg(feature = "thai")]
        ((Script::Thai, Language::Tha), Box::new(ThaiSegmenter) as Box<dyn Segmenter>),
    ]
    .into_iter()
    .collect()
});

/// Picked [`Segmenter`] when no segmenter is specialized to the detected [`Script`].
pub static DEFAULT_SEGMENTER: Lazy<Box<dyn Segmenter>> = Lazy::new(|| Box::new(LatinSegmenter));

/// Iterator over segmented [`Token`]s.
pub struct SegmentedTokenIter<'a> {
    inner: Box<dyn Iterator<Item = Token<'a>> + 'a>,
    char_index: usize,
    byte_index: usize,
}

impl<'a> Iterator for SegmentedTokenIter<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut token = self.inner.next()?;
        let char_index = self.char_index;
        let byte_index = self.byte_index;

        self.char_index += token.lemma().chars().count();
        self.byte_index += token.lemma().len();

        token.char_start = char_index;
        token.char_end = self.char_index;
        token.byte_start = byte_index;
        token.byte_end = self.byte_index;

        Some(token)
    }
}

struct InnerSegmentedTokenIter<'a> {
    inner: Box<dyn Iterator<Item = &'a str> + 'a>,
    script: Script,
    language: Option<Language>,
}

impl<'a> Iterator for InnerSegmentedTokenIter<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let lemma = self.inner.next()?;

        Some(Token {
            lemma: Cow::Borrowed(lemma),
            script: self.script,
            language: self.language,
            ..Default::default()
        })
    }
}

/// Try to Detect Language and Script and return the corresponding segmenter,
/// if no Language is detected or no segmenter corresponds to the Language
/// the function try to get a segmenter corresponding to the script;
/// if no Script is detected or no segmenter corresponds to the Script,
/// the function try to get the default segmenter in the map;
/// if no default segmenter exists in the map return the library DEFAULT_SEGMENTER.
fn segmenter<'a, 'b>(detector: &'a mut StrDetection) -> &'b impl Segmenter {
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
            &*SEGMENTERS
                .get(&(detected_script, detected_language))
                .or_else(|| SEGMENTERS.get(&(detected_script, Language::Other)))
                .unwrap_or_else(|| &DEFAULT_SEGMENTER)
        }
    }
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
    fn segment(&self) -> SegmentedTokenIter<'o>;

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
    fn segment_str(&self) -> Box<dyn Iterator<Item = &'o str> + 'o>;
}

impl<'o> Segment<'o> for &'o str {
    fn segment(&self) -> SegmentedTokenIter<'o> {
        let mut current_script = Script::Other;
        let inner = self
            .linear_group_by_key(move |c| {
                let script = Script::from(c);
                if script != Script::Other && script != current_script {
                    current_script = script
                }
                current_script
            })
            .map(|s| {
                let mut detector = s.detect();
                let segmenter = segmenter(&mut detector);
                let script = detector.script();
                let language = detector.language;
                InnerSegmentedTokenIter { inner: segmenter.segment_str(s), script, language }
            })
            .flatten();

        SegmentedTokenIter { inner: Box::new(inner), char_index: 0, byte_index: 0 }
    }

    fn segment_str(&self) -> Box<dyn Iterator<Item = &'o str> + 'o> {
        let mut detector = self.detect();
        let segmenter = segmenter(&mut detector);

        segmenter.segment_str(self)
    }
}

#[cfg(test)]
mod test {
    macro_rules! test_segmenter {
    ($segmenter:expr, $text:expr, $segmented:expr, $tokenized:expr, $script:expr, $language:expr) => {
            use crate::{Token, Language, Script};
            use crate::segmenter::Segment;
            use crate::tokenizer::Tokenize;
            use super::*;

            #[test]
            fn segmenter_segment_str() {
                let segmented_text: Vec<_> = $segmenter.segment_str($text).collect();
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
                let segmented_text: Vec<_> = $text.segment_str().collect();
                assert_eq!(&segmented_text[..], $segmented, r#"
Segmenter chosen by global segment() function, didn't segment the text as expected.

help: The selected segmenter is probably the wrong one.
Check if the tested segmenter is assigned to the good Script/Language in `SEGMENTERS` global in `src/segmenter/mod.rs`.
"#);
            }

            #[test]
            fn tokenize() {
                let tokens: Vec<_> = $text.tokenize().collect();
                let tokenized_text: Vec<_> = tokens.iter().map(|t| t.lemma()).collect();

                assert_eq!(&tokenized_text[..], $tokenized, r#"
Global tokenize() function didn't tokenize the text as expected.

help: The normalized version of the segmented text is probably wrong, the used normalizers make unexpeted changes to the provided text.
Make sure that normalized text is valid or change the trigger condition of the noisy normalizers by updating `should_normalize`.
"#);
            }
        }
    }
    pub(crate) use test_segmenter;
}
