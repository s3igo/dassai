# Dassai

Dassai is a command-line tool written in Rust
that recursively searches for source code files in specified directories,
formats their contents into Markdown code blocks, and outputs them to standard output.

This tool is intended to format local files for use in LLM prompts or RAG inputs.

## What is Dassai?

Dassai（獺祭）refers to the habit of otters arranging the fish they have caught,
and by extension, it means spreading many reference books around when creating poetry or literature.

## Features

- Recursively scans specified directories.
- Can filter by specific file extensions.
- Outputs source code in Markdown format.
- Capable of processing multiple paths at once.
- Ignores files specified in .gitignore by default.
    (For more complex queries on files, consider using external tools like [fd](https://github.com/sharkdp/fd).)

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
find -name '*.rs' | dassai -
fd -e rs | dassai -

# Copy output (Mac)
dassai . | pbcopy
```

## License

[MIT](LICENSE)
