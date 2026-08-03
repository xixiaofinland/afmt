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
indent_size = 4

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

`src/config.rs` owns application-level TOML loading and file-selection
defaults. `src/discovery.rs` owns path expansion, glob matching, exclusions,
de-duplication, and ordering. `src/formatter.rs` owns formatting and
path-aware per-file outcomes. `src/main.rs` owns CLI output, check/write
policy, timing, and aggregate status.
