# Dassai

Dassai is a command-line tool written in Rust that helps prepare source code for AI interactions.
It recursively searches through directories, converts source files into Markdown code blocks,
and outputs them to standard output - making it perfect for use with LLM prompts or RAG systems.

## What is Dassai?

The name "Dassai"（獺祭）comes from an interesting Japanese concept.
It originally described how otters would neatly arrange their caught fish on riverbanks.
This image evolved to metaphorically represent the scholarly practice of surrounding oneself with reference materials while writing or studying - much like how this tool helps organize code for analysis.

## Features

- Recursively explores directories to find source files
- Filters files by extension to focus on specific languages
- Converts source code into clean Markdown format
- Handles multiple input paths simultaneously
- Automatically respects .gitignore rules
  (Need more advanced file filtering? Try using [fd](https://github.com/sharkdp/fd) in combination with Dassai)

## Installation

You can install Dassai using Cargo:

```sh
cargo install --git https://github.com/s3igo/dassai
```

### Using Nix

If you're using Nix, you can run Dassai directly without installation:

```sh
# Process all files in the current directory
nix run github:s3igo/dassai -- .
```

## Usage

```txt
Dassai: A command-line tool to format source code files into Markdown code blocks.

Usage: dassai [-e=EXTENSIONS] [-E=EXCLUDE] [-V] [PATH]...

Available positional items:
    PATH                   The files or directories to process, if no paths are specified or if '-'
                           is specified, paths will be read from standard input

Available options:
    -e, --extensions=EXTENSIONS  File extensions to include (e.g., 'rs,js,py')
    -E, --exclude=EXCLUDE  File names to exclude (e.g., 'README.md,LICENSE')
    -V, --version          Prints version information
    -h, --help             Prints help information
    -V, --version          Prints version information
```

## Examples

```sh
# Process all files in the current directory
dassai .

# Process only Rust files in a specific directory
dassai --extensions rs /path/to/rust/project

# Process multiple paths
dassai src tests examples

# Read paths from standard input
find -name '*.rs' | dassai
fd -e rs | dassai

# Copy output (Mac)
dassai . | pbcopy
```

## License

[MIT](LICENSE)
