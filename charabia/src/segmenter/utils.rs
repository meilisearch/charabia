use std::num::NonZero;

use fst::raw::{Fst, Output};

/// Final-state-transducer (FST) Segmenter
pub(crate) struct FstSegmenter<'fst> {
    words_fst: &'fst Fst<&'fst [u8]>,
    unmatched_split_strategy: UnmatchedSplitStrategy, // Strategy to use when no sequence matches
}

impl<'fst> FstSegmenter<'fst> {
    pub(crate) fn new(
        words_fst: &'fst Fst<&'fst [u8]>,
        unmatched_split_strategy: UnmatchedSplitStrategy,
    ) -> Self {
        Self { words_fst, unmatched_split_strategy }
    }

    pub fn segment_str<'o>(
        &'fst self,
        to_segment: &'o str,
    ) -> Box<dyn Iterator<Item = &'o str> + 'o>
    where
        'fst: 'o,
    {
        let mut buffering_match_offset = None;
        let mut offset = 0;
        let iter = std::iter::from_fn(move || {
            loop {
                let Some(next_to_segment) = to_segment.get(offset..).filter(|s| !s.is_empty())
                else {
                    // if we reach the end of the text, we return the buffered match if any.
                    return buffering_match_offset
                        .take() // take the buffered match offset and clear it
                        .and_then(|offset| to_segment.get(offset..))
                        .filter(|s| !s.is_empty());
                };

                let match_kind =
                    match find_longest_prefix(self.words_fst, next_to_segment.as_bytes()) {
                        Some((_, length)) => {
                            MatchKind::Match(fix_length_boundary(next_to_segment, length))
                        }
                        None => {
                            match self.unmatched_split_strategy {
                                UnmatchedSplitStrategy::NextMatch { max_char_count } => {
                                    // buffer the match offset if it is not already buffered
                                    let buffered_match_offset =
                                        *buffering_match_offset.get_or_insert(offset);

                                    // if the max char count is reached, return the match
                                    match max_char_count {
                                        Some(max_char_count)
                                            if to_segment[buffered_match_offset..offset]
                                                .chars()
                                                .count()
                                                >= max_char_count.get() =>
                                        {
                                            // return 1 character match to indicate that the max char count is reached
                                            MatchKind::Match(1)
                                        }
                                        _ => {
                                            // get the length of the first character and return it
                                            MatchKind::BufferingMatch(
                                                next_to_segment.chars().next().unwrap().len_utf8(),
                                            )
                                        }
                                    }
                                }
                            }
                        }
                    };

                match match_kind {
                    MatchKind::Match(length) => match buffering_match_offset.take() {
                        Some(buffering_match_offset) => {
                            // return the buffered match instead of the next match
                            return Some(&to_segment[buffering_match_offset..offset]);
                        }
                        None => {
                            // otherwise, return the next match
                            offset += length;
                            return Some(&next_to_segment[..length]);
                        }
                    },
                    MatchKind::BufferingMatch(length) => {
                        offset += length;
                    }
                }
            }
        });

        Box::new(iter)
    }
}

fn fix_length_boundary(s: &str, length: usize) -> usize {
    s.char_indices().find(|(idx, _)| *idx >= length).map(|(idx, _)| idx).unwrap_or(s.len())
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnmatchedSplitStrategy {
    // accumulate characters until the next match is found or the max char count is reached
    NextMatch { max_char_count: Option<NonZero<usize>> },
}

enum MatchKind {
    Match(usize),
    BufferingMatch(usize),
}
