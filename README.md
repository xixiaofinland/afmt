# 🚀 A Blazingly Fast Salesforce Apex Formatter

Afmt is a **Salesforce Apex Code Formatter** written in **Rust**! This tool formats your Apex code for consistency and readability. 🎯

It uses [tree-sitter apex parser](https://github.com/aheber/tree-sitter-sfapex) to traverse AST nodes.

Note. this is a project in its early phase.

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

