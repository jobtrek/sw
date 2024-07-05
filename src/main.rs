use clap::Parser;
use serde::{Deserialize, Serialize};
use std::process::exit;
use std::process::Command;

structstruck::strike! {
    /// structure of the json returned by ast-grep (only the useful parts)
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

/// structure of the clap arguments
/// sw [path = "."]
///
/// options:
/// -e --extensions [extensions = "rs,php,js,ts,java"]
/// --silent [silent = false]
#[derive(Parser, Debug)]
struct Args {
    #[clap(default_values = &["."])]
    paths: Vec<String>,
    #[clap(short, long, value_enum, default_values = &["rs", "js", "ts", "java"])]
    extensions: Vec<Extension>,
    #[clap(long)]
    silent: bool,
}

/// enum to represent all possible extensions
#[derive(clap::ValueEnum, Clone, Debug)]
enum Extension {
    Rs,
    // Php,
    Js,
    Ts,
    Java,
}
impl Extension {
    /// return the string representation of the extension
    fn as_str(&self) -> &str {
        match self {
            Self::Rs => "rs",
            // Self::Php => "php",
            Self::Js => "js",
            Self::Ts => "ts",
            Self::Java => "java",
        }
    }
}
impl std::str::FromStr for Extension {
    type Err = String;
    /// parse the string to an extension
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "rs" => Ok(Self::Rs),
            // "php" => Ok(Self::Php),
            "js" => Ok(Self::Js),
            "ts" => Ok(Self::Ts),
            "java" => Ok(Self::Java),
            _ => Err(format!("invalid extension: {}", s)),
        }
    }
}

/// main function of the program
/// get the list of files matching the arguments given by the user
/// for each of these files, get wich parts are solutions that should be removed
/// remove these parts from the files
fn main() {
    let args = Args::parse();
    check_paths_exist(&args.paths);
    let mut checked_files = Vec::new();

    for extension in args.extensions {
        let extension = extension.as_str();
        for path in args.paths.iter() {
            let files = get_files(path, extension);
            for file in files {
                if checked_files.contains(&file) {
                    // if a file is in multiple paths, it may be checked multiple times so we skip it
                    continue;
                } else {
                    checked_files.push(file.clone());
                }
                if !args.silent {
                    println!("{}", file);
                }
                let parsed = get_removable_parts(extension, &file);
                if parsed.is_empty() {
                    // don't modyfy a file if it has nothing to remove
                    continue;
                }
                match extension {
                    "rs" => remove_parts(&file, &parsed, "todo!()"),
                    _ => remove_parts(&file, &parsed, ""),
                }
                .unwrap_or_else(|e| eprintln!("failed to remove parts from {}: {}", file, e));
            }
        }
    }
}

/// remove the parts of the file that are defined in the given list of programs
/// they are removed in reverse order to avoid changing the line numbers of the area that still haven't been removed
fn remove_parts(file: &str, parts: &[Program], replace_with: &str) -> std::io::Result<()> {
    let content = std::fs::read_to_string(file)?;
    // convert the content of the file from a string to a vector of strings (one string per line)
    let mut content = content
        .lines()
        .map(String::from)
        .collect::<Vec<String>>();
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
/// if the path is a file with the right extension, return a list with only this file
/// if the path is a file with the wrong extension, return an empty list
fn get_files(path: &str, extension: &str) -> Vec<String> {
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

/// get the positions of the comments and block who define the part to remove
fn get_removable_parts(extension: &str, file: &str) -> Vec<Program> {
    serde_json::from_str(&run_command(&format!(
        "ast-grep scan --rule /etc/jobtrek/sw/ast-grep-rules/{}.yaml {} --json",
        extension, file
    )))
    .unwrap_or_else(|e| {
        panic!("failed to parse ast-grep output for {}: {}", file, e);
    })
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

/// check if the given paths exist
/// panic at the end if one of the paths does not exist, with the list of the missing paths
/// paths can be files or directories
fn check_paths_exist(paths: &[String]) {
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
