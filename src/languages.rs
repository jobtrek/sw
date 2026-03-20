pub struct LanguageConfig {
    pub extension: &'static str,
    /// Placeholder inserted after a `--sw-wipe--` block. `None` means no placeholder.
    pub wipe_placeholder: Option<&'static str>,
}

/// Language configurations.
///
/// Keep this list in sync with the `Extension` enum in `main.rs`.
pub const LANGUAGES: &[LanguageConfig] = &[
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
