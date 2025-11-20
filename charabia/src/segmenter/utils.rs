use std::num::NonZero;

use fst::raw::{Fst, Output};

/// Final-state-transducer (FST) Segmenter
pub(crate) struct FstSegmenter<'fst> {
    words_fst: &'fst Fst<&'fst [u8]>,
    buffering_strategy: BufferingStrategy, // Strategy to use when no sequence matches
}

impl<'fst> FstSegmenter<'fst> {
    pub(crate) fn new(
        words_fst: &'fst Fst<&'fst [u8]>,
        buffering_strategy: BufferingStrategy,
    ) -> Self {
        Self { words_fst, buffering_strategy }
    }

    pub fn segment_str<'o>(
        &'fst self,
        to_segment: &'o str,
    ) -> Box<dyn Iterator<Item = &'o str> + 'o>
    where
        'fst: 'o,
    {
        let mut cursor = SegmentationCursor::new(to_segment);
        let iter = std::iter::from_fn(move || {
            loop {
                // get the tail of the string to segment
                let Some(next_to_segment) = cursor.tail() else {
                    // if we reach the end of the text, we return the buffered match if any.
                    return cursor.take_buffered_segment();
                };

                // find the longest prefix in the FST that matches the tail of the string to segment
                let next_match = find_longest_prefix(self.words_fst, next_to_segment.as_bytes());

                if let Some((_, length)) = next_match {
                    // if a match is found, compute the next segment
                    return cursor.compute_next_segment(length);
                } else {
                    // otherwise, use a fallback strategy to compute the next segment
                    match self.buffering_strategy {
                        BufferingStrategy::UntilNextMatch { max_char_count } => {
                            // buffer the next character
                            if cursor.buffer_next_character(max_char_count).is_full() {
                                // if the buffer becomes full, return the buffered segment
                                return cursor.take_buffered_segment();
                            }
                        }
                    }
                }
            }
        });

        Box::new(iter)
    }
}

/// [HOTFIX] floor the char boundary of the string
/// TODO: replace by `std::str::floor_char_boundary` in Rust 1.91+
///
/// It already appened that the fst returned a length that does not necessarily match the characters boundaries.
/// Creating crashes when decoding the string.
fn floor_char_boundary(s: &str, length: usize) -> usize {
    s.char_indices().find(|(idx, _)| *idx >= length).map(|(idx, _)| idx).unwrap_or(s.len())
}

/// check if the max char count is reached
fn is_max_char_count_reached(s: &str, max_char_count: Option<NonZero<usize>>) -> bool {
    if let Some(max_char_count) = max_char_count {
        s.chars().count() >= max_char_count.get()
    } else {
        false
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

/// Strategy for handling unmatched sequences during segmentation.
///
/// Controls how the segmenter behaves when encountering character sequences
/// that don't match any entries in the FST dictionary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferingStrategy {
    /// Accumulate characters until the next match is found or the max char count is reached.
    ///
    /// - `Some(n)`: Buffer up to `n` characters before emitting the buffered sequence
    /// - `None`: Buffer indefinitely until a dictionary match is found
    UntilNextMatch { max_char_count: Option<NonZero<usize>> },
}

/// State of the buffer
enum BufferState {
    Full,
    Buffering,
}

impl BufferState {
    fn is_full(&self) -> bool {
        matches!(self, BufferState::Full)
    }
}
struct SegmentationCursor<'o> {
    to_segment: &'o str,
    buffer_head_offset: Option<usize>,
    offset: usize,
}

impl<'o> SegmentationCursor<'o> {
    fn new(to_segment: &'o str) -> Self {
        Self { to_segment, buffer_head_offset: None, offset: 0 }
    }

    /// get the tail of the string to segment
    /// if the offset is at the end of the string, return None
    ///
    /// this corresponds to the part of the string that is not yet segmented.
    fn tail(&self) -> Option<&'o str> {
        self.to_segment.get(self.offset..).filter(|s| !s.is_empty())
    }

    /// take the buffered segment if any
    fn take_buffered_segment(&mut self) -> Option<&'o str> {
        self.buffer_head_offset
            .take()
            .and_then(|head| self.to_segment.get(head..self.offset))
            .filter(|s| !s.is_empty())
    }

    /// buffer the next character
    ///
    /// if the max char count is reached, return BufferState::Full
    /// otherwise, return BufferState::Buffering
    fn buffer_next_character(&mut self, max_char_count: Option<NonZero<usize>>) -> BufferState {
        // if there is no buffered segment, insert the current offset as the head of the buffered segment
        let head = *self.buffer_head_offset.get_or_insert(self.offset);
        // update the offset to the end of the next character
        let tail = {
            self.offset += self.next_character_length();
            self.offset
        };
        let segment = &self.to_segment[head..tail];

        // check if the max char count is reached
        if is_max_char_count_reached(segment, max_char_count) {
            // if the max char count is reached, return BufferState::Full
            BufferState::Full
        } else {
            // otherwise, return BufferState::Buffering
            BufferState::Buffering
        }
    }

    /// get the length of the next character
    fn next_character_length(&self) -> usize {
        self.to_segment[self.offset..].chars().next().unwrap().len_utf8()
    }

    /// compute the next segment
    ///
    /// if there is a buffered segment, return it
    /// otherwise, return the next segment based on the next segment length
    fn compute_next_segment(&mut self, next_segment_length: usize) -> Option<&'o str> {
        // if there is a buffered segment, return it
        if let Some(buffered_segment) = self.take_buffered_segment() {
            return Some(buffered_segment);
        }

        // otherwise, return the next segment based on the next segment length
        if let Some(tail) = self.tail() {
            let length = floor_char_boundary(tail, next_segment_length);
            // update the offset to the end of the next segment
            self.offset += length;
            return Some(&tail[..length]);
        }

        None
    }
}
