use std::collections::HashMap;
use tree_sitter::{Node, Range};

use crate::{
    accessor::Accessor, data_model::DocBuild, doc::DocRef, doc_builder::DocBuilder,
    utility::panic_unknown_node,
};

pub type CommentMap = HashMap<usize, CommentBucket>;

#[derive(Debug)]
pub struct NodeInfo {
    pub id: usize,
    pub range: Range,
}

impl NodeInfo {
    pub fn from(node: &Node) -> Self {
        Self {
            id: node.id(),
            range: node.range(),
        }
    }
}

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

#[derive(Debug)]
pub enum CommentType {
    Line,
    Block,
}

#[derive(Debug)]
pub struct Comment {
    pub id: usize,
    pub value: String,
    pub comment_type: CommentType,
    pub metadata: CommentMetadata,
}

impl Comment {
    pub fn from_node(node: Node) -> Self {
        let id = node.id();
        let value = node.value();
        let (comment_type, metadata) = match node.kind() {
            "line_comment" => {
                let metadata = CommentMetadata::from(&node, CommentType::Line);
                (CommentType::Line, metadata)
            }
            "block_comment" => {
                let metadata = CommentMetadata::from(&node, CommentType::Block);
                (CommentType::Block, metadata)
            }
            _ => panic_unknown_node(node, "Comment"),
        };

        Self {
            id,
            value,
            comment_type,
            metadata,
        }
    }

    pub fn has_leading_content(&self) -> bool {
        self.metadata.has_leading_content
    }

    pub fn has_trailing_content(&self) -> bool {
        self.metadata.has_trailing_content
    }

    pub fn has_newline_above(&self) -> bool {
        self.metadata.has_newline_above
    }

    pub fn has_newline_below(&self) -> bool {
        self.metadata.has_newline_below
    }
}

impl<'a> DocBuild<'a> for Comment {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self.comment_type {
            CommentType::Line => {
                if self.has_leading_content() {
                    result.push(b.txt(" "));
                    result.push(b.txt(&self.value));
                } else {
                    result.push(b.nl());
                    result.push(b.txt(&self.value));
                }
            }
            CommentType::Block => {
                if self.has_trailing_content() {
                    result.push(b.txt(" "));
                    result.push(b.txt(&self.value));
                } else {
                    result.push(b.nl());
                    result.push(b.txt(&self.value));
                }

                if self.metadata.has_trailing_content {
                    result.push(b.txt(" "));
                } else {
                     result.push(b.nl());
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct CommentMetadata {
    pub has_leading_content: bool,
    pub has_trailing_content: bool,
    pub has_newline_above: bool,
    pub has_newline_below: bool,
}

impl CommentMetadata {
    pub fn from(node: &Node, comment_type: CommentType) -> Self {
        let prev = node.prev_named_sibling();
        let next = node.next_named_sibling();

        let has_leading_content = if let Some(prev_node) = prev {
            node.start_position().row == prev_node.end_position().row
        } else {
            false
        };

        let has_trailing_content = match comment_type {
            CommentType::Line => false,
            CommentType::Block => {
                if let Some(next_node) = next {
                    node.end_position().row == next_node.start_position().row
                } else {
                    false
                }
            }
        };

        let has_newline_above = if let Some(prev_node) = prev {
            node.start_position().row > prev_node.end_position().row + 1
        } else {
            false
        };

        let has_newline_below = if let Some(next_node) = next {
            node.end_position().row < next_node.start_position().row - 1
        } else {
            false
        };

        CommentMetadata {
            has_leading_content,
            has_trailing_content,
            has_newline_above,
            has_newline_below,
        }
    }
}
