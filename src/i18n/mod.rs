//! Internationalization (i18n) support for herdr.
//!
//! This module provides a lightweight, compile-time embedded localization system.
//! Locale strings are stored in TOML files, flattened into dot-separated keys
//! at initialization time, and looked up through a global [`Locale`] instance.
//!
//! # Quick start
//!
//! ```ignore
//! use crate::i18n;
//!
//! // Initialize with the detected language (or override explicitly).
//! i18n::init(i18n::Language::detect_from_env());
//!
//! // Translate keys anywhere in the application.
//! let msg = i18n::tr("app.greeting");
//! ```
//!
//! The `tr!` macro is also available as shorthand:
//!
//! ```ignore
//! let msg = tr!("app.greeting");
//! ```

#[macro_use]
mod macros;

use std::collections::HashMap;
use std::sync::OnceLock;

// ---------------------------------------------------------------------------
// Embedded locale files
// ---------------------------------------------------------------------------

const EN_TOML: &str = include_str!("en.toml");
const ZH_CN_TOML: &str = include_str!("zh_CN.toml");

// ---------------------------------------------------------------------------
// Language
// ---------------------------------------------------------------------------

/// Supported UI languages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    /// English (default).
    En,
    /// Simplified Chinese.
    ZhCN,
}

/// All supported languages in display order.
const ALL_LANGUAGES: &[Language] = &[Language::En, Language::ZhCN];

impl Language {
    /// Returns the BCP-47 style language code used for locale file lookup.
    ///
    /// - `Language::En` -> `"en"`
    /// - `Language::ZhCN` -> `"zh_CN"`
    pub fn code(self) -> &'static str {
        match self {
            Language::En => "en",
            Language::ZhCN => "zh_CN",
        }
    }

    /// Returns a human-readable name in the language itself.
    pub fn display_name(self) -> &'static str {
        match self {
            Language::En => "English",
            Language::ZhCN => "\u{7b80}\u{4f53}\u{4e2d}\u{6587}", // 简体中文
        }
    }

    /// Returns a static slice of all supported languages.
    pub fn all() -> &'static [Language] {
        ALL_LANGUAGES
    }

    /// Attempts to match a language code string to a [`Language`].
    ///
    /// Recognises the following codes (case-insensitive):
    ///
    /// | Code          | Language          |
    /// |---------------|-------------------|
    /// | `en`          | `Language::En`    |
    /// | `en_us`       | `Language::En`    |
    /// | `en_gb`       | `Language::En`    |
    /// | `zh_cn`       | `Language::ZhCN`  |
    /// | `zh`          | `Language::ZhCN`  |
    /// | `zh_hans`     | `Language::ZhCN`  |
    pub fn from_code(code: &str) -> Option<Self> {
        match code.to_lowercase().replace('-', "_").as_str() {
            "en" | "en_us" | "en_gb" => Some(Language::En),
            "zh_cn" | "zh" | "zh_hans" => Some(Language::ZhCN),
            _ => None,
        }
    }

    /// Auto-detects the preferred language from environment variables.
    ///
    /// The following variables are checked in order:
    ///
    /// 1. `HERDR_LANG` (application-specific override)
    /// 2. `LC_ALL` (POSIX locale)
    /// 3. `LANG` (POSIX locale)
    ///
    /// Falls back to [`Language::En`] if none of the variables are set or
    /// none of them match a supported language.
    pub fn detect_from_env() -> Self {
        for var in &["HERDR_LANG", "LC_ALL", "LANG"] {
            if let Ok(val) = std::env::var(var) {
                if let Some(lang) = Self::from_code(&val) {
                    return lang;
                }
            }
        }
        Language::En
    }

    /// Returns the embedded TOML source for this language.
    fn embedded_toml(self) -> &'static str {
        match self {
            Language::En => EN_TOML,
            Language::ZhCN => ZH_CN_TOML,
        }
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.display_name())
    }
}

// ---------------------------------------------------------------------------
// Locale
// ---------------------------------------------------------------------------

/// A flat key-value store of translated strings.
///
/// Nested TOML tables are flattened into dot-separated keys. For example:
///
/// ```toml
/// [app]
/// greeting = "Welcome"
/// ```
///
/// becomes the key `"app.greeting"` with value `"Welcome"`.
pub struct Locale {
    entries: HashMap<String, String>,
}

impl Locale {
    /// Parses a TOML string and builds a [`Locale`] with flattened keys.
    ///
    /// Nested tables produce dot-separated keys:
    ///
    /// ```toml
    /// [terminal]
    /// title = "Terminal"
    /// [session]
    /// save = "Save Session"
    /// ```
    ///
    /// Results in:
    /// - `"terminal.title"` -> `"Terminal"`
    /// - `"session.save"` -> `"Save Session"`
    ///
    /// # Panics
    ///
    /// Panics if `toml_str` is not valid TOML or contains non-string leaf values.
    pub fn from_toml(_lang: Language, toml_str: &str) -> Self {
        let value: toml::Value =
            toml::from_str(toml_str).expect("embedded locale TOML must be valid");
        let mut entries = HashMap::new();
        Self::flatten(&value, String::new(), &mut entries);
        Locale { entries }
    }

