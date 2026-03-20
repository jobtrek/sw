mod languages;

use clap::Parser;
use languages::LANGUAGES;
use rayon::prelude::*;
use sw::{check_paths_exist, get_files_per_extension, SwError};

/// Removes lines between pairs of markers, replacing the block with a language placeholder.
const WIPE_MARKER: &str = "--sw-wipe--";

/// Removes lines between pairs of markers with no placeholder, even on languages that normally use one.
const VANISH_MARKER: &str = "--sw-vanish--";

/// Removes content between pairs of `/* --sw-- */` markers on the same line.
const INLINE_MARKER: &str = "/* --sw-- */";

/// sw [path = "."]
///
/// options:
/// -e --extensions [extensions = "rs,php,js,ts,java"]
/// --dry-run [dry_run = false]
#[derive(Parser, Debug)]
struct Args {
    #[clap(default_values = &["."])]
    paths: Vec<String>,
    #[clap(short, long, value_enum, default_values = &["rs", "php", "js", "ts", "java"])]
    extensions: Vec<Extension>,
    #[clap(long)]
    fd_bin_path: Option<String>,
    /// Validate marker pairing without modifying any files.
    #[clap(long)]
    dry_run: bool,
}

/// All supported file extensions.
///
/// Keep this list in sync with `LANGUAGES` in `languages.rs`.
#[derive(clap::ValueEnum, Clone, Debug)]
enum Extension {
    Rs,
    Php,
    Js,
    Ts,
    Java,
}

