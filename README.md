# Minigrep

A command-line tool for searching text patterns in files, inspired by the Unix `grep` utility. Built with Rust as part of the Rust Programming Language book examples.

## Features

- Search for text patterns in files
- Support for case-sensitive and case-insensitive searches
- Configure search behavior through command-line flags or environment variables

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
minigrep PATTERN FILENAME
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

Force case-sensitive search:

```bash
minigrep to poem.txt -cs
```

### Options

- `-ic`: Ignore case when searching
- `-cs`: Force case-sensitive search

### Environment Variables

- `IGNORE_CASE`: Set to any value to enable case-insensitive search by default
