# Minigrep

A command-line tool for searching text patterns in files, inspired by the Unix `grep` utility. Built with Rust as part of the Rust Programming Language book examples.

## Features

- Search for text patterns in files with highlighted matches
- Support for regex patterns automatically in text pattern
- Support for case-sensitive and case-insensitive searches
- Multiple file search capability
- Context display options (before, after, or both)
- Pattern matching with multiple terms using pipe separator (|)
- Search statistics output
- Configurable via command-line flags or environment variables
- Recursive directory search for searching through all files in a directory and its subdirectories

> **Note:** The recursive search feature currently supports searching through a single directory at a time. Multiple directory recursive search is not yet implemented.

## Troubleshooting

**Error: Is a directory**

- If you get this error, you're trying to search a directory without the `--recursive` flag
- Solution: Add `--recursive` or `--r` flag when searching directories

**No matches found**

- Check if you need case-insensitive search with `-ic` flag
- Verify your regex pattern syntax if using regular expressions

**Performance issues with large directories**

- Use specific file patterns instead of searching all files
- Use literal patterns instead of regex when possible
- Consider limiting the search depth for very nested directories

## Exit Codes

- 0: Successful execution (regardless of whether matches were found)
- 1: Error occurred (invalid arguments, file not found, etc.)

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

### Options

**Search Options:**

- `-ic`: Ignore case when searching
- `-cs`: Force case-sensitive search (overrides IGNORE_CASE env variable)

**Context Options:**

- `--before N, --b N`: Show N lines before each match
- `--after N, --a N`: Show N lines after each match
- `--context N, --c N`: Show N lines before and after each match

**Output Options:**

- `--stats, --s`: Display search statistics (pattern, files searched, matches found, etc.)

**Directory Options:**

- `--recursive, --r`: Recursively search through all files in a directory and its subdirectories

**Other Options:**

- `--help, -h`: Display this help message
- `--version, -v`: Display version information

### Environment Variables

- `IGNORE_CASE`: Set to any value to enable case-insensitive search by default

## Testing

Run the tests with:

```bash
cargo test
```

- Unit tests are available in the lib crate file.
- Integration tests are available in the `tests` directory.

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

Recursively search through a directory:

```bash
minigrep pattern directory/ --recursive
```

Recursively search with case-insensitivity:

```bash
bashminigrep pattern directory/ --recursive -ic
```

Combine recursive search with statistics:

```bash
bashminigrep pattern directory/ --recursive --stats
```

Combine recursive search with stats and case-insensitivity:

```bash
bashminigrep pattern directory/ --recursive --stats -ic
```

Find all words starting with 'w'

```bash
minigrep "\bw\w+" poem.txt
```

Match any character between 's' and 'n'

```bash
minigrep "s.n" sunrise.txt
```

Find words ending in 'ing'

```bash
minigrep "\w+ing\b" poem.txt -ic
```

Find all words starting with 't' recursively in a directory

```bash
minigrep "\bt\w+" my_directory/ --recursive
```

## Contributing

Contributions are welcome! Here's how you can help:

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

Please make sure to update tests as appropriate.