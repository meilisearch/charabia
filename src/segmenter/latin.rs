use unicode_segmentation::UnicodeSegmentation;
use regex::Regex;

use super::Segmenter;

/// Latin specialized [`Segmenter`].
///
/// This Segmenter uses [`UnicodeSegmentation`] internally to segment the provided text.
pub struct LatinSegmenter;

impl Segmenter for LatinSegmenter {
    fn segment_str<'o>(&self, s: &'o str) -> Box<dyn Iterator<Item = &'o str> + 'o> {
        let re = Regex::new(r"([a-z&])([&A-Z0-9])").unwrap();
        Box::new(s.split_word_bounds().map(|x| re.replace_all(x, "{1} {2}").split(" ")).flatten().map(|lemma| lemma.split_inclusive('\'')).flatten())
    }
}

#[cfg(test)]
mod test {
    use crate::segmenter::test::test_segmenter;

    const TEXT: &str = "The quick (\"brown\") fox can't jump 32.3 feet, right? Brr, it's 29.3°F! camelCase PascalCase IJsland CASE";
    const SEGMENTED: &[&str] = &[
        "The", " ", "quick", " ", "(", "\"", "brown", "\"", ")", " ", "fox", " ", "can'", "t", " ",
        "jump", " ", "32.3", " ", "feet", ",", " ", "right", "?", " ", "Brr", ",", " ", "it'", "s",
        " ", "29.3", "°", "F", "!", " ", "camel", "Case", " ", "Pascal", "Case", " ", "IJsland", " ", "CASE",
    ];
    const TOKENIZED: &[&str] = &[
        "the", " ", "quick", " ", "(", "\"", "brown", "\"", ")", " ", "fox", " ", "can'", "t", " ",
        "jump", " ", "32.3", " ", "feet", ",", " ", "right", "?", " ", "brr", ",", " ", "it'", "s",
        " ", "29.3", "deg", "f", "!", " ", "camel", "case", " ", "pascal", "case", " ", "ijsland", " ", "case",
    ];

    test_segmenter!(LatinSegmenter, TEXT, SEGMENTED, TOKENIZED, Script::Latin, Language::Other);
}
