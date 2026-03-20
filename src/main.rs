use clap::Parser;
use rayon::prelude::*;
use sw::{
    check_paths_exist, get_files_per_extension, transform, validate_markers, wipe_placeholder,
    SwError,
};

/// sw [path = "."]
///
/// options:
/// -e --extensions [extensions = "rs,php,js,ts,java"]
/// --dry-run [dry_run = false]
#[derive(Parser, Debug)]
struct Args {
    #[clap(default_values = &["."])]
    paths: Vec<String>,
    #[clap(short, long, value_enum, default_values = &["rs", "php", "js", "ts", "java"])]
    extensions: Vec<Extension>,
    #[clap(long)]
    fd_bin_path: Option<String>,
    /// Validate marker pairing without modifying any files.
    #[clap(long)]
    dry_run: bool,
}

/// All supported file extensions.
///
/// Keep this list in sync with `LANGUAGES` in `languages.rs`.
#[derive(clap::ValueEnum, Clone, Debug)]
enum Extension {
    Rs,
    Php,
    Js,
    Ts,
    Java,
}

impl Extension {
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

fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), SwError> {
    let args = Args::parse();
    check_paths_exist(&args.paths)?;

    // Phase 1: Collect (file path, wipe placeholder) for every file across all
    // extensions and search paths. File discovery is sequential; processing is parallel.
    let mut jobs: Vec<(String, &'static str)> = Vec::new();
    for extension in &args.extensions {
        let ext = extension.as_str();
        let placeholder = wipe_placeholder(ext);
        for path in &args.paths {
            let files = get_files_per_extension(path, ext, args.fd_bin_path.as_deref())?;
            for file in files {
                jobs.push((file, placeholder));
            }
        }
    }

    // Phase 2: Read all files in parallel.
    let jobs_with_content: Vec<(String, String, &'static str)> = jobs
        .into_par_iter()
        .map(|(path, placeholder)| {
            let content = std::fs::read_to_string(&path).map_err(SwError::Io)?;
            Ok((path, content, placeholder))
        })
        .collect::<Result<_, SwError>>()?;

    // Phase 3: Validate marker pairing across every file before touching anything.
    // Fail fast — report all violations and leave all files untouched.
    let errors: Vec<String> = jobs_with_content
        .iter()
        .flat_map(|(path, content, _)| validate_markers(path, content))
        .collect();
    if !errors.is_empty() {
        return Err(SwError::ValidationFailed(errors));
    }

    // --dry-run: validation passed, nothing to write.
    if args.dry_run {
        return Ok(());
    }

    // Phase 4: Transform and write only files whose content actually changes.
    jobs_with_content
        .into_par_iter()
        .try_for_each(|(path, content, placeholder)| -> Result<(), SwError> {
            let new_content = transform(&content, placeholder);
            if new_content != content {
                std::fs::write(&path, new_content).map_err(SwError::Io)?;
            }
            Ok(())
        })
}
