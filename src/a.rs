use std::process::Command;

fn main() {
    println!("{:?}", get_files_per_extension("src", "rs"));
}

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
