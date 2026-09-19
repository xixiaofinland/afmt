# Formatter configuration

`afmt` reads configuration only when a file is explicitly supplied with
`--config` or `-c`. It does not discover `.afmt.toml` implicitly. Omitted
options retain the formatter defaults.

```toml
max_width = 80
indent_size = 2

# Optional style controls
brace_style = "k_and_r"            # or "allman"
wrap_single_statements = false      # or true
indent_style = "space"              # or "tab"
javadoc_star_column = "offset"      # or "flush"
normalize_annotation_casing = false
```

`brace_style = "allman"` puts opening braces on their own lines. The default
`"k_and_r"` keeps them on the construct's header line.

`wrap_single_statements = true` adds braces around otherwise bare `if`,
`else`, and loop bodies. The default is `false`.

`indent_style` selects spaces or tabs for emitted indentation. `indent_size`
controls the logical indentation width; with tabs, it is the number of columns
represented by each indentation level. Line wrapping uses that logical width
when applying `max_width`. `indent_size` must be between 1 and 256; values
outside that range are rejected as a configuration error. To keep deeply nested
input safe, afmt also caps the indentation materialized on any one line at 256
columns.

`max_width = 0` disables line-width wrapping entirely: afmt never breaks a
line to fit a width, regardless of how long it gets.

`javadoc_star_column` controls only the leading star on continuation lines in
Javadoc (`/** ... */`) comments:

- `"offset"` (default) emits ` * content`, preserving existing output.
- `"flush"` emits `* content`, aligning the star with the comment indentation.

In both modes, afmt normalizes the separator after the star to one space.
Blank Javadoc lines and the closing line follow the same star-column choice.
Non-Javadoc block comments and line comments are unaffected by this option.

`normalize_annotation_casing` defaults to `false`, preserving authored
annotation casing. When enabled, known Salesforce Apex annotation names use
canonical casing: `@isTest` becomes `@IsTest`, `@testsetup` becomes
`@TestSetup`, and `@auraenabled` becomes `@AuraEnabled`. Unknown or custom
annotation names remain verbatim. Only the annotation name is normalized;
argument keys, values, strings, and surrounding comments are unchanged.
