use std::path::PathBuf;

use bpaf::Bpaf;

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
pub struct Args {
    /// File extensions to include (e.g., 'rs,js,py')
    #[bpaf(long, short, argument("EXTENSIONS"))]
    pub extensions: Option<String>,

    /// File names to exclude (e.g., 'README.md,LICENSE')
    #[bpaf(long, short('E'), argument("EXCLUDE"))]
    pub exclude: Option<String>,

    /// Print version information
    #[bpaf(long, short('V'), switch)]
    pub version: bool,

    /// The files or directories to process
    #[bpaf(positional("PATH"), many)]
    pub paths: Vec<PathBuf>,
}

impl Args {
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
            version: false,
        };

        assert!(!args.version);
        assert_eq!(args.extensions, Some("rs,js".to_string()));
        assert_eq!(args.paths, vec![
            PathBuf::from("src"),
            PathBuf::from("tests")
        ]);
    }
}