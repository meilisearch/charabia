#[cfg(feature = "latin-camelcase")]
mod camel_case;
#[cfg(feature = "latin-snakecase")]
mod snake_case;

use unicode_segmentation::UnicodeSegmentation;

use crate::segmenter::Segmenter;

/// Latin specialized [`Segmenter`].
///
/// This Segmenter uses [`UnicodeSegmentation`] internally to segment the provided text.
pub struct LatinSegmenter;

impl Segmenter for LatinSegmenter {
    #[cfg(not(feature = "latin-camelcase"))]
    fn segment_str<'o>(&self, s: &'o str) -> Box<dyn Iterator<Item = &'o str> + 'o> {
        let lemmas = s.split_word_bounds().flat_map(|lemma| lemma.split_inclusive('\''));
        Box::new(lemmas)
    }

    #[cfg(feature = "latin-camelcase")]
    fn segment_str<'o>(&self, s: &'o str) -> Box<dyn Iterator<Item = &'o str> + 'o> {
        let lemmas =
            s.split_word_bounds().flat_map(|lemma| lemma.split_inclusive(['\'', '’', '‘', '‛']));

        #[cfg(feature = "latin-camelcase")]
        let lemmas = lemmas.flat_map(camel_case::split_camel_case_bounds);

        #[cfg(feature = "latin-snakecase")]
        let lemmas = lemmas.flat_map(snake_case::split_snake_case_bounds);

        Box::new(lemmas)
    }
}

#[cfg(test)]
mod test {
    use crate::segmenter::test::test_segmenter;

    const TEXT: &str =
        "The quick (\"brown\") fox can’t jump 32.3 feet, right? Brr, it's 29.3°F! camelCase kebab-case snake_case";
    const SEGMENTED: &[&str] = &[
        "The", " ", "quick", " ", "(", "\"", "brown", "\"", ")", " ", "fox", " ", "can’", "t", " ",
        "jump", " ", "32.3", " ", "feet", ",", " ", "right", "?", " ", "Brr", ",", " ", "it'", "s",
        " ", "29.3", "°", "F", "!", " ", "camel", "Case", " ", "kebab", "-", "case", " ", "snake",
        "_", "case",
    ];
    const TOKENIZED: &[&str] = &[
        "the", " ", "quick", " ", "(", "\"", "brown", "\"", ")", " ", "fox", " ", "can'", "t", " ",
        "jump", " ", "32.3", " ", "feet", ",", " ", "right", "?", " ", "brr", ",", " ", "it'", "s",
        " ", "29.3", "°", "f", "!", " ", "camel", "case", " ", "kebab", "-", "case", " ", "snake",
        "_", "case",
    ];

    test_segmenter!(LatinSegmenter, TEXT, SEGMENTED, TOKENIZED, Script::Latin, Language::Other);
}
