use std::{
    fs::File,
    io::{BufRead, BufReader, Read as _, Seek as _, SeekFrom},
    path::{Path, PathBuf},
};

use anyhow::Context as _;
use ignore::Walk;

fn should_process_file(
    path: &Path,
    extensions: &Option<Vec<&str>>,
    exclude: &Option<Vec<&str>>,
) -> bool {
    // Get the file extension as a string, if it exists
    let ext = path.extension().and_then(|e| e.to_str());

    // Get the file name as a string, if it exists
    let file_name = path.file_name().and_then(|f| f.to_str());

    // Check if the file extension is in the allowed extensions list
    // If extensions list is None, all extensions are allowed
    let is_extension_allowed = extensions
        .as_ref()
        .map_or(true, |exts| ext.map_or(false, |e| exts.contains(&e)));

    // Check if the file name is in the exclusion list
    // If exclusion list is None, no files are excluded
    let is_excluded = exclude.as_ref().map_or(false, |excludes| {
        file_name.map_or(false, |f| excludes.contains(&f))
    });

    // Process the file if its extension is allowed and it's not in the exclusion
    // list
    is_extension_allowed && !is_excluded
}

/// # Errors
/// Returns an error if the file cannot be opened or read.
pub fn process_directory(
    dir: &PathBuf,
    extensions: &Option<String>,
    exclude: &Option<String>,
) -> anyhow::Result<()> {
    let extensions: Option<Vec<_>> = extensions.as_ref().map(|ext| ext.split(',').collect());
    let exclude: Option<Vec<_>> = exclude.as_ref().map(|ext| ext.split(',').collect());

    for entry in Walk::new(dir) {
        let entry = entry.context("Failed to read directory entry")?;
        if entry.file_type().map_or(false, |ft| ft.is_file()) {
            let path = entry.path();
            if should_process_file(path, &extensions, &exclude) {
                process_file(path)?;
            }
        }
    }

    Ok(())
}

/// # Errors
/// Returns an error if the file cannot be opened or read.
pub fn process_file(path: &Path) -> anyhow::Result<()> {
    let file = File::open(path).context("Failed to open file")?;
    let mut reader = BufReader::new(file);

    // Check if the file is binary
    let mut buffer = [0; 512];
    let bytes_read = reader.read(&mut buffer)?;
    if buffer[..bytes_read].contains(&0) {
        eprintln!(
            "Warning: '{}' is binary and cannot be processed, skipping.",
            path.display()
        );
        return Ok(());
    }

    // Reset the reader to the beginning of the file
    reader.seek(SeekFrom::Start(0))?;

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
    use assert_fs::{prelude::*, NamedTempFile, TempDir};

    use super::*;

    #[test]
    fn test_should_process_file() {
        let path = PathBuf::from("test.rs");
        let extensions = Some(vec!["rs", "txt"]);
        let exclude = Some(vec!["test.rs"]);

        assert!(should_process_file(&path, &extensions, &None));
        assert!(!should_process_file(
            &PathBuf::from("test.js"),
            &extensions,
            &None
        ));
        assert!(should_process_file(&path, &None, &None));
        assert!(!should_process_file(&path, &None, &exclude));
    }

    #[test]
    fn test_process_file() -> anyhow::Result<()> {
        let file = NamedTempFile::new("test.rs")?;
        file.touch()?;

        process_file(file.path())?;

        Ok(())
    }

    #[test]
    fn test_process_directory() -> anyhow::Result<()> {
        let dir = TempDir::new()?;
        let file = dir.child("test.rs");
        file.write_str("fn main() {}")?;

        process_directory(&dir.to_path_buf(), &None, &None)?;

        Ok(())
    }
}
