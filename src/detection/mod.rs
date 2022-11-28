pub use script_language::{Language, Script};
use std::collections::HashMap;
use whatlang::Detector;

// file copy pasted from whatlang.
#[allow(dead_code)]
mod chars;
mod script_language;

pub struct StrDetection<'o, 'al> {
    inner: &'o str,
    pub script: Option<Script>,
    pub language: Option<Language>,
    allow_list : Option<&'al HashMap<Script,Vec<Language>>>,
}

impl<'o, 'al> StrDetection<'o, 'al> {
    pub fn new(inner: &'o str, allow_list: Option<&'al HashMap<Script,Vec<Language>>>) -> Self {
        Self { inner, script: None, language: None, allow_list }
    }

    pub fn script(&mut self) -> Script {
        let inner = self.inner;
        *self.script.get_or_insert_with(|| Self::detect_script(inner))
    }

    pub fn language(&mut self) -> Language {
        let inner = self.inner;
        let script = self.script();
        *self.language.get_or_insert_with(|| Self::detect_lang(inner, script, self.allow_list))
    }

    /// detect script with whatlang,
    /// if no script is detected, return Script::Other
    fn detect_script(text: &str) -> Script {
        whatlang::detect_script(text).map(Script::from).unwrap_or_default()
    }

    /// detect lang with whatlang
    /// if no language is detected, return Language::Other
    fn detect_lang(text: &str, script: Script, allow_list : Option<&HashMap<Script,Vec<Language>>>) -> Language {
            let detector = allow_list
                .and_then(|allow_list| allow_list.get(&script))
                .and_then(|allow_list| Some(allow_list.iter().map(|lang|(*lang).into()).collect()))
                .and_then(|allow_list| Some(Detector::with_allowlist(allow_list)))
                .unwrap_or_default();
                
            detector.detect_lang(text).map(Language::from).unwrap_or_default()
    }
}

pub trait Detect<'o, 'al> {
    fn detect(&'o self, allow_list: Option<&'al HashMap<Script,Vec<Language>>>) -> StrDetection<'o, 'al>;
}

impl<'o, 'al> Detect<'o, 'al> for &str {
    fn detect(&'o self, allow_list: Option<&'al HashMap<Script,Vec<Language>>>) -> StrDetection<'o, 'al> 
    {
        StrDetection::new(self,allow_list)
    }
}
