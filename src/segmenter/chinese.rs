use jieba_rs::Jieba;
use once_cell::sync::Lazy;

use crate::segmenter::Segmenter;

/// Chinese Script specialized [`Segmenter`].
///
/// This Segmenter uses [`Jieba`] internally to segment the provided text
/// using HMM feature.
pub struct ChineseSegmenter;

impl Segmenter for ChineseSegmenter {
    fn segment_str<'o>(&self, to_segment: &'o str) -> Box<dyn Iterator<Item = &'o str> + 'o> {
        let segmented = JIEBA.cut(to_segment, true);

        Box::new(segmented.into_iter())
    }
}

static JIEBA: Lazy<Jieba> = Lazy::new(|| Jieba::new());

// Publish the newly implemented Segmenter:
//	   - import module by adding `mod dummy;` (filename) in `segmenter/mod.rs`
//	   - publish Segmenter by adding `pub use dummy::ChineseSegmenter;` in `segmenter/mod.rs`
//     - running `cargo doc --open` you should see your Segmenter in the segmenter module

// Test the segmenter:
#[cfg(test)]
mod test {
    use crate::segmenter::test::test_segmenter;

    // Original version of the text.
    const TEXT: &str =
        "人人生而自由﹐在尊嚴和權利上一律平等。他們賦有理性和良心﹐並應以兄弟關係的精神互相對待。";

    // Segmented version of the text.
    const SEGMENTED: &[&str] = &[
        "人人",
        "生而自由",
        "﹐",
        "在",
        "尊嚴",
        "和",
        "權利上",
        "一律平等",
        "。",
        "他們",
        "賦有",
        "理性",
        "和",
        "良心",
        "﹐",
        "並應",
        "以",
        "兄弟",
        "關",
        "係",
        "的",
        "精神",
        "互相",
        "對待",
        "。",
    ];

    // Segmented and normalized version of the text.
    const TOKENIZED: &[&str] = &[
        "人人",
        "生而自由",
        "﹐",
        "在",
        "尊严",
        "和",
        "权利",
        "上",
        "一律平等",
        "。",
        "他们",
        "赋有",
        "理性",
        "和",
        "良心",
        "﹐",
        "并",
        "应以",
        "兄弟",
        "关系",
        "的",
        "精神",
        "互相",
        "对待",
        "。",
    ];

    // Macro that run several tests on the Segmenter.
    test_segmenter!(ChineseSegmenter, TEXT, SEGMENTED, SEGMENTED, Script::Mandarin, Language::Cmn);
}
