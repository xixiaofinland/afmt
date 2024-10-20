use std::fmt::Debug;
use tree_sitter::Node;

use crate::{
    child::Accessor, config::Config, enrich_nodes::Modifiers, utility::get_length_before_brace,
};

pub trait RichNode: Debug {
    fn enrich(&mut self, shape: &mut EShape, context: &EContext);
}

#[derive(Debug)]
pub enum ASTNode<'t> {
    ClassNode(ClassNode<'t>),
    Modifiers(Modifiers<'t>),
}

#[derive(Debug, Default)]
pub struct FormatInfo {
    pub rewritten: String, // The raw printed result without wrapping
    pub length: usize,     // Used in complex nodes (like Class, Method) to decide wrapping
    pub wrappable: bool,
    pub indent_level: usize,
    //pub force_break_before: bool,
    pub force_break_after: bool,   // should add `\n` at the end;
    pub has_new_line_before: bool, // whether the prevous source line is an empty `\n`;
}

#[derive(Debug, Default)]
pub struct CommentBuckets {
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

#[derive(Debug, Default)]
pub struct EShape {
    pub indent_level: usize,
    pub comments: Vec<Comment>,
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

#[derive(Debug)]
pub struct ClassNode<'t> {
    pub inner: Node<'t>,
    pub buckets: CommentBuckets,
    pub children: Vec<ASTNode<'t>>,
    pub format_info: FormatInfo,
}

impl<'t> RichNode for ClassNode<'t> {
    fn enrich(&mut self, shape: &mut EShape, context: &EContext) {
        self.enrich_comments(shape, context);
        self.enrich_data(shape, context);
    }
}

impl<'t> ClassNode<'t> {
    pub fn new(inner: Node<'t>) -> Self {
        Self {
            inner,
            buckets: CommentBuckets::default(),
            children: Vec::new(),
            format_info: FormatInfo::default(),
        }
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
        let rewritten = self.rewrite(shape, context);
        let length = get_length_before_brace(&rewritten);

        self.format_info = FormatInfo {
            rewritten,
            length,
            wrappable: true,
            indent_level: shape.indent_level,
            //force_break_before: false,
            force_break_after: true,
            has_new_line_before: false,
        };
    }

    fn rewrite(&mut self, shape: &mut EShape, context: &EContext) -> String {
        let (node, mut result, source_code, config, children) = self.prepare(context);

        if let Some(c) = node.try_c_by_k("modifiers") {
            let mut modifiers = Modifiers::new(c);
            modifiers.enrich(shape, context);
            result.push_str(&modifiers.format_info.rewritten);
            children.push(ASTNode::Modifiers(modifiers));
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
