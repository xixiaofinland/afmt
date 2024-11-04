use crate::{accessor::Accessor, doc::DocRef, doc_builder::DocBuilder, enum_def::*};
use colored::Colorize;
use std::fmt::Debug;
use tree_sitter::{Node, Range};

pub trait DocBuild<'a> {
    fn build(&self, b: &'a DocBuilder<'a>) -> DocRef<'a> {
        let mut result: Vec<DocRef<'a>> = Vec::new();
        self.build_inner(b, &mut result);
        b.concat(result)
    }

    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>);
}

#[derive(Debug)]
pub struct Root {
    pub class: Option<ClassDeclaration>,
}

impl<'a> DocBuild<'a> for Root {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        if let Some(ref n) = self.class {
            result.push(n.build(b));
        }
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
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        if let Some(ref n) = self.modifiers {
            result.push(n.build(b));
        }

        result.push(b.txt("class "));
        result.push(b.txt(&self.name));

        if self.body.declarations.is_empty() {
            result.push(b.txt(" {\n}\n"));
        } else {
            result.push(b.txt(" {"));
            result.push(b.nl());

            let body_doc = self.body.build(b);
            let indented_body = b.add_indent_level(body_doc);
            result.push(indented_body);

            result.push(b.nl());
            result.push(b.txt("}"));
        }
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

impl<'a> DocBuild<'a> for Modifiers {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        if let Some(ref n) = self.annotation {
            result.push(n.build(b));
        }

        if !self.modifiers.is_empty() {
            let modifiers_doc = b.build_docs(&self.modifiers);
            result.push(b.concat(modifiers_doc));
            result.push(b.space());
        }
    }
}

impl Modifiers {
    pub fn new(node: Node, source_code: &str) -> Self {
        let mut modifiers = Self::default();

        for c in node.children_vec() {
            match c.kind() {
                "annotation" => {
                    modifiers.annotation = Some(Annotation {
                        name: c.cvalue_by_n("name", source_code),
                    });
                }
                "modifier" => match c.first_c().kind() {
                    "public" => modifiers.modifiers.push(Modifier::Public),
                    _ => panic!("## unknown node: {} in Modifier", c.kind().red()),
                },
                "line_comment" | "block_comment" => continue,
                _ => panic!("## unknown node: {} in Modifiers", c.kind().red()),
            }
        }

        modifiers
    }
}

#[derive(Debug)]
pub struct Annotation {
    pub name: String,
}

impl<'a> DocBuild<'a> for Annotation {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        result.push(b.txt(format!("@{}", self.name)));
        result.push(b.nl());
    }
}

#[derive(Debug)]
pub struct ClassBody {
    pub declarations: Vec<ClassMember>,
}

impl<'a> DocBuild<'a> for ClassBody {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        let member_docs = b.build_docs(&self.declarations);
        let body_doc = b.sep_multi_line(&member_docs, "");
        result.push(body_doc);
    }
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

impl<'a> DocBuild<'a> for FieldDeclaration {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        if let Some(ref n) = self.modifiers {
            result.push(n.build(b));
        }

        result.push(self.type_.build(b));
        result.push(b.space());

        let decl_docs = b.build_docs(&self.declarators);

        let declarators_doc = b.separated_choice(&decl_docs, ", ", ", ");
        result.push(declarators_doc);

        result.push(b.txt(";"));
    }
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

impl<'a> DocBuild<'a> for VariableDeclarator {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        result.push(b.txt(&self.name));
        result.push(b.txt(" = "));
        if let Some(ref v) = self.value {
            result.push(v.build(b));
        }
    }
}

impl VariableDeclarator {
    pub fn new(node: Node, source_code: &str, indent: usize) -> Self {
        let name = node.cvalue_by_n("name", source_code);

        let value = node.try_c_by_n("value").map(|v| match v.kind() {
            //"array_initializer" => {
            //    VariableInitializer::ArrayInitializer(ArrayInitializer::new(v, source_code, indent))
            //}
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
