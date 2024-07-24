use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

use anyhow::{Context as _, Result};
use walkdir::WalkDir;

fn should_process_file(path: &Path, extensions: &Option<Vec<&str>>) -> bool {
    if let Some(exts) = extensions {
        path.extension()
            .map(|ext| exts.contains(&ext.to_string_lossy().to_string().as_str()))
            .unwrap_or(false)
    } else {
        // If no extensions are provided, process all files.
        true
    }
}

pub fn process_directory(dir: &PathBuf, extensions: &Option<String>) -> Result<()> {
    let extensions: Option<Vec<_>> = extensions.as_ref().map(|ext| ext.split(',').collect());

    for entry in WalkDir::new(dir) {
        let entry = entry.context("Failed to read directory entry")?;
        if entry.file_type().is_file() {
            let path = entry.path();
            if should_process_file(path, &extensions) {
                process_file(path)?;
            }
        }
    }

    Ok(())
}

pub fn process_file(path: &Path) -> Result<()> {
    let file = File::open(path).context("Failed to open file")?;
    let reader = BufReader::new(file);

    println!(
        "```{} title=\"{}\"",
        path.extension().unwrap_or_default().to_string_lossy(),
        path.display()
    );
    for line in reader.lines() {
        let line = line.context("Failed to read line")?;
        println!("{line}");
    }
    println!("```\n\n");

    Ok(())
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use assert_fs::{prelude::*, NamedTempFile, TempDir};

    use super::*;

    #[test]
    fn test_should_process_file() {
        let path = PathBuf::from("test.rs");
        let extensions = Some(vec!["rs", "txt"]);

        assert!(should_process_file(&path, &extensions));
        assert!(!should_process_file(&PathBuf::from("test.js"), &extensions));
        assert!(should_process_file(&path, &None));
    }

    #[test]
    fn test_process_file() -> Result<()> {
        let file = NamedTempFile::new("test.rs")?;
        file.touch()?;

        process_file(file.path())?;

        Ok(())
    }

    #[test]
    fn test_process_directory() -> Result<()> {
        let dir = TempDir::new()?;
        let file = dir.child("test.rs");
        file.write_str("fn main() {}")?;

        process_directory(&dir.to_path_buf(), &None)?;

        Ok(())
    }
}
