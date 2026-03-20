pub(crate) struct LanguageConfig {
    pub(crate) extension: &'static str,
    /// Placeholder inserted after a `--sw-wipe--` block. `None` means no placeholder.
    pub(crate) wipe_placeholder: Option<&'static str>,
}

pub(crate) const LANGUAGES: &[LanguageConfig] = &[
    LanguageConfig {
        extension: "rs",
        wipe_placeholder: Some("todo!();"),
    },
    LanguageConfig {
        extension: "java",
        wipe_placeholder: Some(
            "throw new UnsupportedOperationException(\"TODO: replace me with your solution !\");",
        ),
    },
    LanguageConfig {
        extension: "php",
        wipe_placeholder: None,
    },
    LanguageConfig {
        extension: "js",
        wipe_placeholder: None,
    },
    LanguageConfig {
        extension: "ts",
        wipe_placeholder: None,
    },
];

/// Return the wipe placeholder for `ext`, or `""` if the language has none.
///
/// # Panics
///
/// Panics if `ext` has no entry in [`LANGUAGES`]. This is a programming error:
/// the `Extension` enum in `main.rs` is out of sync with this table.
pub fn wipe_placeholder(ext: &str) -> &'static str {
    LANGUAGES
        .iter()
        .find(|l| l.extension == ext)
        .unwrap_or_else(|| {
            panic!("BUG: extension '{ext}' has no entry in LANGUAGES — update languages.rs")
        })
        .wipe_placeholder
        .unwrap_or("")
}
