use core::str::FromStr;

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
use serde::{Deserialize, Serialize};

use super::chars;

macro_rules! make_language {
    ($($language:tt), +) => {
        #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize, PartialOrd, Ord)]
        pub enum Language {
            Zho,
            $($language),+,
        }
        impl From<whatlang::Lang> for Language {
            fn from(other: whatlang::Lang) -> Language {
                match other {
                    $(whatlang::Lang::$language => Language::$language), +
                }
            }
        }

        impl From<Language> for whatlang::Lang {
            fn from(other: Language) -> whatlang::Lang {
                match other {
                    Language::Zho => whatlang::Lang::Cmn,
                    $(Language::$language => whatlang::Lang::$language), +,
                }
            }
        }

        impl Language {
            pub fn code(&self) -> &'static str {
                match self {
                    Language::Zho => "zho",
                    $(Language::$language => whatlang::Lang::$language.code()), +,
                }
            }

            pub fn from_code<S: AsRef<str>>(code: S) -> Option<Language> {
                match code.as_ref() {
                    "zho" => Some(Language::Zho),
                    _ => whatlang::Lang::from_code(code.as_ref()).map(Language::from),
                }
            }
        }
    };
}

make_language! {
    Epo,
    Eng,
    Rus,
    Cmn,
    Spa,
    Por,
    Ita,
    Ben,
    Fra,
    Deu,
    Ukr,
    Kat,
    Ara,
    Hin,
    Jpn,
    Heb,
    Yid,
    Pol,
    Amh,
    Jav,
    Kor,
    Nob,
    Dan,
    Swe,
    Fin,
    Tur,
    Nld,
    Hun,
    Ces,
    Ell,
    Bul,
    Bel,
    Mar,
    Kan,
    Ron,
    Slv,
    Hrv,
    Srp,
    Mkd,
    Lit,
    Lav,
    Est,
    Tam,
    Vie,
    Urd,
    Tha,
    Guj,
    Uzb,
    Pan,
    Aze,
    Ind,
    Tel,
    Pes,
    Mal,
    Ori,
    Mya,
    Nep,
    Sin,
    Khm,
    Tuk,
    Aka,
    Zul,
    Sna,
    Afr,
    Lat,
    Slk,
    Cat,
    Tgl,
    Hye
}

macro_rules! make_script {
    ($($script:tt), +) => {
        #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize, PartialOrd, Ord)]
        pub enum Script {
            $($script),+,
            Cj,
            Other,
        }

        impl From<whatlang::Script> for Script {
            fn from(other: whatlang::Script) -> Script {
                match other {
                    whatlang::Script::Hiragana |
                    whatlang::Script::Katakana |
                    whatlang::Script::Mandarin => Script::Cj,
                    $(whatlang::Script::$script => Script::$script), +
                }
            }

        }

        impl Script {
            pub fn name(&self) -> &'static str {
                match self {
                    $(Script::$script => whatlang::Script::$script.name()), +,
                    Script::Cj => whatlang::Script::Mandarin.name(),
                    _other => "other",
                }
            }

            pub fn from_name<S: AsRef<str>>(code: S) -> Script {
                whatlang::Script::from_str(code.as_ref()).map(Script::from).unwrap_or_default()
            }
        }
    };
}

make_script! {
    Arabic,
    Armenian,
    Bengali,
    Cyrillic,
    Devanagari,
    Ethiopic,
    Georgian,
    Greek,
    Gujarati,
    Gurmukhi,
    Hangul,
    Hebrew,
    Kannada,
    Khmer,
    Latin,
    Malayalam,
    Myanmar,
    Oriya,
    Sinhala,
    Tamil,
    Telugu,
    Thai
}

