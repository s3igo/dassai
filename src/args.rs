use std::path::PathBuf;

use bpaf::Bpaf;

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
pub struct Args {
    /// Print version information
    #[bpaf(long("version"), short('V'), switch)]
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
    use std::path::PathBuf;

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

    #[test]
    fn test_args_extensions_vec() {
        let args = Args {
            version: false,
            extensions: Some("rs,js,py".to_string()),
            paths: vec![],
        };

        let extensions_vec = args
            .extensions
            .map(|ext| ext.split(',').map(String::from).collect());

        assert_eq!(
            extensions_vec,
            Some(vec!["rs".to_string(), "js".to_string(), "py".to_string()])
        );
    }
}
