// Import `Segmenter` trait.
use crate::segmenter::Segmenter;
use tokenizer::{Tokenizer, th};
use once_cell::sync::Lazy;



/// Thai specialized [`Segmenter`].
///
/// This Segmenter uses the very creatively named, tokenizer library internally to segment the provided text.

pub struct ThaiSegmenter;
static SOMCHAI: Lazy<tokenizer::th::Tokenizer> = Lazy::new(||

{
    let tokenizer = th::Tokenizer::new("C:\\Users\\macth\\Desktop\\Rust-Playground\\src\\words_th.txt").expect("Dictionary file not found");
    tokenizer
});


impl Segmenter for ThaiSegmenter {
    fn segment_str<'o>(&self, to_segment: &'o str) -> Box<dyn Iterator<Item = &'o str> + 'o> {
        let segmented = SOMCHAI.tokenize(to_segment);
        Box::new(segmented.into_iter())
    }
}


// Test the segmenter:
#[cfg(test)]
mod test {
    use crate::segmenter::test::test_segmenter;

    
    const TEXT: &str = "ภาษาไทยง่ายนิดเดียว";

    const SEGMENTED: &[&str] = &["ภาษาไทย", "ง่าย", "นิดเดียว"];


    const TOKENIZED: &[&str] = SEGMENTED;
    // Macro that run several tests on the Segmenter.
    test_segmenter!(ThaiSegmenter, TEXT, SEGMENTED, TOKENIZED, Script::Thai, Language::Tha);
}

