use fst::Set;

use crate::classifier::{ClassifiedTokenIter, Classify};
use crate::normalizer::Normalize;
use crate::segmenter::{Segment, SegmentedTokenIter};
use crate::Token;

/// Iterator over tuples of [`&str`] (part of the original text) and [`Token`].
pub struct ReconstructedTokenIter<'o, 'sw, A: AsRef<[u8]>> {
    token_iter: ClassifiedTokenIter<'o, 'sw, A>,
    original: &'o str,
}

impl<'o, A: AsRef<[u8]>> Iterator for ReconstructedTokenIter<'o, '_, A> {
    type Item = (&'o str, Token<'o>);

    fn next(&mut self) -> Option<Self::Item> {
        self.token_iter
            .next()
            .map(|token| (&self.original[token.byte_start..token.byte_end], token))
    }
}

/// Trait defining methods to tokenize a text.
pub trait Tokenize<'o, A: AsRef<[u8]>> {
    /// Creates an Iterator over [`Token`]s.
    ///
    /// The provided text is segmented creating tokens,
    /// then tokens are normalized and classified.
    ///
    /// # Example
    ///
    /// ```
    /// use charabia::{Token, TokenKind, Tokenize, SeparatorKind};
    ///
    /// let orig = "The quick (\"brown\") fox can't jump 32.3 feet, right? Brr, it's 29.3°F!";
    ///
    /// let mut tokens = orig.tokenize();
    ///
    /// let Token { lemma, kind, .. } = tokens.next().unwrap();
    /// assert_eq!(lemma, "the");
    /// assert_eq!(kind, TokenKind::Word);
    ///
    /// let Token { lemma, kind, .. } = tokens.next().unwrap();
    /// assert_eq!(lemma, " ");
    /// assert_eq!(kind, TokenKind::Separator(SeparatorKind::Soft));
    ///
    /// let Token { lemma, kind, .. } = tokens.next().unwrap();
    /// assert_eq!(lemma, "quick");
    /// assert_eq!(kind, TokenKind::Word);
    /// ```
    fn tokenize(&self) -> ClassifiedTokenIter<'o, '_, A>;

    /// Attaches each [`Token`] to its corresponding portion of the original text.
    ///
    /// # Example
    ///
    /// ```
    /// use charabia::{Token, TokenKind, Tokenize, SeparatorKind};
    ///
    /// let orig = "The quick (\"brown\") fox can't jump 32.3 feet, right? Brr, it's 29.3°F!";
    ///
    /// let mut pairs = orig.reconstruct();
    ///
    /// let (s, Token { lemma, kind, .. }) = pairs.next().unwrap();
    /// assert_eq!(s, "The");
    /// assert_eq!(lemma, "the");
    /// assert_eq!(kind, TokenKind::Word);
    ///
    /// let (s, Token { lemma, kind, .. }) = pairs.next().unwrap();
    /// assert_eq!(s, " ");
    /// assert_eq!(lemma, " ");
    /// assert_eq!(kind, TokenKind::Separator(SeparatorKind::Soft));
    ///
    /// let (s, Token { lemma, kind, .. }) = pairs.next().unwrap();
    /// assert_eq!(s, "quick");
    /// assert_eq!(lemma, "quick");
    /// assert_eq!(kind, TokenKind::Word);
    /// ```
    fn reconstruct(&self) -> ReconstructedTokenIter<'o, '_, A>;
}

impl<'o> Tokenize<'o, Vec<u8>> for &'o str {
    fn tokenize(&self) -> ClassifiedTokenIter<'o, '_, Vec<u8>> {
        self.segment().normalize().classify()
    }

    fn reconstruct(&self) -> ReconstructedTokenIter<'o, '_, Vec<u8>> {
        ReconstructedTokenIter { token_iter: self.tokenize(), original: self }
    }
}

/// Structure used to tokenize a text with custom configurations.
///
/// See [`TokenizerBuilder`] to know how to build a [`Tokenizer`].
pub struct Tokenizer<'o, 'sw, A> {
    original: &'o str,
    stop_words: Option<&'sw Set<A>>,
}

