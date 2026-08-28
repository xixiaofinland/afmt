use crate::data_model::*;
use crate::doc::{pretty_print, BraceStyle, IndentStyle, JavadocStarColumn, PrettyConfig};
use crate::doc_builder::DocBuilder;
use crate::formatting_session::FormattingSession;
use crate::message_helper::{red, yellow};
use crate::utility::{
    assert_no_missing_comments, enrich, format_source_location, truncate_snippet,
};
use serde::Deserialize;
use std::panic::{catch_unwind, AssertUnwindSafe};
use tree_sitter::{Node, Parser, Tree};

/// Source-only formatter configuration and formatting pipeline.
#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    #[serde(default = "default_max_width")]
    pub max_width: u32,

    #[serde(default = "default_indent_size")]
    pub indent_size: u32,

    /// Opening-brace placement. Default `k_and_r` preserves current output.
    #[serde(default)]
    pub brace_style: BraceStyle,

    /// Wrap single-statement `if`/`else`/loop bodies in braces. Default `false`.
    #[serde(default)]
    pub wrap_single_statements: bool,

    /// Indentation character. Default `space`.
    #[serde(default)]
    pub indent_style: IndentStyle,

    /// JavaDoc continuation-star placement. Default `offset` preserves output.
    #[serde(default)]
    pub javadoc_star_column: JavadocStarColumn,

    /// Normalize known Apex annotation names to Salesforce's canonical casing.
    /// Default `false` preserves source casing.
    #[serde(default)]
    pub normalize_annotation_casing: bool,
}

fn default_max_width() -> u32 {
    80
}

fn default_indent_size() -> u32 {
    2
}

impl Default for Config {
    fn default() -> Self {
        Self {
            max_width: default_max_width(),
            indent_size: default_indent_size(),
            brace_style: BraceStyle::default(),
            wrap_single_statements: false,
            indent_style: IndentStyle::default(),
            javadoc_star_column: JavadocStarColumn::default(),
            normalize_annotation_casing: false,
        }
    }
}

impl Config {
    pub fn new(max_width: u32) -> Self {
        Self {
            max_width,
            ..Config::default()
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.indent_size == 0 {
            return Err("indent_size must be at least 1".to_string());
        }

        Ok(())
    }

    pub fn max_width(&self) -> u32 {
        self.max_width
    }

    pub fn indent_size(&self) -> u32 {
        self.indent_size
    }
}

pub(crate) fn format_one(source_code: &str, config: Config) -> String {
    try_format_source(source_code, config).unwrap_or_else(|message| panic!("{}", message))
}

pub(crate) fn try_format_source(source_code: &str, config: Config) -> Result<String, String> {
    try_format_source_with_origin(source_code, config, None)
}

pub(crate) fn try_format_source_with_origin(
    source_code: &str,
    config: Config,
    origin: Option<&str>,
) -> Result<String, String> {
    match catch_unwind(AssertUnwindSafe(|| {
        try_format_source_unchecked(source_code, config, origin)
    })) {
        Ok(result) => result,
        Err(panic_payload) => Err(format!(
            "Formatting panicked: {}",
            panic_message(panic_payload)
        )),
    }
}

fn try_format_source_unchecked(
    source_code: &str,
    config: Config,
    origin: Option<&str>,
) -> Result<String, String> {
    config
        .validate()
        .map_err(|error| format!("Invalid formatter configuration: {error}"))?;

    let ast_tree = try_parse(source_code, origin)?;
    let _session = FormattingSession::new(source_code, &ast_tree, origin);

    // traverse the tree to build enriched data
    let root: Root = enrich(&ast_tree);

    // traverse enriched data and create pretty print combinators
    let c = PrettyConfig::new(
        config.indent_size,
        config.brace_style,
        config.wrap_single_statements,
        config.indent_style,
        config.javadoc_star_column,
        config.normalize_annotation_casing,
    );
    let b = DocBuilder::new(c);
    let doc_ref = root.build(&b);

    let result = pretty_print(doc_ref, config.max_width, c);

    // debugging tool: use this to print named node value + comments in bucket
    // print_comment_map(&ast_tree);

    assert_no_missing_comments();

    Ok(result)
}

pub(crate) fn parse(source_code: &str) -> Tree {
    try_parse(source_code, None).unwrap_or_else(|message| panic!("{}", message))
}

fn try_parse(source_code: &str, origin: Option<&str>) -> Result<Tree, String> {
    let mut parser = Parser::new();
    let language_fn = tree_sitter_sfapex::apex::LANGUAGE;
    parser
        .set_language(&language_fn.into())
        .expect("Error loading Apex parser");

    let ast_tree = parser.parse(source_code, None).unwrap();
    let root_node = &ast_tree.root_node();

    if root_node.has_error() {
        if let Some(error_node) = find_last_error_node(root_node) {
            let error_snippet =
                truncate_snippet(&source_code[error_node.start_byte()..error_node.end_byte()]);
            // Lead with the location so the first token is the jumpable
            // `path:line:column` an editor can act on, the same shape the
            // unhonored-directive warning uses.
            let mut diagnostic = format!(
                "{}: parse error in node kind: {}, at byte range: {}-{}, snippet: {}",
                node_location(&error_node, origin),
                yellow(error_node.kind()),
                error_node.start_byte(),
                error_node.end_byte(),
                error_snippet,
            );
            if let Some(p) = error_node.parent() {
                let parent_snippet = truncate_snippet(&source_code[p.start_byte()..p.end_byte()]);
                diagnostic.push_str(&format!(
                    "\nParent node kind: {}, at {}, byte range: {}-{}, snippet: {}",
                    yellow(p.kind()),
                    node_location(&p, origin),
                    p.start_byte(),
                    p.end_byte(),
                    parent_snippet,
                ));
            }
            return Err(format!(
                "{}\n{}",
                diagnostic,
                red("Parser encounters an error node in the tree."),
            ));
        }
    }

    Ok(ast_tree)
}

/// Renders a node's one-based start position, prefixed with the source origin
/// when the caller named one. Parsing runs before the formatting session sets
/// the thread-local origin, so it is threaded in explicitly here.
fn node_location(node: &Node<'_>, origin: Option<&str>) -> String {
    let start = node.start_position();
    format_source_location(origin, start.row + 1, start.column + 1)
}

fn find_last_error_node<'tree>(node: &Node<'tree>) -> Option<Node<'tree>> {
    if !node.has_error() {
        return None; // If the current node has no error, return None
    }

    let mut last_error_node = Some(*node);

    for i in 0..node.child_count() {
        if let Some(child) = node.child(i) {
            if child.has_error() {
                last_error_node = find_last_error_node(&child);
            }
        }
    }

    last_error_node // Return the last (deepest) error node
}

