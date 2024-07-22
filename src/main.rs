use anyhow::Result;
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
