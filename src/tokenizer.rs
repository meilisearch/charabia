use std::borrow::Cow;

use std::collections::HashMap;
use crate::internal_tokenizer::InternalTokenizer;
use crate::Token;

pub trait PreProcesor {
    fn process<'a>(&self, s: &'a str) -> Cow<'a, str>;
}

pub trait Normalizer {
    fn normalize<'a>(&self, token: Token<'a>) -> Token<'a>;
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Language;
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Script;

pub struct AnalyzerConfig {
    pub tokenizer_map: HashMap<(Script, Language), (Box<dyn PreProcesor>, Box<dyn InternalTokenizer>, Box<dyn Normalizer>)>,
}

pub struct Analyzer {
    /// script specialized tokenizer, this can be switched during
    /// document tokenization if the document contains several scripts
    tokenizer_map: HashMap<(Script, Language), (Box<dyn PreProcesor>, Box<dyn InternalTokenizer>, Box<dyn Normalizer>)>,
}

impl Analyzer {
    /// create a new tokenizer detecting script
    /// and chose the specialized internal tokenizer
    pub fn new(config: AnalyzerConfig) -> Self {
        Self {
            tokenizer_map: config.tokenizer_map,
        }
    }
    // Analyses the text (typically a field) and select the correct tokenizer from the tokenizer_map, return an iterator of token groups, from the Cow<[Token<'a'>]> emitted from the internal tokenizer
    pub fn tokenize<'a>(&self, s: &'a str) -> impl Iterator<Item = Token<'a>> { 
        let tokenizer = self.tokenizer_map.get(&(Script, Language)).unwrap();
        tokenizer.1.tokenize(s)
    }
}
