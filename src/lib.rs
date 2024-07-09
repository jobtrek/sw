use std::process::exit;
use std::process::Command;

#[derive(Debug)]
pub enum CommandError {
    Io(std::io::Error),
    Utf8(std::string::FromUtf8Error),
}
impl From<std::io::Error> for CommandError {
    fn from(e: std::io::Error) -> Self {
        CommandError::Io(e)
    }
}
impl From<std::string::FromUtf8Error> for CommandError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        CommandError::Utf8(e)
    }
}

/// run a bash command and return the output
///
/// ```
/// assert_eq!(sw::run_command("echo test").unwrap(), "test\n");
/// assert_eq!(sw::run_command("cat src/lib.rs").unwrap(), std::fs::read_to_string("src/lib.rs").unwrap());
/// ```
pub fn run_command(command: &str) -> Result<String, CommandError> {
    Ok(String::from_utf8(
        Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()?
            .stdout,
    )?)
}

/// check if the given paths exist
/// panic at the end if one of the paths does not exist, with the list of the missing paths
/// paths can be files or directories
///
/// ```
/// sw::check_paths_exist(&["src/lib.rs".to_string(), "..".to_string()]);
/// ```
/// ```rust,should_panic
/// sw::check_paths_exist(&["not_existing.nothing".to_string()]);
/// ```
/// ```rust,should_panic
/// sw::check_paths_exist(&["src/lib.rs".to_string(), "not_existing.nothing".to_string()]);
/// ```
pub fn check_paths_exist(paths: &[String]) {
    let missing_paths = paths
        .iter()
        .filter(|&x| !std::path::Path::new(x).exists())
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    if !missing_paths.is_empty() {
        for path in missing_paths {
            eprintln!("path does not exist: {}", path);
        }
        exit(1);
    }
}

/// get the list of files with the given extension in the given path
/// if the path is a file with the right extension, return a list with only this file
/// if the path is a file with the wrong extension, return an empty list
///
/// ```
/// let fd_bin_path = Some("fdfind"); // set on None if your bin is called "fd", otherwise set the path to the binary
/// assert!(sw::get_files_per_extension("src", "rs", fd_bin_path).unwrap().contains(&"src/lib.rs".to_string()));
/// assert_eq!(sw::get_files_per_extension("src/lib.rs", "rs", fd_bin_path).unwrap(), vec!["src/lib.rs".to_string()]);
/// assert_eq!(sw::get_files_per_extension("src/lib.rs", "toml", fd_bin_path).unwrap(), Vec::<String>::new());
/// ```
pub fn get_files_per_extension(
    path: &str,
    extension: &str,
    fd_bin_path: Option<&str>,
) -> Result<Vec<String>, CommandError> {
    if std::fs::metadata(path)?.is_dir() {
        return Ok(run_command(&format!(
            "{} . {} -e {} --type f",
            fd_bin_path.unwrap_or("fd"),
            path,
            extension
        ))?
        .split('\n')
        .filter(|&x| !x.is_empty())
        .map(|x| x.to_string())
        .collect::<Vec<String>>());
    }
    if !path.ends_with(format!(".{}", extension).as_str()) {
        return Ok(vec![]);
    }
    Ok(vec![path.to_string()])
}
