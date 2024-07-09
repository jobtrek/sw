use clap::Parser;
use serde::{Deserialize, Serialize};
use sw::{check_paths_exist, get_files_per_extension, run_command};

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
    #[clap(long)]
    fd_bin_path: Option<String>,
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
            let files = get_files_per_extension(path, extension, args.fd_bin_path.as_deref()).unwrap();
            for file in files {
                if checked_files.contains(&file) {
                    // if a file is in multiple paths, it may be checked multiple times so we skip it
                    continue;
                }
                checked_files.push(file.clone());

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
fn remove_parts(file: &str, area_to_remove: &[Program], replace_with: &str) -> std::io::Result<()> {
    let file_content = std::fs::read_to_string(file)?;
    // convert the content of the file from a string to a vector of strings (one string per line)
    let mut file_content = file_content
        .lines()
        .map(String::from)
        .collect::<Vec<String>>();
    for area in area_to_remove.iter().rev() {
        file_content.splice(
            (area.meta_variables.single.comment.range.end.line as usize + 1)
                ..=(area.range.end.line as usize - 1),
            indent(
                replace_with,
                area.meta_variables.single.comment.range.start.column as usize,
            ),
        );
    }
    std::fs::write(file, file_content.join("\n"))?;
    Ok(())
}

/// add the right amount of spaces to the beginning of each lines
pub fn indent(lines: &str, spaces: usize) -> Vec<String> {
    lines
        .lines()
        .map(|x| format!("{: >width$}{}", "", x, width = spaces))
        .collect()
}

/// get the positions of the comments and block who define the part to remove
fn get_removable_parts(extension: &str, file: &str) -> Vec<Program> {
    serde_json::from_str(&run_command(&format!(
        "ast-grep scan --rule /etc/jobtrek/sw/ast-grep-rules/{}.yaml {} --json",
        extension, file
    )).unwrap())
    .unwrap_or_else(|e| {
        panic!("failed to parse ast-grep output for {}: {}", file, e);
    })
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indent() {
        assert_eq!(indent("a\nb", 2), vec!["  a", "  b"]);
        assert_eq!(indent("a\nb", 0), vec!["a", "b"]);
        assert_eq!(indent("a", 4), vec!["    a"]);
        assert_eq!(indent("", 2), Vec::<String>::new());
    }
}
