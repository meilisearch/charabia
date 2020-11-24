use meilisearch_tokenizer::{Analyzer, AnalyzerConfig};
use meilisearch_tokenizer::token::SeparatorKind;

#[test]
fn test() {
    let stop_words = ["the", "of"].iter().cloned().map(String::from).collect();
    let soft_separators = [' ', ','].iter().cloned().collect();
    let hard_separators = ['.'].iter().cloned().collect();
    let analyzer = Analyzer::new(AnalyzerConfig::default_with_classfier(stop_words, soft_separators, hard_separators));
    let analyzed = analyzer.analyze("Hello, the dog.");
    let mut tokens = analyzed.tokens();
    assert!(tokens.next().unwrap().is_word());
    assert_eq!(tokens.next().unwrap().is_separator(), Some(SeparatorKind::Soft));
    assert_eq!(tokens.next().unwrap().is_separator(), Some(SeparatorKind::Soft));
    assert!(tokens.next().unwrap().is_stopword());
    assert_eq!(tokens.next().unwrap().is_separator(), Some(SeparatorKind::Soft));
    assert!(tokens.next().unwrap().is_word());
    assert_eq!(tokens.next().unwrap().is_separator(), Some(SeparatorKind::Hard));
    assert!(tokens.next().is_none());
}
