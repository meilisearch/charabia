use super::Segmenter;

/// Persian specialized [`Segmenter`].
///
/// Persian text is segmented by word boundaries and by punctuation.
/// We need to handle Persian compound words connected by ZWNJ (Zero Width Non-Joiner).
/// For example, "کتاب‌ها" (books) should be segmented into "کتاب" and "ها".
/// The ZWNJ character itself should not appear in the final segmentation.
pub struct PersianSegmenter;

const ZWNJ: char = '\u{200c}';

impl Segmenter for PersianSegmenter {
    fn segment_str<'o>(&self, to_segment: &'o str) -> Box<dyn Iterator<Item = &'o str> + 'o> {
        // Simple approach: split on ZWNJ and filter out empty parts
        if to_segment.contains(ZWNJ) {
            let parts: Vec<&str> = to_segment.split(ZWNJ).filter(|part| !part.is_empty()).collect();
            Box::new(parts.into_iter())
        } else {
            Box::new(Some(to_segment).into_iter())
        }
    }
}

// Test the segmenter:
#[cfg(test)]
mod test {
    use crate::segmenter::test::test_segmenter;

    // Original version of the text.
    const TEXT: &str = "کتاب‌هایم را می‌خوانم، آیا تو هم می‌خوانی؟ (امیدوارم موفق باشی) ۱۲۳ ۴۵۶";

    // Segmented version of the text.
    const SEGMENTED: &[&str] = &[
        "کتاب",
        "هایم",
        " ",
        "را",
        " ",
        "می",
        "خوانم",
        "،",
        " ",
        "آیا",
        " ",
        "تو",
        " ",
        "هم",
        " ",
        "می",
        "خوانی",
        "؟",
        " ",
        "(",
        "امیدوارم",
        " ",
        "موفق",
        " ",
        "باشی",
        ")",
        " ",
        "۱۲۳",
        " ",
        "۴۵۶",
    ];

    // Segmented and normalized version of the text.
    const TOKENIZED: &[&str] = &[
        "کتاب",
        "هایم",
        " ",
        "را",
        " ",
        "می",
        "خوانم",
        ",",
        " ",
        "ایا",
        " ",
        "تو",
        " ",
        "هم",
        " ",
        "می",
        "خوانی",
        "?",
        " ",
        "(",
        "امیدوارم",
        " ",
        "موفق",
        " ",
        "باشی",
        ")",
        " ",
        "123",
        " ",
        "456",
    ];

    test_segmenter!(PersianSegmenter, TEXT, SEGMENTED, TOKENIZED, Script::Arabic, Language::Pes);
}
