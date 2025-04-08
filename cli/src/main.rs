use charabia::{Language, TokenizerBuilder};
use clap::{Parser, ValueEnum};

#[derive(Parser)]
struct Args {
    command: Command,
    text: String,
    #[arg(short, long)]
    language: Option<String>,
}

#[derive(Parser, Clone, Copy, ValueEnum)]
enum Command {
    Tokenize,
    Segment,
}

fn main() {
    let args = Args::parse();
    let tokenizer = TokenizerBuilder::default().into_tokenizer();

    let language = args.language.and_then(|l| Language::from_code(&l)).map(|l| vec![l]);
    let language = language.as_ref().map(|l| l.as_slice());
    let tokens = match args.command {
        Command::Tokenize => tokenizer
            .tokenize_with_allow_list(args.text.as_str(), language)
            .map(|l| l.lemma().to_owned())
            .collect::<Vec<_>>(),
        Command::Segment => tokenizer
            .segment_with_allow_list(args.text.as_str(), language)
            .map(|l| l.lemma().to_owned())
            .collect::<Vec<_>>(),
    };

    println!("{:?}", tokens);
}
