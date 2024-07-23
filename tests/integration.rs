use std::process::Command;

use anyhow::Result;
use assert_cmd::prelude::*;
use assert_fs::{prelude::*, TempDir};
use predicates::prelude::*;

#[test]
fn test_version_flag() -> Result<()> {
    let mut cmd = Command::cargo_bin("dassai")?;
    cmd.arg("--version");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));

    Ok(())
}

#[test]
fn test_process_directory() -> Result<()> {
    let dir = TempDir::new()?;
    let file = dir.child("test.rs");
    file.write_str("fn main() {}")?;

    let mut cmd = Command::cargo_bin("dassai")?;
    cmd.arg(dir.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "```rs title=\"{}\"",
            file.path().display()
        )))
        .stdout(predicate::str::contains("fn main() {}"));

    Ok(())
}

#[test]
fn test_process_with_extensions() -> Result<()> {
    let dir = TempDir::new()?;
    let rs_file = dir.child("test.rs");
    let js_file = dir.child("test.js");

    rs_file.write_str("fn main() {}")?;
    js_file.write_str("console.log('Hello');")?;

    let mut cmd = Command::cargo_bin("dassai")?;
    cmd.arg("--extensions").arg("rs").arg(dir.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("```rs"))
        .stdout(predicate::str::contains("fn main() {}"))
        .stdout(predicate::str::contains("console.log('Hello');").not());

    Ok(())
}
