use charabia::Tokenize;

fn main() {
    println!(
        "{:?}",
        "Hello, world! 東京のお寿司。".tokenize().map(|t| t.lemma().to_owned()).collect::<Vec<_>>()
    );
}
