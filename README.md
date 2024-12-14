# 🚀 A Blazingly Fast Salesforce Apex Formatter

Afmt is written in **Rust** 🦀. It uses [tree-sitter apex parser](https://github.com/aheber/tree-sitter-sfapex) to traverse AST nodes.

Note. this is a project in its early phase, don't expect to use it in production code yet.

# ✨ Highlights

Blazingly fast - parsing speed of largest open-source Apex files [report](https://xixiaofinland.github.io/afmt/hyperfine.html)

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
supports Linux, MacOS, and Linux.

Extract and run `afmt -h` to check the supported parameters.

```
Format Apex file v0.0.16

Usage: afmt [OPTIONS] [FILE]

Arguments:
  [FILE]  The relative path to the file to parse [default: ./hello.cls]

Options:
  -c, --config <CONFIG>  Path to the .afmt.toml configuration file
  -w, --write            Write the formatted result back to the file
  -h, --help             Print help
  -V, --version          Print version
```

## Simplest use scenario:

- create a `hello.cls` file next to binary with valid Apex format
- run `afmt ./hello.cls` to dry-check the format result
- run `afmt -w ./hello.cls` to write the format result into the file

Dry-check sample result:
```
> afmt ./hello.cls
Result 0: Ok
global class PluginDescribeResult {
    {
        [SELECT FIELDS(STANDARD) FROM Organization LIMIT 1];
    }
}

"global class PluginDescribeResult {\n    {\n        [SELECT FIELDS(STANDARD) FROM Organization LIMIT 1];\n    }\n}\n"
Afmt completed successfully.

Execution time: 491.772┬╡s
```

Format file sample result:
```
> afmt -w ./hello.cls
Result 0: Ok
global class PluginDescribeResult {
    {
        [SELECT FIELDS(STANDARD) FROM Organization LIMIT 1];
    }
}

Formatted content written back to: ./hello.cls

Afmt completed successfully.

Execution time: 591.469┬╡s
```
<br>


# 📡 Technical parts

[Technical Doc](md/Technical.md)

[Config Doc](md/Settings.md)
