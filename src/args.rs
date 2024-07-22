use std::path::PathBuf;

use bpaf::Bpaf;

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
pub struct Args {
    /// Print version information
    #[bpaf(long("version"), short('V'), switch)]
    pub version: bool,

    /// File extensions to include (e.g., "rs,js,py")
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

    pub fn extensions_vec(&self) -> Option<Vec<String>> {
        self.extensions
            .as_ref()
            .map(|ext| ext.split(',').map(String::from).collect())
    }
}
