# Dassai

Dassai is a Rust-based command-line tool
that recursively searches for source code files in specified directories,
formats their contents into Markdown code blocks, and outputs them to standard output.

## Features

- Recursively scans specified directories.
- Can filter by specific file extensions.
- Outputs source code in Markdown format.
- Capable of processing multiple paths at once.

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
Usage: dassai [-V] [-e=EXTENSIONS] [PATH]...

Available positional items:
    PATH           The files or directories to process

Available options:
    -V, --version  Print version information
    -e, --extensions=EXTENSIONS  File extensions to include (e.g., "rs,js,py")
    -h, --help     Prints help information
```

## Examples

```sh
# Process all files in the current directory
dassai .

# Process only Rust files in a specific directory
dassai --extensions rs /path/to/rust/project

# Process multiple paths
dassai src tests examples
```

## License

[MIT](LICENSE)
