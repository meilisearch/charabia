// Import `Segmenter` trait.
use crate::segmenter::Segmenter;

use once_cell::sync::Lazy;
use std::collections::HashSet;
use std::io::{self, BufRead};

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
    for word in io::BufReader::new(cursor).lines().map_while(Result::ok) {
        dictionary.insert(word);
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
                    // Check if the remaining suffix would be at least three characters long
                    if !suffix.is_empty() && suffix.len() < 3 {
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

    const TEXT: &str =
        "Der Dampfschifffahrtskapitän fährt über den Mittellandkanal zur Strombrücke Magdeburg.";

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
        "Magdeburg",
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
        "magdeburg",
        ".",
    ];

    // Macro that runs several tests on the Segmenter.
    test_segmenter!(GermanSegmenter, TEXT, SEGMENTED, TOKENIZED, Script::Latin, Language::Deu);

    macro_rules! test_segmentation {
        ($text:expr, $segmented:expr, $name:ident) => {
            #[test]
            fn $name() {
                let dictionary = &*DICTIONARY;
                let segmented_text: Vec<_> =
                    split_compound_words($text, dictionary).into_iter().collect();
                assert_eq!(segmented_text, $segmented);
            }
        };
    }

    test_segmentation!(
        "Literaturverwaltungsprogramm",
        ["Literatur", "verwaltungs", "programm"],
        word1
    );
    test_segmentation!("Schreibprozess", ["Schreib", "prozess"], word2);
    test_segmentation!("Interkulturalität", ["Inter", "kultur", "alität"], word3);
    test_segmentation!("Wissensorganisation", ["Wissens", "organisation"], word4);
    test_segmentation!("Aufgabenplanung", ["Aufgaben", "planung"], word5);
}
