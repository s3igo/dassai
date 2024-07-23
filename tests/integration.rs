use std::process::Command;

use anyhow::Result;
use assert_cmd::prelude::*;
use assert_fs::{prelude::*, NamedTempFile, TempDir};
use predicates::prelude::*;

#[test]
fn test_specify_directory() -> Result<()> {
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
fn test_specify_file() -> Result<()> {
    let file = NamedTempFile::new("test.rs")?;
    file.write_str("fn main() {}")?;

    let mut cmd = Command::cargo_bin("dassai")?;
    cmd.arg(file.path());
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
fn test_multiple_paths() -> Result<()> {
    let dir = TempDir::new()?;
    let rs_file = dir.child("test.rs");
    rs_file.write_str("fn main() {}")?;

    let js_file = NamedTempFile::new("test.js")?;
    js_file.write_str("console.log('Hello');")?;

    let mut cmd = Command::cargo_bin("dassai")?;
    cmd.arg(dir.path()).arg(js_file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("```rs"))
        .stdout(predicate::str::contains("fn main() {}"))
        .stdout(predicate::str::contains("```js"))
        .stdout(predicate::str::contains("console.log('Hello');"));

    Ok(())
}
