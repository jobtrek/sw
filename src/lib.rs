use std::process::exit;
use std::process::Command;

/// run a bash command and return the output
///
/// ```
/// assert_eq!(sw::run_command("echo test"), "test\n");
/// assert_eq!(sw::run_command("cat src/lib.rs"), std::fs::read_to_string("src/lib.rs").unwrap());
/// ```
pub fn run_command(command: &str) -> String {
    String::from_utf8(
        Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .expect("failed to execute process")
            .stdout,
    )
    .unwrap()
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
/// assert!(sw::get_files_per_extension("src", "rs").contains(&"src/lib.rs".to_string()));
/// assert_eq!(sw::get_files_per_extension("src/lib.rs", "rs"), vec!["src/lib.rs".to_string()]);
/// assert_eq!(sw::get_files_per_extension("src/lib.rs", "toml"), Vec::<String>::new());
/// ```
pub fn get_files_per_extension(path: &str, extension: &str) -> Vec<String> {
    let fail = |e| {
        panic!("failed to get metadata for {}: {}", path, e);
    };
    if std::fs::metadata(path).unwrap_or_else(fail).is_dir() {
        return run_command(&format!("fd . {} -e {} --type f", path, extension))
            .split('\n')
            .filter(|&x| !x.is_empty())
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
    }
    if !path.ends_with(format!(".{}", extension).as_str()) {
        return vec![];
    }
    vec![path.to_string()]
}
