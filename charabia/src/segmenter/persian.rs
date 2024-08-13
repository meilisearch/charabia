use super::Segmenter;

/// Persian specialized [`Segmenter`].
///
/// Persian text is segmented by word boundaries and punctuation.
/// We also need to handle Persian-specific clitics and suffixes, such as:
/// - Prefixes like "می" (for continuous tense verbs) or "ن" (for negation)
/// - Suffixes like "ها" (plural), "ام", "ات", "ش" (possessive pronouns)
/// For example, the word "می‌خواهم" (I want) should be segmented into "می" and "خواهم".
/// Similarly, "کتاب‌ها" (books) should be segmented into "کتاب" and "ها".
/// Additionally, Persian punctuation like '،', '؟', etc., should be treated as separate tokens.

pub struct PersianSegmenter;

// Implement the `Segmenter` trait for `PersianSegmenter`.
impl Segmenter for PersianSegmenter {
    fn segment_str<'o>(&self, to_segment: &'o str) -> Box<dyn Iterator<Item = &'o str> + 'o> {
        let mut segments = Vec::new();
        let mut start = 0;

        while let Some((i, c)) = to_segment[start..].char_indices().next() {
            let slice = &to_segment[start..i + c.len_utf8()];

            if c.is_whitespace() || is_persian_punctuation(c) {
                if start != i {
                    segments.push(&to_segment[start..i]);
                }
                segments.push(&to_segment[i..=i]);
                start = i + c.len_utf8();
            } else if is_persian_prefix(slice) {
                if start != i {
                    segments.push(&to_segment[start..i]);
                }
                start = i;
            } else if is_persian_suffix(slice) {
                if start != i {
                    segments.push(&to_segment[start..i]);
                }
                start = i;
            }
        }

        if start < to_segment.len() {
            segments.push(&to_segment[start..]);
        }

        Box::new(segments.into_iter())
    }
}

// Check if a character is a Persian-specific punctuation mark.
fn is_persian_punctuation(c: char) -> bool {
    matches!(c, '،' | '؟' | '؛' | '.' | ',' | '!' | ':' | '-' | '(' | ')' | '«' | '»')
}

// Check if a string slice is a Persian prefix.
fn is_persian_prefix(slice: &str) -> bool {
    slice == "می" || slice == "ن" || slice == "بی" || slice == "به" || slice == "در"
}

// Check if a string ends with a Persian suffix.
fn is_persian_suffix(slice: &str) -> bool {
    slice.ends_with("ها") ||
        slice.ends_with("ام") ||
        slice.ends_with("ات") ||
        slice.ends_with("اش") ||
        slice.ends_with("شان") ||
        slice.ends_with("یم") ||
        slice.ends_with("ید") ||
        slice.ends_with("ند")
}

// Test the segmenter:
#[cfg(test)]
mod test {
    use crate::segmenter::test::test_segmenter;

    // Original version of the text.
    const TEXT: &str = "می‌روم کتاب‌ها را می‌خوانم. آیا شما می‌خواهید با من بیایید؟";

    // Segmented version of the text.
    const SEGMENTED: &[&str] = &[
        "می",
        "‌",
        "روم",
        " ",
        "کتاب",
        "‌",
        "ها",
        " ",
        "را",
        " ",
        "می",
        "‌",
        "خوانم",
        ".",
        " ",
        "آیا",
        " ",
        "شما",
        " ",
        "می",
        "‌",
        "خواهید",
        " ",
        "با",
        " ",
        "من",
        " ",
        "بیایید",
        "؟",
    ];

    // Segmented and normalized version of the text.
    const TOKENIZED: &[&str] = &[
        "می",
        "‌",
        "روم",
        " ",
        "کتاب",
        "‌",
        "ها",
        " ",
        "را",
        " ",
        "می",
        "‌",
        "خوانم",
        ".",
        " ",
        "آیا",
        " ",
        "شما",
        " ",
        "می",
        "‌",
        "خواهید",
        " ",
        "با",
        " ",
        "من",
        " ",
        "بیایید",
        "؟",
    ];

    // Macro that runs several tests on the Segmenter.
    test_segmenter!(PersianSegmenter, TEXT, SEGMENTED, TOKENIZED, Script::Arabic, Language::Pes);
}
