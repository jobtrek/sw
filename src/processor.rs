/// Removes lines between pairs of markers, replacing the block with a language placeholder.
pub const WIPE_MARKER: &str = "--sw-wipe--";

/// Removes lines between pairs of markers with no placeholder, even on languages that normally
/// use one.
pub const VANISH_MARKER: &str = "--sw-vanish--";

/// Removes content between pairs of these markers on the same line.
pub const INLINE_MARKER: &str = "/* --sw-- */";

// ── Public API ────────────────────────────────────────────────────────────────

/// Return one error message per block marker type that has an odd count in `content`.
///
/// An odd count means markers are unpaired, which would silently truncate content.
pub fn validate_markers(path: &str, content: &str) -> Vec<String> {
    [WIPE_MARKER, VANISH_MARKER]
        .iter()
        .filter_map(|marker| {
            let count = content.lines().filter(|line| line.contains(marker)).count();
            if count % 2 != 0 {
                Some(format!(
                    "{path}: odd number of '{marker}' markers ({count})"
                ))
            } else {
                None
            }
        })
        .collect()
}

/// Apply all three transformations to a file's content.
pub fn transform(content: &str, wipe_placeholder: &str) -> String {
    let content = if content.contains(WIPE_MARKER) {
        remove_parts(content, WIPE_MARKER, wipe_placeholder)
    } else {
        content.to_string()
    };
    let content = if content.contains(VANISH_MARKER) {
        remove_parts(&content, VANISH_MARKER, "")
    } else {
        content
    };
    if content.contains(INLINE_MARKER) {
        remove_inline(&content)
    } else {
        content
    }
}

// ── Private helpers ───────────────────────────────────────────────────────────

/// Detect whether `content` uses CRLF or LF line endings.
fn detect_line_ending(content: &str) -> &str {
    if content.contains("\r\n") {
        "\r\n"
    } else {
        "\n"
    }
}

/// Rejoin `lines` using the same line ending as `original`, restoring a trailing
/// newline if the original had one.
fn reassemble(lines: Vec<String>, original: &str) -> String {
    let le = detect_line_ending(original);
    let mut out = lines.join(le);
    if original.ends_with('\n') && !out.ends_with('\n') {
        out.push_str(le);
    }
    out
}

/// Remove lines between pairs of `marker` lines.
///
/// When `replace_with` is non-empty, it is inserted (with matching indentation)
/// in place of the closing marker line.
fn remove_parts(file_content: &str, marker: &str, replace_with: &str) -> String {
    let mut removing = false;
    let lines = file_content
        .lines()
        .filter_map(|line| {
            if line.contains(marker) {
                removing = !removing;
                if !removing && !replace_with.is_empty() {
                    let indent: String = line
                        .chars()
                        .take_while(|&c| c == ' ' || c == '\t')
                        .collect();
                    return Some(format!("{indent}{replace_with}"));
                }
                return None;
            }
            if removing {
                None
            } else {
                Some(line.to_string())
            }
        })
        .collect();
    reassemble(lines, file_content)
}

/// Remove character ranges between pairs of [`INLINE_MARKER`] on the same line.
///
/// Multiple pairs on the same line are all processed. An unpaired marker leaves
/// the remainder of the line untouched.
fn remove_inline(file_content: &str) -> String {
    let lines = file_content
        .lines()
        .map(|line| {
            let mut s = line.to_string();
            loop {
                let Some(start) = s.find(INLINE_MARKER) else {
                    break;
                };
                let after = start + INLINE_MARKER.len();
                let Some(rel) = s[after..].find(INLINE_MARKER) else {
                    break; // unpaired marker — leave remainder as-is
                };
                let end = after + rel + INLINE_MARKER.len();
                s.replace_range(start..end, "");
            }
            s
        })
        .collect();
    reassemble(lines, file_content)
}
