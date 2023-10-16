use std::collections::HashSet;

use fst::Set;
use once_cell::sync::Lazy;

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
        token.kind = TokenKind::Word;
        let lemma = token.lemma();

        if let Some(stop_words) = &options.classifier.stop_words {
            if stop_words.contains(lemma) {
                token.kind = TokenKind::StopWord;
                return token;
            }
        }

        match options.classifier.separators {
            Some(separators) if separators.contains(&lemma) => {
                token.kind = TokenKind::Separator(separator_kind(lemma));
            }
            None if DEFAULT_SEPARATOR_SET.contains(lemma) => {
                token.kind = TokenKind::Separator(separator_kind(lemma));
            }
            _otherwise => (),
        }

        token
    }

    fn should_normalize(&self, token: &Token) -> bool {
        token.kind == TokenKind::Unknown
    }
}

/// Structure for providing options to the classfier.
#[derive(Debug, Clone, Default)]
pub struct ClassifierOption<'no> {
    pub stop_words: Option<Set<&'no [u8]>>,
    pub separators: Option<&'no [&'no str]>,
}

fn separator_kind(lemma: &str) -> SeparatorKind {
    if CONTEXT_SEPARATOR_SET.contains(lemma) {
        SeparatorKind::Hard
    } else {
        SeparatorKind::Soft
    }
}

pub static DEFAULT_SEPARATOR_SET: Lazy<HashSet<&str>> =
    Lazy::new(|| crate::separators::DEFAULT_SEPARATORS.iter().copied().collect());

pub static CONTEXT_SEPARATOR_SET: Lazy<HashSet<&str>> =
    Lazy::new(|| crate::separators::CONTEXT_SEPARATORS.iter().copied().collect());

#[cfg(test)]
mod test {
    use std::borrow::Cow;

    use crate::normalizer::test::test_normalizer;

    // base tokens to normalize.
    fn tokens() -> Vec<Token<'static>> {
        vec![
            Token { lemma: Cow::Borrowed(" "), ..Default::default() },
            Token { lemma: Cow::Borrowed("\""), ..Default::default() },
            Token { lemma: Cow::Borrowed("@"), ..Default::default() },
            Token { lemma: Cow::Borrowed("."), ..Default::default() },
            Token { lemma: Cow::Borrowed(". "), ..Default::default() },
            Token { lemma: Cow::Borrowed("。"), ..Default::default() },
            Token { lemma: Cow::Borrowed("S.O.S"), ..Default::default() },
            Token { lemma: Cow::Borrowed("ь"), ..Default::default() },
        ]
    }

    // expected result of the current Normalizer.
    fn normalizer_result() -> Vec<Token<'static>> {
        vec![
            Token {
                lemma: Cow::Borrowed(" "),
                kind: TokenKind::Separator(SeparatorKind::Soft),
                ..Default::default()
            },
            Token {
                lemma: Cow::Borrowed("\""),
                kind: TokenKind::Separator(SeparatorKind::Soft),
                ..Default::default()
            },
            Token {
                lemma: Cow::Borrowed("@"),
                kind: TokenKind::Separator(SeparatorKind::Soft),
                ..Default::default()
            },
            Token {
                lemma: Cow::Borrowed("."),
                kind: TokenKind::Separator(SeparatorKind::Soft),
                ..Default::default()
            },
            Token {
                lemma: Cow::Borrowed(". "),
                kind: TokenKind::Separator(SeparatorKind::Hard),
                ..Default::default()
            },
            Token {
                lemma: Cow::Borrowed("。"),
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
                lemma: Cow::Borrowed(" "),
                kind: TokenKind::Separator(SeparatorKind::Soft),
                ..Default::default()
            },
            Token {
                lemma: Cow::Borrowed("\""),
                kind: TokenKind::Separator(SeparatorKind::Soft),
                ..Default::default()
            },
            Token {
                lemma: Cow::Borrowed("@"),
                kind: TokenKind::Separator(SeparatorKind::Soft),
                ..Default::default()
            },
            Token {
                lemma: Cow::Borrowed("."),
                kind: TokenKind::Separator(SeparatorKind::Soft),
                ..Default::default()
            },
            Token {
                lemma: Cow::Borrowed(". "),
                kind: TokenKind::Separator(SeparatorKind::Hard),
                ..Default::default()
            },
            Token {
                lemma: Cow::Borrowed("。"),
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
        let options = NormalizerOption {
            create_char_map: true,
            classifier: ClassifierOption { stop_words: Some(stop_words), separators: None },
            lossy: false,
        };

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

    #[quickcheck]
    fn is_stop_word_iff_stop_words_contain_lemma(
        mut stop_words: Vec<String>,
        lemma: String,
        create_char_map: bool,
        lossy: bool,
        containing: bool,
    ) {
        if containing {
            stop_words.push(lemma.clone());
        } else {
            stop_words.retain(|w| w != &lemma);
        }

        stop_words.sort();
        let stop_words = Set::from_iter(stop_words.iter()).unwrap();
        let stop_words = stop_words.as_fst().as_bytes();
        let stop_words = Set::new(stop_words).unwrap();
        let options = NormalizerOption {
            create_char_map,
            classifier: ClassifierOption { stop_words: Some(stop_words), separators: None },
            lossy,
        };

        let token = Classifier
            .normalize(Token { lemma: Cow::Borrowed(&lemma), ..Default::default() }, &options);
        assert_eq!(token.is_stopword(), containing);
    }

    #[quickcheck]
    fn is_separator_if_separators_contain_lemma(
        mut separators: Vec<String>,
        lemma: String,
        create_char_map: bool,
        lossy: bool,
        containing: bool,
    ) {
        if containing {
            separators.push(lemma.clone());
        } else {
            separators.retain(|w| w != &lemma);
        }
        let separators: Vec<&str> = separators.iter().map(|s| s.as_str()).collect();
        let options = NormalizerOption {
            create_char_map,
            classifier: ClassifierOption { stop_words: None, separators: Some(&separators) },
            lossy,
        };

        let token = Classifier
            .normalize(Token { lemma: Cow::Borrowed(&lemma), ..Default::default() }, &options);
        assert_eq!(token.is_separator(), containing);
        if containing {
            assert!(token.is_separator());
        }
    }

    #[quickcheck]
    fn is_stop_word_if_both_stop_works_and_separators_contain_lemma(
        mut stop_words_and_separators: Vec<String>,
        lemma: String,
        create_char_map: bool,
        lossy: bool,
    ) {
        stop_words_and_separators.push(lemma.clone());
        stop_words_and_separators.sort();
        let stop_words = Set::from_iter(stop_words_and_separators.iter()).unwrap();
        let stop_words = stop_words.as_fst().as_bytes();
        let stop_words = Set::new(stop_words).unwrap();
        let separators: Vec<&str> = stop_words_and_separators.iter().map(|s| s.as_str()).collect();
        let options = NormalizerOption {
            create_char_map,
            classifier: ClassifierOption {
                stop_words: Some(stop_words),
                separators: Some(&separators),
            },
            lossy,
        };

        let token = Classifier
            .normalize(Token { lemma: Cow::Borrowed(&lemma), ..Default::default() }, &options);
        assert!(token.is_stopword());
        assert!(!token.is_separator());
    }
}
