use std::{cell::Cell, collections::HashMap};
use tree_sitter::Node;

use crate::{
    accessor::Accessor,
    data_model::DocBuild,
    doc::DocRef,
    doc_builder::DocBuilder,
    utility::{
        get_comment_bucket, is_bracket_composite_node, is_punctuation_node, panic_unknown_node,
    },
};

pub type CommentMap = HashMap<usize, CommentBucket>;

#[derive(Debug)]
pub struct NodeContext {
    pub id: usize,
    pub punc: Option<Punctuation>,
}

impl NodeContext {
    // Create an instance with punctuation extracted from the following node.
    pub fn with_punctuation(node: &Node) -> Self {
        Self {
            id: node.id(),
            punc: Punctuation::from(node),
        }
    }

    // Create an instance with punctuation extracted from an inner node.
    pub fn with_inner_punctuation(node: &Node) -> Self {
        Self {
            id: node.id(),
            punc: Punctuation::from_inner(node),
        }
    }

    // Create an instance without punctuation consideration.
    pub fn without_punctuation(node: &Node) -> Self {
        Self {
            id: node.id(),
            punc: None,
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

#[derive(Debug, PartialEq, Clone)]
pub enum CommentType {
    Line,
    Block,
}

#[derive(Debug, Clone)]
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
        self.metadata.has_empty_line_above
    }

    pub fn is_followed_by_bracket_composite_node(&self) -> bool {
        self.metadata.is_followed_by_bracket_composite_node
    }

    pub fn has_newline_below(&self) -> bool {
        self.metadata.has_empty_line_below
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
                result.push(b.nl());
            }
            CommentType::Block => {
                let mut lines: Vec<&str> = self.value.split('\n').collect();

                // JavaDoc formatting
                if self.value.starts_with("/**") {
                    // Handle the first line that might contain both /** and content
                    if !lines.is_empty() {
                        let first_line = lines[0].trim();
                        if first_line.len() > 3 {
                            // Has content after /**
                            // Extract the content after /**
                            let content = first_line[3..].trim();
                            if !content.is_empty() {
                                // Replace first line with just /**
                                lines[0] = "/**";
                                // Insert the content as a new second line
                                lines.insert(1, content);
                            }
                        }
                    }

                    for (i, line) in lines.iter().enumerate() {
                        let trimmed = line.trim();

                        if i == 0 {
                            // First line (/**) remains unchanged
                            result.push(b.txt(trimmed));
                        } else if i == lines.len() - 1 {
                            if let Some(before_end) = trimmed.strip_suffix("*/") {
                                let content = before_end.trim();
                                if content.is_empty() {
                                    result.push(b.txt(" */"));
                                } else {
                                    result.push(b.txt(format!(" * {}", content))); // First line: Preserve content
                                    result.push(b.nl()); // Newline before closing */
                                    result.push(b.txt(" */")); // Second line: Properly close the comment
                                }
                            } else {
                                result.push(b.txt(" */")); // Fallback (shouldn't happen in valid JavaDoc)
                            }
                        } else if trimmed.is_empty() {
                            // Handle empty lines
                            result.push(b.txt(" *"));
                        } else {
                            // Handle content lines
                            if let Some(after_star) = trimmed.strip_prefix('*') {
                                // Line has a star - normalize to exactly one space
                                let content = after_star.trim_start(); // Remove all leading spaces
                                if content.is_empty() {
                                    // Just a star with no content
                                    result.push(b.txt(" *"));
                                } else {
                                    // Star with content - add exactly one space
                                    result.push(b.txt(format!(" * {}", content)));
                                }
                            } else {
                                // Line doesn't have a star, add standard formatting
                                result.push(b.txt(format!(" * {}", trimmed)));
                            }
                        }

                        if i < lines.len() - 1 {
                            result.push(b.nl());
                        }
                    }
                } else {
                    // Regular block comment (non-JavaDoc)
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
}

#[derive(Debug, Clone)]
pub struct CommentMetadata {
    has_leading_content: bool,
    has_trailing_content: bool,
    has_empty_line_above: bool,
    has_empty_line_below: bool,
    has_prev_node: bool,
    is_followed_by_bracket_composite_node: bool,
}

impl CommentMetadata {
    pub fn from(node: &Node, comment_type: CommentType) -> Self {
        let prev = node.prev_named_sibling();
        let has_prev_node = prev.is_some();

        let has_leading_content = if let Some(prev_node) = prev {
            node.start_position().row == prev_node.end_position().row
        } else {
            false
        };

        let next = node.next_named_sibling();
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

        let has_empty_line_above = if let Some(prev_node) = prev {
            node.start_position().row > prev_node.end_position().row + 1
        } else {
            false
        };

        let has_empty_line_below = if let Some(next_node) = next {
            node.end_position().row < next_node.start_position().row.saturating_sub(1)
        } else {
            false
        };

        let is_followed_by_bracket_composite_node = if let Some(next_node) = next {
            is_bracket_composite_node(&next_node)
        } else {
            false
        };

        CommentMetadata {
            has_leading_content,
            has_trailing_content,
            has_empty_line_above,
            has_empty_line_below,
            has_prev_node,
            is_followed_by_bracket_composite_node,
        }
    }
}

#[derive(Debug)]
pub struct Punctuation {
    pub type_: PuncuationType,
    pub id: usize,
}

impl Punctuation {
    pub fn new(node: Node) -> Self {
        match node.kind() {
            "," => Self {
                type_: PuncuationType::Comma,
                id: node.id(),
            },
            ";" => Self {
                type_: PuncuationType::Semicolon,
                id: node.id(),
            },
            _ => panic_unknown_node(node, "Puncuation"),
        }
    }

    // check if the node has a following punc
    pub fn from(node: &Node) -> Option<Self> {
        let mut current = *node;
        while let Some(next) = current.next_sibling() {
            if next.is_extra() {
                current = next;
                continue;
            }

            if is_punctuation_node(&next) {
                return Some(Self::new(next));
            }

            return None;
        }
        None
    }

    // when the punc node is within the checking node rather than the following separate node
    pub fn from_inner(node: &Node) -> Option<Self> {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if is_punctuation_node(&child) {
                return Some(Self::new(child));
            }
        }
        None
    }
}

impl<'a> DocBuild<'a> for Punctuation {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        // TODO: merge into normal bucket handling utiltiy method?

        let bucket = get_comment_bucket(&self.id);

        // Separate line comments and block comments from pre_comments
        let (line_comments_in_pre, block_comments_in_pre): (Vec<Comment>, Vec<Comment>) = bucket
            .pre_comments
            .iter()
            .cloned()
            .partition(|comment| matches!(comment.comment_type, CommentType::Line));

        // Merge line_comments with post_comments
        let updated_post_comments: Vec<_> = line_comments_in_pre
            .into_iter()
            .chain(bucket.post_comments.iter().cloned())
            .collect();

        for comment in block_comments_in_pre {
            if comment.has_leading_content() {
                result.push(b.txt(" "));
            } else if comment.has_newline_above() {
                result.push(b.empty_new_line());
            } else {
                result.push(b.nl());
            }

            result.push(comment.build(b));

            if comment.has_trailing_content() && !comment.is_followed_by_bracket_composite_node() {
                result.push(b.txt(" "));
            }
        }

        match self.type_ {
            PuncuationType::Comma => result.push(b.txt(",")),
            PuncuationType::Semicolon => result.push(b.txt(";")),
        }

        for comment in updated_post_comments {
            if comment.has_leading_content() {
                result.push(b.txt(" "));
            } else if comment.has_newline_above() {
                result.push(b.empty_new_line());
            } else {
                result.push(b.nl());
            }

            result.push(comment.build(b));

            if comment.has_trailing_content() && !comment.is_followed_by_bracket_composite_node() {
                result.push(b.txt(" "));
            }
        }

        // we assume all associated comment nodes are handled
        // TODO: this is not ideal due to the comment map is currently defined read-only
        for comment in &bucket.pre_comments {
            comment.mark_as_printed();
        }
        for comment in &bucket.post_comments {
            comment.mark_as_printed();
        }
    }
}

#[derive(Debug)]
pub enum PuncuationType {
    Comma,
    Semicolon,
}
