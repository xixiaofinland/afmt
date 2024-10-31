use crate::{accessor::Accessor, enum_def::*};
use colored::Colorize;
use std::fmt::Debug;
use tree_sitter::Node;

#[derive(Debug)]
pub struct ClassDeclaration {
    pub buckets: Option<CommentBuckets>,
    modifiers: Option<Modifiers>,
    name: String,
    body: ClassBody,
}

impl ClassDeclaration {
    pub fn new(node: Node, source_code: &str, indent: usize) -> Self {
        let buckets = None;

        let modifiers = if let Some(m) = node.try_c_by_k("modifiers") {
            Some(Modifiers::new(m, source_code))
        } else {
            None
        };

        let name = node.cvalue_by_n("name", source_code);
        let body = ClassBody::new(node.c_by_n("body"), source_code, indent + 1);

        Self {
            buckets,
            modifiers,
            name,
            body,
        }
    }
}

#[derive(Debug, Default)]
pub struct Modifiers {
    //pub buckets: CommentBuckets,
    annotation: Option<Annotation>,
    modifiers: Vec<Modifier>,
}

impl Modifiers {
    pub fn new(node: Node, source_code: &str) -> Self {
        let mut modifiers = Self::default();

        for c in node.children_vec() {
            match c.kind() {
                "annotation" => {
                    modifiers.annotation = Some(Annotation {
                        name: c.value(source_code),
                    });
                }
                "modifier" => {
                    modifiers.modifiers.push(Modifier {
                        name: c.value(source_code),
                    });
                }
                "line_comment" | "block_comment" => continue,
                _ => panic!("## unknown node: {} in Modifiers", c.kind().red()),
            }
        }

        modifiers
    }
}

#[derive(Debug)]
pub struct Modifier {
    //pub buckets: CommentBuckets,
    pub name: String,
}

#[derive(Debug)]
pub struct Annotation {
    pub name: String,
}

#[derive(Debug)]
struct ClassBody {
    declarations: Vec<ClassMember>,
}

impl ClassBody {
    pub fn new(node: Node, source_code: &str, indent: usize) -> Self {
        let mut declarations: Vec<ClassMember> = Vec::new();

        for c in node.children_vec() {
            match c.kind() {
                "field_declaration" => declarations.push(ClassMember::Field(
                    FieldDeclaration::new(c, source_code, indent + 1),
                )),
                "line_comment" | "block_comment" => continue,
                _ => panic!("## unknown node: {} in ClassBody ", c.kind().red()),
            }
        }

        Self { declarations }
    }
}

#[derive(Debug)]
pub struct FieldDeclaration {
    pub buckets: Option<CommentBuckets>,
    pub modifiers: Option<Modifiers>,
    pub type_value: String,
}

impl FieldDeclaration {
    pub fn new(node: Node, source_code: &str, indent: usize) -> Self {
        let buckets = None;

        let modifiers = if let Some(m) = node.try_c_by_k("modifiers") {
            Some(Modifiers::new(m, source_code))
        } else {
            None
        };

        let type_value = node.cvalue_by_n("type", source_code);

        Self {
            buckets,
            modifiers,
            type_value,
        }
    }
}

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
    pub fn from_node(node: Node, source_code: &str) -> Self {
        let id = node.id();
        let content = node.v(source_code).to_string();
        Self {
            id,
            content,
            is_processed: false,
            comment_type: match node.kind() {
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
//pub enum ASTNode<'a> {
//    ClassNode(ClassNode<'a>),
//    Modifiers(Modifiers<'a>),
//}
