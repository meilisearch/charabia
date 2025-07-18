use lindera::{
    dictionary::{load_dictionary_from_kind, DictionaryKind},
    mode::{Mode, Penalty},
    segmenter::Segmenter as LinderaSegmenter,
    tokenizer::Tokenizer,
};
use once_cell::sync::Lazy;

use crate::segmenter::Segmenter;

/// Korean specialized [`Segmenter`].
///
/// This Segmenter uses lindera internally to segment the provided text.
pub struct KoreanSegmenter;

static LINDERA: Lazy<Tokenizer> = Lazy::new(|| {
    let dictionary = load_dictionary_from_kind(DictionaryKind::KoDic).unwrap();
    let segmenter = LinderaSegmenter::new(Mode::Decompose(Penalty::default()), dictionary, None);
    Tokenizer::new(segmenter)
});

impl Segmenter for KoreanSegmenter {
    fn segment_str<'o>(&self, to_segment: &'o str) -> Box<dyn Iterator<Item = &'o str> + 'o> {
        let tokens = LINDERA.tokenize(to_segment).unwrap();

        let result: Vec<&'o str> = tokens
            .into_iter()
            .map(|token| {
                let start = token.byte_start;
                let end = token.byte_end;
                &to_segment[start..end]
            })
            .collect();

        Box::new(result.into_iter())
    }
}

#[cfg(test)]
mod test {
    use crate::segmenter::test::test_segmenter;

    const TEXT: &str = "한국어의형태해석을실시할수있습니다 123 456.";

    const SEGMENTED: &[&str] = &[
        "한국어",
        "의",
        "형태",
        "해석",
        "을",
        "실시",
        "할",
        "수",
        "있",
        "습니다",
        " ",
        "123",
        " ",
        "456",
        ".",
    ];

    const TOKENIZED: &[&str] = &[
        "한국어",
        "의",
        "형태",
        "해석",
        "을",
        "실시",
        "할",
        "수",
        "있",
        "습니다",
        " ",
        "123",
        " ",
        "456",
        ".",
    ];

    // Macro that run several tests on the Segmenter.
    test_segmenter!(KoreanSegmenter, TEXT, SEGMENTED, TOKENIZED, Script::Hangul, Language::Kor);
}
