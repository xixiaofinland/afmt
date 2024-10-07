# 🚀 A Blazingly Fast Salesforce Apex Formatter

Afmt is written in **Rust** 🦀.
It uses [tree-sitter apex parser](https://github.com/aheber/tree-sitter-sfapex) to traverse AST nodes.

Note: This is a project in its early phase, don't expect to use it in production code yet.
Project progress can be found below.

# 🔧 Usage

Download the binary from the [release page](https://github.com/xixiaofinland/afmt/releases). It
supports Linux, MacOS, and Linux.

Extract and run `afmt -h` to check the supported parameters.

```
Format Apex file v0.0.7

Usage: afmt [OPTIONS]

Options:
  -f, --file <FILE>      The relative path to the file to parse [default: test.cls]
  -c, --config <CONFIG>  Path to the .afmt.toml configuration file
  -w, --write            Write the formatted result back to the file
  -h, --help             Print help
  -V, --version          Print version
```

## Simplest use scenario:

- create a `test.cls` file next to binary with Apex code
- run `afmt` to dry-check the format result
- run `afmt -w` to write the format result into the file (`test.cls`)

```
» afmt
Result 0: Ok
public class Me {
  public integer prop { get; set {
    prop = value;
  } }
}

Formatted content written back to: test.cls

Afmt completed successfully.

Execution time: 995.869┬╡s

```
<br>

# 📟 Project Progress

| Feature                                         | Progress       | Difficulty   |
| ----------------------------------------------- | -------------- | ------------ |
| Recognize Apex nodes| ████████████ 100% | Easy         |
| Support `.afmt.toml` for configuration | ████████████ 100% | Easy         |
| Proper indentation | ████████░░░ 80%  | Easy         |
| Support SOQL                                    | ████████████ 100% | Medium       |
| Support SOSL                                    | ██████░░░░░ 50%  | Medium       |
| Reformat lines beyond `max_width`               | █░░░░░░░░░ 10%  | Challenging  |
| Support comment (line comment and block comment)| █░░░░░░░░░ 10%  | Challenging  |

<br>

# 📡 Technical parts

[Doc](doc/Technical.md)

<br>

[Doc](doc/Settings.md)