impl<'o, A: AsRef<[u8]>> Tokenize<'o, A> for Tokenizer<'o, '_, A> {
    fn tokenize(&self) -> ClassifiedTokenIter<'o, '_, A> {
        self.original.segment().normalize().classify_with_stop_words(self.stop_words)
    }

    fn reconstruct(&self) -> ReconstructedTokenIter<'o, '_, A> {
        ReconstructedTokenIter { original: self.original, token_iter: self.tokenize() }
    }
}

impl<'o, A: AsRef<[u8]>> Segment<'o> for Tokenizer<'o, '_, A> {
    fn segment(&self) -> SegmentedTokenIter<'o> {
        self.original.segment()
    }

    fn segment_str(&self) -> Box<dyn Iterator<Item = &'o str> + 'o> {
        self.original.segment_str()
    }
}

/// Structure to build a tokenizer with custom settings.
///
/// To use default settings, use directly the `Tokenize` implementation on &str.
///
/// # Example
///
/// ```
/// use fst::Set;
///
/// use charabia::tokenizer::TokenizerBuilder;
///
/// // text to tokenize.
/// let orig = "The quick (\"brown\") fox can't jump 32.3 feet, right? Brr, it's 29.3°F!";
///
/// // create the builder.
/// let builder = TokenizerBuilder::new();
///
/// // create a set of stop words.
/// let stop_words = Set::from_iter(["the"].iter()).unwrap();
///
/// // configurate stop words.
/// let builder = builder.stop_words(&stop_words);
///
/// // build the tokenizer passing the text to tokenize.
/// let tokenizer = builder.build(orig);
/// ```
///
pub struct TokenizerBuilder<'sw, A> {
    stop_words: Option<&'sw Set<A>>,
}

impl<'sw> TokenizerBuilder<'sw, Vec<u8>> {
    /// Create a `TokenizerBuilder` with default settings
    pub fn new() -> TokenizerBuilder<'sw, Vec<u8>> {
        Self { stop_words: None }
    }
}

impl<'sw, A> TokenizerBuilder<'sw, A> {
    /// Configure the words that will be classified as `TokenKind::StopWord`.
    ///
    /// # Arguments
    ///
    /// * `stop_words` - a `Set` of the words to classify as stop words.
    pub fn stop_words<B: AsRef<[u8]>>(self, stop_words: &'sw Set<B>) -> TokenizerBuilder<'sw, B> {
        TokenizerBuilder { stop_words: Some(stop_words) }
    }

    /// Build the configurated `Tokenizer`.
    pub fn build<'o>(&self, original: &'o str) -> Tokenizer<'o, 'sw, A> {
        Tokenizer { original: original, stop_words: self.stop_words }
    }
}

#[cfg(test)]
mod test {
    use fst::Set;

    use crate::tokenizer::{Tokenize, TokenizerBuilder};

    #[test]
    fn check_lifetimes() {
        let text = "Hello world! Pleased to see you.";

        let tokens: Vec<_> = { text.tokenize().collect() };
        assert_eq!(tokens.iter().last().map(|t| t.lemma()), Some("."));

        let tokens: Vec<_> = {
            let builder = TokenizerBuilder::new();
            let tokens = {
                let tokenizer = builder.build(text);
                tokenizer.tokenize().collect()
            };
            tokens
        };
        assert_eq!(tokens.iter().last().map(|t| t.lemma()), Some("."));

        let tokens: Vec<_> = {
            let stop_words = Set::from_iter(["to"].iter()).unwrap();
            let builder = TokenizerBuilder::new();
            let builder = builder.stop_words(&stop_words);
            let tokens = {
                let tokenizer = builder.build(text);
                tokenizer.tokenize().collect()
            };
            tokens
        };
        assert_eq!(tokens.iter().last().map(|t| t.lemma()), Some("."));
    }
}
