use std::sync::LazyLock;

use fst::raw::Fst;

// Import `Segmenter` trait.
use crate::segmenter::Segmenter;

const NIKHAHIT: char = '\u{e4d}';
const SARA_AA: char = '\u{e32}';
const SARA_AM: char = '\u{e33}';
const THANTHAKHAT: char = '\u{e4c}';

/// Thai specialized [`Segmenter`].
///
/// This Segmenter uses a dictionary encoded as an FST to segment the provided text.
/// Dictionary source: PyThaiNLP project on https://github.com/PyThaiNLP/nlpo3
///
/// The text is segmented by maximal matching: the text is only cut between Thai character
/// clusters, so a vowel or a tone mark is never separated from its consonant, and among
/// the possible segmentations, the one with the fewest characters outside of dictionary
/// words is chosen, then the one with the fewest segments.
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

/// Cost of a segmentation, the lower the better.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Cost {
    /// number of characters that are not part of a dictionary word.
    unknown_chars: usize,
    /// number of segments.
    segments: usize,
}

/// Best way found to segment the text up to a given position.
#[derive(Debug, Clone, Copy)]
struct Step {
    cost: Cost,
    /// position, in chars, of the start of the last segment.
    start: usize,
    /// whether the last segment is a dictionary word.
    known: bool,
}

/// Returns the byte offsets of the segment boundaries of the text, including `0` and `text.len()`.
fn segment_boundaries(fst: &Fst<&[u8]>, text: &str) -> Vec<usize> {
    let chars: Vec<(usize, char)> = text.char_indices().collect();
    let byte_offset = |i: usize| chars.get(i).map_or(text.len(), |(offset, _)| *offset);
    let breakable: Vec<bool> = (0..=chars.len()).map(|i| is_breakable(&chars, i)).collect();

    // steps[i] is the best segmentation of the i first chars, if the text can be cut before the i-th char.
    let mut steps: Vec<Option<Step>> = vec![None; chars.len() + 1];
    steps[0] = Some(Step { cost: Cost::default(), start: 0, known: true });
    for start in 0..chars.len() {
        let Some(Step { cost, .. }) = steps[start] else { continue };
        let mut relax = |end: usize, known: bool| {
            let cost = Cost {
                unknown_chars: cost.unknown_chars + if known { 0 } else { end - start },
                segments: cost.segments + 1,
            };
            // on equal cost, prefer the latest start, i.e. the longest preceding segments.
            if steps[end].is_none_or(|step| cost <= step.cost) {
                steps[end] = Some(Step { cost, start, known });
            }
        };

        for_each_dictionary_match(fst, &chars[start..], |len| {
            if breakable[start + len] {
                relax(start + len, true);
            }
        });

        // fallback on the next character cluster, the text can always be cut at its end.
        let next = (start + 1..=chars.len()).find(|&i| breakable[i]).unwrap_or(chars.len());
        relax(next, false);
    }

    // backtrack the best segmentation.
    let mut segments = Vec::new();
    let mut end = chars.len();
    while let Some(step) = steps[end].filter(|_| end > 0) {
        segments.push((step.start, end, step.known));
        end = step.start;
    }

    // merge consecutive unknown segments of the same kind.
    let mut boundaries = vec![0];
    let mut previous_unknown_kind = None;
    for (start, end, known) in segments.into_iter().rev() {
        let unknown_kind = (!known).then(|| {
            let c = chars[start].1;
            (is_thai_letter(c), c.is_numeric())
        });
        match boundaries.last_mut() {
            Some(last) if unknown_kind.is_some() && unknown_kind == previous_unknown_kind => {
                *last = byte_offset(end)
            }
            _ => boundaries.push(byte_offset(end)),
        }
        previous_unknown_kind = unknown_kind;
    }

    boundaries
}

/// Calls `f` with the length, in chars, of each dictionary word that is a prefix of `chars`.
///
/// Nikhahit (U+0E4D) followed by Sara Aa (U+0E32) is matched as Sara Am (U+0E33),
/// because Sara Am is often typed this way.
fn for_each_dictionary_match(fst: &Fst<&[u8]>, chars: &[(usize, char)], mut f: impl FnMut(usize)) {
    let mut node = fst.root();
    let mut buffer = [0; 4];
    let mut len = 0;
    while let Some(&(_, c)) = chars.get(len) {
        let (c, char_count) = match (c, chars.get(len + 1)) {
            (NIKHAHIT, Some((_, SARA_AA))) => (SARA_AM, 2),
            (c, _) => (c, 1),
        };
        for &byte in c.encode_utf8(&mut buffer).as_bytes() {
            let Some(transition) = node.find_input(byte) else { return };
            node = fst.node(node.transition_addr(transition));
        }
        len += char_count;
        if node.is_final() {
            f(len);
        }
    }
}

