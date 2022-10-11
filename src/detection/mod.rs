pub use script_language::{Language, Script};
use std::collections::HashMap;
use whatlang::Detector;

// file copy pasted from whatlang.
#[allow(dead_code)]
mod chars;
mod script_language;

pub struct StrDetection<'a> {
    inner: &'a str,
    pub script: Option<Script>,
    pub language: Option<Language>,
    allow_list : &'a Option<HashMap<Script,Vec<Language>>>,
}

impl<'a> StrDetection<'a> {
    pub fn new(inner: &'a str, allow_list: &'a Option<HashMap<Script,Vec<Language>>>) -> Self {
        Self { inner, script: None, language: None, allow_list: &allow_list }
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
    fn detect_lang(text: &str, script: Option<Script>, allow_list : &Option<HashMap<Script,Vec<Language>>>) -> Language {
        if let Some(script) = script {
            if let Some(allow_list) = allow_list {
                if let Some(allow_list) = allow_list.get(&script){
                    if !allow_list.is_empty() {
                        let allow_list = allow_list.iter().map(|lang| (*lang).into()).collect();
                        let detector = Detector::with_allowlist(allow_list);
                        return detector.detect_lang(text).map(Language::from).unwrap_or_default()
                    }
                }
            }
        } 
        whatlang::detect_lang(text).map(Language::from).unwrap_or_default()
    }
}

pub trait Detect<'a> {
    fn detect(&'a self, allow_list: &'a Option<HashMap<Script,Vec<Language>>>) -> StrDetection<'a>;
}

impl<'a> Detect<'a> for &str {
    fn detect(&'a self, allow_list: &'a Option<HashMap<Script,Vec<Language>>>) -> StrDetection<'a> 
    {
        StrDetection::new(self,&allow_list)
    }
}
