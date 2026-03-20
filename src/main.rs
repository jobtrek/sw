mod languages;

use clap::Parser;
use languages::LANGUAGES;
use rayon::prelude::*;
use sw::{check_paths_exist, get_files_per_extension, unwrap_sw_error};

/// Removes lines between pairs of markers, replacing the block with a language placeholder.
const WIPE_MARKER: &str = "--sw-wipe--";

/// Removes lines between pairs of markers with no placeholder, even on languages that normally use one.
const VANISH_MARKER: &str = "--sw-vanish--";

/// Removes content between pairs of markers at character precision (same line only).
const INLINE_MARKER: &str = "/* --sw-- */";

/// sw [path = "."]
///
/// options:
/// -e --extensions [extensions = "rs,php,js,ts,java"]
/// --silent [silent = false]
#[derive(Parser, Debug)]
struct Args {
    #[clap(default_values = &["."])]
    paths: Vec<String>,
    #[clap(short, long, value_enum, default_values = &["rs", "php", "js", "ts", "java"])]
    extensions: Vec<Extension>,
    #[clap(long)]
    silent: bool,
    #[clap(long)]
    fd_bin_path: Option<String>,
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
    let args = Args::parse();
    unwrap_sw_error(check_paths_exist(&args.paths));

    for extension in &args.extensions {
        let extension = extension.as_str();
        let placeholder = LANGUAGES
            .iter()
            .find(|l| l.extension == extension)
            .and_then(|l| l.wipe_placeholder)
            .unwrap_or("");

        for path in args.paths.iter() {
            let files = unwrap_sw_error(get_files_per_extension(
                path,
                extension,
                args.fd_bin_path.as_deref(),
            ));
            files.par_iter().for_each(|file| {
                if !args.silent {
                    println!("Replacing in {}", file);
                }

                let content = std::fs::read_to_string(file).expect("Cannot read file");
                let content = if content.contains(WIPE_MARKER) {
                    remove_parts(&content, WIPE_MARKER, placeholder)
                } else {
                    content
                };
                let content = if content.contains(VANISH_MARKER) {
                    remove_parts(&content, VANISH_MARKER, "")
                } else {
                    content
                };
                let content = if content.contains(INLINE_MARKER) {
                    remove_inline(&content)
                } else {
                    content
                };

                std::fs::write(file, content).expect("Cannot write file");
            });
        }
    }
}

/// Remove lines between pairs of `matcher` markers.
///
/// When `replace_with` is non-empty, it is inserted (with matching indentation)
/// on the closing marker's line instead of being dropped.
fn remove_parts(file_content: &str, matcher: &str, replace_with: &str) -> String {
    let mut remove = false;
    let wiped: Vec<String> = file_content
        .lines()
        .filter_map(|line| {
            if line.contains(matcher) {
                remove = !remove;
                if !remove && !replace_with.is_empty() {
                    let spaces = line.chars().take_while(|&c| c == ' ').count();
                    return Some(format!("{}{}", " ".repeat(spaces), replace_with));
                }
                return None;
            }
            if remove {
                return None;
            }
            Some(line.to_string())
        })
        .collect();
    let mut result = wiped.join("\n");
    if file_content.ends_with('\n') && !result.ends_with('\n') {
        result.push('\n');
    }
    result
}

/// Remove character ranges between pairs of `/* --sw-- */` on the same line.
///
/// Multiple pairs on the same line are all processed. Unpaired markers are left as-is.
fn remove_inline(file_content: &str) -> String {
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
        .join("\n");
    if file_content.ends_with('\n') && !result.ends_with('\n') {
        result.push('\n');
    }
    result
}
