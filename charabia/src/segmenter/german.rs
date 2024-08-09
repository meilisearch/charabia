// Import `Segmenter` trait.
use crate::segmenter::Segmenter;

use std::collections::HashSet;
use std::io::{self, BufRead};
use once_cell::sync::Lazy;

/// German specialized [`Segmenter`].
///
/// This Segmenter uses a dictionary copied from https://github.com/uschindler/german-decompounder/
pub struct GermanSegmenter;

static DICTIONARY_DATA: &[u8] = include_bytes!("../../dictionaries/txt/german/dictionary-de.txt");

/// Load the dictionary from the hardcoded path.
pub static DICTIONARY: Lazy<HashSet<String>> = Lazy::new(|| {
    let mut dictionary = HashSet::new();
    let data = DICTIONARY_DATA;

    let cursor = io::Cursor::new(data);
    for line in io::BufReader::new(cursor).lines() {
        if let Ok(word) = line {
            dictionary.insert(word);
        }
    }

    dictionary
});

/// Function to split compound words based on the dictionary, ignoring case.
pub(crate) fn split_compound_words<'a>(
    word: &'a str,
    dictionary: &HashSet<String>,
) -> Vec<&'a str> {
    let mut parts = Vec::new();
    let mut remaining = word;

    while !remaining.is_empty() {
        let mut found = false;

        for i in (1..=remaining.len()).rev() {
            // Ensure we are splitting on a valid character boundary
            if remaining.is_char_boundary(i) {
                let prefix = &remaining[..i];
                let suffix = &remaining[i..];

                // Convert the prefix to lowercase for dictionary lookup
                if dictionary.contains(&prefix.to_lowercase()) {
                    // Check if the remaining suffix would be at least two characters long
                    if suffix.len() != 0 && suffix.len() < 2 {
                        continue;
                    }

                    parts.push(prefix);
                    remaining = suffix;
                    found = true;
                    break;
                }
            }
        }

        if !found {
            // If no prefix is found or the suffix would be too short, add the rest as one part
            parts.push(remaining);
            break;
        }
    }

    parts
}

impl Segmenter for GermanSegmenter {
    fn segment_str<'o>(&self, to_segment: &'o str) -> Box<dyn Iterator<Item = &'o str> + 'o> {
        let dictionary = &*DICTIONARY;

        let segments: Vec<&'o str> = to_segment
            .split_whitespace()
            .flat_map(move |word| split_compound_words(word, dictionary))
            .collect();

        Box::new(segments.into_iter())
    }
}

// Test the segmenter:
#[cfg(test)]
mod test {
    use crate::segmenter::test::test_segmenter;

    const TEXT: &str = "Der Dampfschifffahrtskapitän fährt über den Mittellandkanal zur Strombrücke Magdeburg.";

    const SEGMENTED: &[&str] = &[
        "Der",
        " ",
        "Dampf",
        "schifffahrts",
        "kapitän",
        " ",
        "fährt",
        " ",
        "über",
        " ",
        "den",
        " ",
        "Mittel",
        "land",
        "kanal",
        " ",
        "zur",
        " ",
        "Strom",
        "brücke",
        " ",
        "Magd",
        "eburg",
        ".",
    ];

    const TOKENIZED: &[&str] = &[
        "der",
        " ",
        "dampf",
        "schifffahrts",
        "kapitan",
        " ",
        "fahrt",
        " ",
        "uber",
        " ",
        "den",
        " ",
        "mittel",
        "land",
        "kanal",
        " ",
        "zur",
        " ",
        "strom",
        "brucke",
        " ",
        "magd",
        "eburg",
        ".",
    ];

    // Macro that runs several tests on the Segmenter.
    test_segmenter!(GermanSegmenter, TEXT, SEGMENTED, TOKENIZED, Script::Latin, Language::Deu);
}