use std::process::Command;

fn main() {
    let path = "./example";
    let extensions = "rs,php,js,ts,java".split(',').collect::<Vec<&str>>();

    for extension in extensions {
        // get all files with the given extension in the path
        let files = get_files(path, extension);
        for file in files {
            let parts = get_removable_parts(extension, &file);
            println!("{}", parts);
        }
    }
}

/// get the list of files with the given extension in the given path
fn get_files(path: &str, extension: &str) -> Vec<String> {
    run_command(&format!("fd . {} -e {} --type f", path, extension))
        .split('\n')
        .filter(|&x| !x.is_empty())
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
}

/// get the positions of the comments and block who define the beginning of the part to remove
fn get_removable_parts(extension: &str, file: &str) -> String {
    let json = run_command(&format!(
        "ast-grep scan --rule /etc/jobtrek/sw/ast-grep-rules/{}.yaml {} --json=stream",
        extension, file
    ));
    json
}

/// run a bash command and return the output
fn run_command(command: &str) -> String {
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
