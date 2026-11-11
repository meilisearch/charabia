use std::borrow::Cow;

use mlmorph::Analyser;
use unicode_normalization::UnicodeNormalization;

use super::{Normalizer, NormalizerOption};
use crate::{Script, Token};

/// Conservative whole-token Malayalam lemmatization using mlmorph.
///
/// Changed lemmas use whole-word highlighting because mlmorph supplies no character alignment.
pub struct MalayalamNormalizer;

impl Normalizer for MalayalamNormalizer {
    fn normalize<'o>(&self, mut token: Token<'o>, options: &NormalizerOption) -> Token<'o> {
        if !self.should_normalize(&token) {
            return token;
        }

        let Ok(analyser) = Analyser::new() else {
            return token;
        };
        // Analysis expects composed vowel signs; output follows the pipeline's NFKD form.
        let word: String = token.lemma().nfc().collect();
        let Ok(analyses) = analyser.analyse(&word, false, false) else {
            return token;
        };
        if let Some(root) = agreed_root(&analyses) {
            if root != token.lemma() {
                if options.create_char_map || token.char_map.is_some() {
                    // A previous unmapped rewrite cannot be reconstructed from the current lemma.
                    if token.char_map.is_none()
                        && (token.byte_len() != token.original_byte_len()
                            || token.char_count() != token.original_char_count())
                    {
                        return token;
                    }
                    let char_map = token.char_map.get_or_insert_with(|| {
                        token.lemma.chars().map(|c| (c.len_utf8() as u8, 0)).collect()
                    });
                    let Some((last, preceding)) = char_map.split_last_mut() else {
                        return token;
                    };
                    for (_, normalized) in preceding {
                        *normalized = 0;
                    }
                    // Associate the root with the complete source span, including deleted chars.
                    last.1 = root.len() as u8;
                }
                token.lemma = Cow::Owned(root);
            }
        }
        token
    }

    fn should_normalize(&self, token: &Token) -> bool {
        token.script == Script::Malayalam && token.is_word()
    }
}