impl Extension {
    fn as_str(&self) -> &str {
        match self {
            Self::Rs => "rs",
            Self::Php => "php",
            Self::Js => "js",
            Self::Ts => "ts",
            Self::Java => "java",
        }
    }
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), SwError> {
    let args = Args::parse();
    check_paths_exist(&args.paths)?;

    // Phase 1: Collect (file path, wipe placeholder) for every file across all
    // extensions and search paths. File discovery is sequential; processing is parallel.
    let mut jobs: Vec<(String, &'static str)> = Vec::new();
    for extension in &args.extensions {
        let ext = extension.as_str();
        // Issue 9: panic with a clear programmer-error message instead of silently
        // falling back to an empty placeholder when the LANGUAGES table is out of sync.
        let placeholder = LANGUAGES
            .iter()
            .find(|l| l.extension == ext)
            .unwrap_or_else(|| {
                panic!("BUG: extension '{ext}' has no entry in LANGUAGES — update languages.rs")
            })
            .wipe_placeholder
            .unwrap_or("");
        for path in &args.paths {
            let files = get_files_per_extension(path, ext, args.fd_bin_path.as_deref())?;
            for file in files {
                jobs.push((file, placeholder));
            }
        }
    }

    // Phase 2: Read all files in parallel.
    let jobs_with_content: Vec<(String, String, &'static str)> = jobs
        .into_par_iter()
        .map(|(path, placeholder)| {
            // Issue 6: propagate I/O errors instead of panicking.
            let content = std::fs::read_to_string(&path).map_err(SwError::Io)?;
            Ok((path, content, placeholder))
        })
        .collect::<Result<_, SwError>>()?;

    // Phase 3: Validate marker pairing across every file before touching anything.
    // Issue 3: fail fast — report all violations and leave all files untouched.
    let errors: Vec<String> = jobs_with_content
        .iter()
        .flat_map(|(path, content, _)| validate_markers(path, content))
        .collect();
    if !errors.is_empty() {
        return Err(SwError::ValidationFailed(errors));
    }

    // --dry-run: validation passed, nothing to write.
    if args.dry_run {
        return Ok(());
    }

    // Phase 4: Transform and write only files whose content actually changes.
    // Issue 6: propagate I/O errors instead of panicking.
    // Issue 7: skip the write when nothing changed to avoid spurious mtime updates.
    jobs_with_content
        .into_par_iter()
        .try_for_each(|(path, content, placeholder)| -> Result<(), SwError> {
            let new_content = transform(&content, placeholder);
            if new_content != content {
                std::fs::write(&path, new_content).map_err(SwError::Io)?;
            }
            Ok(())
        })
}

// ── Validation ────────────────────────────────────────────────────────────────

/// Return one error message per block marker type that has an odd count in `content`.
fn validate_markers(path: &str, content: &str) -> Vec<String> {
    [WIPE_MARKER, VANISH_MARKER]
        .iter()
        .filter_map(|marker| {
            let count = content.lines().filter(|line| line.contains(marker)).count();
            if count % 2 != 0 {
                Some(format!(
                    "{path}: odd number of '{marker}' markers ({count})"
                ))
            } else {
                None
            }
        })
        .collect()
}

// ── Transformation ────────────────────────────────────────────────────────────

/// Apply all three transformations to a file's content.
fn transform(content: &str, wipe_placeholder: &str) -> String {
    let content = if content.contains(WIPE_MARKER) {
        remove_parts(content, WIPE_MARKER, wipe_placeholder)
    } else {
        content.to_string()
    };
    let content = if content.contains(VANISH_MARKER) {
        remove_parts(&content, VANISH_MARKER, "")
    } else {
        content
    };
    if content.contains(INLINE_MARKER) {
        remove_inline(&content)
    } else {
        content
    }
}

/// Issue 4: detect whether the file uses CRLF or LF line endings so we can
/// preserve them when reassembling the output.
fn detect_line_ending(content: &str) -> &str {
    if content.contains("\r\n") {
        "\r\n"
    } else {
        "\n"
    }
}

/// Remove lines between pairs of `matcher` markers.
///
/// When `replace_with` is non-empty, it is inserted (with matching indentation)
/// on the closing marker's line instead of being dropped.
///
/// Issue 4: original line endings (LF or CRLF) are preserved.
/// Issue 5: indentation is captured from both spaces and tabs.
fn remove_parts(file_content: &str, matcher: &str, replace_with: &str) -> String {
    // Issue 4: capture the original line-ending style before .lines() strips them.
    let line_ending = detect_line_ending(file_content);
    let mut remove = false;
    let wiped: Vec<String> = file_content
        .lines()
        .filter_map(|line| {
            if line.contains(matcher) {
                remove = !remove;
                if !remove && !replace_with.is_empty() {
                    // Issue 5: preserve tabs as well as spaces.
                    let indent: String =
                        line.chars().take_while(|&c| c == ' ' || c == '\t').collect();
                    return Some(format!("{indent}{replace_with}"));
                }
                return None;
            }
            if remove {
                return None;
            }
            Some(line.to_string())
        })
        .collect();
    let mut result = wiped.join(line_ending);
    if file_content.ends_with('\n') && !result.ends_with('\n') {
        result.push_str(line_ending);
    }
    result
}

/// Remove character ranges between pairs of `/* --sw-- */` on the same line.
///
/// Multiple pairs on the same line are all processed. Unpaired markers are left as-is.
///
/// Issue 4: original line endings (LF or CRLF) are preserved.
fn remove_inline(file_content: &str) -> String {
    let line_ending = detect_line_ending(file_content);
    let mut result = file_content
        .lines()
        .map(|line| {
            let mut result = line.to_string();
            loop {
                match result.find(INLINE_MARKER) {
                    None => break,
                    Some(start) => {
                        let after_first = start + INLINE_MARKER.len();
                        match result[after_first..].find(INLINE_MARKER) {
                            None => break, // unpaired marker — leave remainder as-is
                            Some(rel) => {
                                let end = after_first + rel + INLINE_MARKER.len();
                                result = format!("{}{}", &result[..start], &result[end..]);
                            }
                        }
                    }
                }
            }
            result
        })
        .collect::<Vec<_>>()
        .join(line_ending);
    if file_content.ends_with('\n') && !result.ends_with('\n') {
        result.push_str(line_ending);
    }
    result
}
