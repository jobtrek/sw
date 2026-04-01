# SW

**v1.0.1 :** [Read changelog](./CHANGELOG.md) <!-- x-release-please-version -->

> A solution wiper for programming exercises. Strips solution code from files so students can complete them.

## How it works

Mark solution code with paired comment markers. Running `sw` removes the content between each pair and replaces it with a language-appropriate placeholder (or nothing).

**Requires [`fd`](https://github.com/sharkdp/fd)** for directory scanning. Respects `.gitignore` and `.ignore` files.

## Markers

### `--sw-wipe--` — block wipe with placeholder

Content between a pair of `--sw-wipe--` lines is removed and replaced with a placeholder:

- Rust: `todo!();`
- Java: `throw new UnsupportedOperationException("TODO: replace me with your solution !");`
- PHP / JS / TS: nothing (empty)

```rust
// Before
pub fn add(a: i32, b: i32) -> i32 {
    // --sw-wipe--
    a + b
    // --sw-wipe--
}

// After
pub fn add(a: i32, b: i32) -> i32 {
    todo!();
}
```

### `--sw-vanish--` — block wipe without placeholder

Like `--sw-wipe--` but leaves no placeholder, even in Rust and Java. Useful for struct fields or enum variants where `todo!()` would be invalid syntax.

```rust
// Before
pub struct Point {
    // --sw-vanish--
    pub x: f32,
    pub y: f32,
    // --sw-vanish--
}

// After
pub struct Point {
}
```

### `/* --sw-- */` — inline wipe

Removes content between paired `/* --sw-- */` markers on the same line. Multiple pairs per line are supported.

```js
// Before
export function greet(name/* --sw-- */, greeting/* --sw-- */) { ... }

// After
export function greet(name, greeting) { ... }
```

## Usage

```
sw [OPTIONS] [PATHS]...
```

| Argument / Option | Default | Description |
|---|---|---|
| `[PATHS]...` | `.` | Directories or individual files to process |
| `-e, --extensions <EXT>...` | `rs,php,js,ts,java` | File extensions to scan |
| `--dry-run` | — | Validate marker pairing without modifying any files |
| `--fd-bin-path <PATH>` | `fd` | Override the `fd` binary name or path |

**Examples:**

```bash
# Wipe solutions in the current directory
sw

# Wipe only a specific subdirectory
sw exercises/

# Wipe only Rust and TypeScript files
sw -e rs -e ts

# Wipe a single file
sw src/solution.rs

# Check for unpaired markers without touching files
sw --dry-run

# Use fdfind instead of fd (e.g. on Debian/Ubuntu)
sw --fd-bin-path fdfind
```

## Validation

Before modifying any file, `sw` checks that every marker is properly paired. If an odd number of markers is found in any file, **no files are modified** and an error is reported listing each violation.

`--dry-run` runs this validation step only — useful in CI to ensure exercises are correctly annotated.
