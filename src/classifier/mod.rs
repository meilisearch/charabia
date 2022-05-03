mod classifier;

use classifier::TokenClassifier;
use fst::Set;

use crate::Token;

/// Iterator over classified [`Token`]s.
pub struct ClassifiedTokenIter<'o, 'sw, A> {
    inner: Box<dyn Iterator<Item = Token<'o>> + 'o>,
    classifier: TokenClassifier<'sw, A>,
}

impl<'o, A: AsRef<[u8]>> Iterator for ClassifiedTokenIter<'o, '_, A> {
    type Item = Token<'o>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.classifier.classify(self.inner.next()?))
    }
}

/// Trait defining methods to classify [`Token`]s.
pub trait Classify<'o>: Iterator
where
    Self: Sized,
    Self: Iterator<Item = Token<'o>> + 'o,
{
    /// Assign to each [`Token`]s a [`TokenKind`].
    ///
    /// [`TokenKind`]: crate::TokenKind
    fn classify(self) -> ClassifiedTokenIter<'o, 'o, Vec<u8>> {
        self.classify_with_stop_words(None)
    }

    /// Assign to each [`Token`]s a [`TokenKind`] using provided stop words.
    ///
    /// [`TokenKind`]: crate::TokenKind
    ///
    /// Any `Token` that is in the stop words [`Set`] is assigned to [`TokenKind::StopWord`].
    ///
    /// [`TokenKind::StopWord`]: crate::TokenKind#StopWord
    fn classify_with_stop_words<'sw, A: AsRef<[u8]>>(
        self,
        stop_words: Option<&'sw Set<A>>,
    ) -> ClassifiedTokenIter<'o, 'sw, A> {
        ClassifiedTokenIter { inner: Box::new(self), classifier: TokenClassifier::new(stop_words) }
    }
}

impl<'o, T> Classify<'o> for T where T: Iterator<Item = Token<'o>> + 'o {}
