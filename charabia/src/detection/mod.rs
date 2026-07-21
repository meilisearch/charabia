pub use script_language::{Language, Script};
use whatlang::Detector;

// file copy pasted from whatlang.
#[allow(dead_code)]
mod chars;
mod script_language;

pub struct StrDetection<'o, AllowList> {
    inner: &'o str,
    pub script: Option<Script>,
    pub language: Option<Language>,
    allow_list: Option<AllowList>,
}

impl<'o, AllowList> StrDetection<'o, AllowList> {
    pub fn new(inner: &'o str, allow_list: Option<AllowList>) -> Self {
        Self { inner, script: None, language: None, allow_list }
    }

    pub fn script(&mut self) -> Script {
        let inner = self.inner;
        *self.script.get_or_insert_with(|| detect_script(inner))
    }

    pub fn language<'lang, Lang>(&mut self) -> Option<Language>
    where
        AllowList: IntoIterator<Item = Lang> + Copy,
        Lang: std::borrow::Borrow<Language>,
    {
        let inner = self.inner;
        self.language = match self.language.take() {
            Some(lang) => Some(lang),
            None => match self.allow_list {
                Some(ref allow_list) => {
                    let mut iter = allow_list.into_iter();

                    match iter.next() {
                        Some(lang) if iter.next().is_none() => Some((*lang.borrow()).into()),
                        _ => detect_lang(inner, Some(allow_list.into_iter())),
                    }
                }
                None if detect_script(inner) == Script::Latin => None,
                None => detect_lang::<<AllowList as IntoIterator>::IntoIter, Lang>(inner, None),
            },
        };

        self.language
    }
}

/// detect script with whatlang,
/// if no script is detected, return Script::Other
fn detect_script(text: &str) -> Script {
    whatlang::detect_script(text).map(Script::from).unwrap_or_default()
}

/// detect lang with whatlang
/// if no language is detected, return Language::Other
fn detect_lang<'a, I: Iterator<Item = L>, L: std::borrow::Borrow<Language>>(
    text: &str,
    allow_list: Option<I>,
) -> Option<Language> {
    let detector = allow_list
        .map(|allow_list| allow_list.map(|lang| (*lang.borrow()).into()).collect())
        .map(Detector::with_allowlist)
        .unwrap_or_default();

    detector.detect_lang(text).map(Language::from)
}

pub trait Detect<'o> {
    fn detect<AllowList>(&'o self, allow_list: Option<AllowList>) -> StrDetection<'o, AllowList>;
}

impl<'o> Detect<'o> for &str {
    fn detect<AllowList>(&'o self, allow_list: Option<AllowList>) -> StrDetection<'o, AllowList> {
        StrDetection::new(self, allow_list)
    }
}
