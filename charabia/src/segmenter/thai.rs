use std::sync::LazyLock;

use fst::raw::Fst;

// Import `Segmenter` trait.
use crate::segmenter::Segmenter;

/// Thai specialized [`Segmenter`].
///
/// This Segmenter uses a dictionary encoded as an FST to segment the provided text.
/// Dictionary source: PyThaiNLP project on https://github.com/PyThaiNLP/nlpo3
///
/// The text is segmented by maximal matching: among the possible segmentations,
/// the one with the fewest characters outside of dictionary words is chosen,
/// then the one with the fewest segments.
pub struct ThaiSegmenter;

static WORDS_FST: LazyLock<Fst<&[u8]>> = LazyLock::new(|| {
    Fst::new(&include_bytes!("../../dictionaries/fst/thai/words.fst")[..]).unwrap()
});

impl Segmenter for ThaiSegmenter {
    fn segment_str<'o>(&self, to_segment: &'o str) -> Box<dyn Iterator<Item = &'o str> + 'o> {
        let boundaries = segment_boundaries(&WORDS_FST, to_segment);
        Box::new((1..boundaries.len()).map(move |i| &to_segment[boundaries[i - 1]..boundaries[i]]))
    }
}

/// Best way found to segment the text preceding a byte offset.
#[derive(Debug, Clone, Copy)]
struct Step {
    /// whether the text can be cut at this offset.
    breakable: bool,
    /// cost of the segmentation, the lower the better: the number of chars that are not part
    /// of a dictionary word, then the number of segments.
    cost: (usize, usize),
    /// offset of the start of the last segment.
    start: usize,
}

impl Step {
    /// The offset has not been reached.
    const UNREACHED: Step = Step { breakable: false, cost: (usize::MAX, usize::MAX), start: 0 };
}

/// Returns the byte offsets of the segment boundaries of the text, including `0` and `text.len()`.
fn segment_boundaries(fst: &Fst<&[u8]>, text: &str) -> Vec<usize> {
    let mut steps = vec![Step::UNREACHED; text.len() + 1];
    mark_breakable(text, &mut steps);
    steps[0].cost = (0, 0);

    find_best_segmentation(fst, text, &mut steps);

    // backtrack the best segmentation.
    let mut boundaries = vec![text.len()];
    let mut end = text.len();
    while end > 0 {
        end = steps[end].start;
        boundaries.push(end);
    }
    boundaries.reverse();

    boundaries
}

/// Computes, for each offset where the text can be cut, the best segmentation up to it.
///
/// The segments are dictionary words, or the next character cluster.
fn find_best_segmentation(fst: &Fst<&[u8]>, text: &str, steps: &mut [Step]) {
    for start in 0..text.len() {
        let Step { cost: (unknown_chars, segments), .. } = steps[start];
        if unknown_chars == usize::MAX {
            continue;
        }

        // the text can always be cut at the end of the next character cluster.
        let cluster_end = (start + 1..=text.len()).find(|&i| steps[i].breakable).unwrap();

        let mut relax = |end: usize, unknown: usize| {
            let cost = (unknown_chars + unknown, segments + 1);
            // on equal cost, prefer the latest start, i.e. the longest preceding segments.
            if steps[end].breakable && cost <= steps[end].cost {
                steps[end] = Step { cost, start, ..steps[end] };
            }
        };

        for_each_dictionary_match(fst, &text[start..], |len| relax(start + len, 0));

        relax(cluster_end, text[start..cluster_end].chars().count());
    }
}

/// Marks the offsets where the text can be cut.
fn mark_breakable(text: &str, steps: &mut [Step]) {
    for (i, _) in text.char_indices() {
        steps[i].breakable = true;
    }
    steps[text.len()].breakable = true;
}

/// Calls `f` with the length, in bytes, of each dictionary word that is a prefix of the text.
fn for_each_dictionary_match(fst: &Fst<&[u8]>, text: &str, mut f: impl FnMut(usize)) {
    let mut node = fst.root();
    let mut buffer = [0; 4];
    let mut chars = text.chars();
    while let Some(c) = chars.next() {
        for &byte in c.encode_utf8(&mut buffer).as_bytes() {
            let Some(transition) = node.find_input(byte) else { return };
            node = fst.node(node.transition_addr(transition));
        }
        if node.is_final() {
            f(text.len() - chars.as_str().len());
        }
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

    // the normalization keeps the Thai vowels and tone marks.
    const TOKENIZED: &[&str] = SEGMENTED;

    // Macro that run several tests on the Segmenter.
    test_segmenter!(ThaiSegmenter, TEXT, SEGMENTED, TOKENIZED, Script::Thai, Language::Tha);

    fn segment_thai(text: &str) -> Vec<&str> {
        ThaiSegmenter.segment_str(text).collect()
    }

    #[test]
    fn maximal_matching_minimizes_the_chars_outside_of_the_dictionary() {
        assert_eq!(segment_thai("ดีดู"), ["ดี", "ดู"]);
        assert_eq!(segment_thai("หนอนัดดา"), ["หนอ", "นัดดา"]);
        assert_eq!(segment_thai("ผลอภิชน"), ["ผล", "อภิชน"]);
        assert_eq!(segment_thai("ไม่มีใครรู้"), ["ไม่", "มี", "ใคร", "รู้"]);
    }

    #[test]
    fn segments_cover_the_whole_text() {
        for text in ["", "a", "ๆๆ", "เ", "ั", "กกกกกกกกกกกกกกกกก", "ภาษาไทย123abc"]
        {
            assert_eq!(segment_thai(text).concat(), text);
        }
    }
}
