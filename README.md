# Minigrep

A command-line tool for searching text patterns in files, inspired by the Unix `grep` utility. Built with Rust as part of the Rust Programming Language book examples.

## Features

- Search for text patterns in files with highlighted matches
- Support for case-sensitive and case-insensitive searches
- Multiple file search capability
- Context display options (before, after, or both)
- Pattern matching with multiple terms using pipe separator (|)
- Search statistics output
- Configurable via command-line flags or environment variables

## Installation

Clone the repository and build with Cargo:

```bash
git clone https://github.com/devkilyungi/minigrep.git
cd minigrep
cargo build --release
```

The binary will be available at `target/release/minigrep`.

## Usage

Basic usage:

```bash
minigrep PATTERN FILENAME [SECOND_FILENAME] [OPTIONS]
```

### Examples

Search for "to" in poem.txt (case-sensitive):
```bash
minigrep to poem.txt
```

Search for "to" in poem.txt (case-insensitive):
```bash
minigrep to poem.txt -ic
```

Search in multiple files:
```bash
minigrep to poem.txt sunrise.txt
```

Show context around matches (1 line before and after):
```bash
minigrep to poem.txt --context 1
```

Show only lines after matches:
```bash
minigrep to poem.txt --after 2
```

Show only lines before matches:
```bash
minigrep to poem.txt --before 2
```

Search for multiple patterns:
```bash
minigrep "sun|moon" sunrise.txt
```

Show search statistics:
```bash
minigrep to poem.txt --stats
```

Combine multiple options:
```bash
minigrep to poem.txt sunrise.txt -ic --context 2 --stats
```

### Options

- `-ic`: Ignore case when searching
- `-cs`: Force case-sensitive search
- `--before N`: Show N lines before each match
- `--after N`: Show N lines after each match
- `--context N`: Show N lines before and after each match
- `--stats`: Display search statistics (pattern, files searched, matches found, etc.)

### Environment Variables

- `IGNORE_CASE`: Set to any value to enable case-insensitive search by default

## Testing

Run the tests with:

```bash
cargo test
```

Integration tests are available in the `tests` directory.