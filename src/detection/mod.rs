pub use script_language::{Language, Script};

// file copy pasted from whatlang.
#[allow(dead_code)]
mod chars;
mod script_language;

pub struct StrDetection<'a> {
    inner: &'a str,
    pub script: Option<Script>,
    pub language: Option<Language>,
}

impl<'a> StrDetection<'a> {
    pub fn new(inner: &'a str) -> Self {
        Self { inner, script: None, language: None }
    }

    pub fn script(&mut self) -> Script {
        let inner = self.inner;
        *self.script.get_or_insert_with(|| Self::detect_script(inner))
    }

    pub fn language(&mut self) -> Language {
        let inner = self.inner;
        *self.language.get_or_insert_with(|| Self::detect_lang(inner))
    }

    /// detect script with whatlang,
    /// if no script is detected, return Script::Other
    fn detect_script(text: &str) -> Script {
        whatlang::detect_script(text).map(Script::from).unwrap_or_default()
    }

    /// detect lang with whatlang
    /// if no language is detected, return Language::Other
    fn detect_lang(text: &str) -> Language {
        whatlang::detect_lang(text).map(Language::from).unwrap_or_default()
    }
}

pub trait Detect {
    fn detect(&self) -> StrDetection;
}

impl Detect for &str {
    fn detect(&self) -> StrDetection {
        StrDetection::new(self)
    }
}
