use std::path::PathBuf;

use bpaf::Bpaf;

/// Dassai: A command-line tool to format source code files into Markdown code
/// blocks.
#[derive(Debug, Clone, Bpaf)]
#[bpaf(options, version)]
pub struct Args {
    /// File extensions to include (e.g., 'rs,js,py')
    #[bpaf(long, short, argument("EXTENSIONS"))]
    pub extensions: Option<String>,

    /// File names to exclude (e.g., 'README.md,LICENSE')
    #[bpaf(long, short('E'), argument("EXCLUDE"))]
    pub exclude: Option<String>,

    /// The files or directories to process,
    /// if no paths are specified or if '-' is specified, paths will be read
    /// from standard input
    #[bpaf(positional("PATH"), many)]
    pub paths: Vec<PathBuf>,
}

impl Args {
    #[must_use]
    pub fn parse() -> Self {
        args().run()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_invariants() {
        args().check_invariants(false);
    }

    #[test]
    fn test_args_parse() {
        let args = Args {
            extensions: Some("rs,js".to_string()),
            exclude: None,
            paths: vec![PathBuf::from("src"), PathBuf::from("tests")],
        };

        assert_eq!(args.extensions, Some("rs,js".to_string()));
        assert_eq!(args.paths, vec![
            PathBuf::from("src"),
            PathBuf::from("tests")
        ]);
    }
}
