use std::collections::HashMap;
use tree_sitter::{Node, Range};

use crate::{
    accessor::Accessor, data_model::DocBuild, doc::DocRef, doc_builder::DocBuilder,
    utility::panic_unknown_node,
};

pub type CommentMap = HashMap<usize, CommentBucket>;

#[derive(Debug)]
pub struct CommentBucket {
    pub pre_comments: Vec<Comment>,
    pub post_comments: Vec<Comment>,
    pub dangling_comments: Vec<Comment>,
}

impl CommentBucket {
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
    pub value: String,
    pub comment_type: CommentType,
    pub range: Range,
}

impl Comment {
    pub fn from_node(node: Node) -> Self {
        let id = node.id();
        let value = node.value();
        Self {
            id,
            value,
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
        result.push(b.txt(&self.value));
        result.push(b.nl());
    }
}
