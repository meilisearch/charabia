use std::borrow::Cow;

use super::{Normalizer, NormalizerOption};
use crate::{Script, Token};

/// A [`Normalizer`] for the Thai script.
///
/// Thai combining marks are semantically significant characters that must be
/// preserved during normalization:
///
/// - **Vowels** (U+0E31, U+0E34–U+0E3A, U+0E47): Change the meaning of a word.
///   e.g. วิทยุ (radio) vs วิทย (not a word)
/// - **Tone marks** (U+0E48–U+0E4B): Distinguish different words.
///   e.g. ง่าย (easy) vs งาย (different word)
/// - **Thanthakhat / Silence mark** (U+0E4C): Silences a consonant.
///   e.g. มนุษย์ (human) vs มนุษย (incorrect form)
///
/// Unlike Arabic or Hebrew where diacritics are optional annotations,
/// Thai combining marks are integral to correct orthography and must be
/// preserved to ensure accurate search results.
///
/// Additionally, this normalizer **recompose** Sara Am (ำ, U+0E33) which may have
/// been split into Nikhahit (U+0E4D) + Sara Aa (U+0E32) by the
/// [`CompatibilityDecompositionNormalizer`]. Sara Am is a single, meaningful
/// vowel in Thai and should be treated as one unit for indexing purposes.
pub struct ThaiNormalizer;

/// Recompose any occurrences of Nikhahit (U+0E4D) + Sara Aa (U+0E32) back into
/// Sara Am (ำ, U+0E33) which is the correct composed form of the vowel.
fn recompose_sara_am(s: &str) -> Cow<'_, str> {
    // Fast path: if neither character is present, return as-is
    if !s.contains('\u{e4d}') {
        return Cow::Borrowed(s);
    }

    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    let mut modified = false;

    while let Some(c) = chars.next() {
        if c == '\u{e4d}' {
            // Nikhahit — check if followed by Sara Aa
            if chars.peek() == Some(&'\u{e32}') {
                chars.next(); // consume Sara Aa
                result.push('\u{e33}'); // push Sara Am
                modified = true;
            } else {
                result.push(c);
            }
        } else {
            result.push(c);
        }
    }

    if modified { Cow::Owned(result) } else { Cow::Borrowed(s) }
}

impl Normalizer for ThaiNormalizer {
    fn normalize<'o>(&self, mut token: Token<'o>, _options: &NormalizerOption) -> Token<'o> {
        // Recompose Sara Am that was decomposed by CompatibilityDecompositionNormalizer.
        // We only handle the simple (no char_map) case here because tokenization
        // of Thai text does not use char_map by default.
        match recompose_sara_am(token.lemma.as_ref()) {
            Cow::Borrowed(_) => {
                // No change needed
            }
            Cow::Owned(recomposed) => {
                token.lemma = Cow::Owned(recomposed);
                // char_map is invalidated by the length change; drop it.
                // This is acceptable because Thai segmentation creates fresh tokens
                // without a pre-existing char_map.
                token.char_map = None;
            }
        }
        token
    }

    fn should_normalize(&self, token: &Token) -> bool {
        token.script == Script::Thai
    }
}

#[cfg(test)]
mod test {
    use std::borrow::Cow::Owned;

    use crate::normalizer::{Normalizer, NormalizerOption};
    use crate::{Language, Script, Token};

    use super::ThaiNormalizer;

    const NORMALIZER_OPTIONS: NormalizerOption = NormalizerOption {
        create_char_map: true,
        lossy: true,
        classifier: crate::normalizer::ClassifierOption { stop_words: None, separators: None },
    };

