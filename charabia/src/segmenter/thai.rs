use std::sync::LazyLock;

use fst::raw::Fst;

// Import `Segmenter` trait.
use crate::segmenter::Segmenter;

const MAI_HAN_AKAT: char = '\u{e31}';
const THANTHAKHAT: char = '\u{e4c}';

/// Thai specialized [`Segmenter`].
///
/// This Segmenter uses a dictionary encoded as an FST to segment the provided text.
/// Dictionary source: PyThaiNLP project on https://github.com/PyThaiNLP/nlpo3
///
/// The text is segmented by maximal matching: among the possible segmentations,
/// the one with the fewest characters outside of dictionary words is chosen,
/// then the one with the fewest segments.
/// The text is only cut between Thai character clusters, so a vowel or a tone mark
/// is never separated from its consonant.
/// Consecutive characters outside of dictionary words are kept together in a single segment.
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
    /// whether the last segment is a dictionary word.
    known: bool,
}

impl Step {
    /// The offset has not been reached.
    const UNREACHED: Step =
        Step { breakable: false, cost: (usize::MAX, usize::MAX), start: 0, known: false };
}

/// Returns the byte offsets of the segment boundaries of the text, including `0` and `text.len()`.
fn segment_boundaries(fst: &Fst<&[u8]>, text: &str) -> Vec<usize> {
    let mut steps = vec![Step::UNREACHED; text.len() + 1];
    mark_breakable(text, &mut steps);
    steps[0].cost = (0, 0);

    find_best_segmentation(fst, text, &mut steps);

    // backtrack the best segmentation, merging consecutive unknown segments of the same kind.
    let mut boundaries = vec![text.len()];
    let mut next_unknown_kind = None;
    let mut end = text.len();
    while end > 0 {
        let Step { start, known, .. } = steps[end];
        let unknown_kind = (!known).then(|| {
            let c = text[start..].chars().next().unwrap();
            (is_thai_letter(c), is_numeric(c))
        });
        if unknown_kind.is_some() && unknown_kind == next_unknown_kind {
            boundaries.pop();
        }
        boundaries.push(start);
        next_unknown_kind = unknown_kind;
        end = start;
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

        let mut relax = |end: usize, unknown: usize, known: bool| {
            let cost = (unknown_chars + unknown, segments + 1);
            // on equal cost, prefer the latest start, i.e. the longest preceding segments.
            if steps[end].breakable && cost <= steps[end].cost {
                steps[end] = Step { cost, start, known, ..steps[end] };
            }
        };

        for_each_dictionary_match(fst, &text[start..], |len| relax(start + len, 0, true));

        relax(cluster_end, text[start..cluster_end].chars().count(), false);
    }
}

/// Marks the offsets where the text can be cut, which are the ends of the character clusters.
fn mark_breakable(text: &str, steps: &mut [Step]) {
    // the chars preceding the current one, the closest first.
    let mut previous = ['\0'; 4];
    for (i, current) in text.char_indices() {
        let [p1, p2, p3, p4] = previous;

        // a leading vowel is always followed by its consonant.
        let after_leading_vowel = is_leading_vowel(p1);
        // Mai Han-Akat is always followed by a final consonant (e.g. กัน).
        let after_mai_han_akat = p1 == MAI_HAN_AKAT;
        // a consonant with a tone mark and without vowel is followed by its vowel or its
        // final consonant (e.g. ก่อน, ก้น), unlike after a leading vowel (e.g. ไม่|มี, ไหว้|วัน).
        let after_toned_consonant = matches!(p1, '\u{e48}'..='\u{e4b}')
            && is_thai_consonant(p2)
            && !is_leading_vowel(p3)
            && !(is_thai_consonant(p3) && is_leading_vowel(p4));
        // vowels, tone marks and diacritics are always attached to the preceding character.
        let before_dependent_char =
            matches!(current, '\u{e30}'..='\u{e3a}' | '\u{e45}' | '\u{e47}'..='\u{e4e}');
        // numbers are not split.
        let inside_number = is_numeric(p1) && is_numeric(current);

        steps[i].breakable = !(after_leading_vowel
            || after_mai_han_akat
            || after_toned_consonant
            || before_dependent_char
            || inside_number);

        // a silent consonant belongs to the preceding syllable (e.g. สัมพันธ์, ศักดิ์).
        if current == THANTHAKHAT {
            if is_thai_consonant(p1) {
                steps[i - p1.len_utf8()].breakable = false;
            } else if matches!(p1, '\u{e34}' | '\u{e38}') && is_thai_consonant(p2) {
                steps[i - p1.len_utf8() - p2.len_utf8()].breakable = false;
            }
        }

        previous = [current, p1, p2, p3];
    }

    // the text can always be cut at its start and at its end.
    steps[0].breakable = true;
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

fn is_leading_vowel(c: char) -> bool {
    matches!(c, '\u{e40}'..='\u{e44}')
}

fn is_thai_consonant(c: char) -> bool {
    matches!(c, '\u{e01}'..='\u{e2e}')
}

/// Same as [`char::is_numeric`], without looking up the Unicode tables for the Thai chars,
/// which are most of the chars of the text.
fn is_numeric(c: char) -> bool {
    match c {
        '0'..='9' | '\u{e50}'..='\u{e59}' => true,
        '\u{e00}'..='\u{e7f}' => false,
        c => c.is_numeric(),
    }
}

/// Returns `true` if the char is a Thai consonant, vowel, tone mark or diacritic.
fn is_thai_letter(c: char) -> bool {
    matches!(c, '\u{e01}'..='\u{e2e}' | '\u{e30}'..='\u{e3a}' | '\u{e40}'..='\u{e45}' | '\u{e47}'..='\u{e4e}')
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

    #[test]
    fn vowels_and_tone_marks_are_not_split_from_their_consonant() {
        for text in ["ช้อปปี้", "ฟ่อนฟั่น", "ก่อนก้นกิ่ง", "สัมพันธ์ศักดิ์"]
        {
            let segments = segment_thai(text);
            assert_eq!(segments.concat(), text);
            for segment in &segments[1..] {
                let first = segment.chars().next().unwrap();
                assert!(
                    !matches!(first, '\u{e30}'..='\u{e3a}' | '\u{e45}' | '\u{e47}'..='\u{e4e}'),
                    "{segments:?} starts a segment with a vowel or a diacritic"
                );
            }
        }
        assert_eq!(segment_thai("ก่อน"), ["ก่อน"]);
        assert_eq!(segment_thai("สัมพันธ์"), ["สัมพันธ์"]);
        assert_eq!(segment_thai("ไม่มีไหว้วัน"), ["ไม่", "มี", "ไหว้", "วัน"]);
    }

    #[test]
    fn unknown_words_are_not_split_into_chars() {
        assert_eq!(segment_thai("ฟฟฟฟฟ"), ["ฟฟฟฟฟ"]);
        assert_eq!(segment_thai("ช้อปปี้"), ["ช้อป", "ปี้"]);
    }

    #[test]
    fn numbers_are_not_split() {
        assert_eq!(segment_thai("ปี2024ราคา๑๒๓บาท"), ["ปี", "2024", "ราคา", "๑๒๓", "บาท"]);
    }
}
