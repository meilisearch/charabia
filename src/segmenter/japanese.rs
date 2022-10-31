use lindera::mode::{Mode, Penalty};
use lindera::tokenizer::{Tokenizer, TokenizerConfig};
use once_cell::sync::Lazy;

use crate::segmenter::Segmenter;

/// Japanese specialized [`Segmenter`].
///
/// This Segmenter uses lindera internally to segment the provided text.
pub struct JapaneseSegmenter;

static LINDERA: Lazy<Tokenizer> = Lazy::new(|| {
    let config =
        TokenizerConfig { mode: Mode::Decompose(Penalty::default()), ..TokenizerConfig::default() };
    Tokenizer::with_config(config).unwrap()
});

impl Segmenter for JapaneseSegmenter {
    fn segment_str<'o>(&self, to_segment: &'o str) -> Box<dyn Iterator<Item = &'o str> + 'o> {
        let segment_iterator = LINDERA.tokenize(to_segment).unwrap();
        Box::new(segment_iterator.into_iter().map(|token| token.text))
    }
}

#[cfg(test)]
mod test {
    use crate::segmenter::test::test_segmenter;

    const TEXT: &str = "関西国際空港限定トートバッグ すもももももももものうち";

    const SEGMENTED: &[&str] = &[
        "関西",
        "国際",
        "空港",
        "限定",
        "トートバッグ",
        " ",
        "すもも",
        "も",
        "もも",
        "も",
        "もも",
        "の",
        "うち",
    ];

    const TOKENIZED: &[&str] = &[
        "関西",
        "国際",
        "空港",
        "限定",
        // Use "とうとばっぐ" instead when feature "japanese-transliteration" is enabled or become default
        #[cfg(feature = "japanese-transliteration")]
        "とうとばっぐ",
        #[cfg(not(feature = "japanese-transliteration"))]
        "トートバッグ",
        " ",
        "すもも",
        "も",
        "もも",
        "も",
        "もも",
        "の",
        "うち",
    ];

    // Macro that run several tests on the Segmenter.
    test_segmenter!(JapaneseSegmenter, TEXT, SEGMENTED, TOKENIZED, Script::Cj, Language::Jpn);
}
