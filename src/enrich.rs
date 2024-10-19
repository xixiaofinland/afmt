use std::fmt::Debug;
use tree_sitter::Node;

use crate::{child::Accessor, config::Config};

trait Rich: Debug {
    fn enrich(&mut self, shape: &EShape, context: &EContext, comments: &mut Vec<Comment>);
    //fn enrich_comments(&mut self);
    //fn enrich_data(&mut self);
    //fn rewrite(&mut self) -> String;
}

#[derive(Debug, Default)]
struct FormatInfo {
    pub wrappable: bool,
    pub indent_level: usize,
    pub force_break_before: bool,
    pub force_break_after: bool,
    pub offset: usize,
}

#[derive(Debug, Default)]
struct CommentBuckets {
    pub pre_comments: Vec<Comment>,
    pub inline_comments: Vec<Comment>,
    pub post_comments: Vec<Comment>,
}

#[derive(Debug)]
pub struct Comment {
    //pub content: String,
    pub is_processed: bool,
    pub comment_type: CommentType,
}

impl Comment {
    pub fn from_node(node: &Node) -> Self {
        Comment {
            //content: node,
            is_processed: false,
            comment_type: match node.kind() {
                "line_comment" => CommentType::Line,
                "block_comment" => CommentType::Block,
                _ => panic!("Unexpected comment type"),
            },
        }
    }
}

#[derive(Debug)]
enum CommentType {
    Line,
    Block,
}

#[derive(Debug)]
pub struct EShape {
    pub indent_level: usize,
}

impl EShape {
    pub fn empty() -> Self {
        Self { indent_level: 0 }
    }
}

#[derive(Debug)]
pub struct EContext {
    pub config: Config,
    pub source_code: String,
}

impl EContext {
    pub fn new(config: &Config, source_code: &str) -> Self {
        let config = config.clone();
        let source_code = String::from(source_code);
        Self {
            config,
            source_code,
        }
    }
}

#[derive(Debug)]
pub struct ClassNode<'a, 'tree> {
    pub inner: &'a Node<'tree>,
    pub content: String,
    pub comments: CommentBuckets,
    pub children: Vec<Box<dyn Rich>>,
    pub field_name: Option<String>,
    pub formatting_info: FormatInfo,
}

impl<'a, 'tree> ClassNode<'a, 'tree> {
    pub fn new(inner: &'a Node<'tree>) -> Self {
        Self {
            inner,
            content: String::new(),
            comments: CommentBuckets::default(),
            children: Vec::new(),
            field_name: None,
            formatting_info: FormatInfo::default(),
        }
    }
}

impl<'a, 'tree> Rich for ClassNode<'a, 'tree> {
    fn enrich(&mut self, shape: &EShape, context: &EContext, comments: &mut Vec<Comment>) {
        self.enrich_comments(comments);
        self.enrich_data(shape, context);
    }
}

impl<'a, 'tree> ClassNode<'a, 'tree> {
    fn enrich_comments(&mut self, comments: &mut Vec<Comment>) {
        // Check previous siblings for pre-comments
        let mut prev_sibling = self.inner.prev_sibling();
        while let Some(node) = prev_sibling {
            if node.is_comment() {
                self.comments.pre_comments.push(Comment::from_node(&node));
            }
            prev_sibling = node.prev_sibling();
        }

        // Check next siblings for post-comments
        let mut next_sibling = self.inner.next_sibling();
        while let Some(node) = next_sibling {
            if node.is_comment() {
                self.comments.post_comments.push(Comment::from_node(&node));
            }
            next_sibling = node.next_sibling();
        }
    }

    fn enrich_data(&mut self, shape: &EShape, context: &EContext) {
        self.formatting_info = match self.inner.kind() {
            "class_declaration" => FormatInfo {
                ..Default::default()
            },

            _ => FormatInfo {
                wrappable: true,
                ..Default::default()
            },
        }
    }
}
