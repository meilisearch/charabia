use unicode_segmentation::UnicodeSegmentation;

use super::Segmenter;

/// Hebrew specialized [`Segmenter`].
///
/// This Segmenter uses [`UnicodeSegmentation`] internally to segment the provided text.
pub struct HebrewSegmenter;

impl Segmenter for HebrewSegmenter {
    fn segment_str<'o>(&self, s: &'o str) -> Box<dyn Iterator<Item = &'o str> + 'o> {
        Box::new(s.split_word_bounds().flat_map(|lemma| lemma.split_inclusive('\'')))
    }
}

#[cfg(test)]
mod test {
    use crate::segmenter::test::test_segmenter;

    const TEXT: &str = "הַשּׁוּעָל הַמָּהִיר (״הַחוּם״) לֹא יָכוֹל לִקְפֹּץ 8.94 מֶטְרִים, נָכוֹן? ברר, 1.5°C- בַּחוּץ!";
    const SEGMENTED: &[&str] = &[
        "הַשּׁוּעָל",
        " ",
        "הַמָּהִיר",
        " ",
        "(",
        "״",
        "הַחוּם",
        "״",
        ")",
        " ",
        "לֹא",
        " ",
        "יָכוֹל",
        " ",
        "לִקְפֹּץ",
        " ",
        "8.94",
        " ",
        "מֶטְרִים",
        ",",
        " ",
        "נָכוֹן",
        "?",
        " ",
        "ברר",
        ",",
        " ",
        "1.5",
        "°",
        "C",
        "-",
        " ",
        "בַּחוּץ",
        "!",
    ];
    const TOKENIZED: &[&str] = &[
        "השועל",
        " ",
        "המהיר",
        " ",
        "(",
        "״",
        "החום",
        "״",
        ")",
        " ",
        "לא",
        " ",
        "יכול",
        " ",
        "לקפץ",
        " ",
        "8.94",
        " ",
        "מטרים",
        ",",
        " ",
        "נכון",
        "?",
        " ",
        "ברר",
        ",",
        " ",
        "1.5",
        "deg",
        "c",
        "-",
        " ",
        "בחוץ",
        "!",
    ];

    test_segmenter!(HebrewSegmenter, TEXT, SEGMENTED, TOKENIZED, Script::Hebrew, Language::Heb);
}
