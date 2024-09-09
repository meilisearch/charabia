use fst::raw::{Fst, Output};

/// Final-state-transducer (FST) Segmenter
pub(crate) struct FstSegmenter<'fst> {
    words_fst: &'fst Fst<&'fst [u8]>,
    min_length: Option<usize>, // Optional minimum length for a word to be segmented
    allow_char_split: bool,    // Flag to allow or disallow splitting words into characters
}

impl<'fst> FstSegmenter<'fst> {
    pub(crate) fn new(
        words_fst: &'fst Fst<&'fst [u8]>,
        min_length: Option<usize>,
        allow_char_split: bool,
    ) -> Self {
        Self { words_fst, min_length, allow_char_split }
    }

    pub fn segment_str<'o>(
        &'fst self,
        mut to_segment: &'o str,
    ) -> Box<dyn Iterator<Item = &'o str> + 'o>
    where
        'fst: 'o,
    {
        let iter = std::iter::from_fn(move || {
            // if we reach the end of the text, we return None.
            if to_segment.is_empty() {
                return None;
            }

            let mut length = match find_longest_prefix(self.words_fst, to_segment.as_bytes()) {
                Some((_, length)) => length,
                None => {
                    if self.allow_char_split {
                        // if no sequence matches, we return the next character as a lemma.
                        to_segment.chars().next().unwrap().len_utf8()
                    } else {
                        // if splitting is not allowed, return the whole input
                        let result = to_segment;
                        to_segment = "";
                        return Some(result);
                    }
                }
            };

            if let Some(min_len) = self.min_length {
                // enforce minimum lemma length if specified
                if length < min_len && to_segment.len() > length {
                    length = min_len.min(to_segment.len());
                }

                // prevent left over lemmas with a length fewer than min_len
                if to_segment.len() - length < min_len {
                    length = to_segment.len();
                }
            }

            // ensure the length is a valid character boundary
            length = to_segment
                .char_indices()
                .find(|(idx, _)| *idx >= length)
                .map(|(idx, _)| idx)
                .unwrap_or(to_segment.len());

            let (left, right) = to_segment.split_at(length);
            to_segment = right;
            Some(left)
        });

        Box::new(iter)
    }
}
/// find the longest key that is prefix of the given value.
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
