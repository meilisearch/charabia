use lowercase::LowercaseNormalizer;
use once_cell::sync::Lazy;

use crate::detection::{Language, Script};
use crate::Token;

mod lowercase;

/// List of [`Normalizer`]s used by [`Normalize::normalize`].
pub static NORMALIZERS: Lazy<[Box<dyn Normalizer>; 1]> =
    Lazy::new(|| [Box::new(LowercaseNormalizer)]);

/// Iterator over Normalized [`Token`]s.
pub struct NormalizedTokenIter<'o> {
    token_iter: Box<dyn Iterator<Item = Token<'o>> + 'o>,
    inner: Box<dyn Iterator<Item = Token<'o>> + 'o>,
    normalizer: &'static Box<dyn Normalizer>,
}

impl<'o> Iterator for NormalizedTokenIter<'o> {
    type Item = Token<'o>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next() {
            Some(token) => Some(token),
            None => {
                let token = self.token_iter.next()?;
                if self.normalizer.should_normalize(token.script, token.language) {
                    self.inner = self.normalizer.normalize(token);
                    self.next()
                } else {
                    Some(token)
                }
            }
        }
    }
}

/// Trait defining a normalizer.
pub trait Normalizer: Sync + Send {
    /// Normalize the provided [`Token`].
    ///
    /// A Normalizer can return several `Token`s.
    fn normalize<'o>(&self, token: Token<'o>) -> Box<dyn Iterator<Item = Token<'o>> + 'o>;

    /// Return true if the normalizer can process Token of a specific [`Script`] and [`Language`].
    ///
    /// Some normalizer are specialized for a `Script` or/and a `Language` and shouldn't be called on every `Token`s.
    fn should_normalize(&self, script: Script, language: Option<Language>) -> bool;
}

/// Trait defining methods to normalize [`Token`]s.
pub trait Normalize<'o>: Iterator
where
    Self: Sized,
    Self: Iterator<Item = Token<'o>> + 'o,
{
    /// Normalize [`Token`]s using all the compatible Normalizers.
    ///
    /// A Latin `Token` would not be normalized the same as a Chinese `Token`.
    fn normalize(self) -> NormalizedTokenIter<'o> {
        let first = NormalizedTokenIter {
            token_iter: Box::new(self),
            inner: Box::new(None.into_iter()),
            normalizer: NORMALIZERS.first().unwrap(),
        };

        NORMALIZERS.iter().skip(1).fold(first, |token_iter, normalizer| NormalizedTokenIter {
            token_iter: Box::new(token_iter),
            inner: Box::new(None.into_iter()),
            normalizer,
        })
    }
}

impl<'o, T> Normalize<'o> for T where T: Iterator<Item = Token<'o>> + 'o {}

#[cfg(test)]
mod test {
    macro_rules! test_normalizer {
        ($normalizer:expr, $tokens:expr, $normalizer_result:expr, $global_result:expr) => {
            use super::*;
            use crate::normalizer::Normalize;
            use crate::{Script, Token};

            #[test]
            fn normalizer_normalize() {
                let normalized_tokens: Vec<_> = $tokens
                    .into_iter()
                    .map(|token| $normalizer.normalize(token))
                    .flatten()
                    .collect();
                assert_eq!(
                    &normalized_tokens[..],
                    $normalizer_result,
                    r#"
Normalizer {} didn't normalize tokens as expected.

help: The `normalizer_result` provided to `test_normalizer!` does not corresponds to the output of the tested normalizer,
it's probably due to a bug in the normalizer or a mistake in the provided normalized tokens.
"#,
                    stringify!($normalizer)
                );
            }

            #[test]
            fn global_normalize() {
                let normalized_tokens: Vec<_> = $tokens.into_iter().normalize().collect();
                assert_eq!(
                    &normalized_tokens[..],
                    $global_result,
                    r#"
Global normalization pipeline didn't normalize tokens as expected.

help: The `global_result` provided to `test_normalizer!` does not corresponds to the output of the normalizer pipeline, it's probably because the normalizer is missing from `NORMALIZERS` list or because an other normalizer has alterated the token.
Check if the `NORMALIZERS` list in `src/normalizer/mod.rs` contains the tested Normalizer. 
Make sure that normalized tokens are valid or change the trigger condition of the noisy normalizers by updating `should_normalize`.
"#
                );
            }
        };
    }
    pub(crate) use test_normalizer;
}
