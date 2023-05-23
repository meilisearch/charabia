use deunicode::deunicode_char;

use super::{Normalizer, NormalizerOption};
use crate::{SeparatorKind, Token, TokenKind};

/// Classify a Token as a word, a stop_word or a separator.
///
/// Assign to each [`Token`]s a [`TokenKind`] using provided stop words.
///
/// [`TokenKind`]: crate::TokenKind
///
/// Any `Token` that is in the stop words [`Set`] is assigned to [`TokenKind::StopWord`].
///
/// [`TokenKind::StopWord`]: crate::TokenKind#StopWord
pub struct Classifier;

impl Normalizer for Classifier {
    fn normalize<'o>(&self, mut token: Token<'o>, options: &NormalizerOption) -> Token<'o> {
        let lemma = token.lemma();
        let mut is_hard_separator = false;
        if options.stop_words.as_ref().map(|stop_words| stop_words.contains(lemma)).unwrap_or(false)
        {
            token.kind = TokenKind::StopWord;
        } else if lemma.chars().all(|c| match classify_separator(c) {
            Some(SeparatorKind::Hard) => {
                is_hard_separator = true;
                true
            }
            Some(SeparatorKind::Soft) => true,

            None => false,
        }) {
            if is_hard_separator {
                token.kind = TokenKind::Separator(SeparatorKind::Hard);
            } else {
                token.kind = TokenKind::Separator(SeparatorKind::Soft);
            }
        } else {
            token.kind = TokenKind::Word;
        }

        token
    }

    fn should_normalize(&self, token: &Token) -> bool {
        token.kind == TokenKind::Unknown
    }
}

fn classify_separator(c: char) -> Option<SeparatorKind> {
    match deunicode_char(c)?.chars().next()? {
        // Prevent deunicoding cyrillic chars (e.g. ь -> ' is incorrect)
        _ if ('\u{0410}'..='\u{044f}').contains(&c) => None, // russian cyrillic letters [а-яА-Я]
        c if c.is_whitespace() => Some(SeparatorKind::Soft), // whitespaces
        '-' | '_' | '\'' | ':' | '/' | '\\' | '@' | '"' | '+' | '~' | '=' | '^' | '*' | '#' => {
            Some(SeparatorKind::Soft)
        }
        '.' | ';' | ',' | '!' | '?' | '(' | ')' | '[' | ']' | '{' | '}' | '|' => {
            Some(SeparatorKind::Hard)
        }
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use std::borrow::Cow;

    use fst::Set;

    use crate::normalizer::test::test_normalizer;

    // base tokens to normalize.
    fn tokens() -> Vec<Token<'static>> {
        vec![
            Token { lemma: Cow::Borrowed("   "), ..Default::default() },
            Token { lemma: Cow::Borrowed("\" "), ..Default::default() },
            Token { lemma: Cow::Borrowed("@   "), ..Default::default() },
            Token { lemma: Cow::Borrowed("."), ..Default::default() },
            Token { lemma: Cow::Borrowed("   ."), ..Default::default() },
            Token { lemma: Cow::Borrowed("  。"), ..Default::default() },
            Token { lemma: Cow::Borrowed("S.O.S"), ..Default::default() },
            Token { lemma: Cow::Borrowed("ь"), ..Default::default() },
        ]
    }

    // expected result of the current Normalizer.
    fn normalizer_result() -> Vec<Token<'static>> {
        vec![
            Token {
                lemma: Cow::Borrowed("   "),
                kind: TokenKind::Separator(SeparatorKind::Soft),
                ..Default::default()
            },
            Token {
                lemma: Cow::Borrowed("\" "),
                kind: TokenKind::Separator(SeparatorKind::Soft),
                ..Default::default()
            },
            Token {
                lemma: Cow::Borrowed("@   "),
                kind: TokenKind::Separator(SeparatorKind::Soft),
                ..Default::default()
            },
            Token {
                lemma: Cow::Borrowed("."),
                kind: TokenKind::Separator(SeparatorKind::Hard),
                ..Default::default()
            },
            Token {
                lemma: Cow::Borrowed("   ."),
                kind: TokenKind::Separator(SeparatorKind::Hard),
                ..Default::default()
            },
            Token {
                lemma: Cow::Borrowed("  。"),
                kind: TokenKind::Separator(SeparatorKind::Hard),
                ..Default::default()
            },
            Token { lemma: Cow::Borrowed("S.O.S"), kind: TokenKind::Word, ..Default::default() },
            Token { lemma: Cow::Borrowed("ь"), kind: TokenKind::Word, ..Default::default() },
        ]
    }

    // expected result of the complete Normalizer pieline.
    fn normalized_tokens() -> Vec<Token<'static>> {
        vec![
            Token {
                lemma: Cow::Borrowed("   "),
                kind: TokenKind::Separator(SeparatorKind::Soft),
                ..Default::default()
            },
            Token {
                lemma: Cow::Borrowed("\" "),
                kind: TokenKind::Separator(SeparatorKind::Soft),
                ..Default::default()
            },
            Token {
                lemma: Cow::Borrowed("@   "),
                kind: TokenKind::Separator(SeparatorKind::Soft),
                ..Default::default()
            },
            Token {
                lemma: Cow::Borrowed("."),
                kind: TokenKind::Separator(SeparatorKind::Hard),
                ..Default::default()
            },
            Token {
                lemma: Cow::Borrowed("   ."),
                kind: TokenKind::Separator(SeparatorKind::Hard),
                ..Default::default()
            },
            Token {
                lemma: Cow::Borrowed("  。"),
                kind: TokenKind::Separator(SeparatorKind::Hard),
                ..Default::default()
            },
            Token { lemma: Cow::Borrowed("S.O.S"), kind: TokenKind::Word, ..Default::default() },
            Token { lemma: Cow::Borrowed("ь"), kind: TokenKind::Word, ..Default::default() },
        ]
    }

    test_normalizer!(Classifier, tokens(), normalizer_result(), normalized_tokens());

    #[test]
    fn stop_words() {
        let stop_words = Set::from_iter(["the"].iter()).unwrap();
        let stop_words = stop_words.as_fst().as_bytes();
        let stop_words = Set::new(stop_words).unwrap();
        let options = NormalizerOption { create_char_map: true, stop_words: Some(stop_words) };

        let token = Classifier
            .normalize(Token { lemma: Cow::Borrowed("the"), ..Default::default() }, &options);
        assert!(token.is_stopword());

        let token = Classifier
            .normalize(Token { lemma: Cow::Borrowed("The"), ..Default::default() }, &options);
        assert!(token.is_word());

        let token = Classifier
            .normalize(Token { lemma: Cow::Borrowed("foobar"), ..Default::default() }, &options);
        assert!(token.is_word());
    }
}
