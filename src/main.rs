use serde::{Deserialize, Serialize};
use std::process::Command;

// structure of the json returned by ast-grep (only the useful parts)
structstruck::strike! {
    #[strikethrough[derive(Serialize, Deserialize, Debug)]]
    struct Program {
        text: String,
        range: struct {
            start: struct {
                line: u16,
                column: u16,
            },
            end: struct {
                line: u16,
                column: u16,
            }
        },
        #[serde(rename = "metaVariables")]
        meta_variables: struct {
            single: struct {
                #[serde(rename = "COMMENT")]
                comment: struct {
                    text: String,
                    // here we can directly refer to the range struct as we allready defined it
                    range: Range
                }
            }
        }
    }
}

fn main() {
    let path = "./test";
    let extensions = "rs,php,js,ts,java".split(',').collect::<Vec<&str>>();
    if extensions.iter().any(|&x| !["rs", "php", "js", "ts", "java"].contains(&x)) {
        panic!("invalid extensions, only 'rs', 'php', 'js', 'ts' and 'java' are allowed");
    }
    for extension in extensions {
        let files = get_files(path, extension);
        for file in files {
            let parsed = get_removable_parts(extension, &file);
            if parsed.is_empty() {
                continue;
            }
            match extension {
                "rs" => remove_parts(&file, &parsed, "todo!()"),
                _ => remove_parts(&file, &parsed, ""),
            }.unwrap_or_else(|e| eprintln!("failed to remove parts from {}: {}", file, e));
        }
    }
}

/// remove the parts of the file that are defined in the given list of programs
/// parts are removed in reverse order to avoid changing the line numbers of the other parts
fn remove_parts(file: &str, parts: &[Program], replace_with: &str) -> std::io::Result<()> {
    let content = std::fs::read_to_string(file)?;
    let mut content = content.lines().map(String::from).collect::<Vec<String>>();
    for part in parts.iter().rev() {
        content.splice(
            (part.meta_variables.single.comment.range.end.line as usize + 1)
                ..=(part.range.end.line as usize - 1),
            indent(
                replace_with,
                part.meta_variables.single.comment.range.start.column as usize,
            ),
        );
    }
    std::fs::write(file, content.join("\n"))?;
    Ok(())
}

/// add the right amount of spaces to the beginning of each lines
fn indent(lines: &str, spaces: usize) -> Vec<String> {
    lines
        .lines()
        .map(|x| format!("{: >width$}{}", "", x, width = spaces))
        .collect()
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
fn get_removable_parts(extension: &str, file: &str) -> Vec<Program> {
    serde_json::from_str(&run_command(&format!(
        "ast-grep scan --rule /etc/jobtrek/sw/ast-grep-rules/{}.yaml {} --json",
        extension, file
    )))
    .unwrap()
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
