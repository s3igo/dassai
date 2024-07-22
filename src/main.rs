use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use anyhow::{Context, Result};
use bpaf::Bpaf;
use walkdir::WalkDir;

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
struct Args {
    /// Print version information
    #[bpaf(long("version"), short('V'), switch)]
    version: bool,

    /// File extensions to include (e.g., "rs,js,py")
    #[bpaf(long, short, argument("EXTENSIONS"))]
    extensions: Option<String>,

    /// The files or directories to process
    #[bpaf(positional("PATH"), many)]
    paths: Vec<PathBuf>,
}

fn main() -> Result<()> {
    let args = args().run();

    if args.version {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    let extensions: Option<Vec<String>> = args
        .extensions
        .map(|ext| ext.split(',').map(String::from).collect());

    if args.paths.is_empty() {
        anyhow::bail!("No paths specified. Use --help for usage information.");
    }

    for path in args.paths {
        if path.is_file() {
            process_file(&path)?;
        } else if path.is_dir() {
            process_directory(&path, &extensions)?;
        } else {
            eprintln!(
                "Warning: '{}' is neither a file nor a directory, skipping.",
                path.display()
            );
        }
    }

    Ok(())
}

fn process_directory(dir: &PathBuf, extensions: &Option<Vec<String>>) -> Result<()> {
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

fn should_process_file(path: &std::path::Path, extensions: &Option<Vec<String>>) -> bool {
    if let Some(exts) = extensions {
        path.extension()
            .map(|ext| exts.contains(&ext.to_string_lossy().to_string()))
            .unwrap_or(false)
    } else {
        true
    }
}

fn process_file(path: &std::path::Path) -> Result<()> {
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
