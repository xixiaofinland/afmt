# Project-wide formatting

This guide documents the as-built behavior of the bulk `afmt` CLI. The
formatter still accepts a single file, but positional inputs are now one or
more files or directories.

## Inputs and discovery

- A directory is traversed recursively without following directory symlinks.
- Eligible files use the `.cls`, `.trigger`, `.apex`, or `.apexc` extensions.
- Explicit file inputs bypass the include globs, but exclusions still apply.
- Exclusions also apply to explicit directory inputs; excluded directories are
  pruned before descent and do not produce input or traversal errors.
- Overlapping inputs are de-duplicated.
- Results are processed in deterministic normalized path order.
- All paths remain OS-native for file operations; glob matching uses portable
  `/` separators.
- A missing, unsupported, or otherwise invalid non-excluded input is reported
  before formatting. An input set with no eligible files is an error.

The default exclusions are `.git`, `.sfdx`, and `node_modules`. Directory
symlinks are not followed during discovery. `afmt` does not read `.gitignore`.

## Configuration

Configuration is opt-in: pass `--config` (or `-c`) to load a TOML file. No
configuration file is discovered implicitly. Existing flat formatter keys
remain valid:

```toml
max_width = 80
indent_size = 2

[files]
include = ["**/*.cls", "**/*.trigger", "**/*.apex", "**/*.apexc"]
exclude = ["**/.git/**", "**/.sfdx/**", "**/node_modules/**"]
```

The `[files].include` and `[files].exclude` arrays replace their respective
defaults when supplied; they do not merge with them. Exclusions always win
over inclusions. Invalid glob syntax is a configuration error and prevents
formatting.

Patterns are matched against paths relative to the config file's parent when
`--config` is supplied, or relative to the process working directory when no
config is supplied. Patterns should use `/` separators so the same config is
portable across Windows, macOS, and Linux.

## Output and exit behavior

```text
afmt [OPTIONS] <PATH>...
```

- A lone `-` path reads one complete Apex source buffer from stdin and writes
  only the formatted source to stdout. It cannot be combined with another
  path, `--write`, or `--check`; those combinations are usage errors with exit
  status `2`.
- Stdin mode uses the same formatter as file inputs. Configuration is explicit:
  `-c/--config` is honored, but `.afmt.toml` is never discovered implicitly for
  stdin. Formatting and parsing errors are written to stderr, produce exit
  status `1`, and write no partial stdout.
- Without `--write` or `--check`, formatted source is written to stdout.
- A selection containing exactly one file prints plain formatted source.
- A selection containing two or more files separates each source block with a
  deterministic `==> path <==` delimiter.
- `--write` writes changed files in place and leaves unchanged files alone.
  Bulk writes are best effort: failures are reported with their paths while
  other selected files continue.
- `--check` never writes and exits nonzero if any selected file would change,
  if discovery/formatting fails, or if no eligible files are selected. It
  conflicts with `--write`.
- `--time` writes per-file and total timing diagnostics to stderr. It does not
  alter formatted stdout.
- `--time` is ignored for stdin input; stdin output remains a pure formatted
  source stream with no timing or summary diagnostics.
- Bulk operations report selected, changed, written, unchanged, failed, and
  elapsed counts on stderr.

Exit status `0` means the requested operation completed successfully. Exit
status `1` indicates an application error, a changed file in check mode, or a
partial write failure.

### Parse error locations

A parse failure leads with the offending node's one-based line and column so an
editor can jump to it, then reports the node kind, byte range, and snippet:

```
Error: force-app/main/default/classes/Broken.cls:3:19: parse error in node kind: ERROR, at byte range: 57-58, snippet: =
Parent node kind: local_variable_declaration, at force-app/main/default/classes/Broken.cls:3:9, byte range: 47-60, snippet: Integer x = ;
Parser encounters an error node in the tree.
```

The prefix follows the same origin rule as the unhonored-directive warning: the
path afmt was given, `<stdin>` when the source arrived on stdin, and a bare
`line:column` for a library caller that used
`Formatter::try_format_source`. Because the diagnostic locates itself,
`FormatFileError`'s `Display` omits its own path prefix when the message
already opens with that path, so the path appears once per line.

An editor integration that pipes a buffer through `afmt -` gets `<stdin>` as
the origin — afmt is not told which file the buffer came from — so the client
maps the reported line and column back onto the document it sent.

When a buffer being typed into has an unbalanced brace, tree-sitter can wrap
the remaining source in an ERROR node containing the structure it parsed
before running out of input. In that case the location points at the error
node's end — where the source runs out — rather than a less useful recovery
node start. This is the common format-on-save case, so the client still gets a
line and column near the live edge to jump to.

`stdin_parse_errors_locate_the_error_by_line_and_column` and
`file_parse_errors_carry_one_path_line_and_column` in `tests/cli.rs` pin this
contract end to end, and `parse_failures_lead_with_the_error_node_location`,
`parse_failures_locate_stdin_when_no_path_exists`,
`parse_failures_report_bare_line_and_column_without_an_origin`,
`truncated_source_locates_where_it_runs_out_not_the_file_start`, and
`truncated_source_after_a_leading_comment_still_locates_where_it_runs_out` in
`src/source_formatter.rs` pin the origin forms and truncated-source cases.

### Changes to stdout from earlier releases

Two stdout behaviors changed so that stdout carries the formatted document and
nothing else, which is what makes `afmt -` safe to pipe into an editor or LSP
formatter provider. Anything parsing afmt's stdout should be checked against
both:

- **Timing moved to stderr.** `--time` previously wrote
  `\n-- Execution time: <duration>` to stdout. It now writes `Timing: <path>
  <duration>` per file and `Total elapsed: <duration>` to stderr.
- **Exactly one trailing newline.** Formatted source was previously emitted
  with `println!`, which appended a newline unconditionally, so a document
  already ending in a newline produced a trailing blank line. Output is now
  emitted verbatim and a newline is added only when the document does not
  already end in one.

## Implementation boundaries

`src/source_formatter.rs` owns the source-to-source formatting core and
formatter configuration validation. `src/formatter.rs` provides the public
1.x compatibility surface and owns file reads, path-aware per-file outcomes,
elapsed timing, and Rayon batch execution. `src/config.rs` owns
application-level TOML loading and file-selection defaults. `src/discovery.rs`
owns path expansion, glob matching, exclusions, de-duplication, and ordering.
`src/main.rs` owns CLI output, check/write policy, timing display, and aggregate
status. See [Formatter architecture](formatter-architecture.md) for the
boundary details.
