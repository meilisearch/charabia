use lindera_core::mode::Mode;
#[cfg(feature = "japanese-segmentation-ipadic")]
use lindera_core::mode::Penalty;
use lindera_dictionary::{DictionaryConfig, DictionaryKind};
use lindera_tokenizer::tokenizer::{Tokenizer, TokenizerConfig};
use once_cell::sync::Lazy;

use crate::segmenter::Segmenter;

/// Japanese specialized [`Segmenter`].
///
/// This Segmenter uses lindera internally to segment the provided text.
pub struct JapaneseSegmenter;

static LINDERA: Lazy<Tokenizer> = Lazy::new(|| {
    #[cfg(feature = "japanese-segmentation-ipadic")]
    let config = TokenizerConfig {
        dictionary: DictionaryConfig { kind: Some(DictionaryKind::IPADIC), path: None },
        mode: Mode::Decompose(Penalty::default()),
        ..TokenizerConfig::default()
    };
    #[cfg(feature = "japanese-segmentation-unidic")]
    let config = TokenizerConfig {
        dictionary: DictionaryConfig { kind: Some(DictionaryKind::UniDic), path: None },
        mode: Mode::Normal,
        ..TokenizerConfig::default()
    };
    Tokenizer::from_config(config).unwrap()
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

    #[cfg(feature = "japanese-segmentation-ipadic")]
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
    #[cfg(feature = "japanese-segmentation-unidic")]
    const SEGMENTED: &[&str] = &[
        "関西",
        "国際",
        "空港",
        "限定",
        "トート",
        "バッグ",
        " ",
        "すもも",
        "も",
        "もも",
        "も",
        "もも",
        "の",
        "うち",
    ];

    #[cfg(feature = "japanese-segmentation-ipadic")]
    const TOKENIZED: &[&str] = &[
        "関西",
        "国際",
        "空港",
        "限定",
        // Use "とうとばっぐ" instead when feature "japanese-transliteration" is enabled or become default
        #[cfg(feature = "japanese-transliteration")]
        "とうとは\u{3099}っく\u{3099}",
        #[cfg(not(feature = "japanese-transliteration"))]
        "トートハ\u{3099}ック\u{3099}",
        " ",
        "すもも",
        "も",
        "もも",
        "も",
        "もも",
        "の",
        "うち",
    ];
    #[cfg(feature = "japanese-segmentation-unidic")]
    const TOKENIZED: &[&str] = &[
        "関西",
        "国際",
        "空港",
        "限定",
        // Use "とうとばっぐ" instead when feature "japanese-transliteration" is enabled or become default
        #[cfg(feature = "japanese-transliteration")]
        "とうと",
        #[cfg(not(feature = "japanese-transliteration"))]
        "トート",
        #[cfg(feature = "japanese-transliteration")]
        "は\u{3099}っく\u{3099}",
        #[cfg(not(feature = "japanese-transliteration"))]
        "ハ\u{3099}ック\u{3099}",
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
