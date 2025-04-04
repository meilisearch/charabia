#[cfg(feature = "japanese-segmentation-external")]
use std::{env, path::PathBuf};

#[cfg(not(feature = "japanese-segmentation-external"))]
use lindera::DictionaryKind;
#[cfg(feature = "japanese-segmentation-ipadic")]
use lindera::Penalty;
use lindera::{DictionaryConfig, Mode, Tokenizer, TokenizerConfig};
use once_cell::sync::Lazy;

use crate::segmenter::Segmenter;

/// Japanese specialized [`Segmenter`].
///
/// This Segmenter uses lindera internally to segment the provided text.
pub struct JapaneseSegmenter;

static LINDERA: Lazy<Tokenizer> = Lazy::new(|| {
    #[cfg(all(feature = "japanese-segmentation-ipadic", feature = "japanese-segmentation-unidic"))]
    compile_error!("Feature japanese-segmentation-ipadic and japanese-segmentation-unidic are mutually exclusive and cannot be enabled together");

    #[cfg(all(
        feature = "japanese-segmentation-external",
        any(feature = "japanese-segmentation-ipadic", feature = "japanese-segmentation-unidic")
    ))]
    compile_error!("Feature japanese-segmentation-external and either japanese-segmentation-unidic or japanese-segmentation-ipadic are mutually exclusive and cannot be enabled together");

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
    #[cfg(feature = "japanese-segmentation-external")]
    let config = TokenizerConfig {
        dictionary: DictionaryConfig { kind: None, path: Some(PathBuf::from(env::var("MEILISEARCH_JAPANESE_EXTERNAL_DICTIONARY").expect("japanese-segmentation-external feature requires MEILISEARCH_JAPANESE_EXTERNAL_DICTIONARY env var to be set"))) },
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
#[cfg(not(feature = "japanese-segmentation-external"))]
mod test {
    use crate::segmenter::test::test_segmenter;

    const TEXT: &str = "関西国際空港限定トートバッグ すもももももももものうち 123 456";

    const SEGMENTED: &[&str] = if cfg!(feature = "japanese-segmentation-ipadic") {
        &[
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
            " ",
            "123",
            " ",
            "456",
        ]
    } else if cfg!(feature = "japanese-segmentation-unidic") {
        &[
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
            " ",
            "123",
            " ",
            "456",
        ]
    } else {
        &[]
    };

    const TOKENIZED: &[&str] = if cfg!(feature = "japanese-segmentation-ipadic") {
        &[
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
            " ",
            "123",
            " ",
            "456",
        ]
    } else if cfg!(feature = "japanese-segmentation-unidic") {
        &[
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
            " ",
            "123",
            " ",
            "456",
        ]
    } else {
        &[]
    };

    #[cfg(all(feature = "japanese-segmentation-ipadic", feature = "japanese-segmentation-unidic"))]
    compile_error!("Feature japanese-segmentation-ipadic and japanese-segmentation-unidic are mutually exclusive and cannot be enabled together");

    // Macro that run several tests on the Segmenter.
    test_segmenter!(JapaneseSegmenter, TEXT, SEGMENTED, TOKENIZED, Script::Cj, Language::Jpn);
}
