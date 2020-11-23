use whatlang;

pub fn is_cjk(c: char) -> bool {
    (c >= '\u{1100}' && c <= '\u{11ff}')  // Hangul Jamo
        || (c >= '\u{2e80}' && c <= '\u{2eff}')  // CJK Radicals Supplement
        || (c >= '\u{2f00}' && c <= '\u{2fdf}') // Kangxi radical
        || (c >= '\u{3000}' && c <= '\u{303f}') // Japanese-style punctuation
        || (c >= '\u{3040}' && c <= '\u{309f}') // Japanese Hiragana
        || (c >= '\u{30a0}' && c <= '\u{30ff}') // Japanese Katakana
        || (c >= '\u{3100}' && c <= '\u{312f}')
        || (c >= '\u{3130}' && c <= '\u{318F}') // Hangul Compatibility Jamo
        || (c >= '\u{3200}' && c <= '\u{32ff}') // Enclosed CJK Letters and Months
        || (c >= '\u{3400}' && c <= '\u{4dbf}') // CJK Unified Ideographs Extension A
        || (c >= '\u{4e00}' && c <= '\u{9fff}') // CJK Unified Ideographs
        || (c >= '\u{a960}' && c <= '\u{a97f}') // Hangul Jamo Extended-A
        || (c >= '\u{ac00}' && c <= '\u{d7a3}') // Hangul Syllables
        || (c >= '\u{d7b0}' && c <= '\u{d7ff}') // Hangul Jamo Extended-B
        || (c >= '\u{f900}' && c <= '\u{faff}') // CJK Compatibility Ideographs
        || (c >= '\u{ff00}' && c <= '\u{ffef}') // Full-width roman characters and half-width katakana
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum SeparatorCategory {
    Soft,
    Hard,
}

impl SeparatorCategory {
    fn merge(self, other: SeparatorCategory) -> SeparatorCategory {
        if let (Soft, Soft) = (self, other) {
            Soft
        } else {
            Hard
        }
    }

    fn to_usize(self) -> usize {
        match self {
            Soft => 1,
            Hard => 8,
        }
    }
}

fn is_separator(c: char) -> bool {
    classify_separator(c).is_some()
}

fn classify_separator(c: char) -> Option<SeparatorCategory> {
    match c {
        c if c.is_whitespace() => Some(Soft), // whitespaces
        c if deunicode_char(c) == Some("'") => Some(Soft), // quotes
        c if deunicode_char(c) == Some("\"") => Some(Soft), // double quotes
        '-' | '_' | '\'' | ':' | '/' | '\\' | '@' => Some(Soft),
        '.' | ';' | ',' | '!' | '?' | '(' | ')' => Some(Hard),
        _ => None,
    }
}
