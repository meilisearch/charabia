use std::borrow::Cow;

use aho_corasick::{AhoCorasick, MatchKind};
use fst::Set;

use crate::detection::Language;
use crate::normalizer::{NormalizedTokenIter, NormalizerOption};
use crate::segmenter::{Segment, SegmentedStrIter, SegmentedTokenIter, SegmenterOption};
use crate::separators::DEFAULT_SEPARATORS;
use crate::Token;

/// Iterator over tuples of [`&str`] (part of the original text) and [`Token`].
pub struct ReconstructedTokenIter<'o, 'aho, 'lang, 'tb> {
    token_iter: NormalizedTokenIter<'o, 'aho, 'lang, 'tb>,
    original: &'o str,
}

impl<'o> Iterator for ReconstructedTokenIter<'o, '_, '_, '_> {
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
        self.segment().normalize(&crate::normalizer::DEFAULT_NORMALIZER_OPTION)
    }

    fn reconstruct(&self) -> ReconstructedTokenIter {
        ReconstructedTokenIter { original: self, token_iter: self.tokenize() }
    }
}

/// Structure used to tokenize a text with custom configurations.
///
/// See [`TokenizerBuilder`] to know how to build a [`Tokenizer`].
#[derive(Debug)]
pub struct Tokenizer<'tb> {
    segmenter_option: Cow<'tb, SegmenterOption<'tb>>,
    normalizer_option: Cow<'tb, NormalizerOption<'tb>>,
}

impl Tokenizer<'_> {
    /// Creates an Iterator over [`Token`]s.
    ///
    /// The provided text is segmented creating tokens,
    /// then tokens are normalized and classified depending on the list of normalizers and classifiers in [`normalizer::NORMALIZERS`].
    pub fn tokenize<'t, 'o>(&'t self, original: &'o str) -> NormalizedTokenIter<'o, 't, 't, 't> {
        original
            .segment_with_option(
                self.segmenter_option.aho.as_ref(),
                self.segmenter_option.allow_list,
            )
            .normalize(&self.normalizer_option)
    }

    /// Creates an Iterator over [`Token`]s.
    ///
    /// The provided text is segmented creating tokens,
    /// then tokens are normalized and classified depending on the list of normalizers and classifiers in [`normalizer::NORMALIZERS`].
    ///
    /// # Arguments
    ///
    /// * `allow_list` - a slice of [`Language`] to allow during autodetection.
    pub fn tokenize_with_allow_list<'t, 'o, 'lang>(
        &'t self,
        original: &'o str,
        allow_list: Option<&'lang [Language]>,
    ) -> NormalizedTokenIter<'o, 't, 'lang, 't> {
        original
            .segment_with_option(self.segmenter_option.aho.as_ref(), allow_list)
            .normalize(&self.normalizer_option)
    }

    /// Same as [`tokenize`] but attaches each [`Token`] to its corresponding portion of the original text.
    pub fn reconstruct<'t, 'o>(
        &'t self,
        original: &'o str,
    ) -> ReconstructedTokenIter<'o, 't, 't, 't> {
        ReconstructedTokenIter { original, token_iter: self.tokenize(original) }
    }

    /// Segments the provided text creating an Iterator over [`Token`].
    pub fn segment<'t, 'o>(&'t self, original: &'o str) -> SegmentedTokenIter<'o, 't, 't> {
        original.segment_with_option(
            self.segmenter_option.aho.as_ref(),
            self.segmenter_option.allow_list,
        )
    }

    /// Segments the provided text creating an Iterator over `&str`.
    pub fn segment_str<'t, 'o>(&'t self, original: &'o str) -> SegmentedStrIter<'o, 't, 't> {
        original.segment_str_with_option(
            self.segmenter_option.aho.as_ref(),
            self.segmenter_option.allow_list,
        )
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
pub struct TokenizerBuilder<'tb, A> {
    stop_words: Option<&'tb Set<A>>,
    words_dict: Option<&'tb [&'tb str]>,
    normalizer_option: NormalizerOption<'tb>,
    segmenter_option: SegmenterOption<'tb>,
}

