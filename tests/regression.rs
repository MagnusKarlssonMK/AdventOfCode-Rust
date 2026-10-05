//! Regression test suite
//!
//! Data driven end-to-end regression tests. For every day that has a
//! stored expected-answer file, this runs the real solver via
//! [`advent_of_code_rust::dispatch`] and asserts the output matches.
//!
//! ## Layout
//!
//! ```text
//! AdventOfCode-Input/<year>/day<DD>.txt      # puzzle input
//! AdventOfCode-Input/<year>/day<DD>.answer   # expected output
//! ```
//!
//! ## `.answer` format
//!
//! Part 1 and part 2 are separated by a line containing exactly `---`.
//! Each part may span multiple lines (for grid / ASCII-art answers). A
//! trailing newline on each part is ignored.
//!
//! ```text
//! 111326
//! ---
//! 1019
//! ```
//!
//! ## Notes
//!
//! * Days without a corresponding `.answer` file are skipped. An `.answer`
//!   file whose matching `.txt` file is missing is also skipped.
//! * Some days are slow; run in release mode:
//!   `cargo test --release --test regression`.
use std::fs;
use std::path::Path;

use advent_of_code_rust::dispatch;

fn parse_answer(content: &str) -> (String, String) {
    match content.split_once("\n---\n") {
        Some((p1, p2)) => (
            p1.trim_end_matches('\n').to_string(),
            p2.trim_end_matches('\n').to_string(),
        ),
        None => (content.trim_end_matches('\n').to_string(), String::new()),
    }
}

#[test]
fn stored_answers_match() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("AdventOfCode-Input");

    let mut checked = 0usize;
    let mut skipped_missing_input = Vec::new();
    let mut failures = Vec::new();

    let Ok(years) = fs::read_dir(&root) else {
        eprintln!(
            "regression: no input directory at {} - nothing to check",
            root.display()
        );
        return;
    };

    // Collect and sort year directories for deterministic output.
    let mut year_dirs: Vec<_> = years
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.is_dir())
        .collect();
    year_dirs.sort();

    for year_dir in year_dirs {
        let Some(year) = year_dir.file_name().and_then(|s| s.to_str()) else {
            continue;
        };

        let mut answer_files: Vec<_> = fs::read_dir(&year_dir)
            .into_iter()
            .flatten()
            .flatten()
            .map(|e| e.path())
            .filter(|p| p.extension().and_then(|s| s.to_str()) == Some("answer"))
            .collect();
        answer_files.sort();

        for answer_path in answer_files {
            let Some(day_key) = answer_path.file_stem().and_then(|s| s.to_str()) else {
                continue;
            };
            let year_key = format!("year{year}");

            let input_path = year_dir.join(format!("{day_key}.txt"));
            let Ok(input_raw) = fs::read_to_string(&input_path) else {
                skipped_missing_input.push(format!("{year} {day_key}"));
                continue;
            };
            let input = input_raw.trim_end_matches('\n').to_string();

            let expected_raw =
                fs::read_to_string(&answer_path).expect("answer file readable after listing");
            let (exp1, exp2) = parse_answer(&expected_raw);

            match dispatch(&year_key, day_key, &input) {
                Ok((p1, p2)) => {
                    checked += 1;
                    if p1 != exp1 || p2 != exp2 {
                        failures.push(format!(
                            "{year} {day_key}:\n  part 1: got {p1:?}, expected {exp1:?}\n  part 2: got {p2:?}, expected {exp2:?}"
                        ));
                    }
                }
                Err(e) => {
                    if !e.to_string().contains("not implemented") {
                        failures.push(format!("{year} {day_key}: solver returned error {e}"));
                    }
                }
            }
        }
    }

    if !skipped_missing_input.is_empty() {
        eprintln!(
            "regression: skipped {} answer(s) with no local input file: {}",
            skipped_missing_input.len(),
            skipped_missing_input.join(", ")
        );
    }
    eprintln!("regression: verified {checked} day(s)");

    assert!(
        failures.is_empty(),
        "{} regression failure(s) out of {} checked:\n{}",
        failures.len(),
        checked,
        failures.join("\n")
    );
}
