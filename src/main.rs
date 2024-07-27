use std::{
    io::{self, BufRead, IsTerminal},
    path::PathBuf,
};

use anyhow::{bail, Result};
use dassai::{
    args::Args,
    processor::{process_directory, process_file},
};

fn main() -> Result<()> {
    let args = Args::parse();

    if args.version {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    let paths = {
        let stdin = io::stdin();
        if args.paths.is_empty() && !stdin.is_terminal() {
            // Read paths from stdin if piped
            let lines = stdin.lock().lines().collect::<Result<Vec<_>, _>>()?;
            lines.into_iter().map(PathBuf::from).collect()
        } else {
            args.paths
        }
    };

    if paths.is_empty() {
        bail!("No paths specified. Use --help for usage information.");
    }

    for path in paths {
        match path {
            path if path.is_file() => process_file(&path)?,
            path if path.is_dir() => process_directory(&path, &args.extensions, &args.exclude)?,
            _ => eprintln!(
                "Warning: '{}' is neither a file nor a directory, skipping.",
                path.display()
            ),
        }
    }

    Ok(())
}