fn agreed_root(analyses: &[(String, i32)]) -> Option<String> {
    let mut root = None;
    for (analysis, _) in analyses {
        let parsed = Analyser::parse_analysis(analysis).ok()?;
        let [morpheme] = parsed.morphemes.as_slice() else {
            return None;
        };
        if morpheme.root.trim().is_empty()
            || root.as_ref().is_some_and(|root| root != &morpheme.root)
        {
            return None;
        }
        root = Some(morpheme.root.clone());
    }
    let root: String = root?.nfkd().collect();
    // shortcut: retain roots over 255 bytes until char_map supports larger whole-word rewrites.
    (root.len() <= usize::from(u8::MAX)).then_some(root)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Normalize, TokenKind, TokenizerBuilder};

    #[test]
    fn selection_depends_on_script_and_word_kind_not_language_detection() {
        let token = Token {
            lemma: Cow::Borrowed("കേരളത്തിന്റെ"),
            kind: TokenKind::Word,
            script: Script::Malayalam,
            language: None,
            ..Default::default()
        };
        assert!(MalayalamNormalizer.should_normalize(&token));
        let options = NormalizerOption { lossy: true, ..Default::default() };
        assert_eq!(MalayalamNormalizer.normalize(token.clone(), &options).lemma(), "കേരളം");
        for token in [
            Token { script: Script::Latin, ..token.clone() },
            Token { kind: TokenKind::Separator(crate::SeparatorKind::Soft), ..token.clone() },
            Token { kind: TokenKind::Unknown, ..token },
        ] {
            assert!(!MalayalamNormalizer.should_normalize(&token));
            assert_eq!(MalayalamNormalizer.normalize(token.clone(), &options), token);
        }
    }

    #[test]
    fn base_words_keep_pipeline_unicode_form() {
        for word in ["കേരളം", "വനം", "കൊച്ചി"] {
            for create_char_map in [false, true] {
                let mut builder = TokenizerBuilder::default();
                builder.create_char_map(create_char_map);
                let tokenizer = builder.build();
                let tokens: Vec<_> = tokenizer.tokenize(word).collect();
                assert_eq!(tokens.len(), 1);
                assert_eq!(tokens[0].lemma(), word.nfkd().collect::<String>());
            }
        }
    }

    #[test]
    fn pipeline_preserves_ranges_and_highlights_whole_lemmatized_words() {
        let text = "Hello, കേരളത്തിന്റെ വനങ്ങളിൽ കേരളം വനം 123!";
        for create_char_map in [false, true] {
            for lossy in [false, true] {
                let mut builder = TokenizerBuilder::default();
                builder.create_char_map(create_char_map).lossy_normalization(lossy);
                let tokenizer = builder.build();
                let pairs: Vec<_> = tokenizer.reconstruct(text).collect();
                assert_eq!(pairs.iter().map(|(surface, _)| *surface).collect::<String>(), text);
                for (surface, token) in pairs {
                    assert_eq!(&text[token.byte_start..token.byte_end], surface);
                    assert_eq!(token.char_start, text[..token.byte_start].chars().count());
                    assert_eq!(token.char_end, text[..token.byte_end].chars().count());
                    let expected = match surface {
                        "Hello" if lossy => "hello",
                        "കേരളത്തിന്റെ" if lossy => "കേരളം",
                        "വനങ്ങളിൽ" if lossy => "വനം",
                        _ => surface,
                    };
                    assert_eq!(token.lemma(), expected);
                    if create_char_map {
                        if surface == "Hello" && lossy {
                            assert_eq!(token.original_lengths(1), (1, 1));
                        }
                        assert_eq!(
                            token.original_lengths(token.byte_len()),
                            (surface.chars().count(), surface.len())
                        );
                        if surface != expected && token.script == Script::Malayalam {
                            assert_eq!(token.original_lengths(0), (0, 0));
                            for prefix_bytes in 1..=token.byte_len() + 1 {
                                assert_eq!(
                                    token.original_lengths(prefix_bytes),
                                    (surface.chars().count(), surface.len())
                                );
                            }
                            let mut expected_map = vec![(3, 0); surface.chars().count()];
                            expected_map.last_mut().unwrap().1 = expected.len() as u8;
                            assert_eq!(token.char_map.as_ref(), Some(&expected_map));
                        }
                    } else {
                        assert_eq!(
                            token.original_lengths(token.byte_len()),
                            (expected.chars().count(), expected.len())
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn ambiguity_and_unknown_words_retain_existing_maps() {
        for word in ["നീലത്താമര", "ആനകൾ", "അആഇഈഉഊഋഌ", "ൺ", "ന്‍", "ന്‌", "ം", "\u{200d}", "\u{200c}"]
        {
            let token = Token {
                lemma: Cow::Borrowed(word),
                kind: TokenKind::Word,
                script: Script::Malayalam,
                ..Default::default()
            };
            for create_char_map in [false, true] {
                let options =
                    NormalizerOption { lossy: true, create_char_map, ..Default::default() };
                assert_eq!(MalayalamNormalizer.normalize(token.clone(), &options), token, "{word}");
                let mapped = Token {
                    char_map: Some(
                        word.chars().map(|c| (c.len_utf8() as u8, c.len_utf8() as u8)).collect(),
                    ),
                    ..token.clone()
                };
                assert_eq!(
                    MalayalamNormalizer.normalize(mapped.clone(), &options),
                    mapped,
                    "{word}"
                );
            }
        }
    }

    #[test]
    fn whole_word_mapping_composes_with_decomposition_and_deleted_characters() {
        let surface = "\0കൊച്ചിയുടെ\0";
        let original = Token {
            lemma: Cow::Borrowed(surface),
            kind: TokenKind::Word,
            script: Script::Malayalam,
            byte_start: 17,
            byte_end: 17 + surface.len(),
            char_start: 5,
            char_end: 5 + surface.chars().count(),
            ..Default::default()
        };
        let mut options = NormalizerOption { create_char_map: true, ..Default::default() };
        let before = original.clone().normalize(&options);
        let source_widths: Vec<_> =
            before.char_map.as_ref().unwrap().iter().map(|(original, _)| *original).collect();
        assert!(before.char_map.as_ref().unwrap().contains(&(3, 6)));
        assert!(before.char_map.as_ref().unwrap().contains(&(1, 0)));
        options.lossy = true;
        let after = original.normalize(&options);
        assert_eq!(after.lemma(), "കൊച്ചി".nfkd().collect::<String>());
        assert_eq!((after.byte_start, after.byte_end), (before.byte_start, before.byte_end));
        assert_eq!((after.char_start, after.char_end), (before.char_start, before.char_end));
        assert_eq!(
            after
                .char_map
                .as_ref()
                .unwrap()
                .iter()
                .map(|(original, _)| *original)
                .collect::<Vec<_>>(),
            source_widths
        );
        for prefix_bytes in 1..=after.byte_len() {
            assert_eq!(
                after.original_lengths(prefix_bytes),
                (surface.chars().count(), surface.len())
            );
        }
        // An existing map must also be updated when creating new maps is disabled.
        options.create_char_map = false;
        assert_eq!(MalayalamNormalizer.normalize(before, &options), after);
    }

    #[test]
    fn missing_mapping_from_an_earlier_rewrite_retains_the_lemma() {
        let token = Token {
            lemma: Cow::Borrowed("കേരളത്തിന്റെ"),
            kind: TokenKind::Word,
            script: Script::Malayalam,
            byte_end: "\0കേരളത്തിന്റെ".len(),
            char_end: "\0കേരളത്തിന്റെ".chars().count(),
            ..Default::default()
        };
        let options = NormalizerOption { create_char_map: true, lossy: true, ..Default::default() };
        assert_eq!(MalayalamNormalizer.normalize(token.clone(), &options), token);
    }

    #[test]
    fn composes_with_earlier_unicode_normalization() {
        for word in ["കൊച്ചി", "കോ", "ണ്‍", "ന്‌"] {
            let mut builder = TokenizerBuilder::default();
            builder.create_char_map(true);
            let tokenizer = builder.build();
            for (surface, token) in tokenizer.reconstruct(word) {
                assert_eq!(
                    token.original_lengths(token.byte_len()),
                    (surface.chars().count(), surface.len())
                );
                let unchanged =
                    MalayalamNormalizer.normalize(token.clone(), &NormalizerOption::default());
                assert_eq!(unchanged, token);
            }
        }
    }

    #[test]
    fn roots_must_be_nonempty_single_morphemes_and_unanimous() {
        for analyses in [
            vec![],
            vec![""],
            vec!["not an analysis"],
            vec!["<n>"],
            vec!["കേരളം<n>", "വനം<n>"],
            vec!["കരുത്ത്<n><sociative>എ<indeclinable>"],
            vec!["കേരളം<n>", ""],
        ] {
            let analyses: Vec<_> = analyses.into_iter().map(|s| (s.to_owned(), 0)).collect();
            assert_eq!(agreed_root(&analyses), None, "{analyses:?}");
        }
        assert_eq!(
            agreed_root(&[("കേരളം<n><genitive>".into(), 0), ("കേരളം<np><genitive>".into(), 0)]),
            Some("കേരളം".into())
        );
    }

    #[test]
    fn roots_must_fit_the_whole_word_map_after_unicode_decomposition() {
        let root = "ക".repeat(85);
        assert_eq!(agreed_root(&[(format!("{root}<n>"), 0)]), Some(root));
        let root = "ക".repeat(86);
        assert_eq!(agreed_root(&[(format!("{root}<n>"), 0)]), None);
        let root = "കൊ".repeat(30);
        assert!(root.len() < 255);
        assert_eq!(agreed_root(&[(format!("{root}<n>"), 0)]), None);
    }

    #[test]
    fn published_analyser_recognizes_inflections_and_base_words() {
        let analyser = Analyser::new().unwrap();
        for (word, expected) in
            [("കേരളത്തിന്റെ", "കേരളം"), ("കേരളം", "കേരളം"), ("വനങ്ങളിൽ", "വനം"), ("വനം", "വനം")]
        {
            let analyses = analyser.analyse(word, false, false).unwrap();
            assert_eq!(agreed_root(&analyses).as_deref(), Some(expected), "{word}");
        }
        assert_eq!(agreed_root(&analyser.analyse("unknownwordxyz", false, false).unwrap()), None);
    }

    #[quickcheck]
    fn arbitrary_unicode_analyses_do_not_panic(analyses: Vec<String>) -> bool {
        let analyses: Vec<_> = analyses.into_iter().map(|s| (s, 0)).collect();
        let _ = agreed_root(&analyses);
        true
    }

    #[quickcheck]
    fn arbitrary_unicode_tokens_preserve_ranges(
        text: String,
        create_char_map: bool,
        lossy: bool,
    ) -> bool {
        let mut builder = TokenizerBuilder::default();
        builder.create_char_map(create_char_map).lossy_normalization(lossy);
        let tokenizer = builder.build();
        let options = NormalizerOption { create_char_map, lossy, ..Default::default() };
        for original in tokenizer.segment(&text) {
            let token = original.clone().normalize(&options);
            assert_eq!(
                (token.byte_start, token.byte_end),
                (original.byte_start, original.byte_end)
            );
            assert_eq!(
                (token.char_start, token.char_end),
                (original.char_start, original.char_end)
            );
            if token.script != Script::Malayalam {
                assert_eq!(MalayalamNormalizer.normalize(token.clone(), &options), token);
                continue;
            }
            assert!(text.is_char_boundary(token.byte_start));
            assert!(text.is_char_boundary(token.byte_end));
            assert_eq!(token.char_start, text[..token.byte_start].chars().count());
            assert_eq!(token.char_end, text[..token.byte_end].chars().count());
            if let Some(map) = &token.char_map {
                assert_eq!(map.len(), token.original_char_count());
                assert_eq!(
                    map.iter().map(|(original, _)| usize::from(*original)).sum::<usize>(),
                    token.original_byte_len()
                );
                assert_eq!(
                    map.iter().map(|(_, normalized)| usize::from(*normalized)).sum::<usize>(),
                    token.byte_len()
                );
            }
            let _ = token.original_lengths(token.byte_len());
        }
        true
    }

    #[quickcheck]
    fn arbitrary_unicode_with_malayalam_script_does_not_panic(text: String) -> bool {
        let byte_end = text.len();
        let char_end = text.chars().count();
        let token = Token {
            lemma: Cow::Owned(text),
            kind: TokenKind::Word,
            script: Script::Malayalam,
            byte_end,
            char_end,
            ..Default::default()
        };
        let mut options = NormalizerOption { lossy: true, ..Default::default() };
        let unmapped = token.clone().normalize(&options);
        options.create_char_map = true;
        let mapped = token.normalize(&options);
        assert_eq!(mapped.lemma(), unmapped.lemma());
        true
    }
}
