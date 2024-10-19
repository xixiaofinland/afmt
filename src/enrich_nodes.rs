use std::fmt::Debug;
use tree_sitter::Node;

use crate::child::Accessor;
use crate::config::Config;
use crate::enrich::*;
use crate::utility::*;

#[derive(Debug)]
pub struct Modifiers<'a, 'tree> {
    pub inner: &'a Node<'tree>,
    pub comments: CommentBuckets,
    pub children: Vec<Box<dyn RichNode>>,
    pub format_info: FormatInfo,
    //pub field_name: Option<String>, // Stores the field_name from ts-apex API
}

impl<'a, 'tree> RichNode for Modifiers<'a, 'tree> {
    fn enrich(&mut self, shape: &mut EShape, context: &EContext, comments: &mut Vec<Comment>) {
        self.enrich_comments(comments, context);
        self.enrich_data(shape, context);
    }
}

impl<'a, 'tree> Modifiers<'a, 'tree> {
    pub fn new(inner: &'a Node<'tree>) -> Self {
        Self {
            inner,
            comments: CommentBuckets::default(),
            children: Vec::new(),
            format_info: FormatInfo::default(),
            //field_name: None,
        }
    }

    fn enrich_comments(&mut self, comments: &mut Vec<Comment>, context: &EContext) {
        let mut prev_sibling = self.inner.prev_sibling();
        while let Some(node) = prev_sibling {
            if node.is_comment() && !is_processed(node.id(), comments) {
                self.comments
                    .pre_comments
                    .push(Comment::from_node(&node, context));
            }
            prev_sibling = node.prev_sibling();
        }

        let mut next_sibling = self.inner.next_sibling();
        while let Some(node) = next_sibling {
            if node.is_comment() && !is_processed(node.id(), comments) {
                self.comments
                    .post_comments
                    .push(Comment::from_node(&node, context));
            }
            next_sibling = node.next_sibling();
        }
    }

    fn enrich_data(&mut self, shape: &mut EShape, context: &EContext) {
        let rewritten = self.rewrite(shape, context);
        let length = get_length_before_brace(&rewritten);

        self.format_info = FormatInfo {
            rewritten,
            length,
            wrappable: false,
            indent_level: shape.indent_level,
            //force_break_before: false,
            force_break_after: false,
            has_new_line_before: false,
        };
    }

    fn rewrite(&self, shape: &mut EShape, context: &EContext) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);

        if let Some(ref a) = node.try_c_by_k("modifiers") {
            //result.push_str(&rewrite::<Modifiers>(a, shape, context));

            if let Some(_) = a.try_c_by_k("modifier") {
                result.push(' ');
            }
        }

        result.push_str("class ");
        result.push_str(node.cv_by_n("name", source_code));

        //if let Some(ref c) = node.try_c_by_n("type_parameters") {
        //    //result.push_str(&rewrite_shape::<TypeParameters>(c, shape, false, context));
        //}

        //if let Some(ref c) = node.try_c_by_n("superclass") {
        //    //result.push_str(&rewrite_shape::<SuperClass>(c, shape, false, context));
        //}

        if let Some(ref c) = node.try_c_by_n("interfaces") {
            //result.push_str(&rewrite_shape::<Interfaces>(c, shape, false, context));
        }

        result.push_str(" {\n");

        let body_node = node.c_by_n("body");
        //result.push_str(&body_node.apply_to_standalone_children(
        //    shape,
        //    context,
        //    |c, c_shape, c_context| c._visit(c_shape, c_context),
        //));

        result
    }

    pub fn prepare<'b>(
        &self,
        context: &'b EContext,
    ) -> (&'a Node<'tree>, String, &'b str, &'b Config) {
        let node = self.inner;
        let result = String::new();
        let source_code = &context.source_code;
        let config = &context.config;
        (node, result, source_code, config)
    }
}
