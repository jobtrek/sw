use std::process::exit;
use std::process::Command;

#[derive(Debug)]
pub enum SwError {
    Io(std::io::Error),
    Utf8(std::string::FromUtf8Error),
    PathsDoNotExist(Vec<String>),
    AstGrepParseError(serde_json::Error),
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

pub fn unwrap_sw_error<T>(result: Result<T, SwError>) -> T {
    match result {
        Ok(v) => v,
        Err(e) => {
            eprintln!(
                "{}",
                match e {
                    SwError::Io(e) => e.to_string(),
                    SwError::Utf8(e) => e.to_string(),
                    SwError::PathsDoNotExist(paths) => {
                        format!("The following paths do not exist: {}", paths.join(", "))
                    }
                    SwError::AstGrepParseError(e) => e.to_string(),
                }
            );
            exit(1);
        }
    }
}

/// run a bash command and return the output
///
/// ```
/// assert_eq!(sw::run_command("echo test").unwrap(), "test\n");
/// assert_eq!(sw::run_command("cat src/lib.rs").unwrap(), std::fs::read_to_string("src/lib.rs").unwrap());
/// ```
pub fn run_command(command: &str) -> Result<String, SwError> {
    Ok(String::from_utf8(
        Command::new("sh").arg("-c").arg(command).output()?.stdout,
    )?)
}

/// check if the given paths exist
/// return an error with the list of the missing paths at the end if one of the paths does not exist
/// paths can be files or directories
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
    let missing_paths = paths
        .iter()
        .filter(|&x| !std::path::Path::new(x).exists())
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    if !missing_paths.is_empty() {
        return Err(SwError::PathsDoNotExist(missing_paths));
    }
    Ok(())
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
) -> Result<Vec<String>, SwError> {
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
