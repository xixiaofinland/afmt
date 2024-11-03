use crate::{
    accessor::Accessor, config::FmtContext, doc::DocRef, doc_builder::DocBuilder, enum_def::*,
};
use colored::Colorize;
use std::fmt::Debug;
use tree_sitter::{Node, Range};

pub trait DocBuild<'a> {
    fn build(&self, b: &'a DocBuilder<'a>) -> DocRef<'a>;
}

#[derive(Debug)]
pub struct Root {
    pub class: Option<ClassDeclaration>,
}

impl<'a> DocBuild<'a> for Root {
    fn build(&self, b: &'a DocBuilder<'a>) -> DocRef<'a> {
        let mut result: Vec<DocRef<'a>> = Vec::new();
        if let Some(ref c) = self.class {
            result.push(c.build(b));
        }
        b.concat(result)
    }
}

impl Root {
    pub fn new(node: Node, source_code: &str) -> Self {
        let class = node
            .try_c_by_k("class_declaration")
            .map(|n| ClassDeclaration::new(n, source_code, 0));

        Self { class }
    }
}

#[derive(Debug)]
pub struct ClassDeclaration {
    pub buckets: Option<CommentBuckets>,
    pub modifiers: Option<Modifiers>,
    pub name: String,
    pub body: ClassBody,
    pub range: Range,
}

impl<'a> DocBuild<'a> for ClassDeclaration {
    fn build(&self, b: &'a DocBuilder<'a>) -> DocRef<'a> {
        b.txt("class")
    }
}

impl ClassDeclaration {
    pub fn new(node: Node, source_code: &str, indent: usize) -> Self {
        let buckets = None;

        let modifiers = node
            .try_c_by_k("modifiers")
            .map(|m| Modifiers::new(m, source_code));

        let name = node.cvalue_by_n("name", source_code);
        let body = ClassBody::new(node.c_by_n("body"), source_code, indent + 1);

        Self {
            buckets,
            modifiers,
            name,
            body,
            range: node.range(),
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
pub struct ClassBody {
    declarations: Vec<ClassMember>,
}

impl ClassBody {
    pub fn new(node: Node, source_code: &str, indent: usize) -> Self {
        let mut declarations: Vec<ClassMember> = Vec::new();

        for c in node.children_vec() {
            match c.kind() {
                "field_declaration" => declarations.push(ClassMember::Field(Box::new(
                    FieldDeclaration::new(c, source_code, indent + 1),
                ))),
                "class_declaration" => declarations.push(ClassMember::NestedClass(Box::new(
                    ClassDeclaration::new(c, source_code, indent + 1),
                ))),
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
    pub type_: UnnanotatedType,
    pub declarators: Vec<VariableDeclarator>,
    pub range: Range,
}

impl FieldDeclaration {
    pub fn new(node: Node, source_code: &str, indent: usize) -> Self {
        let buckets = None;

        let modifiers = node
            .try_c_by_k("modifiers")
            .map(|n| Modifiers::new(n, source_code));

        let type_node = node.c_by_n("type");
        let type_ = match type_node.kind() {
            "type_identifier" => UnnanotatedType::Identifier(Identifier {
                value: type_node.value(source_code),
            }),
            _ => panic!(
                "## unknown node: {} in FieldDeclaration ",
                type_node.kind().red()
            ),
        };

        let declarators = node
            .cs_by_n("declarator")
            .into_iter()
            .map(|n| VariableDeclarator::new(n, source_code, indent))
            .collect();

        Self {
            buckets,
            modifiers,
            type_,
            declarators,
            range: node.range(),
        }
    }
}

#[derive(Debug)]
pub struct VariableDeclarator {
    pub name: String,
    pub value: Option<VariableInitializer>,
}

impl VariableDeclarator {
    pub fn new(node: Node, source_code: &str, indent: usize) -> Self {
        let name = node.cvalue_by_n("name", source_code);

        let value = node.try_c_by_n("value").map(|v| match v.kind() {
            "array_initializer" => {
                VariableInitializer::ArrayInitializer(ArrayInitializer::new(v, source_code, indent))
            }
            _ => VariableInitializer::Expression(Expression::Primary(
                PrimaryExpression::Identifier(Identifier {
                    value: v.value(source_code),
                }),
            )),
        });

        Self { name, value }
    }
}

#[derive(Debug, Default)]
pub struct ArrayInitializer {
    variable_initializers: Vec<VariableInitializer>,
}

impl ArrayInitializer {
    pub fn new(node: Node, source_code: &str, indent: usize) -> Self {
        ArrayInitializer::default()
    }
}

#[derive(Debug, Default)]
pub struct AssignmentExpression {
    pub left: String,
    pub op: String,
    pub right: String,
}

impl AssignmentExpression {
    pub fn new(node: Node, source_code: &str, indent: usize) -> Self {
        let left = node.cvalue_by_n("left", source_code);
        let op = node.cvalue_by_n("operator", source_code);
        let right = node.cvalue_by_n("right", source_code);
        Self { left, op, right }
    }
}

#[derive(Debug)]
pub struct Identifier {
    pub value: String,
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
    pub range: Range,
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
            range: node.range(),
        }
    }
}

#[derive(Debug)]
pub enum CommentType {
    Line,
    Block,
}
