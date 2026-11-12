use std::borrow::Cow;

use super::{Normalizer, NormalizerOption};
use crate::{Script, Token};

const NIKHAHIT: char = '\u{e4d}';
const SARA_AA: char = '\u{e32}';
const SARA_AM: char = '\u{e33}';
const DECOMPOSED_SARA_AM: &str = "\u{e4d}\u{e32}";

/// Thai specialized [`Normalizer`].
///
/// This Normalizer recomposes Sara Am (ำ, U+0E33), which is decomposed into
/// Nikhahit (U+0E4D) + Sara Aa (U+0E32) by the
/// [`CompatibilityDecompositionNormalizer`](super::CompatibilityDecompositionNormalizer),
/// and which is sometimes typed as these two characters.
///
/// Sara Am is a single vowel, so both forms must give the same lemma.
/// The recomposition is not lossy, so this Normalizer is not part of the lossy normalizers.
pub struct ThaiNormalizer;

impl Normalizer for ThaiNormalizer {
    fn normalize<'o>(&self, mut token: Token<'o>, options: &NormalizerOption) -> Token<'o> {
        // `Normalize for &str` calls every normalizer without calling `should_normalize`,
        // so return early to avoid allocating when there is nothing to recompose.
        if !token.lemma.contains(DECOMPOSED_SARA_AM) {
            return token;
        }

        // if no char_map exists, the lemma is the original text.
        let mut char_map = token.char_map.take().or_else(|| {
            options.create_char_map.then(|| {
                token.lemma.chars().map(|c| (c.len_utf8() as u8, c.len_utf8() as u8)).collect()
            })
        });

        token.lemma = Cow::Owned(match char_map.as_deref_mut() {
            Some(char_map) => recompose_with_char_map(&token.lemma, char_map),
            None => recompose(&token.lemma).filter_map(|(_, c)| c).collect(),
        });
        token.char_map = char_map;

        token
    }

    fn should_normalize(&self, token: &Token) -> bool {
        token.script == Script::Thai && token.lemma.contains(DECOMPOSED_SARA_AM)
    }
}

/// Recomposes Nikhahit + Sara Aa into Sara Am.
///
/// Yields, for each char of the lemma, its length and the char replacing it:
/// Nikhahit is removed and Sara Aa is replaced by Sara Am.
fn recompose(lemma: &str) -> impl Iterator<Item = (usize, Option<char>)> + '_ {
    let mut chars = lemma.chars().peekable();
    let mut after_nikhahit = false;
    std::iter::from_fn(move || {
        let c = chars.next()?;
        let recomposed = match c {
            NIKHAHIT if chars.peek() == Some(&SARA_AA) => {
                after_nikhahit = true;
                None
            }
            SARA_AA if after_nikhahit => {
                after_nikhahit = false;
                Some(SARA_AM)
            }
            c => Some(c),
        };

        Some((c.len_utf8(), recomposed))
    })
}

/// Recomposes the lemma, see [`recompose`], recomputing the normalized lengths of the `char_map`.
///
/// Nikhahit and Sara Aa are in the same `char_map` entry when an original Sara Am has been
/// decomposed, and in two entries when the original text contains the two characters.
fn recompose_with_char_map(lemma: &str, char_map: &mut [(u8, u8)]) -> String {
    let mut recomposed = String::with_capacity(lemma.len());
    let mut chars = recompose(lemma);
    for (_, normalized_len) in char_map.iter_mut() {
        let start = recomposed.len();
        let mut remaining = *normalized_len as usize;
        while remaining > 0 {
            let Some((len, c)) = chars.next() else { break };
            remaining = remaining.saturating_sub(len);
            recomposed.extend(c);
        }
        *normalized_len = (recomposed.len() - start) as u8;
    }

    recomposed
}

#[cfg(test)]
mod test {
    use std::borrow::Cow::{Borrowed, Owned};

    use super::*;
    use crate::normalizer::Normalize;