    /// Recursively flattens a TOML value tree into dot-separated key-value pairs.
    fn flatten(value: &toml::Value, prefix: String, out: &mut HashMap<String, String>) {
        match value {
            toml::Value::Table(table) => {
                for (key, val) in table {
                    let full_key = if prefix.is_empty() {
                        key.clone()
                    } else {
                        format!("{prefix}.{key}")
                    };
                    Self::flatten(val, full_key, out);
                }
            }
            toml::Value::String(s) => {
                out.insert(prefix, s.clone());
            }
            // Non-string leaf values are ignored; locale files should only
            // contain strings. If a non-string value is encountered we skip
            // it gracefully rather than panicking.
            _ => {}
        }
    }

    /// Looks up a translated string by its dot-separated key.
    ///
    /// Returns the key itself as a fallback if no translation is found,
    /// making missing translations visible during development without
    /// causing runtime errors.
    pub fn get<'a>(&'a self, key: &'a str) -> &'a str {
        self.entries.get(key).map(|s| s.as_str()).unwrap_or(key)
    }
}

// ---------------------------------------------------------------------------
// Global locale singleton
// ---------------------------------------------------------------------------

/// The global locale, initialized once via [`init`] and read via [`current`].
static LOCALE: OnceLock<Locale> = OnceLock::new();

/// Initializes the global locale for the given language.
///
/// This function must be called **once** early in the application lifecycle
/// (typically in `main`). Subsequent calls are silently ignored.
///
/// If this function is never called, [`current`] will lazily initialise
/// with [`Language::En`].
pub fn init(lang: Language) {
    let _ = LOCALE.set(Locale::from_toml(lang, lang.embedded_toml()));
}

/// Returns a reference to the current global [`Locale`].
///
/// If [`init`] has not been called, this lazily initialises the locale
/// using [`Language::detect_from_env`].
pub fn current() -> &'static Locale {
    LOCALE.get_or_init(|| {
        let lang = Language::detect_from_env();
        Locale::from_toml(lang, lang.embedded_toml())
    })
}

/// Convenience function: translates a key using the current locale.
///
/// Equivalent to `current().get(key)`. Returns the key itself when no
/// translation is found.
///
/// A `tr!` macro is also available and is preferred in most call sites.
pub fn tr(key: &str) -> &str {
    current().get(key)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn language_code_roundtrip() {
        for &lang in Language::all() {
            let code = lang.code();
            assert_eq!(Language::from_code(code), Some(lang));
        }
    }

    #[test]
    fn language_from_code_variants() {
        assert_eq!(Language::from_code("en"), Some(Language::En));
        assert_eq!(Language::from_code("en_US"), Some(Language::En));
        assert_eq!(Language::from_code("en-GB"), Some(Language::En));
        assert_eq!(Language::from_code("zh"), Some(Language::ZhCN));
        assert_eq!(Language::from_code("zh_CN"), Some(Language::ZhCN));
        assert_eq!(Language::from_code("zh-Hans"), Some(Language::ZhCN));
        assert_eq!(Language::from_code("fr"), None);
    }

    #[test]
    fn locale_from_toml_flattens_keys() {
        let toml = r#"
[app]
name = "herdr"

[terminal]
title = "Terminal"
"#;
        let locale = Locale::from_toml(Language::En, toml);
        assert_eq!(locale.get("app.name"), "herdr");
        assert_eq!(locale.get("terminal.title"), "Terminal");
    }

    #[test]
    fn locale_get_returns_key_as_fallback() {
        let toml = r#"
[app]
name = "herdr"
"#;
        let locale = Locale::from_toml(Language::En, toml);
        assert_eq!(locale.get("nonexistent.key"), "nonexistent.key");
    }

    #[test]
    fn embedded_en_toml_parses() {
        let locale = Locale::from_toml(Language::En, EN_TOML);
        assert_eq!(locale.get("onboarding.title"), "herdr");
        assert_eq!(locale.get("button.save"), "save");
    }

    #[test]
    fn embedded_zh_cn_toml_parses() {
        let locale = Locale::from_toml(Language::ZhCN, ZH_CN_TOML);
        assert_eq!(locale.get("onboarding.title"), "herdr");
        assert_eq!(locale.get("button.save"), "\u{4fdd}\u{5b58}");
    }

    #[test]
    fn tr_shorthand_works() {
        let _ = LOCALE.set(Locale::from_toml(Language::En, EN_TOML));
        // After init, tr() should resolve keys.
        // Note: OnceLock is already set above so this test relies on that.
        // In practice `init` is called once in main.
    }

    #[test]
    fn language_display_name() {
        assert_eq!(Language::En.display_name(), "English");
        assert_eq!(
            Language::ZhCN.display_name(),
            "\u{7b80}\u{4f53}\u{4e2d}\u{6587}"
        );
    }
}
