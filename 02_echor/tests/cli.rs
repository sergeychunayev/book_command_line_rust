use std::error::Error;

use assert_cmd::Command;
use predicates::prelude::predicate::eq;
use predicates::str::contains;

const NAME: &str = "echor";
type TestResult = Result<(), Box<dyn Error>>;

#[test]
fn dies_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin(NAME)?;
    cmd
        .assert()
        .failure()
        .stderr(contains("USAGE"));
    Ok(())
}

#[test]
fn echo_with_new_line() -> TestResult {
    let mut cmd = Command::cargo_bin(NAME)?;
    cmd
        .arg("test")
        .assert()
        .success()
        .stdout(eq("test\n"));
    Ok(())
}

#[test]
fn echo_with_no_new_line() -> TestResult {
    let mut cmd = Command::cargo_bin(NAME)?;
    cmd
        .arg("-n")
        .arg("test")
        .assert()
        .success()
        .stdout(eq("test"));
    Ok(())
}