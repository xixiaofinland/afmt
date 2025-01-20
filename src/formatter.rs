use crate::context::CommentMap;
use crate::data_model::*;
use crate::doc::{pretty_print, PrettyConfig};
use crate::doc_builder::DocBuilder;
use crate::message_helper::{red, yellow};
use crate::utility::{
    assert_no_missing_comments, collect_comments, enrich, set_thread_comment_map,
    set_thread_source_code,
};
use serde::Deserialize;
use std::fs;
use std::io::Read;
use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;
use tree_sitter::{Node, Parser, Tree};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Mode {
    Check,
    Write,
    Std,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    #[serde(default = "default_max_width")]
    pub max_width: u32,

    #[serde(default = "default_indent_size")]
    pub indent_size: u32,
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
        }
    }
}

impl Config {
    pub fn new(max_width: u32) -> Self {
        Self {
            max_width,
            indent_size: 2,
        }
    }

    pub fn from_file(path: &str) -> Result<Self, String> {
        let content =
            fs::read_to_string(path).map_err(|e| format!("Failed to read config file: {}", e))?;
        let config: Config =
            toml::from_str(&content).map_err(|e| format!("Failed to parse config file: {}", e))?;
        Ok(config)
    }

    pub fn max_width(&self) -> u32 {
        self.max_width
    }

    pub fn indent_size(&self) -> u32 {
        self.indent_size
    }
}

pub struct FormatResult {
    is_changed: bool,
    source_code: String,
}

#[derive(Debug)]
pub struct Formatter {
    config: Config,
    mode: Mode,
    paths: Vec<PathBuf>,
    //pub errors: ReportedErrors,
}

impl Formatter {
    pub fn new(config: Config, mode: Mode, paths: Vec<PathBuf>) -> Self {
        Self {
            config,
            mode,
            paths, //errors: ReportedErrors::default(),
        }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn create_from_config(
        config_path: Option<&str>,
        mode: Mode,
        paths: Vec<PathBuf>,
    ) -> Result<Formatter, String> {
        let config = match config_path {
            Some(path) => Config::from_file(path)
                .map_err(|e| format!("{}: {}", yellow(&e.to_string()), path))?,
            None => Config::default(),
        };
        Ok(Formatter::new(config, mode, paths))
    }

    pub fn format(&self) -> Vec<Result<String, String>> {
        let (tx, rx) = mpsc::channel();
        let config = self.config.clone();

        if self.paths.is_empty() {
            let mut buf = String::new();
            std::io::stdin()
                .read_to_string(&mut buf)
                .expect("Failed to read from stdin");

            return vec![Ok(Formatter::format_one(&buf, config))];
        }

        for file in &self.paths {
            let tx = tx.clone();
            let config = config.clone();
            let mode = self.mode.clone();
            let file = file.clone();

            thread::spawn(move || {
                let result = || -> Result<String, String> {
                    let source_code = fs::read_to_string(&file).map_err(|e| {
                        format!(
                            "Failed to read file: {} {}",
                            red(file.to_str().unwrap()),
                            yellow(e.to_string().as_str())
                        )
                    })?;

                    let formatted = Formatter::format_one(&source_code, config);

                    if mode == Mode::Write {
                        fs::write(&file, &formatted).map_err(|e| {
                            format!(
                                "Failed to write formatted content to {}: {}",
                                file.to_str().unwrap(),
                                e
                            )
                        })?;
                        // println!(
                        //     "Formatted content written back to: {}\n",
                        //     file.to_str().unwrap()
                        // );
                    } else {
                        // println!("{0}", formatted);
                    }

                    Ok(formatted)
                }();

                match result {
                    Ok(result) => {
                        tx.send(Ok(result)).expect("failed to send result in tx");
                    }
                    Err(e) => tx
                        .send(Err(format!("Thread panicked: {}", e)))
                        .expect("failed to send error in tx"),
                }
            });
        }

        drop(tx);

        rx.into_iter().collect()
    }

    pub fn format_one(source_code: &str, config: Config) -> String {
        let ast_tree = Formatter::parse(source_code);
        set_thread_source_code(source_code.to_string()); // important to set thread level source code now;

        let mut cursor = ast_tree.walk();
        let mut comment_map = CommentMap::new();
        collect_comments(&mut cursor, &mut comment_map);
        set_thread_comment_map(comment_map); // important to set thread level comment map;

        // traverse the tree to build enriched data
        let root: Root = enrich(&ast_tree);

        // traverse enriched data and create pretty print combinators
        let c = PrettyConfig::new(config.indent_size);
        let b = DocBuilder::new(c);
        let doc_ref = root.build(&b);

        let result = pretty_print(doc_ref, config.max_width);

        //print_comment_map(&ast_tree);

        assert_no_missing_comments();

        result
    }

    pub fn parse(source_code: &str) -> Tree {
        let mut parser = Parser::new();
        let language_fn = tree_sitter_sfapex::apex::LANGUAGE;
        parser
            .set_language(&language_fn.into())
            .expect("Error loading Apex parser");

        let ast_tree = parser.parse(source_code, None).unwrap();
        let root_node = &ast_tree.root_node();

        if root_node.has_error() {
            if let Some(error_node) = Self::find_last_error_node(root_node) {
                let error_snippet = &source_code[error_node.start_byte()..error_node.end_byte()];
                println!(
                    "Error in node kind: {}, at byte range: {}-{}, snippet: {}",
                    yellow(error_node.kind()),
                    error_node.start_byte(),
                    error_node.end_byte(),
                    error_snippet,
                );
                if let Some(p) = error_node.parent() {
                    let parent_snippet = &source_code[p.start_byte()..p.end_byte()];
                    println!(
                        "Parent node kind: {}, at byte range: {}-{}, snippet: {}",
                        yellow(p.kind()),
                        p.start_byte(),
                        p.end_byte(),
                        parent_snippet,
                    );
                }
            }
            panic!("{}", red("Parser encounters an error node in the tree."));
        }

        ast_tree
    }

    fn find_last_error_node<'tree>(node: &Node<'tree>) -> Option<Node<'tree>> {
        if !node.has_error() {
            return None; // If the current node has no error, return None
        }

        let mut last_error_node = Some(*node);

        for i in 0..node.child_count() {
            if let Some(child) = node.child(i) {
                if child.has_error() {
                    last_error_node = Self::find_last_error_node(&child);
                }
            }
        }

        last_error_node // Return the last (deepest) error node
    }
}
