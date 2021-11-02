use deunicode::deunicode_char;

use crate::token::SeparatorKind;

pub fn is_cjk(c: char) -> bool {
    (c >= '\u{2e80}' && c <= '\u{2eff}')  // CJK Radicals Supplement
        || (c >= '\u{2f00}' && c <= '\u{2fdf}') // Kangxi radical
        || (c >= '\u{3000}' && c <= '\u{303f}') // Japanese-style punctuation
        || (c >= '\u{3040}' && c <= '\u{309f}') // Japanese Hiragana
        || (c >= '\u{30a0}' && c <= '\u{30ff}') // Japanese Katakana
        || (c >= '\u{3100}' && c <= '\u{312f}')
        || (c >= '\u{3200}' && c <= '\u{32ff}') // Enclosed CJK Letters and Months
        || (c >= '\u{3400}' && c <= '\u{4dbf}') // CJK Unified Ideographs Extension A
        || (c >= '\u{4e00}' && c <= '\u{9fff}') // CJK Unified Ideographs
        || (c >= '\u{f900}' && c <= '\u{faff}') // CJK Compatibility Ideographs
        || (c >= '\u{ff00}' && c <= '\u{ffef}') // Full-width roman characters and half-width katakana
}

// https://en.wikipedia.org/wiki/Latin_script_in_Unicode
pub fn is_latin(ch: char) -> bool {
    matches!(ch,
        'a'..='z'
        | 'A'..='Z'
        | '\u{0080}'..='\u{00FF}'
        | '\u{0100}'..='\u{017F}'
        | '\u{0180}'..='\u{024F}'
        | '\u{0250}'..='\u{02AF}'
        | '\u{1D00}'..='\u{1D7F}'
        | '\u{1D80}'..='\u{1DBF}'
        | '\u{1E00}'..='\u{1EFF}'
        | '\u{2100}'..='\u{214F}'
        | '\u{2C60}'..='\u{2C7F}'
        | '\u{A720}'..='\u{A7FF}'
        | '\u{AB30}'..='\u{AB6F}'
    )
}

pub fn classify_separator(c: char) -> Option<SeparatorKind> {
    match deunicode_char(c)?.chars().next()? {
        // Prevent deunicoding cyrillic chars (e.g. ь -> ' is incorrect)
        _ if ('\u{0410}'..='\u{044f}').contains(&c) => None, // russian cyrillic letters [а-яА-Я]
        _ if c == '\u{00a0}' => None,                        // non-breaking space
        c if c.is_whitespace() => Some(SeparatorKind::Soft), // whitespaces
        '-' | '_' | '\'' | ':' | '/' | '\\' | '@' | '"' | '+' | '~' | '=' | '^' | '*' | '#' => {
            Some(SeparatorKind::Soft)
        }
        '.' | ';' | ',' | '!' | '?' | '(' | ')' | '[' | ']' | '{' | '}' | '|' => {
            Some(SeparatorKind::Hard)
        }
        _ => None,
    }
}
