use self::SeparatorCategory::*;
use deunicode::deunicode_char;
use slice_group_by::StrGroupBy;
use std::iter::Peekable;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum CharCategory {
    Separator(SeparatorCategory),
    Cjk,
    Other,
}

fn classify_char(c: char) -> CharCategory {
    if let Some(category) = classify_separator(c) {
        CharCategory::Separator(category)
    } else if is_cjk(c) {
        CharCategory::Cjk
    } else {
        CharCategory::Other
    }
}

fn is_str_word(s: &str) -> bool {
    !s.chars().any(is_separator)
}

fn same_group_category(a: char, b: char) -> bool {
    match (classify_char(a), classify_char(b)) {
        (CharCategory::Cjk, _) | (_, CharCategory::Cjk) => false,
        (CharCategory::Separator(_), CharCategory::Separator(_)) => true,
        (a, b) => a == b,
    }
}

// fold the number of chars along with the index position
fn chars_count_index((n, _): (usize, usize), (i, c): (usize, char)) -> (usize, usize) {
    (n + 1, i + c.len_utf8())
}

pub fn split_query_string(query: &str) -> impl Iterator<Item = &str> {
    Tokenizer::new(query).map(|t| t.word)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Token<'a> {
    pub word: &'a str,
    /// index of the token in the token sequence
    pub index: usize,
    pub word_index: usize,
    pub char_index: usize,
}

pub struct Tokenizer<'a> {
    count: usize,
    inner: &'a str,
    word_index: usize,
    char_index: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(string: &str) -> Tokenizer {
        // skip every separator and set `char_index`
        // to the number of char trimmed
        let (count, index) = string
            .char_indices()
            .take_while(|(_, c)| is_separator(*c))
            .fold((0, 0), chars_count_index);

        Tokenizer {
            count: 0,
            inner: &string[index..],
            word_index: 0,
            char_index: count,
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut iter = self.inner.linear_group_by(same_group_category).peekable();

        while let (Some(string), next_string) = (iter.next(), iter.peek()) {
            let (count, index) = string.char_indices().fold((0, 0), chars_count_index);

            if !is_str_word(string) {
                self.word_index += string
                    .chars()
                    .filter_map(classify_separator)
                    .fold(Soft, |a, x| a.merge(x))
                    .to_usize();
                self.char_index += count;
                self.inner = &self.inner[index..];
                continue;
            }

            let token = Token {
                word: string,
                index: self.count,
                word_index: self.word_index,
                char_index: self.char_index,
            };

            if next_string.filter(|s| is_str_word(s)).is_some() {
                self.word_index += 1;
            }

            self.count += 1;
            self.char_index += count;
            self.inner = &self.inner[index..];

            return Some(token);
        }

        self.inner = "";
        None
    }
}
