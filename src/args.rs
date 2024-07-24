use std::path::PathBuf;

use bpaf::Bpaf;

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
pub struct Args {
    /// Print version information
    #[bpaf(long, short('V'), switch)]
    pub version: bool,

    /// File extensions to include (e.g., 'rs,js,py')
    #[bpaf(long, short, argument("EXTENSIONS"))]
    pub extensions: Option<String>,

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
    fn test_args_parse() {
        let args = Args {
            version: false,
            extensions: Some("rs,js".to_string()),
            paths: vec![PathBuf::from("src"), PathBuf::from("tests")],
        };

        assert!(!args.version);
        assert_eq!(args.extensions, Some("rs,js".to_string()));
        assert_eq!(args.paths, vec![
            PathBuf::from("src"),
            PathBuf::from("tests")
        ]);
    }
}
