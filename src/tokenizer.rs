use std::collections::HashMap;
use std::collections::HashSet;

use once_cell::sync::Lazy;

use crate::Token;
use crate::internal_tokenizer::{Jieba, UnicodeSegmenter, TokenStream, InternalTokenizer};
use crate::normalizer::{Normalizer, IdentityNormalizer, TokenClassifier};
use crate::processors::{PreProcessor, IdentityPreProcessor, ProcessedText};

static DEFAULT_PIPELINE: Lazy<Pipeline> = Lazy::new(|| Pipeline::default());

pub struct Pipeline {
    pre_processor: Box<dyn PreProcessor + 'static>,
    tokenizer: Box<dyn InternalTokenizer + 'static>,
    normalizer: Box<dyn Normalizer + 'static>,
}

impl Default for Pipeline {
    fn default() -> Self {
        Self {
            pre_processor: Box::new(IdentityPreProcessor),
            tokenizer: Box::new(UnicodeSegmenter),
            normalizer: Box::new(IdentityNormalizer),
        }
    }
}

impl Pipeline {
    fn set_tokenizer(mut self, tokenizer: impl InternalTokenizer + 'static) -> Self {
        self.tokenizer = Box::new(tokenizer);
        self
    }

    fn set_normalizer(mut self, normalizer: impl Normalizer + 'static) -> Self {
        self.normalizer = Box::new(normalizer);
        self
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Language {
    English,
    Other,
}

macro_rules! make_script {
    ($($script:tt), +) => {
        #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
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

pub struct AnalyzerConfig {
    /// language specialized tokenizer, this can be switched during
    /// document tokenization if the document contains several languages
    pub pipeline_map: HashMap<(Script, Language), Pipeline>,
}

impl Default for AnalyzerConfig {
    fn default() -> Self {
        let mut pipeline_map: HashMap<(Script, Language), Pipeline> = HashMap::new();
        pipeline_map.insert((Script::Latin, Language::Other), Pipeline::default());
        pipeline_map.insert((Script::Mandarin, Language::Other), Pipeline::default().set_tokenizer(Jieba::default()));

        AnalyzerConfig { pipeline_map }
    }
}

impl AnalyzerConfig {
    pub fn default_with_classfier(stop_words: HashSet<String>, soft_separators: HashSet<char>, hard_separators: HashSet<char>) -> Self {
        let mut pipeline_map: HashMap<(Script, Language), Pipeline> = HashMap::new();
        let classifier = Box::new(TokenClassifier::new(stop_words, soft_separators, hard_separators));

        pipeline_map.insert((Script::Latin, Language::Other), Pipeline::default().set_normalizer(classifier.clone()));
        pipeline_map.insert((Script::Mandarin, Language::Other), Pipeline::default().set_tokenizer(Jieba::default()).set_normalizer(classifier));

        AnalyzerConfig { pipeline_map }
    }
}

pub struct Analyzer {
    config: AnalyzerConfig,
}

pub struct AnalyzedText<'a> {
    /// Reference to the original text
    text: &'a str,
    /// Processed text
    processed: ProcessedText<'a>,
    /// Pipeline used to proccess the text
    pipeline: &'a Pipeline,
}

impl<'a> AnalyzedText<'a> {
    /// Returns a `TokenStream` for the Analyzed text.
    pub fn tokens(&'a self) -> TokenStream<'a> {
        let stream = self.pipeline.tokenizer
            .tokenize(&self.processed)
            .map(move |t| self.pipeline.normalizer.normalize(t));
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
            config,
        }
    }

    /// Builds an `AnalyzedText` instance with the correct analyzer pipeline, and pre-processes the
    /// text.
    ///
    /// If an analysis pipeline exists for the inferred `(Script, Language)`, the analyzer will look
    /// for a user specified default `(Script::Other, Language::Other)`. If the user default is not
    /// specified, it will fallback to `(IdentityPreProcessor, UnicodeSegmenter, IdentityNormalizer)`.
    ///
    /// ```rust
    /// use meilisearch_tokenizer::{Analyzer, AnalyzerConfig};
    /// // defaults to unicode segmenter with identity preprocessor and normalizer.
    /// let analyzer = Analyzer::new(AnalyzerConfig::default());
    /// let analyzed = analyzer.analyze("The quick (\"brown\") fox can't jump 32.3 feet, right? Brr, it's 29.3°F!");
    /// let mut tokens = analyzed.tokens();
    /// assert!("The" == tokens.next().unwrap().text());
    /// ```
    pub fn analyze<'a>(&'a self, text: &'a str) -> AnalyzedText<'a> {
        let pipeline = self.pipeline_from(text);
        let processed = pipeline.pre_processor.process(text);

        AnalyzedText {
            text,
            processed,
            pipeline,
        }
    }

    /// Try to Detect Language and Script and return the corresponding pipeline,
    /// if no Language is detected or no pipeline corresponds to the Language
    /// the function try to get a pipeline corresponding to the script;
    /// if no Script is detected or no pipeline corresponds to the Script,
    /// the function try to get the default pipeline in the map;
    /// if no default pipeline exist in the map return the librairy DEFAULT_PIPELINE.
    fn pipeline_from(&self, text: &str) -> &Pipeline {
        let script = self.detect_script(text);
        let language = self.detect_lang(text);
        self.config.pipeline_map.get(&(script, language))
            .or_else(|| self.config.pipeline_map.get(&(script, Language::Other)))
            .or_else(|| self.config.pipeline_map.get(&(Script::Other, Language::Other)))
            .unwrap_or_else(|| &*DEFAULT_PIPELINE)
    }

    /// detect script with whatlang,
    /// if no script is detected, return Script::Other
    fn detect_script(&self, text: &str) -> Script {
        whatlang::detect_script(text).map(Script::from).unwrap_or(Script::Other)
    }

    /// detect lang (dummy)
    fn detect_lang(&self, _text: &str) -> Language {
        Language::Other
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use crate::normalizer::LowercaseNormalizer;

    #[test]
    fn test_simple_latin() {
        let analyzer = Analyzer::new(AnalyzerConfig::default());

        let orig = "The quick (\"brown\") fox can't jump 32.3 feet, right? Brr, it's 29.3°F!";
        let analyzed = analyzer.analyze(orig);
        let mut analyzed = analyzed.tokens();
        assert_eq!("The", analyzed.next().unwrap().text());
        assert_eq!(" ", analyzed.next().unwrap().text());
        assert_eq!("quick", analyzed.next().unwrap().text());
        assert_eq!(" ", analyzed.next().unwrap().text());
        assert_eq!("(", analyzed.next().unwrap().text());
        assert_eq!("\"", analyzed.next().unwrap().text());
        assert_eq!("brown", analyzed.next().unwrap().text());
    }

    #[test]
    fn test_simple_chinese() {
        let analyzer = Analyzer::new(AnalyzerConfig::default());

        let orig = "為一包含一千多萬目詞的帶標記平衡語料庫";
        let analyzed = analyzer.analyze(orig);
        let mut analyzed = analyzed.tokens();
        assert_eq!("為", analyzed.next().unwrap().text());
        assert_eq!("一", analyzed.next().unwrap().text());
        assert_eq!("包含", analyzed.next().unwrap().text());
        assert_eq!("一千多", analyzed.next().unwrap().text());
        assert_eq!("萬", analyzed.next().unwrap().text());
        assert_eq!("目", analyzed.next().unwrap().text());
        assert_eq!("詞", analyzed.next().unwrap().text());
        assert_eq!("的", analyzed.next().unwrap().text());
    }

    #[test]
    fn test_simple_latin_with_lowercase_normalizer() {
        let mut pipeline_map: HashMap<(Script, Language), Pipeline> = HashMap::new();
        pipeline_map.insert((Script::Latin, Language::Other), Pipeline::default().set_normalizer(LowercaseNormalizer));

        let analyzer = Analyzer::new(AnalyzerConfig { pipeline_map });
        let orig = "The quick (\"brown\") fox can't jump 32.3 feet, right? Brr, it's 29.3°F!";
        let analyzed = analyzer.analyze(orig);
        assert_eq!("the", analyzed.tokens().next().unwrap().text());
    }

    #[test]
    fn test_reconstruct_latin() {
        let analyzer = Analyzer::new(AnalyzerConfig::default());
        let orig = "The quick (\"brown\") fox can't jump 32.3 feet, right? Brr, it's 29.3°F!";
        let tokens = analyzer.analyze(orig);
        assert_eq!(orig, tokens.reconstruct().map(|(t, _)| t).collect::<String>());
    }

    #[test]
    fn test_reconstruct_chinese() {
        let analyzer = Analyzer::new(AnalyzerConfig::default());
        let orig = "為一包含一千多萬目詞的帶標記平衡語料庫";
        let tokens = analyzer.analyze(orig);
        assert_eq!(orig, tokens.reconstruct().map(|(t, _)| t).collect::<String>());
    }
}
