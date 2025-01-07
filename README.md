# 🚀 A Fast Configrable Salesforce Apex Formatter
<div align="center">
  <img src="https://github.com/user-attachments/assets/5cf08fdb-aaa2-4556-83d7-2e9d2a99f86f" alt="afmt_logo" width="300"/>
</div>
<br>

`afmt` (Apex format tool) is written in Rust 🦀 and utilizes the [tree-sitter sfapex parser](https://github.com/aheber/tree-sitter-sfapex).

It's in its early stages, please [report
issues](https://github.com/xixiaofinland/afmt/issues)
and we will fix them asap!

Download the binary in [release page](https://github.com/xixiaofinland/afmt/releases/latest)

Use it in VSCode? [Instruction](https://github.com/xixiaofinland/afmt/blob/main/md/VSCode_Setup.md)

# ✨ vs. Prettier Apex

While currently showing minor formatting differences, the tools fundamentally
diverge in their design philosophy. Prettier Apex maintains an opinionated
approach to code formatting, whereas afmt is designed with extensibility
in mind.

This means afmt will progressively introduce more configuration
options, addressing user customization needs that Prettier's design
intentionally avoids.

## Other Highlights:

| Feature                   | afmt                                      | Prettier Apex                             |
|---------------------------|-------------------------------------------|-------------------------------------------|
| **Maturity**              | Brand new | Battle tested for years|
| **Dependencies**       | N/A (standalone binary) | Node.js + prettier package|
| **Performance**            |Fast (Rust) |Relatively slower (Node.js)|
| **Parser**            |sfapex (C / open source) |Jorje (Java / close source)|
| **Open Source**           | Yes| Yes|
<br>

# 🔧 How to use

## Video version

[afmt intro - made on 15/12/2024](https://youtu.be/2tBctZqdjMU?si=j5Lmip8sAg_AKTK1&t=148)

Note. the info in the video is not up-to-date as comment nodes are supported now

## Text version

1. Download the binary:
- visit the [release page](https://github.com/xixiaofinland/afmt/releases/latest)
to download the appropriate binary for your OS (Linux, MacOS, or Windows).

2. Extract and run:
- Extract the downloaded `afmt` binary, such as to `~/`.
- Run `afmt -h` to view the supported parameters.

## Simple use scenarios:

### Dry Run:

1. Create a `file.cls` file with valid Apex code.
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

## Configuration

in `.afmt.toml` config file, two options are supported

```toml
# Maximum line width
max_width = 80

# Indentation size in spaces
indent_size = 4
```


## Use it in editors:

[Guidance in VSCode](./md/VSCode_Setup.md)

# 📡 Technical Documentation

[Technical Doc](md/Technical.md)

[Config Doc](md/Settings.md)
