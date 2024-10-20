use std::fmt::Debug;
use tree_sitter::Node;

use crate::child::Accessor;
use crate::config::Config;
use crate::enrich::*;
use crate::utility::*;

#[derive(Debug)]
pub enum ASTNode<'t> {
    ClassNode(ClassNode<'t>),
    Modifiers(Modifiers<'t>),
}

#[derive(Debug)]
pub struct Modifiers<'t> {
    pub inner: Node<'t>,
    pub content: String, // The raw printed result without wrapping
    pub buckets: CommentBuckets,
    pub children: Vec<ASTNode<'t>>,
    pub format_info: FormatInfo,
}

impl<'t> RichNode for Modifiers<'t> {
    fn enrich(&mut self, shape: &mut EShape, context: &EContext) {
        self.enrich_comments(shape, context);
        self.enrich_data(shape, context);
    }
}

impl<'t> Modifiers<'t> {
    pub fn build(inner: Node<'t>, shape: &mut EShape, context: &EContext) -> Self {
        let mut n = Self {
            inner,
            content: String::new(),
            buckets: CommentBuckets::default(),
            children: Vec::new(),
            format_info: FormatInfo::default(),
        };
        n.enrich(shape, context);
        n
    }

    fn enrich_comments(&mut self, shape: &mut EShape, context: &EContext) {
        let mut prev_sibling = self.inner.prev_sibling();
        while let Some(node) = prev_sibling {
            if node.is_comment() {
                let comment_id = node.id();
                if let Some(comment) = shape.comments.iter_mut().find(|c| c.id == comment_id) {
                    if !comment.is_processed {
                        self.buckets
                            .pre_comments
                            .push(Comment::from_node(&node, context));
                        comment.is_processed = true;
                    }
                } else {
                    self.buckets
                        .pre_comments
                        .push(Comment::from_node(&node, context));
                }
            }
            prev_sibling = node.prev_sibling();
        }

        let mut next_sibling = self.inner.next_sibling();
        while let Some(node) = next_sibling {
            if node.is_comment() {
                let comment_id = node.id();
                if let Some(comment) = shape.comments.iter_mut().find(|c| c.id == comment_id) {
                    if !comment.is_processed {
                        self.buckets
                            .post_comments
                            .push(Comment::from_node(&node, context));
                        comment.is_processed = true;
                    }
                } else {
                    self.buckets
                        .post_comments
                        .push(Comment::from_node(&node, context));
                }
            }
            next_sibling = node.next_sibling();
        }
    }

    fn enrich_data(&mut self, shape: &mut EShape, context: &EContext) {
        self.content = self.rewrite(shape, context);
        let offset = get_length_before_brace(&self.content);

        self.format_info = FormatInfo {
            offset,
            wrappable: false,
            indent_level: shape.indent_level,
            //force_break_before: false,
            force_break_after: false,
            has_new_line_before: false,
        };
    }

    fn rewrite(&mut self, shape: &mut EShape, context: &EContext) -> String {
        let (node, mut result, source_code, config, children) = self.prepare(context);
        result
    }

    pub fn prepare<'a>(
        &mut self,
        context: &'a EContext,
    ) -> (
        &Node<'t>,
        String,
        &'a str,
        &'a Config,
        &mut Vec<ASTNode<'t>>,
    ) {
        let node = &self.inner;
        let result = String::new();
        let source_code = context.source_code.as_str();
        let config = &context.config;
        (node, result, source_code, config, &mut self.children)
    }
}
