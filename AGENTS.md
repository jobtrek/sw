# Agents instructions

This file provides instructions agents must follow when working with this repository.

## What this project does

`sw` (solution wiper) is a Rust CLI tool that removes solution code from programming exercises. It scans files for lines containing `--sw-wipe--` and removes all content between pairs of those marker lines. For Rust files it replaces the wiped content with `todo!();`, for Java with a `throw new UnsupportedOperationException(...)`, and for other languages with nothing.

## Commands

```bash
# Build
cargo build

# Run doc tests (in src/lib.rs)
cargo test

# Run a single doc test by name
cargo test <test_name>

# Lint
cargo clippy

# End-to-end tests (requires fd or fdfind in PATH)
./test/e2e.sh
```

The e2e test runs `sw` against `test/` and diffs the result against `test/expected/`. It restores `test/` via `git restore` on exit.

## Architecture

The project has two source files:

- **`src/main.rs`** — CLI (clap), argument parsing, file discovery loop, and `remove_parts()` which implements the wipe logic. Uses rayon for parallel file processing.
- **`src/lib.rs`** — Three public utilities used by `main.rs`: `run_command`, `get_files_per_extension` (wraps `fd`/`fdfind`), and `check_paths_exist`. Doc tests live here.

**`ast-grep-rules/`** contains per-language `ast-grep` rule files (`.yaml`) for a separate, optional workflow that finds scopes containing specific comments ("Write your code here" / "Write your logic here"). These are not called by the binary itself — they are standalone `sg` rules.

## Runtime dependencies

The binary shells out to `fd` (or `fdfind`) to enumerate files. The `--fd-bin-path` flag overrides the binary name if needed.
