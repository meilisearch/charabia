#[cfg(feature = "japanese-segmentation-ipadic")]
use lindera::Penalty;
use lindera::{
    dictionary::{load_dictionary_from_kind, DictionaryKind},
    mode::{Mode, Penalty},
    segmenter::Segmenter as LinderaSegmenter,
    tokenizer::Tokenizer,
};
use once_cell::sync::Lazy;

use crate::segmenter::Segmenter;

/// Japanese specialized [`Segmenter`].
///
/// This Segmenter uses lindera internally to segment the provided text.
pub struct JapaneseSegmenter;

static LINDERA: Lazy<Tokenizer> = Lazy::new(|| {
    #[cfg(all(feature = "japanese-segmentation-ipadic", feature = "japanese-segmentation-unidic"))]
    compile_error!("Feature japanese-segmentation-ipadic and japanese-segmentation-unidic are mutually exclusive and cannot be enabled together");

    #[cfg(feature = "japanese-segmentation-ipadic")]
    {
        let dictionary = load_dictionary_from_kind(DictionaryKind::IPADIC).unwrap();
        let segmenter =
            LinderaSegmenter::new(Mode::Decompose(Penalty::default()), dictionary, None);
        Tokenizer::new(segmenter)
    }
    #[cfg(feature = "japanese-segmentation-unidic")]
    {
        let dictionary = load_dictionary_from_kind(DictionaryKind::UniDic).unwrap();
        let segmenter =
            LinderaSegmenter::new(Mode::Decompose(Penalty::default()), dictionary, None);
        Tokenizer::new(segmenter)
    }
});

impl Segmenter for JapaneseSegmenter {
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
