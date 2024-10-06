use fst::raw::Fst;
use once_cell::sync::Lazy;

use crate::segmenter::utils::FstSegmenter;
use crate::segmenter::Segmenter;

/// German specialized [`Segmenter`].
///
/// This Segmenter uses a dictionary encoded as an FST to segment the provided text.
pub struct GermanSegmenter;

static WORDS_FST: Lazy<Fst<&[u8]>> =
    Lazy::new(|| Fst::new(&include_bytes!("../../dictionaries/fst/german/words.fst")[..]).unwrap());

static FST_SEGMENTER: Lazy<FstSegmenter> =
    Lazy::new(|| FstSegmenter::new(&WORDS_FST, Some(2), true));

impl Segmenter for GermanSegmenter {
    fn segment_str<'o>(&self, to_segment: &'o str) -> Box<dyn Iterator<Item = &'o str> + 'o> {
        FST_SEGMENTER.segment_str(to_segment)
    }
}

// Test the segmenter:
#[cfg(test)]
mod test {
    use crate::segmenter::test::test_segmenter;

    const TEXT: &str =
        "Der Dampfschifffahrtskapitän fährt über den Mittellandkanal zur Strombrücke Magdeburg 123 456.";

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
        " ",
        "123",
        " ",
        "456",
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
        " ",
        "123",
        " ",
        "456",
        ".",
    ];

    // Macro that runs several tests on the Segmenter.
    test_segmenter!(GermanSegmenter, TEXT, SEGMENTED, TOKENIZED, Script::Latin, Language::Deu);

    macro_rules! test_segmentation {
        ($text:expr, $segmented:expr, $name:ident) => {
            #[test]
            fn $name() {
                let segmented_text: Vec<_> = FST_SEGMENTER.segment_str($text).collect::<Vec<_>>();
                assert_eq!(segmented_text, $segmented);
            }
        };
    }

    test_segmentation!(
        "Literaturverwaltungsprogramm",
        &["Literatur", "verwaltungs", "programm"],
        word1
    );
    test_segmentation!("Schreibprozess", &["Schreib", "prozess"], word2);
    test_segmentation!("Interkulturalität", &["Inter", "kulturalität"], word3);
    test_segmentation!("Wissensorganisation", &["Wissens", "organisation"], word4);
    test_segmentation!("Aufgabenplanung", &["Aufgaben", "planung"], word5);
    test_segmentation!("Eisbrecher", &["Eis", "brecher"], word6);
    test_segmentation!("Zuckerei", &["Zucker", "ei"], word7);
    test_segmentation!("Glatteis", &["Glatt", "eis"], word8);
    test_segmentation!("Sinnfindung", &["Sinn", "findung"], word9);
    test_segmentation!(
        "Donaudampfschifffahrtsgesellschaftskapitän",
        &["Donau", "dampf", "schifffahrts", "gesellschafts", "kapitän"],
        word10
    );
    test_segmentation!(
        "Rindfleischetikettierungsüberwachungsaufgabenübertragungsgesetz",
        &[
            "Rind",
            "fleisch",
            "etikettierungs",
            "überwachungs",
            "aufgaben",
            "übertragungs",
            "gesetz"
        ],
        word11
    );
    test_segmentation!(
        "Nahrungsmittelunverträglichkeitsdiagnoseverfahren",
        &["Nahrungs", "mittel", "un", "verträglichkeits", "diagnose", "verfahren"],
        word12
    );
}
