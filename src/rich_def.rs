use crate::accessor::Accessor;
use crate::config::FmtContext;
use crate::doc::Doc;
use std::fmt::Debug;
use tree_sitter::Node;

pub struct ClassDeclaration<'t> {
    pub node: Node<'t>,
    pub source_code: &'t str,
    pub buckets: CommentBuckets,
}

//pub trait RichNode: Debug {
//    fn enrich(&mut self, shape: &mut EShape, context: &EContext);
//}

//#[derive(Debug, Default)]
//pub struct FormatInfo {
//    pub offset: usize, // Used in complex nodes (like Class, Method) to decide wrapping
//    pub wrappable: bool,
//    pub indent_level: usize,
//    pub force_break_after: bool,
//    pub has_new_line_before: bool,
//}

#[derive(Debug, Default)]
pub struct CommentBuckets {
    pub pre_comments: Vec<Comment>,
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
    pub fn from_node(inner: Node, context: &FmtContext) -> Self {
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
pub enum CommentType {
    Line,
    Block,
}

//rich_struct!(ClassNode, Modifiers);

//#[derive(Debug)]
//pub enum ASTNode<'t> {
//    ClassNode(ClassNode<'t>),
//    Modifiers(Modifiers<'t>),
//}
