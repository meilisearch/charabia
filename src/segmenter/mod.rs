use std::borrow::Cow;
use std::collections::HashMap;

pub use latin::LatinSegmenter;
use maplit::hashmap;
use once_cell::sync::Lazy;

use crate::detection::{Detect, Language, Script, StrDetection};
use crate::token::{Token, TokenKind};

mod latin;

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
    hashmap! {
        (Script::Latin, Language::Other) => Box::new(LatinSegmenter) as Box<dyn Segmenter>,
    }
});

/// Picked [`Segmenter`] when no segmenter is specialized to the detected [`Script`].
pub static DEFAULT_SEGMENTER: Lazy<Box<dyn Segmenter>> = Lazy::new(|| Box::new(LatinSegmenter));

/// Iterator over segmented [`Token`]s.
pub struct SegmentedTokenIter<'a> {
    inner: Box<dyn Iterator<Item = &'a str> + 'a>,
    char_index: usize,
    byte_index: usize,
    script: Script,
    language: Option<Language>,
}

impl<'a> Iterator for SegmentedTokenIter<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let word = self.inner.next()?;
        let char_index = self.char_index;
        let byte_index = self.byte_index;

        self.char_index += word.chars().count();
        self.byte_index += word.len();

        Some(Token {
            kind: TokenKind::Unknown,
            word: Cow::Borrowed(word),
            char_start: char_index,
            char_end: self.char_index,
            byte_start: byte_index,
            byte_end: self.byte_index,
            char_map: None,
            script: self.script,
            language: self.language,
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
    fn segment_str<'a>(&self, s: &'a str) -> Box<dyn Iterator<Item = &'a str> + 'a>;
}

impl Segmenter for Box<dyn Segmenter> {
    fn segment_str<'a>(&self, s: &'a str) -> Box<dyn Iterator<Item = &'a str> + 'a> {
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
    /// use meilisearch_tokenizer::{Token, TokenKind, Segment};
    ///
    /// let orig = "The quick (\"brown\") fox can't jump 32.3 feet, right? Brr, it's 29.3°F!";
    ///
    /// let mut tokens = orig.segment();
    ///
    /// let Token { word, kind, .. } = tokens.next().unwrap();
    /// // the token isn't normalized.
    /// assert_eq!(word, "The");
    /// // the token isn't classified and defaultly set to Unknown.
    /// assert_eq!(kind, TokenKind::Unknown);
    ///
    /// let Token { word, kind, .. } = tokens.next().unwrap();
    /// assert_eq!(word, " ");
    /// assert_eq!(kind, TokenKind::Unknown);
    ///
    /// let Token { word, kind, .. } = tokens.next().unwrap();
    /// assert_eq!(word, "quick");
    /// assert_eq!(kind, TokenKind::Unknown);
    /// ```
    fn segment(&self) -> SegmentedTokenIter<'o>;

    /// Segments the provided text creating an Iterator over `&str`.
    ///
    /// # Example
    ///
    /// ```
    /// use meilisearch_tokenizer::Segment;
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
        let mut detector = self.detect();
        let segmenter = segmenter(&mut detector);
        let script = detector.script();
        let language = detector.language;

        SegmentedTokenIter {
            inner: segmenter.segment_str(self),
            script,
            language,
            char_index: 0,
            byte_index: 0,
        }
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
                assert_eq!(&segmented_text[..], $segmented, "Segmenter {} didn't segment the text as expected", stringify!($segmenter));
            }

            #[test]
            fn text_lang_script_assignment() {
                let Token {script, language, ..} = $text.segment().next().unwrap();
                assert_eq!((script, language.unwrap_or(Language::Other)), ($script, $language), "Provided text is not detected as the expected Script or Language to be segmented by {}", stringify!($segmenter));
            }

            #[test]
            fn segment() {
                let segmented_text: Vec<_> = $text.segment_str().collect();
                assert_eq!(&segmented_text[..], $segmented, "Segmenter, chosen by global segment() function, didn't segment the text as expected");
            }

            #[test]
            fn tokenize() {
                let tokens: Vec<_> = $text.tokenize().collect();
                let tokenized_text: Vec<_> = tokens.iter().map(|t| t.text()).collect();
                assert_eq!(&tokenized_text[..], $tokenized, "Global tokenize() function didn't tokenize the text as expected");
            }
        }
    }
    pub(crate) use test_segmenter;
}
