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
