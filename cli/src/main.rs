use charabia::{Language, TokenizerBuilder};
use clap::{Parser, ValueEnum};
use tokenizers::Tokenizer;

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
    let text = args.text.as_str();

    let language = args.language.and_then(|l| Language::from_code(&l)).map(|l| vec![l]);
    let language = language.as_ref().map(|l| l.as_slice());
    let tokens = match args.command {
        Command::Tokenize => tokenizer
            .tokenize_with_allow_list(text, language)
            .map(|l| l.lemma().to_owned())
            .collect::<Vec<_>>(),
        Command::Segment => tokenizer
            .segment_with_allow_list(text, language)
            .map(|l| l.lemma().to_owned())
            .collect::<Vec<_>>(),
    };

    println!("Charabia: {:?}", tokens);
    println!(
        "Llama: {:?}",
        Tokenizer::from_pretrained("Xenova/llama4-tokenizer", None)
            .expect("Failed to load LLaMA tokenizer")
            .encode(text, false)
            .expect("Failed to encode text")
            .get_offsets()
            .iter()
            .map(|(start, end)| (&text[*start..*end]).to_owned())
            .collect::<Vec<_>>()
    );
}