    /// Helper to normalize a single token with ThaiNormalizer
    fn normalize(token: Token<'static>) -> Token<'static> {
        if ThaiNormalizer.should_normalize(&token) {
            ThaiNormalizer.normalize(token, &NORMALIZER_OPTIONS)
        } else {
            token
        }
    }

    // --- Tests for Sara Am recomposition ---

    #[test]
    fn recompose_sara_am_trailing() {
        // วิทยุ does not contain Sara Am — should be unchanged
        let token = Token {
            lemma: Owned("วิทยุ".to_string()),
            script: Script::Thai,
            ..Default::default()
        };
        let result = normalize(token);
        assert_eq!(result.lemma(), "วิทยุ");
    }

    #[test]
    fn recompose_sara_am_decomposed() {
        // น้ำ after NFKD decomposition: น + ้ + U+0E4D + า → should become น + ้ + ำ
        let decomposed = "\u{e19}\u{e49}\u{e4d}\u{e32}"; // น + mai tho + Nikhahit + Sara Aa
        let token = Token {
            lemma: Owned(decomposed.to_string()),
            script: Script::Thai,
            ..Default::default()
        };
        let result = normalize(token);
        // Expected: น + ้ + ำ (Sara Am U+0E33)
        assert_eq!(result.lemma(), "\u{e19}\u{e49}\u{e33}");
    }

    #[test]
    fn recompose_sara_am_in_word() {
        // น้ำยา decomposed → น + ้ + U+0E4D + า + ย + า
        let decomposed = "\u{e19}\u{e49}\u{e4d}\u{e32}\u{e22}\u{e32}";
        let token = Token {
            lemma: Owned(decomposed.to_string()),
            script: Script::Thai,
            ..Default::default()
        };
        let result = normalize(token);
        assert_eq!(result.lemma(), "\u{e19}\u{e49}\u{e33}\u{e22}\u{e32}");
    }

    // --- Integration test: full normalization pipeline ---

    #[test]
    fn full_pipeline_preserves_thai_marks() {
        use crate::normalizer::Normalize;

        let options = NormalizerOption { create_char_map: false, lossy: true, ..Default::default() };

        // วิทยุ — trailing sara u should be preserved (the bug from issue #371)
        let token = Token {
            lemma: Owned("วิทยุ".to_string()),
            char_end: "วิทยุ".chars().count(),
            byte_end: "วิทยุ".len(),
            script: Script::Thai,
            language: Some(Language::Tha),
            ..Default::default()
        };
        let normalized = token.normalize(&options);
        assert_eq!(normalized.lemma(), "วิทยุ", "trailing sara u (ุ) must be preserved");

        // ธาตุ — trailing sara u
        let token = Token {
            lemma: Owned("ธาตุ".to_string()),
            char_end: "ธาตุ".chars().count(),
            byte_end: "ธาตุ".len(),
            script: Script::Thai,
            language: Some(Language::Tha),
            ..Default::default()
        };
        let normalized = token.normalize(&options);
        assert_eq!(normalized.lemma(), "ธาตุ", "trailing sara u (ุ) must be preserved");

        // มนุษย์ — trailing thanthakhat (silence mark)
        let token = Token {
            lemma: Owned("มนุษย์".to_string()),
            char_end: "มนุษย์".chars().count(),
            byte_end: "มนุษย์".len(),
            script: Script::Thai,
            language: Some(Language::Tha),
            ..Default::default()
        };
        let normalized = token.normalize(&options);
        assert_eq!(normalized.lemma(), "มนุษย์", "thanthakhat (์) must be preserved");

        // ง่าย — tone mark
        let token = Token {
            lemma: Owned("ง่าย".to_string()),
            char_end: "ง่าย".chars().count(),
            byte_end: "ง่าย".len(),
            script: Script::Thai,
            language: Some(Language::Tha),
            ..Default::default()
        };
        let normalized = token.normalize(&options);
        assert_eq!(normalized.lemma(), "ง่าย", "tone mark (่) must be preserved");

        // น้ำ — Sara Am must survive (not decomposed)
        let token = Token {
            lemma: Owned("น้ำ".to_string()),
            char_end: "น้ำ".chars().count(),
            byte_end: "น้ำ".len(),
            script: Script::Thai,
            language: Some(Language::Tha),
            ..Default::default()
        };
        let normalized = token.normalize(&options);
        assert_eq!(normalized.lemma(), "น้ำ", "Sara Am (ำ) must not be decomposed");
    }
}
