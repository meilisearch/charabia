use fst::Set;

use crate::classifier::{ClassifiedTokenIter, Classify};
use crate::normalizer::Normalize;
use crate::segmenter::Segment;
use crate::Token;

/// Iterator over tuples of [`&str`] (part of the original text) and [`Token`].
pub struct ReconstructedTokenIter<'o, 'sw, A: AsRef<[u8]>> {
    token_iter: ClassifiedTokenIter<'o, 'sw, A>,
    original: &'o str,
}

impl<'o, 'sw, A: AsRef<[u8]>> Iterator for ReconstructedTokenIter<'o, 'sw, A> {
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
    /// use meilisearch_tokenizer::{Token, TokenKind, Tokenize, SeparatorKind};
    ///
    /// let orig = "The quick (\"brown\") fox can't jump 32.3 feet, right? Brr, it's 29.3°F!";
    ///
    /// let mut tokens = orig.tokenize();
    ///
    /// let Token { word, kind, .. } = tokens.next().unwrap();
    /// assert_eq!(word, "the");
    /// assert_eq!(kind, TokenKind::Word);
    ///
    /// let Token { word, kind, .. } = tokens.next().unwrap();
    /// assert_eq!(word, " ");
    /// assert_eq!(kind, TokenKind::Separator(SeparatorKind::Soft));
    ///
    /// let Token { word, kind, .. } = tokens.next().unwrap();
    /// assert_eq!(word, "quick");
    /// assert_eq!(kind, TokenKind::Word);
    /// ```
    fn tokenize(&self) -> ClassifiedTokenIter<'_, '_, A>;

    /// Attaches each [`Token`] to its corresponding portion of the original text.
    ///
    /// # Example
    ///
    /// ```
    /// use meilisearch_tokenizer::{Token, TokenKind, Tokenize, SeparatorKind};
    ///
    /// let orig = "The quick (\"brown\") fox can't jump 32.3 feet, right? Brr, it's 29.3°F!";
    ///
    /// let mut pairs = orig.reconstruct();
    ///
    /// let (s, Token { word, kind, .. }) = pairs.next().unwrap();
    /// assert_eq!(s, "The");
    /// assert_eq!(word, "the");
    /// assert_eq!(kind, TokenKind::Word);
    ///
    /// let (s, Token { word, kind, .. }) = pairs.next().unwrap();
    /// assert_eq!(s, " ");
    /// assert_eq!(word, " ");
    /// assert_eq!(kind, TokenKind::Separator(SeparatorKind::Soft));
    ///
    /// let (s, Token { word, kind, .. }) = pairs.next().unwrap();
    /// assert_eq!(s, "quick");
    /// assert_eq!(word, "quick");
    /// assert_eq!(kind, TokenKind::Word);
    /// ```
    fn reconstruct(&self) -> ReconstructedTokenIter<'_, '_, A>;
}

impl Tokenize<'_, Vec<u8>> for &str {
    fn tokenize(&self) -> ClassifiedTokenIter<'_, '_, Vec<u8>> {
        self.segment().normalize().classify()
    }

    fn reconstruct(&self) -> ReconstructedTokenIter<'_, '_, Vec<u8>> {
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

impl<'o, 'sw, A: AsRef<[u8]>> Tokenize<'_, A> for Tokenizer<'o, 'sw, A> {
    fn tokenize(&self) -> ClassifiedTokenIter<'_, '_, A> {
        self.original.segment().normalize().classify_with_stop_words(self.stop_words)
    }

    fn reconstruct(&self) -> ReconstructedTokenIter<'_, '_, A> {
        ReconstructedTokenIter { token_iter: self.tokenize(), original: self.original }
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
/// use meilisearch_tokenizer::tokenizer::TokenizerBuilder;
///
/// // text to tokenize.
/// let orig = "The quick (\"brown\") fox can't jump 32.3 feet, right? Brr, it's 29.3°F!";
///
/// // create the builder passing the text to tokenize.
/// let builder = TokenizerBuilder::new(orig);
///
/// // create a set of stop words.
/// let stop_words = Set::from_iter(["the"].iter()).unwrap();
///
/// // configurate stop words.
/// let builder = builder.stop_words(&stop_words);
///
/// // build the tokenizer.
/// let tokenizer = builder.build();
/// ```
///
pub struct TokenizerBuilder<'o, 'sw, A> {
    original: &'o str,
    stop_words: Option<&'sw Set<A>>,
}

impl<'o, 'sw> TokenizerBuilder<'o, 'sw, Vec<u8>> {
    /// Create a `TokenizerBuilder` with default settings
    ///
    /// # Arguments
    ///
    /// * `original` - the text to tokenize.
    pub fn new(original: &'o str) -> TokenizerBuilder<'o, 'sw, Vec<u8>> {
        Self { original, stop_words: None }
    }
}

impl<'o, 'sw, A> TokenizerBuilder<'o, 'sw, A> {
    /// Configure the words that will be classified as `TokenKind::StopWord`.
    ///
    /// # Arguments
    ///
    /// * `stop_words` - a `Set` of the words to classify as stop words.
    pub fn stop_words<B: AsRef<[u8]>>(
        self,
        stop_words: &'sw Set<B>,
    ) -> TokenizerBuilder<'o, 'sw, B> {
        TokenizerBuilder { original: self.original, stop_words: Some(stop_words) }
    }

    /// Build the configurated `Tokenizer`.
    pub fn build(self) -> Tokenizer<'o, 'sw, A> {
        Tokenizer { original: self.original, stop_words: self.stop_words }
    }
}
