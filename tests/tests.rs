use meilisearch_tokenizer::analyzer::{Analyzer, AnalyzerConfig};

#[test]
fn test_apostrophe_latin() {
    let config = AnalyzerConfig::<Vec<u8>>::default();
    let analyzer = Analyzer::new(config);
    let analyzed = analyzer.analyze("Zut, l’aspirateur, j’ai oublié de l’éteindre !");
    println!("{:?}", analyzed.tokens().map(|t| t.text().to_string()).collect::<Vec<_>>());
    println!("{:?}", analyzed.reconstruct().map(|(s, _)| s.to_string()).collect::<String>());
}
//#[cfg(test)]
//mod tests {
//use meilisearch_tokenizer::*;

//#[test]
//fn easy() {
//let mut tokenizer = Tokenizer::new("salut");

//assert_eq!(
//tokenizer.next(),
//Some(Token {
//word: "salut",
//index: 0,
//word_index: 0,
//char_index: 0
//})
//);
//assert_eq!(tokenizer.next(), None);

//let mut tokenizer = Tokenizer::new("yo    ");

//assert_eq!(
//tokenizer.next(),
//Some(Token {
//word: "yo",
//index: 0,
//word_index: 0,
//char_index: 0
//})
//);
//assert_eq!(tokenizer.next(), None);
//}

//#[test]
//fn hard() {
//let mut tokenizer = Tokenizer::new(" .? yo lolo. aïe (ouch)");

//assert_eq!(
//tokenizer.next(),
//Some(Token {
//word: "yo",
//index: 0,
//word_index: 0,
//char_index: 4
//})
//);
//assert_eq!(
//tokenizer.next(),
//Some(Token {
//word: "lolo",
//index: 1,
//word_index: 1,
//char_index: 7
//})
//);
//assert_eq!(
//tokenizer.next(),
//Some(Token {
//word: "aïe",
//index: 2,
//word_index: 9,
//char_index: 13
//})
//);
//assert_eq!(
//tokenizer.next(),
//Some(Token {
//word: "ouch",
//index: 3,
//word_index: 17,
//char_index: 18
//})
//);
//assert_eq!(tokenizer.next(), None);

//let mut tokenizer = Tokenizer::new("yo ! lolo ? wtf - lol . aïe ,");

//assert_eq!(
//tokenizer.next(),
//Some(Token {
//word: "yo",
//index: 0,
//word_index: 0,
//char_index: 0
//})
//);
//assert_eq!(
//tokenizer.next(),
//Some(Token {
//word: "lolo",
//index: 1,
//word_index: 8,
//char_index: 5
//})
//);
//assert_eq!(
//tokenizer.next(),
//Some(Token {
//word: "wtf",
//index: 2,
//word_index: 16,
//char_index: 12
//})
//);
//assert_eq!(
//tokenizer.next(),
//Some(Token {
//word: "lol",
//index: 3,
//word_index: 17,
//char_index: 18
//})
//);
//assert_eq!(
//tokenizer.next(),
//Some(Token {
//word: "aïe",
//index: 4,
//word_index: 25,
//char_index: 24
//})
//);
//assert_eq!(tokenizer.next(), None);
//}

//#[test]
//fn hard_long_chars() {
//let mut tokenizer = Tokenizer::new(" .? yo 😂. aïe");

//assert_eq!(
//tokenizer.next(),
//Some(Token {
//word: "yo",
//index: 0,
//word_index: 0,
//char_index: 4
//})
//);
//assert_eq!(
//tokenizer.next(),
//Some(Token {
//word: "😂",
//index: 1,
//word_index: 1,
//char_index: 7
//})
//);
//assert_eq!(
//tokenizer.next(),
//Some(Token {
//word: "aïe",
//index: 2,
//word_index: 9,
//char_index: 10
//})
//);
//assert_eq!(tokenizer.next(), None);

//let mut tokenizer = Tokenizer::new("yo ! lolo ? 😱 - lol . 😣 ,");

//assert_eq!(
//tokenizer.next(),
//Some(Token {
//word: "yo",
//index: 0,
//word_index: 0,
//char_index: 0
//})
//);
//assert_eq!(
//tokenizer.next(),
//Some(Token {
//word: "lolo",
//index: 1,
//word_index: 8,
//char_index: 5
//})
//);
//assert_eq!(
//tokenizer.next(),
//Some(Token {
//word: "😱",
//index: 2,
//word_index: 16,
//char_index: 12
//})
//);
//assert_eq!(
//tokenizer.next(),
//Some(Token {
//word: "lol",
//index: 3,
//word_index: 17,
//char_index: 16
//})
//);
//assert_eq!(
//tokenizer.next(),
//Some(Token {
//word: "😣",
//index: 4,
//word_index: 25,
//char_index: 22
//})
//);
//assert_eq!(tokenizer.next(), None);
//}

//#[test]
//fn hard_kanjis() {
//let mut tokenizer = Tokenizer::new("\u{2ec4}lolilol\u{2ec7}");

//assert_eq!(
//tokenizer.next(),
//Some(Token {
//word: "\u{2ec4}",
//index: 0,
//word_index: 0,
//char_index: 0
//})
//);
//assert_eq!(
//tokenizer.next(),
//Some(Token {
//word: "lolilol",
//index: 1,
//word_index: 1,
//char_index: 1
//})
//);
//assert_eq!(
//tokenizer.next(),
//Some(Token {
//word: "\u{2ec7}",
//index: 2,
//word_index: 2,
//char_index: 8
//})
//);
//assert_eq!(tokenizer.next(), None);

//let mut tokenizer = Tokenizer::new("\u{2ec4}\u{2ed3}\u{2ef2} lolilol - hello    \u{2ec7}");

//assert_eq!(
//tokenizer.next(),
//Some(Token {
//word: "\u{2ec4}",
//index: 0,
//word_index: 0,
//char_index: 0
//})
//);
//assert_eq!(
//tokenizer.next(),
//Some(Token {
//word: "\u{2ed3}",
//index: 1,
//word_index: 1,
//char_index: 1
//})
//);
//assert_eq!(
//tokenizer.next(),
//Some(Token {
//word: "\u{2ef2}",
//index: 2,
//word_index: 2,
//char_index: 2
//})
//);
//assert_eq!(
//tokenizer.next(),
//Some(Token {
//word: "lolilol",
//index: 3,
//word_index: 3,
//char_index: 4
//})
//);
//assert_eq!(
//tokenizer.next(),
//Some(Token {
//word: "hello",
//index: 4,
//word_index: 4,
//char_index: 14
//})
//);
//assert_eq!(
//tokenizer.next(),
//Some(Token {
//word: "\u{2ec7}",
//index: 5,
//word_index: 5,
//char_index: 23
//})
//);
//assert_eq!(tokenizer.next(), None);
//}
//}
