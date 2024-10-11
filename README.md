# 🚀 A Blazingly Fast Salesforce Apex Formatter

Afmt is written in **Rust** 🦀. It uses [tree-sitter apex parser](https://github.com/aheber/tree-sitter-sfapex) to traverse AST nodes.

Note. this is a project in its early phase, don't expect to use it in production code yet.

# ✨ Highlights

Blazingly fast - parsing speed of largest open-source Apex files [report](https://xixiaofinland.github.io/afmt/results.html)

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

# 🔧 How to use

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
public class S {
  {
    rd.RecurringDonationSchedules__r?.get(0)?.nextDonationDate;
  }
}

"public class S {\n  {\n    rd.RecurringDonationSchedules__r?.get(0)?.nextDonationDate;\n  }\n}\n"
Afmt completed successfully.

Execution time: 585.1┬╡s
```
```
» afmt -w
Result 0: Ok
public Date getExpectedDonationDate(RD2_ScheduleService scheduleService) {
  return rd.RecurringDonationSchedules__r?.get(0)?.nextDonationDate;
}

Formatted content written back to: test.cls

Afmt completed successfully.

Execution time: 724.826┬╡s
```
<br>


# 📡 Technical parts

[Performance site](https://xixiaofinland.github.io/afmt/)

[Technical Doc](doc/Technical.md)

[Config Doc](doc/Settings.md)
