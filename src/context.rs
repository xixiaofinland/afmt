use std::collections::HashMap;
use tree_sitter::{Node, Range};

use crate::{
    accessor::Accessor,
    data_model::{Block, DocBuild},
    doc::DocRef,
    doc_builder::DocBuilder,
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
    Line(LineCommentMetadata),
    Block(BlockCommentMetadata),
}

#[derive(Debug)]
pub struct Comment {
    pub id: usize,
    pub value: String,
    pub comment_type: CommentType,
}

impl Comment {
    pub fn from_node(node: Node) -> Self {
        let id = node.id();
        let value = node.value();
        let comment_type = match node.kind() {
            "line_comment" => CommentType::Line(LineCommentMetadata::from(&node)),
            "block_comment" => CommentType::Block(BlockCommentMetadata::from(&node)),
            _ => panic_unknown_node(node, "Comment"),
        };

        Self {
            id,
            value,
            comment_type,
        }
    }

    pub fn has_newline_above(&self) -> bool {
        match &self.comment_type {
            CommentType::Line(metadata) => metadata.has_newline_above,
            CommentType::Block(metadata) => metadata.has_newline_above,
        }
    }

    pub fn has_newline_below(&self) -> bool {
        match &self.comment_type {
            CommentType::Line(metadata) => metadata.has_newline_below,
            CommentType::Block(metadata) => metadata.has_newline_below,
        }
    }
}

impl<'a> DocBuild<'a> for Comment {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match &self.comment_type {
            CommentType::Line(m) => {
                if m.has_leading_content {
                    result.push(b.txt(" "));
                    result.push(b.txt(&self.value));
                } else {
                    result.push(b.nl());
                    result.push(b.txt(&self.value));
                }
            }
            CommentType::Block(m) => {
                if m.has_leading_content {
                    result.push(b.txt(" "));
                    result.push(b.txt(&self.value));
                } else {
                    result.push(b.nl());
                    result.push(b.txt(&self.value));
                }

                if m.has_trailing_content {
                    result.push(b.txt(" "));
                } else {
                    result.push(b.nl());
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct LineCommentMetadata {
    pub has_leading_content: bool,
    pub has_newline_above: bool,
    pub has_newline_below: bool,
}

impl LineCommentMetadata {
    pub fn from(node: &Node) -> Self {
        let prev = node.prev_named_sibling();
        let next = node.next_named_sibling();

        let has_leading_content = if let Some(prev_node) = prev {
            prev_node.end_position().row == node.start_position().row
        } else {
            false
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

        LineCommentMetadata {
            has_leading_content,
            has_newline_above,
            has_newline_below,
        }
    }
}

#[derive(Debug)]
pub struct BlockCommentMetadata {
    pub has_leading_content: bool,
    pub has_trailing_content: bool,
    pub has_newline_above: bool,
    pub has_newline_below: bool,
}

impl BlockCommentMetadata {
    pub fn from(node: &Node) -> Self {
        let prev = node.prev_named_sibling();
        let next = node.next_named_sibling();

        let has_leading_content = if let Some(prev_node) = prev {
            node.start_position().row == prev_node.end_position().row
        } else {
            false
        };

        let has_trailing_content = if let Some(next_node) = next {
            node.end_position().row == next_node.start_position().row
        } else {
            false
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

        BlockCommentMetadata {
            has_leading_content,
            has_trailing_content,
            has_newline_above,
            has_newline_below,
        }
    }
}
