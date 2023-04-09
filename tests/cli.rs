use assert_cmd::Command;
use predicates::prelude::*;
use std::error::Error;
use std::fs;
type TestResult = Result<(), Box<dyn Error>>;

const NAME: &str = "cheatsheet";

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin(NAME)?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn help() -> TestResult {
    for flag in &["-h", "--help"] {
        Command::cargo_bin(NAME)?
            .arg(flag)
            .assert()
            .stdout(predicate::str::contains("Usage"));
    }
    Ok(())
}

#[test]
fn missing_sheet() -> TestResult {
    for flag in &["missing"] {
        Command::cargo_bin(NAME)?
            .arg(flag)
            .assert()
            .stderr(predicate::str::contains(
                "./tests/inputs/missing.md: No such file or directory (os error 2)\n",
            ));
    }
    Ok(())
}

#[test]
fn test_full_document() -> TestResult {
    run(&["basic"], "tests/expected/full-output.txt")
}

#[test]
fn test_query_by_first_anchor() -> TestResult {
    run(
        &["basic", "first-block"],
        "tests/expected/first-block-output.txt",
    )
}

#[test]
fn test_sheet_list() -> TestResult {
    run(&["-l"], "tests/expected/list-basic.txt")
}

#[test]
fn test_basic_sheet_list_anchors() -> TestResult {
    run(&["basic", "-l"], "tests/expected/list-basic-anchors.txt")
}
