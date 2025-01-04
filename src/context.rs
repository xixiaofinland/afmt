use std::{cell::Cell, collections::HashMap};
use tree_sitter::{Node, Range};

use crate::{
    accessor::Accessor,
    data_model::DocBuild,
    doc::DocRef,
    doc_builder::DocBuilder,
    utility::{is_bracket_composite_node, panic_unknown_node},
};

pub type CommentMap = HashMap<usize, CommentBucket>;

#[derive(Debug)]
pub struct NodeInfo {
    pub id: usize,
    //pub range: Range,
}

impl NodeInfo {
    pub fn from(node: &Node) -> Self {
        Self {
            id: node.id(),
            //range: node.range(),
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

#[derive(Debug, PartialEq)]
pub enum CommentType {
    Line,
    Block,
}

#[derive(Debug)]
pub struct Comment {
    //pub id: usize,
    pub value: String,
    pub comment_type: CommentType,
    pub metadata: CommentMetadata,
    pub is_printed: Cell<bool>,
}

impl Comment {
    pub fn from_node(node: Node) -> Self {
        //let id = node.id();
        let value = node.value().trim_end().to_string();
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
            //id,
            value,
            comment_type,
            metadata,
            is_printed: Cell::new(false),
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

    pub fn is_followed_by_bracket_composite_node(&self) -> bool {
        self.metadata.is_followed_by_bracket_composite_node
    }

    pub fn has_newline_below(&self) -> bool {
        self.metadata.has_newline_below
    }

    pub fn has_prev_node(&self) -> bool {
        self.metadata.has_prev_node
    }

    pub fn mark_as_printed(&self) {
        self.is_printed.set(true);
    }

    pub fn is_printed(&self) -> bool {
        self.is_printed.get()
    }
}

impl<'a> DocBuild<'a> for Comment {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self.comment_type {
            CommentType::Line => {
                result.push(b.txt(&self.value));
            }
            CommentType::Block => {
                let lines: &Vec<&str> = &self.value.split('\n').collect();
                for (i, line) in lines.iter().enumerate() {
                    result.push(b.txt(line.trim()));

                    if i < lines.len() - 1 {
                        result.push(b.nl());
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct CommentMetadata {
    has_leading_content: bool,
    has_trailing_content: bool,
    has_newline_above: bool,
    has_newline_below: bool,
    has_prev_node: bool,
    is_followed_by_bracket_composite_node: bool,
    pub is_line_comment_and_need_newline: bool,
}

impl CommentMetadata {
    pub fn from(node: &Node, comment_type: CommentType) -> Self {
        let prev = node.prev_named_sibling();
        let next = node.next_named_sibling();

        let has_prev_node = prev.is_some();

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
            node.end_position().row < next_node.start_position().row.saturating_sub(1)
        } else {
            false
        };

        let is_followed_by_bracket_composite_node = if let Some(next_node) = next {
            is_bracket_composite_node(&next_node)
        } else {
            false
        };

        let is_line_comment_and_need_newline =
            Self::is_line_comment_and_need_newline(node, comment_type);

        CommentMetadata {
            has_leading_content,
            has_trailing_content,
            has_newline_above,
            has_newline_below,
            has_prev_node,
            is_followed_by_bracket_composite_node,
            is_line_comment_and_need_newline,
        }
    }

    fn is_line_comment_and_need_newline(node: &Node, comment_type: CommentType) -> bool {
        if comment_type != CommentType::Line {
            return false;
        }

        let parent_node = match node.parent() {
            Some(parent) => parent,
            None => return false,
        };

        if is_bracket_composite_node(&parent_node) || parent_node.kind() == "parser_output" {
            return false;
        }

        if let Some(prev) = node.prev_named_sibling() {
            if prev.kind() == "annotation" {
                return false;
            }
        }

        if let Some(next_node) = node.next_sibling() {
            return true;
        }

        false
    }
}
