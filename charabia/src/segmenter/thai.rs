// Import `Segmenter` trait.
use fst::raw::Fst;
use once_cell::sync::Lazy;

use crate::segmenter::utils::FstSegmenter;
use crate::segmenter::Segmenter;

/// Thai specialized [`Segmenter`].
///
/// This Segmenter uses a dictionary encoded as an FST to segment the provided text.
/// Dictionary source: PyThaiNLP project on https://github.com/PyThaiNLP/nlpo3
pub struct ThaiSegmenter;

static WORDS_FST: Lazy<Fst<&[u8]>> =
    Lazy::new(|| Fst::new(&include_bytes!("../../dictionaries/fst/thai/words.fst")[..]).unwrap());

static FST_SEGMENTER: Lazy<FstSegmenter> = Lazy::new(|| FstSegmenter::new(&WORDS_FST, None, true));

impl Segmenter for ThaiSegmenter {
    fn segment_str<'o>(&self, to_segment: &'o str) -> Box<dyn Iterator<Item = &'o str> + 'o> {
        FST_SEGMENTER.segment_str(to_segment)
    }
}

// Test the segmenter:
#[cfg(test)]
mod test {
    use crate::segmenter::test::test_segmenter;

    const TEXT: &str = "ภาษาไทยง่ายนิดเดียว ไก่ขันตอนเช้าบนขันน้ำ ฉันสระผมที่สระน้ำด้วยน้ำยาสระผม 123 456";

    const SEGMENTED: &[&str] = &[
        "ภาษาไทย",
        "ง่าย",
        "นิดเดียว",
        " ",
        "ไก่",
        "ขัน",
        "ตอนเช้า",
        "บน",
        "ขันน้ำ",
        " ",
        "ฉัน",
        "สระผม",
        "ที่",
        "สระน้ำ",
        "ด้วย",
        "น้ำยา",
        "สระผม",
        " ",
        "123",
        " ",
        "456",
    ];

    const TOKENIZED: &[&str] = &[
        "ภาษาไทย",
        "งาย",
        "นดเดยว",
        " ",
        "ไก",
        "ขน",
        "ตอนเชา",
        "บน",
        "ขนนา",
        " ",
        "ฉน",
        "สระผม",
        "ท",
        "สระนา",
        "ดวย",
        "นายา",
        "สระผม",
        " ",
        "123",
        " ",
        "456",
    ];
    // Macro that run several tests on the Segmenter.
    test_segmenter!(ThaiSegmenter, TEXT, SEGMENTED, TOKENIZED, Script::Thai, Language::Tha);
}
