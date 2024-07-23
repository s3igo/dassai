use std::{fs::File, io::Write, process::Command};

use anyhow::Result;
use assert_cmd::prelude::*;
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
    let dir = tempfile::tempdir()?;
    let file_path = dir.path().join("test.rs");
    let mut file = File::create(&file_path)?;
    writeln!(file, "fn main() {{}}")?;

    let mut cmd = Command::cargo_bin("dassai")?;
    cmd.arg(dir.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "```rs title=\"{}\"",
            file_path.display()
        )))
        .stdout(predicate::str::contains("fn main() {}"));

    Ok(())
}

#[test]
fn test_process_with_extensions() -> Result<()> {
    let dir = tempfile::tempdir()?;
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