    fn thai_token(lemma: &str) -> Token<'_> {
        Token {
            lemma: Borrowed(lemma),
            char_end: lemma.chars().count(),
            byte_end: lemma.len(),
            script: Script::Thai,
            ..Default::default()
        }
    }

    fn options(create_char_map: bool) -> NormalizerOption<'static> {
        NormalizerOption { create_char_map, ..Default::default() }
    }

    #[test]
    fn decomposed_sara_am_is_recomposed() {
        for (decomposed, expected) in [
            ("น\u{e49}\u{e4d}\u{e32}", "น้ำ"),
            ("ท\u{e4d}\u{e32}งาน", "ทำงาน"),
            ("น\u{e49}\u{e4d}\u{e32}ท\u{e4d}\u{e32}", "น้ำทำ"),
        ] {
            for create_char_map in [false, true] {
                let token = thai_token(decomposed);
                assert!(ThaiNormalizer.should_normalize(&token));
                let token = ThaiNormalizer.normalize(token, &options(create_char_map));
                assert_eq!(token.lemma(), expected);
                assert_eq!(token.char_map.is_some(), create_char_map);
            }
        }
    }

    #[test]
    fn text_without_decomposed_sara_am_is_not_reallocated() {
        // a Nikhahit that is not followed by Sara Aa is kept.
        for lemma in ["น้ำ", "วิทยุ", "\u{e4d}", "น\u{e4d}", "hello"] {
            let token = thai_token(lemma);
            assert!(!ThaiNormalizer.should_normalize(&token));
            let token = ThaiNormalizer.normalize(token, &options(true));
            assert!(matches!(token.lemma, Borrowed(_)), "{lemma} must not be reallocated");
            assert_eq!(token.char_map, None);
        }
    }

    #[test]
    fn char_map_of_sara_am_typed_as_two_chars() {
        // ทํางาน: Nikhahit and Sara Aa are two original chars.
        let typed = "ท\u{e4d}\u{e32}งาน";
        let expected = vec![(3, 3), (3, 0), (3, 3), (3, 3), (3, 3), (3, 3)];

        // the char_map is created.
        let token = ThaiNormalizer.normalize(thai_token(typed), &options(true));
        assert_eq!(token.lemma(), "ทำงาน");
        assert_eq!(token.char_map, Some(expected.clone()));
        assert_eq!(token.original_lengths(token.lemma().len()), (6, 18));

        // the char_map already exists.
        let token = Token { char_map: Some(vec![(3, 3); 6]), ..thai_token(typed) };
        let token = ThaiNormalizer.normalize(token, &options(true));
        assert_eq!(token.lemma(), "ทำงาน");
        assert_eq!(token.char_map, Some(expected));
    }

    #[test]
    fn char_map_of_sara_am_decomposed_by_a_previous_normalizer() {
        // น้ำ: the original Sara Am (3 bytes) has been decomposed into 6 bytes.
        let token = Token {
            lemma: Owned("น\u{e49}\u{e4d}\u{e32}".to_string()),
            char_map: Some(vec![(3, 3), (3, 3), (3, 6)]),
            ..thai_token("น้ำ")
        };
        let token = ThaiNormalizer.normalize(token, &options(true));
        assert_eq!(token.lemma(), "น้ำ");
        assert_eq!(token.char_map, Some(vec![(3, 3), (3, 3), (3, 3)]));
    }

    #[test]
    fn vowels_and_tone_marks_are_kept_by_the_lossy_normalization() {
        let options = NormalizerOption { lossy: true, ..Default::default() };
        for word in ["วิทยุ", "ธาตุ", "มนุษย์", "ง่าย", "น้ำ"]
        {
            assert_eq!(thai_token(word).normalize(&options).lemma(), word);
        }
    }

    #[test]
    fn sara_am_is_kept_by_the_non_lossy_normalization() {
        for create_char_map in [false, true] {
            for (word, expected) in
                [("น้ำ", "น้ำ"), ("ทำงาน", "ทำงาน"), ("น\u{e49}\u{e4d}\u{e32}", "น้ำ")]
            {
                let token = thai_token(word).normalize(&options(create_char_map));
                assert_eq!(token.lemma(), expected);
                if create_char_map {
                    assert_eq!(
                        token.original_lengths(token.lemma().len()),
                        (word.chars().count(), word.len())
                    );
                }
            }
        }
    }
}
