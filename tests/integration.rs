use std::{fs::File, io::Write, process::Command};

use assert_cmd::prelude::*;
use predicates::prelude::*;
use tempfile::tempdir;

#[test]
fn test_version_flag() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("dassai")?;
    cmd.arg("--version");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
    Ok(())
}

#[test]
fn test_process_directory() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let file_path = dir.path().join("test.rs");
    let mut file = File::create(file_path)?;
    writeln!(file, "fn main() {{}}")?;

    let mut cmd = Command::cargo_bin("dassai")?;
    cmd.arg(dir.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("```rs"))
        .stdout(predicate::str::contains("// File:"))
        .stdout(predicate::str::contains("fn main() {}"));

    Ok(())
}

#[test]
fn test_process_with_extensions() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let rs_file = dir.path().join("test.rs");
    let js_file = dir.path().join("test.js");

    File::create(rs_file)?.write_all(b"fn main() {}")?;
    File::create(js_file)?.write_all(b"console.log('Hello');")?;

    let mut cmd = Command::cargo_bin("dassai")?;
    cmd.arg("--extensions").arg("rs").arg(dir.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("```rs"))
        .stdout(predicate::str::contains("fn main() {}"))
        .stdout(predicate::str::contains("console.log('Hello');").not());

    Ok(())
}
