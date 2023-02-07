use unicode_segmentation::UnicodeSegmentation;

use super::camel_case::CamelCaseSegmentation;
use super::Segmenter;

/// Latin specialized [`Segmenter`].
///
/// This Segmenter uses [`UnicodeSegmentation`] internally to segment the provided text.
pub struct LatinSegmenter;

impl Segmenter for LatinSegmenter {
    fn segment_str<'o>(&self, s: &'o str) -> Box<dyn Iterator<Item = &'o str> + 'o> {
        let lemmas = s
            .split_word_bounds()
            .flat_map(|lemma| lemma.split_inclusive('\''))
            .flat_map(|lemma| lemma.split_camel_case_bounds());

        Box::new(lemmas)
    }
}

#[cfg(test)]
mod test {
    use crate::segmenter::test::test_segmenter;

    const TEXT: &str = "The quick (\"brown\") fox can't jump 32.3 feet, right? Brr, it's 29.3°F! camelCase PascalCase IJsland CASE resuméWriter";
    const SEGMENTED: &[&str] = &[
        "The", " ", "quick", " ", "(", "\"", "brown", "\"", ")", " ", "fox", " ", "can'", "t", " ",
        "jump", " ", "32.3", " ", "feet", ",", " ", "right", "?", " ", "Brr", ",", " ", "it'", "s",
        " ", "29.3", "°", "F", "!", " ", "camel", "Case", " ", "Pascal", "Case", " ", "IJsland",
        " ", "CASE", " ", "resumé", "Writer",
    ];
    const TOKENIZED: &[&str] = &[
        "the", " ", "quick", " ", "(", "\"", "brown", "\"", ")", " ", "fox", " ", "can'", "t", " ",
        "jump", " ", "32.3", " ", "feet", ",", " ", "right", "?", " ", "brr", ",", " ", "it'", "s",
        " ", "29.3", "°", "f", "!", " ", "camel", "case", " ", "pascal", "case", " ", "ijsland",
        " ", "case", " ", "resume", "writer",
    ];

    test_segmenter!(LatinSegmenter, TEXT, SEGMENTED, TOKENIZED, Script::Latin, Language::Other);
}
