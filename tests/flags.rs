use std::process::Command;

use anyhow::Result;
use assert_cmd::prelude::*;
use assert_fs::{prelude::*, TempDir};
use predicates::prelude::*;

#[test]
fn test_extensions() -> Result<()> {
    let dir = TempDir::new()?;
    let rs_file = dir.child("test.rs");
    let js_file = dir.child("test.js");

    rs_file.write_str("fn main() {}")?;
    js_file.write_str("console.log('Hello');")?;

    // only .rs files
    let mut cmd = Command::cargo_bin("dassai")?;
    cmd.arg("--extensions").arg("rs").arg(dir.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("```rs"))
        .stdout(predicate::str::contains("fn main() {}"))
        .stdout(predicate::str::contains("console.log('Hello');").not());

    Ok(())
}

#[test]
fn test_exclude() -> Result<()> {
    let dir = TempDir::new()?;
    let rs_file = dir.child("test.rs");
    let md_file = dir.child("test.md");

    rs_file.write_str("fn main() {}")?;
    md_file.write_str("# Hello")?;

    // exclude test.md file
    let mut cmd = Command::cargo_bin("dassai")?;
    cmd.arg("--exclude").arg("test.md").arg(dir.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("```rs"))
        .stdout(predicate::str::contains("fn main() {}"))
        .stdout(predicate::str::contains("# Hello").not());

    Ok(())
}

#[test]
fn test_help() -> Result<()> {
    let mut cmd = Command::cargo_bin("dassai")?;
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Usage"));

    Ok(())
}
