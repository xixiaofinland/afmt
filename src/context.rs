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
}

impl<'a> DocBuild<'a> for Comment {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        result.push(b.txt(&self.value));
        result.push(b.nl());
    }
}

#[derive(Debug)]
pub struct LineCommentMetadata {
    pub is_inline: bool,
    pub has_newline_above: bool,
    pub has_newline_below: bool,
}

impl LineCommentMetadata {
    pub fn from(node: &Node) -> Self {
        let prev = node.prev_named_sibling();
        let next = node.next_named_sibling();

        let is_inline = if let Some(prev_node) = prev {
            prev_node.end_position().row == node.start_position().row
        } else {
            false
        };

        let has_newline_above = if let Some(prev_node) = prev {
            prev_node.end_position().row < node.start_position().row
        } else {
            node.start_position().row > 0
        };

        let has_newline_below = if let Some(next_node) = next {
            node.end_position().row < next_node.start_position().row
        } else {
            // Depending on your use case, you can set this to `false` or perform additional checks.
            false
        };

        LineCommentMetadata {
            is_inline,
            has_newline_above,
            has_newline_below,
        }
    }
}

#[derive(Debug)]
pub struct BlockCommentMetadata {
    pub is_inline: bool,
    pub has_newline_above: bool,
    pub has_newline_below: bool,
    pub has_trailing_content: bool,
}

impl BlockCommentMetadata {
    /// Creates a `BlockCommentMetadata` instance from a given block comment `node`.
    pub fn from(node: &Node) -> Self {
        // Retrieve the previous and next named siblings of the current node.
        let prev = node.prev_named_sibling();
        let next = node.next_named_sibling();

        // Determine if the comment is inline.
        // A comment is considered inline if either:
        // - The previous node ends on the same line as the comment starts.
        // - The next node starts on the same line as the comment ends.
        let is_inline = if let Some(prev_node) = prev {
            prev_node.end_position().row == node.start_position().row
        } else if let Some(next_node) = next {
            next_node.start_position().row == node.end_position().row
        } else {
            false
        };

        // Determine if there's a newline above the comment.
        // This is true if the previous node ends on a line before the comment starts.
        let has_newline_above = if let Some(prev_node) = prev {
            prev_node.end_position().row < node.start_position().row
        } else {
            // If there's no previous node, check if the comment doesn't start on the first line.
            node.start_position().row > 0
        };

        // Determine if there's a newline below the comment.
        // This is true if the comment ends on a line before the next node starts.
        let has_newline_below = if let Some(next_node) = next {
            node.end_position().row < next_node.start_position().row
        } else {
            // If there's no next node, we might not have enough information.
            // Depending on your use case, you can set this to `false` or perform additional checks.
            false
        };

        // Determine if there's trailing content after the comment on the same line.
        // Instead of inspecting the source code bytes, we check if the next node starts on the same row.
        let has_trailing_content = if let Some(next_node) = next {
            next_node.start_position().row == node.end_position().row
        } else {
            false
        };

        BlockCommentMetadata {
            is_inline,
            has_newline_above,
            has_newline_below,
            has_trailing_content,
        }
    }
}
