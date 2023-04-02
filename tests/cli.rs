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
fn runs_basic() -> TestResult {
    run(&["basic"], "tests/expected/basic.md")
}
