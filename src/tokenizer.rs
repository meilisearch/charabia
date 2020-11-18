use std::borrow::Cow;
use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::Token;
use crate::internal_tokenizer::{UnicodeSegmenter, TokenStream, InternalTokenizer};
use crate::normalizer::{Normalizer, IdentityNormalizer};
use crate::processors::{PreProcessor, IdentityPreProcessor};

pub type Pipeline = (Box<dyn PreProcessor + 'static>, Box<dyn InternalTokenizer + 'static>, Box<dyn Normalizer + 'static>);

static DEFAULT_ANALYZER: Lazy<Pipeline> = Lazy::new(||
    (Box::new(IdentityPreProcessor), Box::new(UnicodeSegmenter), Box::new(IdentityNormalizer)));

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Language {
    English,
    Other,
}

macro_rules! make_script {
    ($($script:tt), +) => {
        #[derive(Debug, PartialEq, Eq, Hash)]
        pub enum Script {
            $($script),+,
            Other,
        }

        impl From<whatlang::Script> for Script {
            fn from(other: whatlang::Script) -> Script {
                match other {
                    $(whatlang::Script::$script => Script::$script), +
                }
            }

        }
    };
}

make_script! {
    Arabic,
    Bengali,
    Cyrillic,
    Devanagari,
    Ethiopic,
    Georgian,
    Greek,
    Gujarati,
    Gurmukhi,
    Hangul,
    Hebrew,
    Hiragana,
    Kannada,
    Katakana,
    Khmer,
    Latin,
    Malayalam,
    Mandarin,
    Myanmar,
    Oriya,
    Sinhala,
    Tamil,
    Telugu,
    Thai
}

#[derive(Default)]
pub struct AnalyzerConfig {
    pub tokenizer_map: HashMap<(Script, Language), Pipeline>,
}

pub struct Analyzer {
    /// script specialized tokenizer, this can be switched during
    /// document tokenization if the document contains several scripts
    tokenizer_map: HashMap<(Script, Language), Pipeline>,
}

pub struct AnalyzedText<'a> {
    /// Reference to the original text
    text: &'a str,
    /// Processed text
    processed: Cow<'a, str>,
    /// Pipeline used to proccess the text
    pipeline: &'a Pipeline,
}

impl<'a> AnalyzedText<'a> {
    /// Returns a `TokenStream` for the Analyzed text.
    pub fn tokens(&'a self) -> TokenStream<'a> {
        let stream = self.pipeline.1
            .tokenize(self.processed.as_ref())
            .map(move |t| self.pipeline.2.normalize(t));
        TokenStream {
            inner: Box::new(stream)
        }
    }

    /// Attaches each token to its corresponding portion of the original text.
    pub fn reconstruct(&'a self) -> impl Iterator<Item = (&'a str, Token<'a>)> {
        self.tokens().map(move |t| (&self.text[t.byte_start..t.byte_end], t))
    } 
}

impl Analyzer {
    /// create a new tokenizer detecting script
    /// and chose the specialized internal tokenizer
    pub fn new(config: AnalyzerConfig) -> Self {
        Self {
            tokenizer_map: config.tokenizer_map,
        }
    }

    /// Builds an `AnalyzedText` instance with the correct analyzer pipeline, and pre-processes the
    /// text. E.G:
    /// ```rust
    /// use meilisearch_tokenizer::{Analyzer, AnalyzerConfig};
    /// // defaults to unicode segmenter with identity preprocessor and normalizer.
    /// let analyzer = Analyzer::new(AnalyzerConfig::default());
    /// let analyzed = analyzer.analyze("The quick (\"brown\") fox can't jump 32.3 feet, right? Brr, it's 29.3°F!");
    /// let mut tokens = analyzed.tokens();
    /// assert!("The" == tokens.next().unwrap().text());
    /// ```
    pub fn analyze<'a>(&'a self, text: &'a str) -> AnalyzedText<'a> { 
        let tuple_lang = detect_lang(text);
        let pipeline = self.tokenizer_map.get(&tuple_lang).unwrap_or_else(|| &*DEFAULT_ANALYZER);
        let processed = pipeline.0.process(text);
        AnalyzedText {
            text,
            processed,
            pipeline,
        }
    }
}

fn detect_lang(s: &str) -> (Script, Language) {
    let script = whatlang::detect_script(s)
        .map(Script::from)
        .unwrap_or(Script::Other);
    (script, Language::Other)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::normalizer::LowercaseNormalizer;

    #[test]
    fn test_simple() {
        let analyzer = Analyzer::new(AnalyzerConfig::default());
        let orig = "The quick (\"brown\") fox can't jump 32.3 feet, right? Brr, it's 29.3°F!";
        let tokens = analyzer.analyze(orig);
        assert_eq!(orig, tokens.tokens().map(|t| &orig[t.byte_start..t.byte_end]).collect::<String>());
    }

    #[test]
    fn test_simple2() {
        let mut tokenizer_map: HashMap<(Script, Language), Pipeline> = HashMap::new();
        tokenizer_map.insert((Script::Latin, Language::Other), (Box::new(IdentityPreProcessor), Box::new(UnicodeSegmenter), Box::new(LowercaseNormalizer)));
        let analyzer = Analyzer::new(AnalyzerConfig { tokenizer_map });
        let orig = "The quick (\"brown\") fox can't jump 32.3 feet, right? Brr, it's 29.3°F!";
        let analyzed = analyzer.analyze(orig);
        assert_eq!("the", analyzed.tokens().next().unwrap().text());
    }

    #[test]
    fn test_reconstruct() {
        let analyzer = Analyzer::new(AnalyzerConfig::default());
        let orig = "The quick (\"brown\") fox can't jump 32.3 feet, right? Brr, it's 29.3°F!";
        let tokens = analyzer.analyze(orig);
        assert_eq!(orig, tokens.reconstruct().map(|(t, _)| t).collect::<String>());
    }
}
