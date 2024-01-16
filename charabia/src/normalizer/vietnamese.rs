use super::{CharNormalizer, CharOrStr};
use crate::Script;
use crate::Token;

pub struct VietnameseNormalizer;

impl CharNormalizer for VietnameseNormalizer {
    fn normalize_char(&self, c: char) -> Option<CharOrStr> {
        match c {
            'Ð' | 'Đ' | 'đ' => Some("d".to_string().into()), // not only Vietnamese, but also many European countries use these letters
            _ => None,
        }
    }

    fn should_normalize(&self, token: &Token) -> bool {
        token.script == Script::Latin && token.lemma.chars().any(is_should_normalize)
    } 
}

fn is_should_normalize(c: char) -> bool {
    matches!(c, 'Ð' | 'Đ' | 'đ')
}
