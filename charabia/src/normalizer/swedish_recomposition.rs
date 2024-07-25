use std::borrow::Cow;

use aho_corasick::AhoCorasick;
use once_cell::sync::Lazy;

use super::Normalizer;
use crate::normalizer::NormalizerOption;
use crate::{Language, Token};

static MATCHING_STR: Lazy<AhoCorasick> = Lazy::new(|| {
    AhoCorasick::new(["A\u{30a}", "a\u{30a}", "A\u{308}", "a\u{308}", "O\u{308}", "o\u{308}"])
        .unwrap()
});

/// Swedish specialized [`Normalizer`].
///
/// This Normalizer recompose swedish characters containing diacritics.
///
/// This avoids the diacritic removal from the letter and preserves expected swedish character ordering.
pub struct SwedishRecompositionNormalizer;

impl Normalizer for SwedishRecompositionNormalizer {
    fn normalize<'o>(&self, mut token: Token<'o>, options: &NormalizerOption) -> Token<'o> {
        match token.char_map.take() {
            Some(mut char_map) => {
                // if a char_map already exists,iterate over it to reconstruct sub-strings.
                let mut lemma = String::new();
                let mut tail = token.lemma.as_ref();
                let mut normalized = String::new();
                for (_, normalized_len) in char_map.iter_mut() {
                    let (head, t) = tail.split_at(*normalized_len as usize);
                    tail = t;
                    normalized.clear();
                    // then normalize each sub-strings recomputing the size in the char_map.
                    let mut peekable = head.chars().peekable();
                    while let Some(c) = peekable.next() {
                        let (c, peek_consumed) = recompose_swedish(c, peekable.peek());
                        if peek_consumed {
                            peekable.next();
                        }

                        normalized.push(c);
                    }

                    *normalized_len = normalized.len() as u8;
                    lemma.push_str(normalized.as_ref());
                }

                token.lemma = Cow::Owned(lemma);
                token.char_map = Some(char_map);
            }
            None => {
                // if no char_map exists, iterate over the lemma recomposing characters.
                let mut char_map = Vec::new();
                let mut lemma = String::new();
                let mut peekable = token.lemma.chars().peekable();
                while let Some(c) = peekable.next() {
                    let (normalized, peek_consumed) = recompose_swedish(c, peekable.peek());
                    if peek_consumed {
                        peekable.next();
                    }

                    if options.create_char_map {
                        char_map.push((c.len_utf8() as u8, normalized.len_utf8() as u8));
                    }
                    lemma.push(normalized);
                }
                token.lemma = Cow::Owned(lemma);
                if options.create_char_map {
                    token.char_map = Some(char_map);
                }
            }
        }

        token
    }

    // Returns `true` if the Normalizer should be used.
    fn should_normalize(&self, token: &Token) -> bool {
        token.language == Some(Language::Swe) && MATCHING_STR.is_match(token.lemma())
    }
}

fn recompose_swedish(current: char, next: Option<&char>) -> (char, bool) {
    match (current, next) {
        ('A', Some('\u{30a}')) => ('Å', true),
        ('a', Some('\u{30a}')) => ('å', true),
        ('A', Some('\u{308}')) => ('Ä', true),
        ('a', Some('\u{308}')) => ('ä', true),
        ('O', Some('\u{308}')) => ('Ö', true),
        ('o', Some('\u{308}')) => ('ö', true),
        (c, _) => (c, false),
    }
}

// Test the normalizer:
#[cfg(test)]
mod test {
    use std::borrow::Cow::Owned;

    use crate::normalizer::test::test_normalizer;
    use crate::normalizer::Normalizer;
    use crate::token::TokenKind;
    use crate::Script;

    // base tokens to normalize.
    fn tokens() -> Vec<Token<'static>> {
        vec![Token {
            lemma: Owned("öpÅscålcäsÄÖs".to_string()),
            char_end: 13,
            byte_end: 19,
            script: Script::Latin,
            language: Some(Language::Swe),
            ..Default::default()
        }]
    }

    // expected result of the current Normalizer.
    fn normalizer_result() -> Vec<Token<'static>> {
        vec![Token {
            // lowercased
            lemma: Owned("öpÅscålcäsÄÖs".to_string()),
            char_end: 13,
            byte_end: 19,
            script: Script::Latin,
            language: Some(Language::Swe),
            ..Default::default()
        }]
    }

    // expected result of the complete Normalizer pieline.
    fn normalized_tokens() -> Vec<Token<'static>> {
        vec![Token {
            lemma: Owned("öpåscålcäsäös".to_string()),
            char_end: 13,
            byte_end: 19,
            char_map: Some(vec![
                (2, 2),
                (1, 1),
                (2, 2),
                (1, 1),
                (1, 1),
                (2, 2),
                (1, 1),
                (1, 1),
                (2, 2),
                (1, 1),
                (2, 2),
                (2, 2),
                (1, 1),
            ]),
            script: Script::Latin,
            kind: TokenKind::Word,
            language: Some(Language::Swe),
            ..Default::default()
        }]
    }

    test_normalizer!(
        SwedishRecompositionNormalizer,
        tokens(),
        normalizer_result(),
        normalized_tokens()
    );
}
