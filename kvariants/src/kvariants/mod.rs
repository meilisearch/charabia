use once_cell::sync::Lazy;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum KVariantClass {
    Wrong,
    SementicVariant,
    Simplified,
    Old,
    Equal,
}

#[derive(Debug, PartialEq)]
pub struct KVariant {
    pub source_ideograph: char,
    pub classification: KVariantClass,
    pub destination_ideograph: char,
}

#[derive(Deserialize)]
pub struct TsvRow {
    lhs: String,
    relation: String,
    rhs: String,
}

pub static KVARIANTS: Lazy<HashMap<char, KVariant>> = Lazy::new(|| {
    // The tab separated format is like:
    //
    //   㨲 (U+3A32)	wrong!	㩍 (U+3A4D)
    //   铿 (U+94FF)	simp	鏗 (U+93D7)
    //   㓻 (U+34FB)	sem	    剛 (U+525B)
    //   ...
    //
    let dictionary = include_str!("../../dictionaries/compressed/kVariants.csv");
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(dictionary.as_bytes());

    let mut map: HashMap<char, KVariant> = HashMap::new();
    for result in reader.deserialize() {
        let line: TsvRow = result.unwrap();
        let rhs = line.rhs.chars().nth(0).unwrap();
        let lhs = line.lhs.chars().nth(0).unwrap();

        if let Some(classification) = match line.relation.as_str() {
            "wrong!" => Some(KVariantClass::Wrong),
            "sem" => Some(KVariantClass::SementicVariant),
            "simp" => Some(KVariantClass::Simplified),
            "old" => Some(KVariantClass::Old),
            "=" => Some(KVariantClass::Equal),
            unexpected_classification => {
                debug_assert!(
                    false,
                    "Unexpected classification {:?} encountered. Consider handling or ignore explicaitly.",
                    unexpected_classification,
                );
                None
            }
        } {
            debug_assert!(
                !map.contains_key(&lhs),
                "Unexpected one source ideograph mapping to multiple destination ideographs.
                 If this happens in the future when we update kVariants.tsv, we would need to handle it
                 by, for example, deciding priorities for different classification types. "
            );

            map.insert(
                lhs,
                KVariant {
                    source_ideograph: lhs,
                    classification,
                    destination_ideograph: rhs,
                },
            );
        }
    }

    map
});

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_kvariants() {
        assert_eq!(
            KVARIANTS.get(&'澚'),
            Some(&KVariant {
                source_ideograph: '澚',
                classification: KVariantClass::Wrong,
                destination_ideograph: '澳'
            }),
        );
        assert_eq!(
            KVARIANTS.get(&'䀾'),
            Some(&KVariant {
                source_ideograph: '䀾',
                classification: KVariantClass::SementicVariant,
                destination_ideograph: '䁈',
            }),
        );
        assert_eq!(
            KVARIANTS.get(&'亚'),
            Some(&KVariant {
                source_ideograph: '亚',
                classification: KVariantClass::Simplified,
                destination_ideograph: '亞',
            }),
        );
        assert_eq!(
            KVARIANTS.get(&'㮺'),
            Some(&KVariant {
                source_ideograph: '㮺',
                classification: KVariantClass::Old,
                destination_ideograph: '本',
            }),
        );
        assert_eq!(
            KVARIANTS.get(&'刄'),
            Some(&KVariant {
                source_ideograph: '刄',
                classification: KVariantClass::Equal,
                destination_ideograph: '刃',
            }),
        );
        assert_eq!(KVARIANTS.get(&'刃'), None);
    }

    #[test]
    fn test_no_loop() {
        for value in KVARIANTS.values() {
            match KVARIANTS.get(&value.destination_ideograph) {
                // e.g. when value is "栄", reverse lookup would yield nothing.
                None => (),

                // e.g. when value is "椉", reverse lookup would yield "椉 old 乘".
                Some(reverse_lookup_value) => {
                    assert_ne!(
                        value.destination_ideograph,
                        reverse_lookup_value.destination_ideograph
                    );
                }
            }
        }
    }
}
