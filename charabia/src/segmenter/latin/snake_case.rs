use finl_unicode::categories::CharacterCategories;
use slice_group_by::StrGroupBy;

/// Returns an iterator over substrings of `str` separated on snake_case boundaries.
/// For instance, "snake_case" is split into ["snake", "_", "case"].
pub(crate) fn split_snake_case_bounds(str: &str) -> impl Iterator<Item = &str> {
    let mut last_char_was_underscore =
        str.chars().next().map_or(false, |c| c.is_punctuation_connector());

    str.linear_group_by(move |_, c| {
        let same_group =
            c.is_mark_nonspacing() || last_char_was_underscore == c.is_punctuation_connector();
        if !same_group {
            last_char_was_underscore = c.is_punctuation_connector();
        }
        same_group
    })
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! test_segmentation {
        ($text:expr, $segmented:expr, $name:ident) => {
            #[test]
            fn $name() {
                let segmented_text: Vec<_> = split_snake_case_bounds($text).collect();
                assert_eq!(segmented_text, $segmented);
            }
        };
    }

    test_segmentation!("a", ["a"], one_letter_is_preserved);
    test_segmentation!("_", ["_"], one_underscore_is_preserved);
    test_segmentation!("__", ["__"], sequence_of_underscores_is_preserved);
    test_segmentation!("snake_case", ["snake", "_", "case"], snake_case_is_split);
    test_segmentation!("kebab-case", ["kebab-case"], kebab_case_is_preserved);
    test_segmentation!(
        "SCREAMING_SNAKE_CASE",
        ["SCREAMING", "_", "SNAKE", "_", "CASE"],
        screaming_snake_case_is_split
    );
    test_segmentation!("resumé_writer", ["resumé", "_", "writer"], non_ascii_boundary_on_left);
    test_segmentation!("Karel_Čapek", ["Karel", "_", "Čapek"], non_ascii_boundary_on_right);
    test_segmentation!("a\u{0301}_b", ["a\u{0301}", "_", "b"], non_spacing_mark_on_left);
    test_segmentation!("a_\u{0301}b", ["a", "_\u{0301}", "b"], non_spacing_mark_on_right);
    test_segmentation!(
        "a\u{0301}_\u{0301}b",
        ["a\u{0301}", "_\u{0301}", "b"],
        non_spacing_marks_on_both_sides
    );
    test_segmentation!("_\u{0301}_", ["_\u{0301}_"], non_spacing_mark_between_underscores);
    test_segmentation!(
        "a\u{0301}_",
        ["a\u{0301}", "_"],
        non_spacing_mark_between_letter_and_underscore
    );
    test_segmentation!(
        "_\u{0301}b",
        ["_\u{0301}", "b"],
        non_spacing_mark_between_underscore_and_letter
    );
    test_segmentation!(
        "__double_leading_underscore",
        ["__", "double", "_", "leading", "_", "underscore"],
        double_leading_underscore
    );
    test_segmentation!(
        "__double_leading_and_trailing_underscore__",
        ["__", "double", "_", "leading", "_", "and", "_", "trailing", "_", "underscore", "__"],
        double_leading_and_trailing_underscore
    );
}
