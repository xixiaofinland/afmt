# 🚀 A Blazingly Fast Salesforce Apex Formatter

Afmt is a **Salesforce Apex Code Formatter** written in **Rust**! This tool formats your Apex code for consistency and readability. 🎯 It uses [tree-sitter apex parser](https://github.com/aheber/tree-sitter-sfapex) to traverse AST nodes.

Note: This is a project in its early phase, don't expect to use it in production code yet. 
Project Project can be found in this section.

# 🔧 Usage

Download the binary from the [release page](https://github.com/xixiaofinland/afmt/releases). It
supports Linux, MacOS, and Linux.

Run `afmt -h` to check the supported parameters.

## Simplest use scenario:

- create a `test.cls` file next to binary
- run `afmt` to dry-check the format result
- run `afmt -w` to write the format result back to `test.cls`

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
