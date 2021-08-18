pub mod analyzer;
pub mod detection;
pub mod normalizer;
pub mod processors;
pub mod token;
pub mod tokenizer;

mod token_classifier;

pub use analyzer::{Analyzer, AnalyzerConfig};
pub use token::{Token, TokenKind};
