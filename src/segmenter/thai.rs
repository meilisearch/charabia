// Import `Segmenter` trait.
use fst::raw::Fst;
use once_cell::sync::Lazy;

use crate::segmenter::Segmenter;
use crate::segmenter::utils::FstSegmenter;

/// Thai specialized [`Segmenter`].
///
/// This Segmenter uses a dictionary encoded as an FST to segment the provided text.
/// Dictionary source: PyThaiNLP project on https://github.com/PyThaiNLP/nlpo3
pub struct ThaiSegmenter;

static WORDS_FST: Lazy<Fst<&[u8]>> =
    Lazy::new(|| Fst::new(&include_bytes!("../../dictionaries/fst/thai/words.fst")[..]).unwrap());

static FST_SEGMENTER: Lazy<FstSegmenter> = Lazy::new(|| FstSegmenter::new(&WORDS_FST));

impl Segmenter for ThaiSegmenter {
    fn segment_str<'o>(&self, to_segment: &'o str) -> Box<dyn Iterator<Item = &'o str> + 'o> {
        FST_SEGMENTER.segment_str(to_segment)
    }
}

// Test the segmenter:
#[cfg(test)]
mod test {
    use crate::segmenter::test::test_segmenter;

    const BASE: &str = "ภาษาไทยง่ายนิดเดียว";
    const FIRST_HOMOGRAPH: &str = "ไก่ขันตอนเช้าบนขันน้ำ";
    const SECOND_HOMOGRAPH: &str = "ฉันสระผมที่สระน้ำด้วยน้ำยาสระผม";

    const SEGMENTED_BASE: &[&str] = &["ภาษาไทย", "ง่าย", "นิดเดียว"];
    const SEGMENTED_FIRST_HOMOGRAPH: &[&str] = &["ไก่", "ขัน", "ตอนเช้า", "บน", "ขันน้ำ"];
    const SEGMENTED_SECOND_HOMOGRAPH: &[&str] =
        &["ฉัน", "สระผม", "ที่", "สระน้ำ", "ด้วย", "น้ำยา", "สระผม"];

    const TOKENIZED_BASE: &[&str] = &["ภาษาไทย", "งาย", "นดเดยว"];
    const TOKENIZED_FIRST_HOMOGRAPH: &[&str] = &["ไก", "ขน", "ตอนเชา", "บน", "ขนนำ"];
    const TOKENIZED_SECOND_HOMOGRAPH: &[&str] =
        &["ฉน", "สระผม", "ท", "สระนำ", "ดวย", "นำยา", "สระผม"];
    // Macro that run several tests on the Segmenter.
    test_segmenter!(
        ThaiSegmenter,
        Script::Thai,
        Language::Tha,
        default,
        BASE,
        SEGMENTED_BASE,
        TOKENIZED_BASE,
        first_homograph,
        FIRST_HOMOGRAPH,
        SEGMENTED_FIRST_HOMOGRAPH,
        TOKENIZED_FIRST_HOMOGRAPH,
        second_homograph,
        SECOND_HOMOGRAPH,
        SEGMENTED_SECOND_HOMOGRAPH,
        TOKENIZED_SECOND_HOMOGRAPH
    );
}