fn panic_message(payload: Box<dyn std::any::Any + Send>) -> String {
    if let Some(message) = payload.downcast_ref::<&str>() {
        (*message).to_string()
    } else if let Some(message) = payload.downcast_ref::<String>() {
        message.clone()
    } else {
        "unknown panic payload".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::{try_format_source, try_format_source_with_origin, Config};

    const VALID_SOURCE: &str = "class T { void m() {} }\n";

    #[test]
    fn source_core_formats_valid_source_without_file_setup() {
        let formatted = try_format_source(VALID_SOURCE, Config::default()).expect("source formats");

        assert!(formatted.contains("class T"));
        assert!(formatted.contains("void m()"));
    }

    #[test]
    fn source_core_reports_parse_failures_without_path_decoration() {
        let error = try_format_source("class Broken {", Config::default())
            .expect_err("invalid source should fail");

        assert!(error.contains("Parser encounters an error node"));
        assert!(!error.contains(".cls:"));
    }

    #[test]
    fn parse_failures_lead_with_the_error_node_location() {
        let error = try_format_source_with_origin(
            "class Broken {\n    void run() {\n        Integer x = ;\n    }\n}\n",
            Config::default(),
            Some("Broken.cls"),
        )
        .expect_err("invalid source should fail");

        // The offending `=` sits at line 3, column 19. Pinning the column too
        // keeps the location honest: an editor jumps to the token, not to the
        // start of the line.
        assert!(
            error.starts_with("Broken.cls:3:19:"),
            "diagnostic should lead with its location: {error}"
        );
        assert!(error.contains("byte range"));
    }

    #[test]
    fn parse_failures_locate_stdin_when_no_path_exists() {
        let error = try_format_source_with_origin(
            "class Broken {\n    void run() {\n        Integer x = ;\n    }\n}\n",
            Config::default(),
            Some("<stdin>"),
        )
        .expect_err("invalid source should fail");

        assert!(
            error.starts_with("<stdin>:3:"),
            "stdin diagnostic should carry the stdin origin: {error}"
        );
    }

    #[test]
    fn parse_failures_report_bare_line_and_column_without_an_origin() {
        let error = try_format_source(
            "class Broken {\n    void run() {\n        Integer x = ;\n    }\n}\n",
            Config::default(),
        )
        .expect_err("invalid source should fail");

        assert!(
            error.starts_with("3:"),
            "library diagnostic should still locate the error: {error}"
        );
    }

    #[test]
    fn source_core_reports_config_validation_errors() {
        let error = try_format_source(
            VALID_SOURCE,
            Config {
                indent_size: 0,
                ..Config::default()
            },
        )
        .expect_err("invalid configuration should fail");

        assert_eq!(
            error,
            "Invalid formatter configuration: indent_size must be at least 1"
        );
    }

    #[test]
    fn source_core_formatting_is_idempotent() {
        let first = try_format_source(VALID_SOURCE, Config::default()).expect("source formats");
        let second =
            try_format_source(&first, Config::default()).expect("formatted source formats");

        assert_eq!(first, second);
    }
}
