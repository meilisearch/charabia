use std::collections::HashMap;

use fst::Set;

use crate::detection::{Language, Script};
use crate::normalizer::{NormalizedTokenIter, NormalizerOption};
use crate::segmenter::{Segment, SegmentedStrIter, SegmentedTokenIter};
use crate::Token;

/// Iterator over tuples of [`&str`] (part of the original text) and [`Token`].
pub struct ReconstructedTokenIter<'o, 'al, 'no> {
    token_iter: NormalizedTokenIter<'o, 'al, 'no>,
    original: &'o str,
}

impl<'o> Iterator for ReconstructedTokenIter<'o, '_, '_> {
    type Item = (&'o str, Token<'o>);

    fn next(&mut self) -> Option<Self::Item> {
        self.token_iter
            .next()
            .map(|token| (&self.original[token.byte_start..token.byte_end], token))
    }
}

/// Trait defining methods to tokenize a text.
pub trait Tokenize<'o> {
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
    fn tokenize(&self) -> NormalizedTokenIter;

    /// Same as [`tokenize`] but attaches each [`Token`] to its corresponding portion of the original text.
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
    fn reconstruct(&self) -> ReconstructedTokenIter;
}

impl Tokenize<'_> for &str {
    fn tokenize(&self) -> NormalizedTokenIter {
        const NO: NormalizerOption = NormalizerOption { create_char_map: false, stop_words: None };
        self.segment().normalize(&NO)
    }

    fn reconstruct(&self) -> ReconstructedTokenIter {
        ReconstructedTokenIter { original: self, token_iter: self.tokenize() }
    }
}

/// Structure used to tokenize a text with custom configurations.
///
/// See [`TokenizerBuilder`] to know how to build a [`Tokenizer`].
pub struct Tokenizer<'al, 'no> {
    allow_list: Option<&'al HashMap<Script, Vec<Language>>>,
    normalizer_option: &'no NormalizerOption<'no>,
}

impl<'al, 'no> Tokenizer<'al, 'no> {
    /// Creates an Iterator over [`Token`]s.
    ///
    /// The provided text is segmented creating tokens,
    /// then tokens are normalized and classified.
    pub fn tokenize<'o>(&self, original: &'o str) -> NormalizedTokenIter<'o, 'al, 'no> {
        original.segment_with_allowlist(self.allow_list).normalize(self.normalizer_option)
    }

    /// Same as [`tokenize`] but attaches each [`Token`] to its corresponding portion of the original text.
    pub fn reconstruct<'o>(&self, original: &'o str) -> ReconstructedTokenIter<'o, 'al, 'no> {
        ReconstructedTokenIter { original, token_iter: self.tokenize(original) }
    }

    /// Segments the provided text creating an Iterator over [`Token`].
    pub fn segment<'o>(&self, original: &'o str) -> SegmentedTokenIter<'o, 'al> {
        original.segment_with_allowlist(self.allow_list)
    }

    /// Segments the provided text creating an Iterator over `&str`.
    pub fn segment_str<'o>(&self, original: &'o str) -> SegmentedStrIter<'o, 'al> {
        original.segment_str_with_allowlist(self.allow_list)
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
/// use charabia::TokenizerBuilder;
///
/// // text to tokenize.
/// let orig = "The quick (\"brown\") fox can't jump 32.3 feet, right? Brr, it's 29.3°F!";
///
/// // create the builder.
/// let mut builder = TokenizerBuilder::new();
///
/// // create a set of stop words.
/// let stop_words: Set<Vec<u8>> = Set::from_iter(["the"].iter()).unwrap();
///
/// // configurate stop words.
/// builder.stop_words(&stop_words);
///
/// // build the tokenizer passing the text to tokenize.
/// let tokenizer = builder.build();
/// ```
///
pub struct TokenizerBuilder<'al, 'no, A> {
    allow_list: Option<&'al HashMap<Script, Vec<Language>>>,
    stop_words: Option<&'no Set<A>>,
    normalizer_option: NormalizerOption<'no>,
}

impl<'al, 'no, A> TokenizerBuilder<'al, 'no, A> {
    /// Create a `TokenizerBuilder` with default settings,
    ///
    /// if you don't plan to set stop_words, prefer use [`TokenizerBuilder::default`]
    pub fn new() -> TokenizerBuilder<'al, 'no, A> {
        Self { normalizer_option: NormalizerOption::default(), allow_list: None, stop_words: None }
    }
}

impl<'al, 'no, A: AsRef<[u8]>> TokenizerBuilder<'al, 'no, A> {
    /// Configure the words that will be classified as `TokenKind::StopWord`.
    ///
    /// # Arguments
    ///
    /// * `stop_words` - a `Set` of the words to classify as stop words.
    pub fn stop_words(&mut self, stop_words: &'no Set<A>) -> &mut Self {
        self.stop_words = Some(stop_words);
        self.normalizer_option.stop_words = self.stop_words.map(|sw| {
            let sw = sw.as_fst().as_bytes();
            Set::new(sw).unwrap()
        });
        self
    }

    /// Enable or disable the creation of `char_map`.
    ///
    /// # Arguments
    ///
    /// * `create_char_map` - a `bool` that indicates whether a `char_map` should be created.
    pub fn create_char_map(&mut self, create_char_map: bool) -> &mut Self {
        self.normalizer_option.create_char_map = create_char_map;
        self
    }

    /// Configure which languages can be used for which script
    ///
    /// # Arguments
    ///
    /// * `allow_list` - a `HashMap` of the selection of languages associated with a script to limit during autodetection.
    pub fn allow_list(&mut self, allow_list: &'al HashMap<Script, Vec<Language>>) -> &mut Self {
        self.allow_list = Some(allow_list);
        self
    }

    /// Build the configurated `Tokenizer`.
    pub fn build(&'no self) -> Tokenizer<'al, 'no> {
        Tokenizer { normalizer_option: &self.normalizer_option, allow_list: self.allow_list }
    }
}

impl Default for TokenizerBuilder<'_, '_, Vec<u8>> {
    fn default() -> Self {
        Self::new()
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
            let builder = TokenizerBuilder::default();
            let tokens = {
                let tokenizer = builder.build();
                tokenizer.tokenize(text).collect()
            };
            tokens
        };
        assert_eq!(tokens.iter().last().map(|t| t.lemma()), Some("."));

        let tokens: Vec<_> = {
            let stop_words: Set<Vec<u8>> = Set::from_iter(["to"].iter()).unwrap();
            let mut builder = TokenizerBuilder::new();
            let builder = builder.stop_words(&stop_words);
            let tokens = {
                let tokenizer = builder.build();
                tokenizer.tokenize(text).collect()
            };
            tokens
        };
        assert_eq!(tokens.iter().last().map(|t| t.lemma()), Some("."));
    }
}
