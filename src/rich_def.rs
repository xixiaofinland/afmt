use crate::accessor::Accessor;
use crate::config::FmtContext;
use std::fmt::Debug;
use tree_sitter::Node;

#[derive(Debug)]
pub struct ClassDeclaration<'t> {
    pub node: Node<'t>,
    pub source_code: &'t str,
    pub buckets: CommentBuckets,
    //annotation: Annotation,
    modifiers: Option<Modifiers<'t>>,
    name: &'t str,
    //body: ClassBody<'t>,
}

impl<'t> ClassDeclaration<'t> {
    fn from_node(node: Node<'t>, source_code: &'t str) -> Self {
        let modifiers = if let Some(m) = node.try_c_by_k("modifiers") {
            Some(Modifiers {
                node: m,
                source_code,
            })
        } else {
            None
        };

        let buckets = CommentBuckets::default();
        let name = node.cv_by_n("name", source_code);

        Self {
            node,
            source_code,
            buckets,
            modifiers,
            name,
        }
    }
}

#[derive(Debug)]
pub struct Modifiers<'t> {
    pub node: Node<'t>,
    pub source_code: &'t str,
    //pub buckets: CommentBuckets,
    //modifiers: Vec<Modifier<'t>>,
}

impl<'t> Modifiers<'t> {
    fn from_node(node: &Node, source_code: &'t str) -> Self {
        Self { node, source_code }
    }
}

//impl<'t> Modifiers<'t> {
//    fn from_node(node: &Node, source_code: &'t str) -> Self {
//        if let Some(m) = node.try_c_by_k("modifier") {
//
//            let superclass = if node.child_by_field_name("superclass").is_some() {
//            Some(Superclass {
//                extends_keyword: "extends".to_string(),
//                superclass_type: Type::from_node(node.child_by_field_name("_type").unwrap()),
//            })
//        } else {
//            None
//        };
//
//            result.push_str(&rewrite::<Modifiers>(a, shape, context));
//
//            if let Some(_) = a.try_c_by_k("modifier") {
//                result.push(' ');
//            }
//        }
//
//        Self {}
//    }
//}

pub struct Modifier<'t> {
    pub node: Node<'t>,
    pub name: &'t str,
    pub buckets: CommentBuckets,
}

impl<'t> Modifier<'t> {}

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