/// Returns `true` if the text can be cut before the `i`-th char.
fn is_breakable(chars: &[(usize, char)], i: usize) -> bool {
    let char_at = |i: usize| chars.get(i).map(|(_, c)| *c);
    let (Some(previous), Some(current)) = (i.checked_sub(1).and_then(char_at), char_at(i)) else {
        return true;
    };

    // a leading vowel is always followed by its consonant.
    let after_leading_vowel = is_leading_vowel(previous);
    // Mai Han-Akat is always followed by a final consonant (e.g. กัน).
    let after_mai_han_akat = previous == '\u{e31}';
    // a consonant with a tone mark and without vowel is followed by its vowel or its
    // final consonant (e.g. ก่อน, ก้น), unlike after a leading vowel (e.g. ไม่|มี, ไหว้|วัน).
    let after_toned_consonant = matches!(previous, '\u{e48}'..='\u{e4b}') && {
        let before = |n: usize| i.checked_sub(n).and_then(char_at);
        before(2).is_some_and(is_thai_consonant)
            && !before(3).is_some_and(is_leading_vowel)
            && !(before(3).is_some_and(is_thai_consonant)
                && before(4).is_some_and(is_leading_vowel))
    };
    // vowels, tone marks and diacritics are always attached to the preceding character.
    let before_dependent_char =
        matches!(current, '\u{e30}'..='\u{e3a}' | '\u{e45}' | '\u{e47}'..='\u{e4e}');
    // a silent consonant belongs to the preceding syllable (e.g. สัมพันธ์, ศักดิ์).
    let before_silent_consonant = is_thai_consonant(current)
        && matches!(
            (char_at(i + 1), char_at(i + 2)),
            (Some(THANTHAKHAT), _) | (Some('\u{e34}' | '\u{e38}'), Some(THANTHAKHAT))
        );
    // numbers are not split.
    let inside_number = previous.is_numeric() && current.is_numeric();

    !(after_leading_vowel
        || after_mai_han_akat
        || after_toned_consonant
        || before_dependent_char
        || before_silent_consonant
        || inside_number)
}

fn is_leading_vowel(c: char) -> bool {
    matches!(c, '\u{e40}'..='\u{e44}')
}

fn is_thai_consonant(c: char) -> bool {
    matches!(c, '\u{e01}'..='\u{e2e}')
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

    // TOKENIZED contains the expected output after the full normalization pipeline.
    //
    // All Thai combining marks are preserved (ThaiNormalizer prevents stripping):
    // - Vowels (e.g. ิ ั ี ื ุ ู ็): preserved
    // - Tone marks (e.g. ่ ้ ๊ ๋): preserved
    // - Silence mark ์: preserved
    //
    // Sara Am (ำ, U+0E33) is preserved as-is (ThaiNormalizer recomposes it back
    // after CompatibilityDecompositionNormalizer may have split it).
    const TOKENIZED: &[&str] = &[
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
    // Macro that run several tests on the Segmenter.
    test_segmenter!(ThaiSegmenter, TEXT, SEGMENTED, TOKENIZED, Script::Thai, Language::Tha);

    fn segment_thai(text: &str) -> Vec<&str> {
        ThaiSegmenter.segment_str(text).collect()
    }

    #[test]
    fn vowels_and_tone_marks_are_not_split_from_their_consonant() {
        assert_eq!(segment_thai("ดีดู"), ["ดี", "ดู"]);
        assert_eq!(segment_thai("หนอนัดดา"), ["หนอ", "นัดดา"]);
        assert_eq!(segment_thai("ผลอภิชน"), ["ผล", "อภิชน"]);
        assert_eq!(segment_thai("ไม่มีใครรู้"), ["ไม่", "มี", "ใคร", "รู้"]);
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

    #[test]
    fn decomposed_sara_am_is_matched_as_sara_am() {
        // Sara Am typed as Nikhahit (U+0E4D) + Sara Aa (U+0E32).
        assert_eq!(
            segment_thai("น้\u{e4d}\u{e32}ตาลท\u{e4d}\u{e32}งาน"),
            ["น้\u{e4d}\u{e32}ตาล", "ท\u{e4d}\u{e32}งาน"]
        );

        let tokens: Vec<_> = crate::Tokenize::tokenize(&"น้\u{e4d}\u{e32}ตาล")
            .map(|t| t.lemma().to_string())
            .collect();
        assert_eq!(tokens, ["น้ำตาล"]);
    }

    #[test]
    fn segments_cover_the_whole_text() {
        for text in ["", "a", "ๆๆ", "เ", "ั", "\u{e4d}", "กกกกกกกกกกกกกกกก", "ภาษาไทย123abc"]
        {
            assert_eq!(segment_thai(text).concat(), text);
        }
    }
}
