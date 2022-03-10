use unicode_segmentation::UnicodeSegmentation;

use crate::segmenter::Segmenter;

/// Latin specialized [`Segmenter`].
///
/// This Segmenter uses [`UnicodeSegmentation`] internally to segment the provided text.
pub struct LatinSegmenter;

impl Segmenter for LatinSegmenter {
    fn segment_str<'a>(&self, s: &'a str) -> Box<dyn Iterator<Item = &'a str> + 'a> {
        Box::new(s.split_word_bounds())
    }
}
