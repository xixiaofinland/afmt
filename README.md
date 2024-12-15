# 🚀 A Blazingly Fast Salesforce Apex Formatter

Afmt is written in **Rust** 🦀. It uses [tree-sitter apex parser](https://github.com/aheber/tree-sitter-sfapex) to traverse AST nodes.

Note. this is a project in its early phase, don't expect to use it in production code yet.

# ✨ afmt vs Prettier Apex

[Prettier Apex Repo](https://github.com/dangmai/prettier-plugin-apex)

| Feature                   | afmt                                      | Prettier Apex                             |
|---------------------------|-------------------------------------------|-------------------------------------------|
| **Maturity**              | Brand new | Battle tested for years|
| **Dependencies**       |A binnary file only| NodeJS + prettier package|
| **Performance**            |Fast |Relatively slower|
| **Open Source**           | Yes| Yes|

<br>

# 📟 Project Progress

| Feature                                         | Progress       |
| ----------------------------------------------- | -------------- |
| Recognize Apex nodes                            | ████████████ 100%  |
| Support `.afmt.toml` for configuration          | ████████████ 100%         |
| Proper indentation                              | ████████████ 100%  |
| Support SOQL                                    | ████████████ 100%  |
| Support SOSL                                    | ████████████ 100%  |
| Reformat lines beyond `max_width`               | ████████████ 100%  |
| Support comment (line comment and block comment)| █░░░░░░░░░ 10%  |

<br>

# 🔧 How to use

Download the binary from the [release page](https://github.com/xixiaofinland/afmt/releases). It
supports Linux, MacOS, and Windows.

Extract and run `afmt -h` to check the supported parameters.

```
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

## Simplest use scenario:

- create a `file.cls` file next to binary with valid Apex format
- run `afmt ./file.cls` to dry-check the format result
- run `afmt -w ./file.cls` to write the format result into the file

Dry-check sample result:
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


Format file sample result:
```
Formatted content written back to: ./file.cls

Afmt completed successfully.

Execution time: 555.29┬╡s
```
<br>

# 📡 Technical parts

[Technical Doc](md/Technical.md)

[Config Doc](md/Settings.md)
