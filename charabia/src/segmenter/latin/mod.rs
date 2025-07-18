#[cfg(feature = "latin-camelcase")]
mod camel_case;

use crate::segmenter::Segmenter;

/// Latin specialized [`Segmenter`].
///
pub struct LatinSegmenter;

impl Segmenter for LatinSegmenter {
    #[cfg(not(feature = "latin-camelcase"))]
    fn segment_str<'o>(&self, s: &'o str) -> Box<dyn Iterator<Item = &'o str> + 'o> {
        Box::new(Some(s).into_iter())
    }

    #[cfg(feature = "latin-camelcase")]
    fn segment_str<'o>(&self, s: &'o str) -> Box<dyn Iterator<Item = &'o str> + 'o> {
        let lemmas = camel_case::split_camel_case_bounds(s);

        Box::new(lemmas)
    }
}

#[cfg(test)]
mod test {
    use crate::segmenter::test::test_segmenter;

    const TEXT: &str =
        "The quick (\"brown\") fox can’t jump 32.3 feet, right? Brr, it's 29.3°F! camelCase kebab-case snake_case 123 456";

    #[rustfmt::skip]
    #[cfg(feature = "latin-camelcase")]
    const SEGMENTED: &[&str] = &[
        "The", " ", "quick", " ", "(", "\"", "brown", "\"", ")", " ", "fox", " ", "can", "’", "t",
        " ", "jump", " ", "32", ".", "3", " ", "feet", ", ", "right", "?", " ", "Brr", ", ", "it",
        "'", "s", " ", "29", ".", "3°F", "!", " ", "camel", "Case", " ", "kebab", "-", "case", " ",
        "snake", "_", "case", " ", "123", " ", "456",
    ];

    #[rustfmt::skip]
    #[cfg(feature = "latin-camelcase")]
    const TOKENIZED: &[&str] = &[
        "the", " ", "quick", " ", "(", "\"", "brown", "\"", ")", " ", "fox", " ", "can", "'", "t",
        " ", "jump", " ", "32", ".", "3", " ", "feet", ", ", "right", "?", " ", "brr", ", ", "it",
        "'", "s", " ", "29", ".", "3°f", "!", " ", "camel", "case", " ", "kebab", "-", "case", " ",
        "snake", "_", "case", " ", "123", " ", "456",
    ];

    #[rustfmt::skip]
    #[cfg(not(feature = "latin-camelcase"))]
    const SEGMENTED: &[&str] = &[
        "The", " ", "quick", " ", "(", "\"", "brown", "\"", ")", " ", "fox", " ", "can", "’", "t",
        " ", "jump", " ", "32", ".", "3", " ", "feet", ", ", "right", "?", " ", "Brr", ", ", "it",
        "'", "s", " ", "29", ".", "3°F", "!", " ", "camelCase", " ", "kebab", "-", "case", " ",
        "snake", "_", "case", " ", "123", " ", "456",
    ];

    #[rustfmt::skip]
    #[cfg(not(feature = "latin-camelcase"))]
    const TOKENIZED: &[&str] = &[
        "the", " ", "quick", " ", "(", "\"", "brown", "\"", ")", " ", "fox", " ", "can", "'", "t",
        " ", "jump", " ", "32", ".", "3", " ", "feet", ", ", "right", "?", " ", "brr", ", ", "it",
        "'", "s", " ", "29", ".", "3°f", "!", " ", "camelcase", " ", "kebab", "-", "case", " ",
        "snake", "_", "case", " ", "123", " ", "456",
    ];

    test_segmenter!(LatinSegmenter, TEXT, SEGMENTED, TOKENIZED, Script::Latin, Language::Eng);
}