impl<'tb, A> TokenizerBuilder<'tb, A> {
    /// Create a `TokenizerBuilder` with default settings,
    ///
    /// if you don't plan to set stop_words, prefer use [`TokenizerBuilder::default`]
    pub fn new() -> TokenizerBuilder<'tb, A> {
        Self {
            normalizer_option: crate::normalizer::DEFAULT_NORMALIZER_OPTION,
            segmenter_option: SegmenterOption::default(),
            stop_words: None,
            words_dict: None,
        }
    }
}

impl<'tb, A: AsRef<[u8]>> TokenizerBuilder<'tb, A> {
    /// Configure the words that will be classified as `TokenKind::StopWord`.
    ///
    /// # Arguments
    ///
    /// * `stop_words` - a `Set` of the words to classify as stop words.
    pub fn stop_words(&mut self, stop_words: &'tb Set<A>) -> &mut Self {
        self.stop_words = Some(stop_words);
        self.normalizer_option.classifier.stop_words = self.stop_words.map(|sw| {
            let sw = sw.as_fst().as_bytes();
            Set::new(sw).unwrap()
        });
        self
    }

    /// Configure the words that will be used to separate words and classified as `TokenKind::Separator`.
    ///
    /// # Arguments
    ///
    /// * `separators` - a slice of str to classify as separator.
    ///
    /// # Example
    ///
    /// ```
    /// use charabia::TokenizerBuilder;
    ///
    /// // create the builder.
    /// let mut builder = TokenizerBuilder::default();
    ///
    /// // create a custom list of separators.
    /// let separators = [" ", ", ", ". ", "?", "!"];
    ///
    /// // configurate separators.
    /// builder.separators(&separators);
    ///
    /// // build the tokenizer passing the text to tokenize.
    /// let tokenizer = builder.build();
    ///
    /// // text to tokenize.
    /// let orig = "The quick (\"brown\") fox can't jump 32.3 feet, right? Brr, it's 29.3°F!";
    ///
    /// let output: Vec<_> = tokenizer.segment_str(orig).collect();
    /// assert_eq!(
    ///   &output,
    ///   &["The", " ", "quick", " ", "(\"brown\")", " ", "fox", " ", "can't", " ", "jump", " ", "32.3", " ", "feet", ", ", "right", "?", " ", "Brr", ", ", "it's", " ", "29.3°F", "!"]
    /// );
    /// ```
    ///
    pub fn separators(&mut self, separators: &'tb [&'tb str]) -> &mut Self {
        self.normalizer_option.classifier.separators = Some(separators);
        self
    }

    /// Configure the words that will be segmented before any other segmentation.
    ///
    /// This words dictionary is used to override the segmentation over these words,
    /// the tokenizer will find all the occurences of these words before any Language based segmentation.
    /// If some of the words are in the stop_words' list or in the separators' list,
    /// then they will be categorized as `TokenKind::StopWord` or as `TokenKind::Separator` aswell.
    ///
    /// # Arguments
    ///
    /// * `words` - a slice of str.
    ///
    /// # Example
    ///
    /// ```
    /// use charabia::TokenizerBuilder;
    ///
    /// // create the builder.
    /// let mut builder = TokenizerBuilder::default();
    ///
    /// // create a custom list of words.
    /// let words = ["J. R. R.", "Dr.", "J. K."];
    ///
    /// // configurate words.
    /// builder.words_dict(&words);
    ///
    /// // build the tokenizer passing the text to tokenize.
    /// let tokenizer = builder.build();
    ///
    /// // text to tokenize.
    /// let orig = "J. R. R. Tolkien. J. K. Rowling. Dr. Seuss";
    ///
    /// let output: Vec<_> = tokenizer.segment_str(orig).collect();
    /// assert_eq!(
    ///   &output,
    ///   &["J. R. R.", " ", "Tolkien", ". ", "J. K.", " ", "Rowling", ". ", "Dr.", " ", "Seuss"]
    /// );
    /// ```
    ///
    pub fn words_dict(&mut self, words: &'tb [&'tb str]) -> &mut Self {
        self.words_dict = Some(words);
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

    /// Enable or disable the lossy normalization.
    ///
    /// A lossy normalization is a kind of normalization that could change the meaning in some way.
    /// Removing diacritics is considered lossy; for instance, in French the word `maïs` (`corn`) will be normalized as `mais` (`but`) which changes the meaning.
    ///
    /// # Arguments
    ///
    /// * `lossy` - a `bool` that enable or disable the lossy normalization.
    pub fn lossy_normalization(&mut self, lossy: bool) -> &mut Self {
        self.normalizer_option.lossy = lossy;
        self
    }

    /// Configure which languages can be used for which script
    ///
    /// # Arguments
    ///
    /// * `allow_list` - a `HashMap` of the selection of languages associated with a script to limit during autodetection.
    pub fn allow_list(&mut self, allow_list: &'tb [Language]) -> &mut Self {
        self.segmenter_option.allow_list = Some(allow_list);
        self
    }

    /// Build the configurated `Tokenizer`.
    pub fn build(&mut self) -> Tokenizer {
        // If a custom list of separators or/and a custom list of words have been given,
        // then an Aho-Corasick automaton is created to pre-segment the text during the tokenization process
        // TODO: avoid recreating the automaton if nothing changed
        match (self.normalizer_option.classifier.separators, self.words_dict) {
            (Some(separators), None) => {
                let pattern = separators.iter().filter(|s| !s.is_empty());
                let aho = AhoCorasick::builder()
                    .match_kind(MatchKind::LeftmostLongest)
                    .build(pattern)
                    .unwrap();

                self.segmenter_option.aho = Some(aho).filter(|aho| aho.patterns_len() != 0);
            }
            (separators, Some(words)) => {
                // use the default separators' list if a custom words' list is given but no custom separators' list.
                let separators = separators.unwrap_or(DEFAULT_SEPARATORS);
                // merge both lists together and create the Aho-Corasick automaton.
                let pattern = words.iter().chain(separators).filter(|s| !s.is_empty());
                let aho = AhoCorasick::builder()
                    .match_kind(MatchKind::LeftmostLongest)
                    .build(pattern)
                    .unwrap();

                self.segmenter_option.aho = Some(aho).filter(|aho| aho.patterns_len() != 0);
            }
            // reset the state in case the builder is reused.
            (None, None) => self.segmenter_option.aho = None,
        }

        Tokenizer {
            normalizer_option: Cow::Borrowed(&self.normalizer_option),
            segmenter_option: Cow::Borrowed(&self.segmenter_option),
        }
    }

    /// Build the configurated `Tokenizer` consumming self.
    ///
    /// This method allows to drop the tokenizer builder without having to drop the Tokenizer itself.
    pub fn into_tokenizer(mut self) -> Tokenizer<'tb> {
        drop(self.build());

        Tokenizer {
            normalizer_option: Cow::Owned(self.normalizer_option),
            segmenter_option: Cow::Owned(self.segmenter_option),
        }
    }
}

impl Default for TokenizerBuilder<'_, Vec<u8>> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use fst::Set;
    use quickcheck::quickcheck;

    use crate::{Tokenize, TokenizerBuilder};

    #[test]
    fn check_lifetimes() {
        let text = "Hello world! Pleased to see you.";

        let tokens: Vec<_> = { text.tokenize().collect() };
        assert_eq!(tokens.iter().last().map(|t| t.lemma()), Some("."));

        let tokens: Vec<_> = {
            let mut builder = TokenizerBuilder::default();
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

    #[quickcheck]
    fn shorten_after_tokenized(text: String) -> bool {
        let text = text.as_str();
        let tokens: Vec<_> = text.tokenize().collect();
        tokens.len() <= text.len()
    }
}
