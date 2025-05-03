pub struct Language {
    pub code: &'static str,
}


pub const LANGUAGES: &[Language] = &[
    Language { code: "en_US"},
    Language { code: "es_ES"},
    Language { code: "fr_FR" },
    Language { code: "de_DE"},
];

pub fn get_language_name(code: &str) -> Option<&'static str> {
    for lang in LANGUAGES {
        if lang.code == code {
            return Some(lang.code);
        }
    }
    None
}

pub fn get_system_language() -> Option<&'static str> {
    if let Ok(locale) = std::env::var("LANG") {
        let code = locale.split('.').next().unwrap_or("");
        return get_language_name(code);
    }
    None
}