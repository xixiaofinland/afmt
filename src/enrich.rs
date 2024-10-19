use std::fmt::Debug;
use tree_sitter::Node;

use crate::{child::Accessor, config::Config, utility::is_processed};

pub trait RichNode: Debug {
    fn enrich(&mut self, shape: &mut EShape, context: &EContext, comments: &mut Vec<Comment>);
    //fn enrich_comments(&mut self);
    //fn enrich_data(&mut self);
    //fn rewrite(&mut self) -> String;
}

#[derive(Debug)]
pub struct ClassNode<'a, 'tree> {
    pub inner: &'a Node<'tree>,
    pub field_name: Option<String>, // Stores the field_name from ts-apex API
    pub comments: CommentBuckets,
    pub children: Vec<Box<dyn RichNode>>,
    pub format_info: FormatInfo,
}

#[derive(Debug, Default)]
struct FormatInfo {
    pub rewritten: String, // The raw printed result without wrapping
    pub wrappable: bool,
    pub indent_level: usize,
    pub force_break_before: bool,
    pub force_break_after: bool,
    //pub offset: usize,
}

#[derive(Debug, Default)]
struct CommentBuckets {
    pub pre_comments: Vec<Comment>,
    //pub inline_comments: Vec<Comment>,
    pub post_comments: Vec<Comment>,
}

#[derive(Debug)]
pub struct Comment {
    pub id: usize,
    pub content: String,
    pub comment_type: CommentType,
    pub is_processed: bool,
}

impl Comment {
    pub fn from_node(inner: &Node, context: &EContext) -> Self {
        let id = inner.id();
        let content = inner.v(&context.source_code).to_string();
        Self {
            id,
            content,
            is_processed: false,
            comment_type: match inner.kind() {
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

impl<'a, 'tree> ClassNode<'a, 'tree> {
    pub fn new(inner: &'a Node<'tree>) -> Self {
        Self {
            inner,
            comments: CommentBuckets::default(),
            children: Vec::new(),
            field_name: None,
            format_info: FormatInfo::default(),
        }
    }
}

impl<'a, 'tree> RichNode for ClassNode<'a, 'tree> {
    fn enrich(&mut self, shape: &mut EShape, context: &EContext, comments: &mut Vec<Comment>) {
        self.enrich_comments(comments, context);
        self.enrich_data(shape, context);
    }
}

impl<'a, 'tree> ClassNode<'a, 'tree> {
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
        self.format_info.rewritten = self.rewrite(shape, context);
        self.format_info.wrappable = true;
        self.format_info.indent_level = shape.indent_level;
        self.format_info.force_break_before = false;
        self.format_info.force_break_after = true;
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

        if let Some(ref c) = node.try_c_by_n("type_parameters") {
            //result.push_str(&rewrite_shape::<TypeParameters>(c, shape, false, context));
        }

        if let Some(ref c) = node.try_c_by_n("superclass") {
            //result.push_str(&rewrite_shape::<SuperClass>(c, shape, false, context));
        }

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

        //result.push_str(&format!("{}}}", shape.indent.as_string(context.config)));
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
