

# 🚀 A Fast Configurable Salesforce Apex Formatter

![Release](https://img.shields.io/github/v/release/xixiaofinland/afmt)
![License](https://img.shields.io/github/license/xixiaofinland/afmt)
![Stars](https://img.shields.io/github/stars/xixiaofinland/afmt?style=social)

<div align="center">
  <img src="md/afmt-logo.png" alt="afmt_logo" width="300"/>
</div>
<br>

## Table of Contents
- [📘 Introduction](#-introduction)
- [🌐 Playground](#-playground)
- [⭐ Features](#-features)
- [✨ vs. Prettier Apex](#-vs-prettier-apex)
- [📥 Installation](#-installation)
- [💻 Usage](#-usage)
- [🔧 Configuration](#-configuration)
- [❓ FAQ](#-faq)
- [🤝 Contribution](#-contribution)

<br>

## 📘 Introduction

`afmt` (Apex formatting tool) is written in Rust 🦀 and leverages the [tree-sitter sfapex parser](https://github.com/aheber/tree-sitter-sfapex).

> [!NOTE]
> A VS Code extension is now available: [afmt Formatter](https://marketplace.visualstudio.com/items?itemName=jprichter.afmt-code-ext).
> It formats the in-memory buffer via `afmt -`, and expects the `afmt` binary to be installed separately.

<br>

## 🌐 Playground

Try the browser version [playground](https://xixiaofinland.github.io/afmt-web-service/), and its source code [here](https://github.com/xixiaofinland/afmt-web-service).

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

### 1. Script Install

#### For Linux/MacOS

```bash
curl -sL https://raw.githubusercontent.com/xixiaofinland/afmt/main/scripts/install-afmt.sh | bash
```

#### For Windows (PowerShell)

```ps1
iwr -useb https://raw.githubusercontent.com/xixiaofinland/afmt/main/scripts/install-afmt.ps1 | iex
```

> [!NOTE]
> If you see an error like "This script contains malicious content and has been
> blocked by your antivirus software", it means Microsoft Defender flagged it
> for downloading and executing content from the internet. To proceed, either
> lower Defender’s protection or break the script into smaller steps:

```ps1
# Step 1: Review the script manually
Invoke-WebRequest -Uri https://raw.githubusercontent.com/xixiaofinland/afmt/main/scripts/install-afmt.ps1 -OutFile install-afmt.ps1
notepad install-afmt.ps1  # Inspect the content

# Step 2: Run after trust
powershell -ExecutionPolicy Bypass -File install-afmt.ps1
```

<br>

### 2. Cargo Install

`afmt` is published on crates.io [here](https://crates.io/crates/sf-afmt).
Run cmd below if you have the `Cargo` tool.

```bash
cargo install sf-afmt
```

<br>

### 3. Manual Download

Visit the [release page](https://github.com/xixiaofinland/afmt/releases/latest) and download the appropriate binary for your operating system (Linux, macOS, or Windows).

<br>

## 💻 Usage

Create a `file.cls` file with valid Apex code.

### Common commands

```bash
# Format one file to stdout
afmt AccountService.cls

# Format an in-memory Apex buffer from stdin
cat AccountService.cls | afmt -

# Recursively format a directory in place
afmt --write force-app

# Check multiple files and directories without writing
afmt --check force-app packages/shared.cls

# Use an explicitly supplied config and print per-file timing
afmt --config .afmt.toml --time --write .
```

Directory inputs recursively select `.cls`, `.trigger`, `.apex`, and `.apexc`
files in deterministic path order. Overlapping paths are processed once.
Explicit files bypass the include patterns but still honor exclusions, and an
exclude match always wins. The default exclusions are `.git`, `.sfdx`, and
`node_modules`; a supplied `[files].exclude` array replaces those defaults, so
repeat any defaults you still want.

### Dry Run:

Run `afmt ./file.cls` to preview the formatting result.

```bash
> afmt ./file.cls
global class PluginDescribeResult {
    {
        [SELECT FIELDS(STANDARD) FROM Organization LIMIT 1];
    }
}
```

### Format and Write:

Run `afmt -w ./file.cls` to format the file and overwrite it with the
formatted code. Unchanged files are not rewritten.

```bash
> afmt -w ./file.cls
```
<br>

For multiple files, `--write` is best effort: a read, parse, format, or write
failure is reported with its path, while other valid files continue. A final
stderr summary reports selected, changed, written, unchanged, failed, and
elapsed counts. `--check` never writes, lists every file that would change,
and exits nonzero for any changed file, processing failure, or empty selection.
`--time` adds one stderr timing line per selected file and a total; it never
changes formatted stdout.

Use `afmt -` when an editor or another tool already holds Apex source in memory.
It reads the complete buffer from stdin and writes only the formatted source to
stdout. Supply `-c/--config` when the buffer should use a specific config file;
stdin mode does not discover `.afmt.toml` implicitly and cannot be combined with
`--write` or `--check`.

Dry runs selecting one file print only formatted Apex source. Dry runs
selecting multiple files print each source block with a deterministic
`==> path <==` delimiter. Exit code `0` means formatting or checking
succeeded; exit code `1` means an application error, changed file in check
mode, or partial write failure.

## 🔧 Configuration:

`-c` parameter can read configuration settings from a toml file.

Example: `afmt -c .afmt.toml ./file.cls`

In an explicitly supplied `.afmt.toml` config file, formatter settings, optional
style controls, and optional file-selection settings are supported. A config
file is not loaded implicitly.

```toml
# Maximum line width
max_width = 80

# Indentation size in spaces
indent_size = 2

# Optional style controls (defaults preserve afmt's existing output)
# brace_style = "k_and_r"            # or "allman"
# wrap_single_statements = false      # add braces to bare clause bodies
# indent_style = "space"              # or "tab"
# javadoc_star_column = "offset"      # or "flush"
# normalize_annotation_casing = false  # canonicalize known annotation names

# Optional replacement selection arrays. Uncomment to customize them.
# [files]
# include = ["**/*.cls", "**/*.trigger", "**/*.apex", "**/*.apexc"]
# exclude = ["**/.git/**", "**/.sfdx/**", "**/node_modules/**"]
```

`javadoc_star_column = "flush"` aligns JavaDoc continuation stars with the
comment's indentation column (`* content`); the default `"offset"` preserves
afmt's existing style (` * content`). In either mode, afmt normalizes the
separator after the star to one space.

When `indent_style = "tab"`, `indent_size` controls the number of columns per
indent level, and line wrapping accounts for that configured width when applying
`max_width`.

When `normalize_annotation_casing = true`, known Apex annotation names are
written with Salesforce's canonical casing, such as `@IsTest`, `@TestSetup`,
and `@AuraEnabled`. Unknown annotation names remain verbatim. This option only
normalizes the annotation name; annotation argument keys and values are left
unchanged.

Allman formatting places property and accessor body braces on their own lines.
Compact auto-properties without accessor bodies keep their `{ get; set; }`
contents on one line.

Each supplied include or exclude array replaces its corresponding default.
Patterns use portable `/` separators and are matched relative to the config
directory when possible. Exclusions apply to explicit files and directories;
excluded directories are not traversed.

See the [formatter configuration guide](docs/configuration.md) for the
complete option behavior and defaults. For the complete project-wide behavior
and contributor validation workflow, see
[the project-wide formatting guide](docs/project-wide-formatting.md) and
[the validation and benchmark guide](docs/validation-and-benchmarks.md).

### Ignoring a node

Place `// afmt:ignore` as the last standalone pre-comment for a node (such as a
statement or declaration) to preserve that node's original source bytes,
including internal whitespace and blank lines:

```apex
// afmt:ignore
Integer[] matrix = new Integer[]{ 1, 0, 0,
                                  0, 1, 0,
                                  0, 0, 1 };
```

`//afmt:ignore` and `/* afmt:ignore */` work too, and the marker may carry a
free-text reason, as in `// afmt:ignore column alignment is meaningful here`. A
marker between annotations and a declaration applies to the complete
declaration, whether or not that declaration has an access modifier.

The marker stays in the output, so an ignored node stays ignored on every later
run and `afmt --check` passes on a file that `afmt --write` produced. Multiline
preserved source keeps its authored interior indentation and forces surrounding
layout to break; afmt does not re-anchor its interior lines.

If a recognized marker has no eligible following node, afmt preserves it and
writes a warning to stderr naming the marker's position:

```
Warning: force-app/main/default/classes/Example.cls:12:5: afmt:ignore could not be applied; directive was preserved
```

See the [comment placement and idempotency
guide](docs/comment-placement-and-idempotency.md) for the exact behavior.

<br>

## ❓ FAQ

- "TLTR, what features afmt has?" Run `afmt -h`.
- "How do I set up afmt in VS Code?"
Install the [afmt Formatter extension](https://marketplace.visualstudio.com/items?itemName=jprichter.afmt-code-ext),
or wire `afmt` up manually as a task: [Setup in VSCode](./md/VSCode_Setup.md)

- "How do I set up afmt in Neovim?"
See the [Neovim setup guide](./md/Neovim_Setup.md).

- "Can afmt formats exactly the same as Prettier Apex?"
No.

<br>

## 🤝 Contribution

We greatly value contributions! You can help by reporting [issues](https://github.com/xixiaofinland/afmt/issues) or submitting
PRs.

### PR Contribution Guidelines

Scenarios (e.g., new features, bug fixes) must be covered by tests, and `cargo test` passes.
Refer to `*.in` (before format) and `*.cls` (after format) files in the [test folder](./tests/static).

The active static, prettier80, and comments fixtures each have a separate
two-pass idempotency test. For local bulk-performance measurements, run
`tests/battle_test/benchmark_bulk.sh /path/to/a/local/corpus`; it builds once,
keeps the corpus unchanged, and compares sequential process startup with one
bulk invocation. The battle test uses the bulk command first and retains a
per-file diagnostic fallback for tolerated managed-package templates.

Also, our CI [pipeline](.github/workflows/pr-ci-merge-main.yml) ensures high-quality contributions.

CI Rules:

1. Use [conventional commit](https://www.conventionalcommits.org/en/v1.0.0/#summary) for commit messages. Example: the project [commit history](https://github.com/xixiaofinland/afmt/commits/)
2. Ensure code passes [rustfmt](https://github.com/rust-lang/rustfmt) and [clippy](https://github.com/rust-lang/rust-clippy): `cargo fmt -- --check` and `cargo clippy`
3. Run and pass all unit tests: `cargo test --all-features`
4. Pass battle tests by running `afmt` on a list of [popular Apex repos](./tests/battle_test/repos.txt)