impl From<char> for Script {
    fn from(other: char) -> Script {
        if chars::is_latin(other) {
            Script::Latin
        } else if chars::is_cyrillic(other) {
            Script::Cyrillic
        } else if chars::is_arabic(other) {
            Script::Arabic
        } else if chars::is_devanagari(other) {
            Script::Devanagari
        } else if chars::is_hebrew(other) {
            Script::Hebrew
        } else if chars::is_ethiopic(other) {
            Script::Ethiopic
        } else if chars::is_georgian(other) {
            Script::Georgian
        } else if chars::is_bengali(other) {
            Script::Bengali
        } else if chars::is_hangul(other) {
            Script::Hangul
        } else if chars::is_hiragana(other)
            || chars::is_katakana(other)
            || chars::is_mandarin(other)
        {
            Script::Cj
        } else if chars::is_greek(other) {
            Script::Greek
        } else if chars::is_kannada(other) {
            Script::Kannada
        } else if chars::is_tamil(other) {
            Script::Tamil
        } else if chars::is_thai(other) {
            Script::Thai
        } else if chars::is_gujarati(other) {
            Script::Gujarati
        } else if chars::is_gurmukhi(other) {
            Script::Gurmukhi
        } else if chars::is_telugu(other) {
            Script::Telugu
        } else if chars::is_malayalam(other) {
            Script::Malayalam
        } else if chars::is_oriya(other) {
            Script::Oriya
        } else if chars::is_myanmar(other) {
            Script::Myanmar
        } else if chars::is_sinhala(other) {
            Script::Sinhala
        } else if chars::is_khmer(other) {
            Script::Khmer
        } else {
            Script::Other
        }
    }
}

impl Default for Script {
    fn default() -> Self {
        Self::Other
    }
}

// impl Arbitrary for Script {
#[cfg(test)]
impl Arbitrary for Script {
    fn arbitrary(g: &mut Gen) -> Self {
        *g.choose(&[
            Script::Arabic,
            Script::Armenian,
            Script::Bengali,
            Script::Cyrillic,
            Script::Devanagari,
            Script::Ethiopic,
            Script::Georgian,
            Script::Greek,
            Script::Gujarati,
            Script::Gurmukhi,
            Script::Hangul,
            Script::Hebrew,
            Script::Kannada,
            Script::Khmer,
            Script::Latin,
            Script::Malayalam,
            Script::Myanmar,
            Script::Oriya,
            Script::Sinhala,
            Script::Tamil,
            Script::Telugu,
            Script::Thai,
        ])
        .unwrap()
    }
}

#[cfg(test)]
impl Arbitrary for Language {
    fn arbitrary(g: &mut Gen) -> Self {
        *g.choose(&[
            Language::Epo,
            Language::Eng,
            Language::Rus,
            Language::Cmn,
            Language::Spa,
            Language::Por,
            Language::Ita,
            Language::Ben,
            Language::Fra,
            Language::Deu,
            Language::Ukr,
            Language::Kat,
            Language::Ara,
            Language::Hin,
            Language::Jpn,
            Language::Heb,
            Language::Yid,
            Language::Pol,
            Language::Amh,
            Language::Jav,
            Language::Kor,
            Language::Nob,
            Language::Dan,
            Language::Swe,
            Language::Fin,
            Language::Tur,
            Language::Nld,
            Language::Hun,
            Language::Ces,
            Language::Ell,
            Language::Bul,
            Language::Bel,
            Language::Mar,
            Language::Kan,
            Language::Ron,
            Language::Slv,
            Language::Hrv,
            Language::Srp,
            Language::Mkd,
            Language::Lit,
            Language::Lav,
            Language::Est,
            Language::Tam,
            Language::Vie,
            Language::Urd,
            Language::Tha,
            Language::Guj,
            Language::Uzb,
            Language::Pan,
            Language::Aze,
            Language::Ind,
            Language::Tel,
            Language::Pes,
            Language::Mal,
            Language::Ori,
            Language::Mya,
            Language::Nep,
            Language::Sin,
            Language::Khm,
            Language::Tuk,
            Language::Aka,
            Language::Zul,
            Language::Sna,
            Language::Afr,
            Language::Lat,
            Language::Slk,
            Language::Cat,
            Language::Tgl,
            Language::Hye,
        ])
        .unwrap()
    }
}

#[cfg(test)]
mod test {
    use crate::{Language, Script};

    #[test]
    fn from_into_language() {
        assert_eq!(Language::Eng.code(), "eng");
        assert_eq!(Language::from_code("eng"), Some(Language::Eng));
        assert_eq!(Language::Jpn.code(), "jpn");
        assert_eq!(Language::from_code("jpn"), Some(Language::Jpn));
        assert_eq!(Language::Cmn.code(), "cmn");
        assert_eq!(Language::from_code("cmn"), Some(Language::Cmn));
    }

    #[test]
    fn from_into_script() {
        assert_eq!(Script::Latin.name(), "Latin");
        assert_eq!(Script::from_name("Latin"), Script::Latin);
        assert_eq!(Script::Cj.name(), "Mandarin");
        assert_eq!(Script::from_name("Mandarin"), Script::Cj);
    }
}
