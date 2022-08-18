// Import `Segmenter` trait.
use fst::raw::{Fst, Output};
use once_cell::sync::Lazy;

use crate::segmenter::Segmenter;

/// Thai specialized [`Segmenter`].
///
/// This Segmenter uses a dictionary encoded as an FST to segment the provided text.
/// Dictionary source: PyThaiNLP project on https://github.com/PyThaiNLP/nlpo3
pub struct ThaiSegmenter;

static WORDS_FST: Lazy<Fst<&[u8]>> =
    Lazy::new(|| Fst::new(&include_bytes!("../../dictionaries/fst/thai/words.fst")[..]).unwrap());

impl Segmenter for ThaiSegmenter {
    fn segment_str<'o>(&self, mut to_segment: &'o str) -> Box<dyn Iterator<Item = &'o str> + 'o> {
        let iter = std::iter::from_fn(move || {
            // if we reach the end of the text, we return None.
            if to_segment.is_empty() {
                return None;
            }

            let length = match find_longest_prefix(&WORDS_FST, to_segment.as_bytes()) {
                Some((_, length)) => length,
                None => {
                    // if no sequence matches, we return the next character as a lemma.
                    let first = to_segment.chars().next().unwrap();
                    first.len_utf8()
                }
            };

            let (left, right) = to_segment.split_at(length);
            to_segment = right;
            Some(left)
        });

        Box::new(iter)
    }
}

/// Thanks to @llogiq for this function
/// https://github.com/BurntSushi/fst/pull/104/files
///
/// find the longest key that is prefix of the given value.
///
/// If the key exists, then `Some((value, key_len))` is returned, where
/// `value` is the value associated with the key, and `key_len` is the
/// length of the found key. Otherwise `None` is returned.
///
/// This can be used to e.g. build tokenizing functions.
#[inline]
fn find_longest_prefix(fst: &Fst<&[u8]>, value: &[u8]) -> Option<(u64, usize)> {
    let mut node = fst.root();
    let mut out = Output::zero();
    let mut last_match = None;
    for (i, &b) in value.iter().enumerate() {
        if let Some(trans_index) = node.find_input(b) {
            let t = node.transition(trans_index);
            node = fst.node(t.addr);
            out = out.cat(t.out);
            if node.is_final() {
                last_match = Some((out.cat(node.final_output()).value(), i + 1));
            }
        } else {
            return last_match;
        }
    }
    last_match
}

// Test the segmenter:
#[cfg(test)]
mod test {
    use crate::segmenter::test::test_segmenter;

    const TEXT: &str = "ภาษาไทยง่ายนิดเดียว";

    const SEGMENTED: &[&str] = &["ภาษาไทย", "ง่าย", "นิดเดียว"];

    const TOKENIZED: &[&str] = SEGMENTED;
    // Macro that run several tests on the Segmenter.
    test_segmenter!(ThaiSegmenter, TEXT, SEGMENTED, TOKENIZED, Script::Thai, Language::Tha);
}
