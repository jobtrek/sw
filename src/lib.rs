mod languages;
mod processor;

pub use languages::wipe_placeholder;
pub use processor::{transform, validate_markers, INLINE_MARKER, VANISH_MARKER, WIPE_MARKER};

use std::fmt;
use std::process::Command;

// ── Error type ────────────────────────────────────────────────────────────────

#[derive(Debug)]
pub enum SwError {
    Io(std::io::Error),
    Utf8(std::string::FromUtf8Error),
    PathsDoNotExist(Vec<String>),
    /// `fd` exited with a non-zero status.
    CommandFailed {
        exit_code: Option<i32>,
        stderr: String,
    },
    /// One or more files contain an odd number of block markers.
    ValidationFailed(Vec<String>),
}

impl fmt::Display for SwError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SwError::Io(e) => write!(f, "I/O error: {e}"),
            SwError::Utf8(e) => write!(f, "UTF-8 decoding error: {e}"),
            SwError::PathsDoNotExist(paths) => {
                write!(f, "The following paths do not exist: {}", paths.join(", "))
            }
            SwError::CommandFailed { exit_code, stderr } => {
                let code = exit_code
                    .map(|c| c.to_string())
                    .unwrap_or_else(|| "unknown".to_string());
                write!(f, "Command failed (exit code {code}): {}", stderr.trim())
            }
            SwError::ValidationFailed(messages) => {
                writeln!(f, "Validation failed:")?;
                for msg in messages {
                    writeln!(f, "  {msg}")?;
                }
                Ok(())
            }
        }
    }
}

impl std::error::Error for SwError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            SwError::Io(e) => Some(e),
            SwError::Utf8(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for SwError {
    fn from(e: std::io::Error) -> Self {
        SwError::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for SwError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        SwError::Utf8(e)
    }
}

// ── Utilities ─────────────────────────────────────────────────────────────────

/// Return `Ok(())` if every path in `paths` exists on disk, or an error listing the missing ones.
///
/// ```
/// sw::check_paths_exist(&["src/lib.rs".to_string(), "..".to_string()]).unwrap();
/// ```
/// ```rust,should_panic
/// sw::check_paths_exist(&["not_existing.nothing".to_string()]).unwrap();
/// ```
/// ```rust,should_panic
/// sw::check_paths_exist(&["src/lib.rs".to_string(), "not_existing.nothing".to_string()]).unwrap();
/// ```
pub fn check_paths_exist(paths: &[String]) -> Result<(), SwError> {
    let missing: Vec<String> = paths
        .iter()
        .filter(|p| !std::path::Path::new(p.as_str()).exists())
        .cloned()
        .collect();
    if !missing.is_empty() {
        return Err(SwError::PathsDoNotExist(missing));
    }
    Ok(())
}

/// Return the list of files with the given extension reachable from `path`.
///
/// - If `path` is a directory, `fd` is used to enumerate files recursively.
/// - If `path` is a file whose name ends with `.{extension}`, it is returned as-is.
/// - Otherwise an empty list is returned.
///
/// The `fd_bin_path` parameter overrides the `fd` binary name/path.
///
/// ```
/// // Pass Some("fdfind") or Some("/path/to/fd") to override the binary name.
/// let fd_bin_path = None; // uses "fd" from PATH
/// assert!(sw::get_files_per_extension("src", "rs", fd_bin_path).unwrap().contains(&"src/lib.rs".to_string()));
/// assert_eq!(sw::get_files_per_extension("src/lib.rs", "rs", fd_bin_path).unwrap(), vec!["src/lib.rs".to_string()]);
/// assert_eq!(sw::get_files_per_extension("src/lib.rs", "toml", fd_bin_path).unwrap(), Vec::<String>::new());
/// ```
pub fn get_files_per_extension(
    path: &str,
    extension: &str,
    fd_bin_path: Option<&str>,
) -> Result<Vec<String>, SwError> {
    if std::fs::metadata(path)?.is_dir() {
        // Use Command::args to avoid any shell interpolation of user-supplied values.
        let output = Command::new(fd_bin_path.unwrap_or("fd"))
            .args([".", path, "-e", extension, "--type", "f"])
            .output()?;
        if !output.status.success() {
            return Err(SwError::CommandFailed {
                exit_code: output.status.code(),
                stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
            });
        }
        return Ok(String::from_utf8(output.stdout)?
            .split('\n')
            .filter(|s| !s.is_empty())
            .map(str::to_string)
            .collect());
    }
    if !path.ends_with(&format!(".{extension}")) {
        return Ok(vec![]);
    }
    Ok(vec![path.to_string()])
}
