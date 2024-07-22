use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use anyhow::{Context, Result};
use walkdir::WalkDir;

pub fn process_directory(dir: &PathBuf, extensions: &Option<Vec<String>>) -> Result<()> {
    for entry in WalkDir::new(dir) {
        let entry = entry.context("Failed to read directory entry")?;
        if entry.file_type().is_file() {
            let path = entry.path();
            if should_process_file(path, extensions) {
                process_file(path)?;
            }
        }
    }
    Ok(())
}

pub fn process_file(path: &std::path::Path) -> Result<()> {
    let file = File::open(path).context("Failed to open file")?;
    let reader = BufReader::new(file);

    println!(
        "```{}",
        path.extension().unwrap_or_default().to_string_lossy()
    );
    println!("// File: {}", path.display());
    for line in reader.lines() {
        let line = line.context("Failed to read line")?;
        println!("{}", line);
    }
    println!("```\n");

    Ok(())
}

fn should_process_file(path: &std::path::Path, extensions: &Option<Vec<String>>) -> bool {
    if let Some(exts) = extensions {
        path.extension()
            .map(|ext| exts.contains(&ext.to_string_lossy().to_string()))
            .unwrap_or(false)
    } else {
        true
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Write, path::PathBuf};

    use super::*;

    #[test]
    fn test_process_file() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("test.rs");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "fn main() {{}}").unwrap();

        let result = process_file(&file_path);
        assert!(result.is_ok());
        // Note: We can't easily test the stdout without additional setup,
        // but we can verify that the function completes without error.
    }

    #[test]
    fn test_process_directory() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("test.rs");
        let mut file = File::create(file_path).unwrap();
        writeln!(file, "fn main() {{}}").unwrap();

        let result = process_directory(&dir.path().to_path_buf(), &None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_should_process_file() {
        let path = PathBuf::from("test.rs");
        let extensions = Some(vec!["rs".to_string(), "txt".to_string()]);

        assert!(should_process_file(&path, &extensions));
        assert!(!should_process_file(&PathBuf::from("test.js"), &extensions));
        assert!(should_process_file(&path, &None));
    }
}
