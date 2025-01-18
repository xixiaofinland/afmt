# 🚀 A Fast Configurable Salesforce Apex Formatter

![Release](https://img.shields.io/github/v/release/xixiaofinland/afmt)
![License](https://img.shields.io/github/license/xixiaofinland/afmt)
![Stars](https://img.shields.io/github/stars/xixiaofinland/afmt?style=social)

<div align="center">
  <img src="https://github.com/user-attachments/assets/5cf08fdb-aaa2-4556-83d7-2e9d2a99f86f" alt="afmt_logo" width="300"/>
</div>
<br>

## Table of Contents
- [📘 Introduction](#-introduction)
- [⭐ Features](#-features)
- [✨ vs. Prettier Apex](#-vs-prettier-apex)
- [📥 Installation](#-installation)
- [💻 Usage](#-usage)
- [🔧 Configuration](#-configuration)
- [❓ FAQ](#-faq)

<br>

## 📘 Introduction

`afmt` (Apex formatting tool) is written in Rust 🦀 and leverages the [tree-sitter sfapex parser](https://github.com/aheber/tree-sitter-sfapex).

<br>

## ⭐ Features

- **Performant**
- **Configurable:** via `.afmt.toml`.
- **Standalone:** CLI with no dependencies.
- **Open Source**

<br>

## ✨ vs. Prettier Apex

While both `afmt` and Prettier Apex aim to format Salesforce Apex code, they differ fundamentally in their design philosophies:

- **Prettier Apex:** Maintains an opinionated approach with limited customization to ensure consistency.
- **afmt:** Focuses on extensibility, offering more configuration options to cater to diverse user preferences.

This means `afmt` will progressively introduce more configuration options, addressing user customization needs that Prettier's design intentionally avoids.

### Other Highlights:

| Feature          | afmt                      | Prettier Apex               |
|------------------|---------------------------|-----------------------------|
| **Maturity**     | Brand new                 | Battle tested for years     |
| **Dependencies** | N/A (standalone binary)   | Node.js + Prettier package  |
| **Performance**  | Fast (Rust)               | Relatively slower (Node.js) |
| **Parser**       | sfapex (C / Open Source)  | Jorje (Java / Closed Source)|
| **Open Source**  | Yes                       | Yes                         |
<br>

## 📥 Installation

Visit the [release page](https://github.com/xixiaofinland/afmt/releases/latest) and download the appropriate binary for your operating system (Linux, macOS, or Windows).

<br>

## 💻 Usage

Create a `file.cls` file with valid Apex code.

### Dry Run:

Run `afmt ./file.cls` to preview the formatting result.

```bash
> afmt ./file.cls
Result 0: Ok
global class PluginDescribeResult {
    {
        [SELECT FIELDS(STANDARD) FROM Organization LIMIT 1];
    }
}
```

### Format and Write:

Run `afmt -w ./file.cls` to format the file and overwrite it with the
   formatted code.

```bash
> afmt -w ./file.cls
Formatted content written back to: ./file.cls
Afmt completed successfully.
```
<br>

## 🔧 Configuration:

in `.afmt.toml` config file, two options are supported

```toml
# Maximum line width
max_width = 80

# Indentation size in spaces
indent_size = 4
```

<br>

## ❓ FAQ

- "How do I set up afmt in VSCode?"
[Setup in VSCode](./md/VSCode_Setup.md)

- "Can afmt formats exactly the same as Prettier Apex?"
No.

- "How to load a .afmt.toml file?"
`afmt -c path_to_file`. You can refer to `afmt -h` for more help info.

<br>

## 🤝 Contribution

We greatly value contributions! You can help by [reporting issues](https://github.com/xixiaofinland/afmt/issues) or submitting
PRs.

### PR Contribution Guidelines

Scenarios (e.g., new features, bug fixes) must be covered by tests, and `cargo test` passes.
Refer to `*.in` (before format) and `*.cls` (after format) files in the [test folder](./tests/static).

Also, our [CI pipeline](.github/workflows/pr-ci-merge-main.yml) ensures high-quality contributions.

Rules:

1. Name your branch: `feature/what_ever_short_name`
2. Use [conventional commit](https://www.conventionalcommits.org/en/v1.0.0/#summary) for commit messages. Example: the project [commit history](https://github.com/xixiaofinland/afmt/commits/)
3. Ensure code passes [rustfmt](https://github.com/rust-lang/rustfmt) and [clippy](https://github.com/rust-lang/rust-clippy): `cargo fmt -- --check` and `cargo clippy`
4. Run and pass all unit tests: `cargo test --all-features`
5. Pass battle tests by running `afmt` on [popular Apex repos](./tests/battle_test/repos.txt)
