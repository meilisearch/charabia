//! Charabia library tokenize a text detecting the Script/Language, segmenting, normalizing, and classifying it.
//!
//! Examples
//! --------
//! #### Tokenization
//! ```
//! use charabia::{Token, TokenKind, Tokenize, SeparatorKind};
//!
//! let orig = "The quick (\"brown\") fox can't jump 32.3 feet, right? Brr, it's 29.3°F!";
//!
//! // tokenize the text.
//! let mut tokens = orig.tokenize();
//!
//! let Token { lemma, kind, .. } = tokens.next().unwrap();
//! assert_eq!(lemma, "the");
//! assert_eq!(kind, TokenKind::Word);
//!
//! let Token { lemma, kind, .. } = tokens.next().unwrap();
//! assert_eq!(lemma, " ");
//! assert_eq!(kind, TokenKind::Separator(SeparatorKind::Soft));
//!
//! let Token { lemma, kind, .. } = tokens.next().unwrap();
//! assert_eq!(lemma, "quick");
//! assert_eq!(kind, TokenKind::Word);
//! ```
//!
//! #### Segmentation
//! ```
//! use charabia::Segment;
//!
//! let orig = "The quick (\"brown\") fox can't jump 32.3 feet, right? Brr, it's 29.3°F!";
//!
//! let mut segments = orig.segment_str();
//!
//! assert_eq!(segments.next(), Some("The"));
//! assert_eq!(segments.next(), Some(" "));
//! assert_eq!(segments.next(), Some("quick"));
//! ```
//!
//! Build features
//! --------
//! Charabia comes with default features that can be deactivated at compile time,
//! this features are additional Language supports that need to download and/or build a specialized dictionary that impact the compilation time.
//! Theses features are listed in charabia's `cargo.toml` and can be deactivated via [dependency features](https://doc.rust-lang.org/cargo/reference/features.html#dependency-features).

pub mod classifier;
pub mod normalizer;
pub mod segmenter;
pub mod tokenizer;

mod detection;
mod token;

pub use classifier::Classify;
pub use detection::{Language, Script};
pub use normalizer::Normalize;
pub use segmenter::Segment;
pub use token::{SeparatorKind, Token, TokenKind};
pub use tokenizer::Tokenize;
