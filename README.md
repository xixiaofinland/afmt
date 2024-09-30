# 🚀 A Blazingly Fast Salesforce Apex Formatter

Afmt is a **Salesforce Apex Code Formatter** written in **Rust**! This tool formats your Apex code for consistency and readability. 🎯

It uses [tree-sitter apex parser](https://github.com/aheber/tree-sitter-sfapex) to traverse AST nodes.

Note: This is a project in its early phase.

## ✨ Features

```bash
$ > afmt --help
A CLI tool for formatting Apex code

Usage: afmt [OPTIONS]

Options:
  -f, --file <FILE>  The relative path to the file to parse [default: tests/files/1.cls]
  -h, --help         Print help
  -V, --version      Print version
```

## 📦 Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) must be installed.

### Steps

1. Clone the repository:
   ```bash
   git clone https://github.com/xixiaofinland/afmt.git
   cd afmt
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

## 🚀 Running the Formatter

### Get help:
```bash
./target/release/afmt --help
```

### Format a file:
```bash
./target/release/afmt --file path/to/your_apex_file.cls
```

### Run with enabled backtrace:
```bash
RUST_BACKTRACE=1 ./target/release/afmt --file path/to/your_apex_file.cls
```
