use clap::Parser;
use sw::{check_paths_exist, get_files_per_extension, unwrap_sw_error};

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
    #[clap(short, long, value_enum, default_values = &["rs", "php", "js", "ts", "java"])]
    extensions: Vec<Extension>,
    #[clap(long)]
    silent: bool,
    #[clap(long)]
    fd_bin_path: Option<String>,
    #[clap(long, default_value = "--sw-wipe--")]
    matcher: String,
}

/// enum to represent all possible extensions
#[derive(clap::ValueEnum, Clone, Debug)]
enum Extension {
    Rs,
    Php,
    Js,
    Ts,
    Java,
}
impl Extension {
    /// return the string representation of the extension
    fn as_str(&self) -> &str {
        match self {
            Self::Rs => "rs",
            Self::Php => "php",
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
    unwrap_sw_error(check_paths_exist(&args.paths));
    let mut checked_files = Vec::new();

    for extension in args.extensions {
        let extension = extension.as_str();
        for path in args.paths.iter() {
            let files = unwrap_sw_error(get_files_per_extension(
                path,
                extension,
                args.fd_bin_path.as_deref(),
            ));
            for file in files {
                if checked_files.contains(&file) {
                    // if a file is in multiple paths, it may be checked multiple times so we skip it
                    continue;
                }
                checked_files.push(file.clone());

                if !args.silent {
                    println!("Replacing in {}", file);
                }

                let file_content = std::fs::read_to_string(&file).expect("Cannot read file");

                std::fs::write(file, match extension {
                    "rs" => remove_parts(&file_content, &args.matcher, "todo!()"),
                    _ => remove_parts(&file_content, &args.matcher, ""),
                }).expect("Cannot write file");
            }
        }
    }
}

/// remove the parts of the file that are defined in the given list of programs
/// they are removed in reverse order to avoid changing the line numbers of the area that still haven't been removed
fn remove_parts(file_content: &str, matcher: &str, replace_with: &str) -> String {
    // convert the content of the file from a string to a vector of strings (one string per line)
    let file_content = file_content
        .lines()
        .map(String::from)
        .collect::<Vec<String>>();
    
    // iterate through all lines in reverse and remove all lines between lines that contains matcher
    let mut remove = false;
    let wiped_file: Vec<&str> = file_content.iter().filter_map(|line| {
        if line.contains(matcher) {
            remove = !remove;
            if !remove {
                return Some(replace_with)
            }
        }
        if remove {
            return None
        }
        Some(line)
    }).collect();
    wiped_file.join("\n")
}
