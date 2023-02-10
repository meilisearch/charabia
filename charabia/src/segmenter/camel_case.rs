use once_cell::sync::Lazy;
use regex::Regex;

pub(crate) trait CamelCaseSegmentation {
    /// Returns an iterator over substrings of `self` separated on camelCase boundaries.
    /// For instance, "camelCase" is split into ["camel", "Case"].
    /// A camelCase boundary constitutes a lowercase letter directly followed by an uppercase letter
    /// where lower and uppercase letters are defined by the corresponding Unicode General Categories.
    fn split_camel_case_bounds(&self) -> CamelCaseParts;
}

pub(crate) struct CamelCaseParts<'t> {
    state: State<'t>,
}

enum State<'t> {
    InProgress { remainder: &'t str },
    Exhausted,
}

impl CamelCaseSegmentation for str {
    fn split_camel_case_bounds(&self) -> CamelCaseParts {
        CamelCaseParts { state: State::InProgress { remainder: self } }
    }
}

static CAMEL_CASE_BOUNDARY_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\p{Ll}\p{Lu}").unwrap());

impl<'t> Iterator for CamelCaseParts<'t> {
    type Item = &'t str;

    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            State::Exhausted => None,
            State::InProgress { remainder } => {
                // CamelCase boundary consists of 2 code-points. Avoid expensive regex evaluation on shorter strings.
                // Note that using `remainder.chars().count() == 1` may catch more cases (non-ASCII strings)
                // but the main focus here is on " ", "-" and similar that are abundantly produced
                // by `split_word_bounds()` in the Latin segmenter and mere `len()` performs better at that.
                if remainder.len() == 1 {
                    self.state = State::Exhausted;
                    return Some(remainder);
                }

                match CAMEL_CASE_BOUNDARY_REGEX.find(remainder) {
                    Some(mat) => {
                        // By the nature of the regex, the match must contain exactly two chars and this should never panic.
                        let lowercase_letter_length =
                            mat.as_str().chars().next().unwrap().len_utf8();
                        let boundary = mat.start() + lowercase_letter_length;

                        self.state = State::InProgress { remainder: &remainder[boundary..] };
                        Some(&remainder[..boundary])
                    }
                    None => {
                        // All boundaries processed. Mark `self` as exhausted.
                        self.state = State::Exhausted;
                        // But don't forget to yield the part of the string remaining after the last boundary.
                        Some(remainder)
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::CamelCaseSegmentation;

    macro_rules! test_segmentation {
        ($text:expr, $segmented:expr, $name:ident) => {
            #[test]
            fn $name() {
                let segmented_text: Vec<_> = $text.split_camel_case_bounds().collect();
                assert_eq!(segmented_text, $segmented);
            }
        };
    }

    test_segmentation!("camelCase", ["camel", "Case"], camel_case_is_split);
    test_segmentation!("SCREAMING", ["SCREAMING"], all_caps_is_not_split);
    test_segmentation!("resuméWriter", ["resumé", "Writer"], non_ascii_boundary_on_left);
    test_segmentation!("KarelČapek", ["Karel", "Čapek"], non_ascii_boundary_on_right);
}
