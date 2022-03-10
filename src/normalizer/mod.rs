use lowercase::LowercaseNormalizer;
use once_cell::sync::Lazy;

use crate::detection::{Language, Script};
use crate::Token;

mod lowercase;

/// List of [`Normalizer`]s used by [`Normalize::normalize`].
pub static NORMALIZERS: Lazy<[Box<dyn Normalizer>; 1]> =
    Lazy::new(|| [Box::new(LowercaseNormalizer)]);

/// Iterator over Normalized [`Token`]s.
pub struct NormalizedTokenIter<'a> {
    token_iter: Box<dyn Iterator<Item = Token<'a>> + 'a>,
    inner: Box<dyn Iterator<Item = Token<'a>> + 'a>,
    normalizer: &'static Box<dyn Normalizer>,
}

impl<'a> Iterator for NormalizedTokenIter<'a> {
    type Item = Token<'a>;

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
    fn normalize<'a>(&self, token: Token<'a>) -> Box<dyn Iterator<Item = Token<'a>> + 'a>;

    /// Return true if the normalizer can process Token of a specific [`Script`] and [`Language`].
    ///
    /// Some normalizer are specialized for a `Script` or/and a `Language` and shouldn't be called on every `Token`s.
    fn should_normalize(&self, script: Script, language: Option<Language>) -> bool;
}

/// Trait defining methods to normalize [`Token`]s.
pub trait Normalize<'a>: Iterator
where
    Self: Sized,
    Self: Iterator<Item = Token<'a>> + 'a,
{
    /// Normalize [`Token`]s using all the compatible Normalizers.
    ///
    /// A Latin `Token` would not be normalized the same as a Chinese `Token`.
    fn normalize(self) -> NormalizedTokenIter<'a> {
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

impl<'a, T> Normalize<'a> for T where T: Iterator<Item = Token<'a>> + 'a {}
