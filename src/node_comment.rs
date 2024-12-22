use std::collections::HashMap;
use tree_sitter::{Node, Range};

use crate::{accessor::Accessor, data_model::DocBuild, doc::DocRef, doc_builder::DocBuilder, utility::panic_unknown_node};

pub type CommentMap<'t> = HashMap<usize, NodeComment<'t>>;

#[derive(Debug)]
pub struct NodeComment<'t> {
    pub pre_comments: Vec<Node<'t>>,
    pub post_comments: Vec<Node<'t>>,
    pub dangling_comments: Vec<Node<'t>>,
}

impl<'t> NodeComment<'t> {
    pub fn new() -> Self {
        Self {
            pre_comments: Vec::new(),
            post_comments: Vec::new(),
            dangling_comments: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum CommentType {
    Line,
    Block,
}

#[derive(Debug, Clone)]
pub struct Comment {
    pub id: usize,
    pub content: String,
    pub comment_type: CommentType,
    pub is_processed: bool,
    pub range: Range,
}

impl Comment {
    pub fn from_node(node: Node) -> Self {
        let id = node.id();
        let content = node.value();
        Self {
            id,
            content,
            is_processed: false,
            comment_type: match node.kind() {
                "line_comment" => CommentType::Line,
                "block_comment" => CommentType::Block,
                _ => panic_unknown_node(node, "Comment"),
            },
            range: node.range(),
        }
    }
}

impl<'a> DocBuild<'a> for Comment {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        result.push(b.txt(&self.content));
        result.push(b.nl());
    }
}

