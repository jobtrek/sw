use std::process::Command;

fn main() {
    let path = "./example";
    let extensions = "rs,php,js,ts,java".split(',').collect::<Vec<&str>>();

    for extension in extensions {
        // get all files with the given extension in the path
        let files = run_command(&format!("fd . {} -e {} --type f", path, extension));
        let files = files.split('\n').collect::<Vec<&str>>();
        let files = files.iter().filter(|&x| !x.is_empty()).collect::<Vec<&&str>>();
        for file in files {

            println!("{}", file);
        }
    }
}
    println!("Hello, world!");
}

/// run a bash command and return the output
fn run_command(command: &str) -> String {
    let output = Command::new("bash")
        .arg("-c")
        .arg(command)
        .output()
        .expect("failed to execute process");
    String::from_utf8(output.stdout).unwrap()
}

