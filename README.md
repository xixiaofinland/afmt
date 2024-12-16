# 🚀 A Fast Configrable Salesforce Apex Formatter
<div align="center">
  <img src="https://github.com/user-attachments/assets/5cf08fdb-aaa2-4556-83d7-2e9d2a99f86f" alt="afmt_logo" width="300"/>
</div>
<br>

`afmt` (Apex format tool) is written in Rust 🦀 and utilizes the [tree-sitter sfapex parser](https://github.com/aheber/tree-sitter-sfapex).

**Note:** it's in its early stages. Currently the code comments are not
implemented yet ([progress
track](https://github.com/xixiaofinland/afmt#-progress)), they are
ignored/erased, so don't expect to use it in production code yet.

# ✨ vs. Prettier Apex

Both `afmt` and [Prettier Apex](https://github.com/dangmai/prettier-plugin-apex)
provide well-formatted Apex code by leveraging the same line-wrapping algorithm:
[Wadler's
Pretty-Print](https://homepages.inf.ed.ac.uk/wadler/papers/prettier/prettier.pdf).

However, unlike Prettier's "opinionated" approach, `afmt` strives to provide a
more configurable user experience. As a result, the formatting outcomes
of the two tools vary, reflecting their fundamentally different guiding
principles.
<br>

## Other Highlights:

| Feature                   | afmt                                      | Prettier Apex                             |
|---------------------------|-------------------------------------------|-------------------------------------------|
| **Maturity**              | Brand new | Battle tested for years|
| **Dependencies**       | N/A (standalone binary) | Node.js + prettier package|
| **Performance**            |Fast (Rust) |Relatively slower (Node.js)|
| **Parser**            |sfapex (C / open source) |Jorje (Java / close source)|
| **Open Source**           | Yes| Yes|

<br>

# 📟 Progress

| Feature                                         | Progress       |
| ----------------------------------------------- | -------------- |
| Recognize Apex nodes                            | ████████████ 100%  |
| Support `.afmt.toml` for configuration          | ████████████ 100%         |
| Proper indentation                              | ████████████ 100%  |
| Support SOQL                                    | ████████████ 100%  |
| Support SOSL                                    | ████████████ 100%  |
| Line wrapping               | ████████████ 100%  |
| Support comment (line and block comment)| █░░░░░░░░░ 10%  |

<br>

# 🔧 How to use

## Video version

[afmt intro - made on 15/12/2024](https://youtu.be/2tBctZqdjMU?si=j5Lmip8sAg_AKTK1&t=148)

## Text version

1. Download the binary:
- visit the [release page](https://github.com/xixiaofinland/afmt/releases/latest)
to download the appropriate binary for your OS (Linux, MacOS, or Windows).

2. Extract and run:
- Extract the downloaded `afmt` binary, such as to `~/`.
- Run `afmt -h` to view the supported parameters.

```
> ./afmt -h
Apex format tool (afmt): v0.0.19

Usage: afmt [OPTIONS] <FILE>

Arguments:
  <FILE>  The relative path to the file to parse

Options:
  -c, --config <CONFIG>  Path to the .afmt.toml configuration file
  -w, --write            Write the formatted result back to the file
  -h, --help             Print help
  -V, --version          Print version

EXAMPLES:

# Dry run: print the result without overwriting the file
afmt ./file.cls

# Format and write changes back to the file
afmt --write src/file.cls

# Use a specific config file
afmt --config .afmt.toml ./file.cls
```

## Simple use scenarios:

### Dry Run:

1. Create a `file.cls` file next to binary with valid Apex code.
2. Run `afmt ./file.cls` to preview the formatting result.

```
> afmt ./file.cls
Result 0: Ok
global class PluginDescribeResult {
    {
        [SELECT FIELDS(STANDARD) FROM Organization LIMIT 1];
    }
}

Execution time: 491.772┬╡s
```

### Format and Write:

1. Run `afmt -w ./file.cls` to format the file and overwrite it with the
   formatted code.

```
> afmt -w ./file.cls
Formatted content written back to: ./file.cls

Afmt completed successfully.

Execution time: 555.29┬╡s
```
<br>

# 📡 Technical Documentation

[Technical Doc](md/Technical.md)

[Config Doc](md/Settings.md)
