use colored::Colorize;
use serde::Serialize;
use tree_sitter::Node;

use crate::{
    accessor::Accessor, data_model::*, doc::DocRef, doc_builder::DocBuilder, utility::source_code,
};

#[derive(Debug, Serialize)]
pub enum RootMember {
    Class(Box<ClassDeclaration>),
}

impl<'a> DocBuild<'a> for RootMember {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            RootMember::Class(class) => {
                result.push(class.build(b));
            }
        }
    }
}

#[derive(Debug, Serialize)]
pub enum ClassMember {
    Field(Box<FieldDeclaration>),
    NestedClass(Box<ClassDeclaration>),
    Method(Box<MethodDeclaration>),
    Block(Box<Block>),
    //Method(MethodDeclaration<'a>),
    //Interface(InterfaceDeclaration<'a>),
    //Enum(EnumDeclaration<'a>),
    //StaticInitializer(StaticInitializer<'a>),
    //Constructor(ConstructorDeclaration<'a>),
    //EmptyStatement, // Represents the ";" case
}

impl ClassMember {
    pub fn new(n: Node) -> Self {
        match n.kind() {
            "field_declaration" => Self::Field(Box::new(FieldDeclaration::new(n))),
            "class_declaration" => Self::NestedClass(Box::new(ClassDeclaration::new(n))),
            "method_declaration" => Self::Method(Box::new(MethodDeclaration::new(n))),
            "block" => Self::Block(Box::new(Block::new(n))),
            _ => panic!("## unknown node: {} in UnnanotatedType ", n.kind().red()),
        }
    }
}

impl<'a> DocBuild<'a> for ClassMember {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            Self::Field(field_decl) => {
                result.push(field_decl.build(b));
            }
            Self::NestedClass(class_decl) => {
                result.push(class_decl.build(b));
            }
            Self::Method(method) => {
                result.push(method.build(b));
            }
            Self::Block(block) => {
                result.push(block.build(b));
            }
        }
    }
}

//_unannotated_type: ($) => choice($._simple_type, $.array_type),
#[derive(Debug, Serialize)]
pub enum UnnanotatedType {
    Simple(SimpleType),
    //Array(ArrayType),
}

impl UnnanotatedType {
    pub fn new(n: Node) -> Self {
        match n.kind() {
            "type_identifier" => Self::Simple(SimpleType::Identifier(n.value(source_code()))),
            "void_type" => Self::Simple(SimpleType::Void(VoidType::new(n))),
            "generic_type" => Self::Simple(SimpleType::Generic(GenericType::new(n))),
            _ => panic!("## unknown node: {} in UnnanotatedType ", n.kind().red()),
        }
    }
}

impl<'a> DocBuild<'a> for UnnanotatedType {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            Self::Simple(s) => result.push(s.build(b)),
        }
    }
}

#[derive(Debug, Serialize)]
pub enum SimpleType {
    Identifier(String),
    Void(VoidType),
    Generic(GenericType),
}

impl<'a> DocBuild<'a> for SimpleType {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            Self::Identifier(i) => {
                result.push(b.txt(i));
            }
            Self::Void(v) => {
                result.push(b.txt(&v.value));
            }
            Self::Generic(g) => {
                result.push(g.build(b));
            }
        }
    }
}

#[derive(Debug, Serialize)]
pub enum VariableInitializer {
    Expression(Expression),
    //ArrayInitializer(ArrayInitializer),
}

impl<'a> DocBuild<'a> for VariableInitializer {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            Self::Expression(exp) => {
                result.push(exp.build(b));
            }
        }
    }
}

//expression: ($) =>
//  choice(
//    $.assignment_expression,
//    $.binary_expression,
//    $.instanceof_expression,
//    $.ternary_expression,
//    $.update_expression,
//    $.primary_expression,
//    $.unary_expression,
//    $.cast_expression,
//    $.dml_expression
//  ),

#[derive(Debug, Serialize)]
pub enum Expression {
    StringLiteral(String),
    Binary(Box<BinaryExpression>),
    Primary(Box<PrimaryExpression>),
    //Assignment(Box<AssignmentExpression>),
}

impl Expression {
    pub fn new(n: Node) -> Self {
        match n.kind() {
            "string_literal" => Self::StringLiteral(n.value(source_code())),
            "binary_expression" => Self::Binary(Box::new(BinaryExpression::new(n))),
            "boolean" | "identifier" | "null_literal" | "method_invocation" => {
                Self::Primary(Box::new(PrimaryExpression::new(n)))
            }
            _ => panic!("## unknown node: {} in Expression", n.kind().red()),
        }
    }
}

impl<'a> DocBuild<'a> for Expression {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            Self::StringLiteral(s) => {
                result.push(b.txt(s));
            }
            Self::Binary(binary) => {
                result.push(binary.build(b));
            }
            Self::Primary(p) => {
                result.push(p.build(b));
            }
        }
    }
}

//primary_expression: ($) =>
//  choice(
//    $._literal,
//    $.class_literal,
//    $.this,
//    $.identifier,
//    $.parenthesized_expression,
//    $.object_creation_expression,
//    $.field_access,
//    $.java_field_access,
//    $.array_access,
//    $.method_invocation,
//    $.array_creation_expression,
//    $.map_creation_expression,
//    $.query_expression,
//    $.version_expression
//  ),

#[derive(Debug, Serialize)]
pub enum PrimaryExpression {
    Literal(Literal_),
    Identifier(String),
    Method(MethodInvocation),
}

impl PrimaryExpression {
    pub fn new(n: Node) -> Self {
        match n.kind() {
            "identifier" => Self::Identifier(n.value(source_code())),
            "method_invocation" => Self::Method(MethodInvocation::new(n)),
            "boolean" | "null_literal" => Self::Literal(Literal_::new(n)),
            _ => panic!("## unknown node: {} in PrimaryExpression", n.kind().red()),
        }
    }
}

impl<'a> DocBuild<'a> for PrimaryExpression {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            Self::Literal(l) => {
                result.push(l.build(b));
            }
            Self::Identifier(i) => {
                result.push(b.txt(i));
            }
            Self::Method(m) => {
                result.push(m.build(b));
            }
        }
    }
}

#[derive(Debug, Serialize)]
pub enum Literal_ {
    Bool(String),
    Null,
    //Int(String),
    //Decimal(String),
    //Str(String),
}

impl Literal_ {
    pub fn new(n: Node) -> Self {
        match n.kind() {
            "boolean" => Self::Bool(n.value(source_code()).to_lowercase()),
            "null" => Self::Null,
            _ => panic!("## unknown node: {} in Literal", n.kind().red()),
        }
    }
}

impl<'a> DocBuild<'a> for Literal_ {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            Self::Bool(v) => {
                result.push(b.txt(v));
            }
            Self::Null => {
                result.push(b.txt("null"));
            }
        }
    }
}

#[derive(Debug, Serialize)]
pub enum Modifier {
    Abstract,
    Final,
    Global,
    InheritedSharing,
    Override,
    Private,
    Protected,
    Public,
    Static,
    TestMethod,
    Transient,
    Virtual,
    Webservice,
    WithSharing,
    WithoutSharing,
}

impl Modifier {
    pub fn new(n: Node) -> Self {
        let kind = n.kind();
        match kind {
            "public" => Self::Public,
            "with_sharing" => Self::WithSharing,
            "without_sharing" => Self::WithoutSharing,
            "private" => Self::Private,
            "override" => Self::Override,
            "static" => Self::Static,
            _ => panic!("## unknown node: {} in Modifier", kind),
        }
    }
}

impl<'a> DocBuild<'a> for Modifier {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            Self::Abstract => {
                result.push(b.txt("abstract"));
            }
            Self::Final => {
                result.push(b.txt("final"));
            }
            Self::Global => {
                result.push(b.txt("global"));
            }
            Self::InheritedSharing => {
                result.push(b.txt("inherited sharing"));
            }
            Self::Override => {
                result.push(b.txt("override"));
            }
            Self::Private => {
                result.push(b.txt("private"));
            }
            Self::Protected => {
                result.push(b.txt("Protected"));
            }
            Self::Public => {
                result.push(b.txt("public"));
            }
            Self::Static => {
                result.push(b.txt("static"));
            }
            Self::TestMethod => {
                result.push(b.txt("testmethod"));
            }
            Self::Transient => {
                result.push(b.txt("transient"));
            }
            Self::Virtual => {
                result.push(b.txt("virtual"));
            }
            Self::Webservice => {
                result.push(b.txt("webserivce"));
            }
            Self::WithSharing => {
                result.push(b.txt("with sharing"));
            }
            Self::WithoutSharing => {
                result.push(b.txt("without sharing"));
            }
        }
    }
}

//statement: ($) =>
//  choice(
//    $.declaration,
//    $.expression_statement,
//    $.if_statement,
//    $.while_statement,
//    $.for_statement,
//    $.enhanced_for_statement,
//    $.block,
//    ";",
//    $.do_statement,
//    $.break_statement,
//    $.continue_statement,
//    $.return_statement,
//    $.switch_expression,
//    $.local_variable_declaration,
//    $.throw_statement,
//    $.try_statement,
//    $.run_as_statement

#[derive(Debug, Serialize)]
pub enum Statement {
    If(Box<IfStatement>),
    Exp(Expression),
    Local(LocalVariableDeclaration),
    Block(Block),
}

impl Statement {
    pub fn new(n: Node) -> Self {
        match n.kind() {
            "if_statement" => Self::If(Box::new(IfStatement::new(n))),
            "expression_statement" => Self::Exp(Expression::new(n.first_c())),
            "local_variable_declaration" => Self::Local(LocalVariableDeclaration::new(n)),
            "block" => Self::Block(Block::new(n)),
            _ => panic!("## unknown node: {} in Statement", n.kind().red()),
        }
    }
}

impl<'a> DocBuild<'a> for Statement {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            Self::If(i) => {
                result.push(i.build(b));
            }
            Self::Exp(exp) => {
                result.push(exp.build(b));
                result.push(b.txt(";"));
            }
            Self::Local(l) => {
                result.push(l.build(b));
                result.push(b.txt(";"));
            }
            Self::Block(v) => {
                result.push(v.build(b));
            }
        }
    }
}

#[derive(Debug, Serialize)]
pub enum Type {
    Unnanotated(UnnanotatedType),
}

impl Type {
    pub fn new(n: Node) -> Self {
        match n.kind() {
            "type_identifier" => Self::Unnanotated(UnnanotatedType::Simple(
                SimpleType::Identifier(n.value(source_code())),
            )),
            _ => panic!("## unknown node: {} in Type ", n.kind().red()),
        }
    }
}

impl<'a> DocBuild<'a> for Type {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            Self::Unnanotated(u) => {
                result.push(u.build(b));
            }
        }
    }
}

#[derive(Debug, Serialize)]
pub enum PropertyNavigation {
    SafeNavigationOperator,
    Dot,
}

impl<'a> DocBuild<'a> for PropertyNavigation {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            Self::SafeNavigationOperator => {
                result.push(b.txt("?."));
            }
            Self::Dot => {
                result.push(b.txt("."));
            }
        }
    }
}

#[derive(Debug, Serialize)]
pub enum AnnotationArgumentList {
    Nil,
    Value(String),
    KeyValues(Vec<AnnotationKeyValue>),
}

impl AnnotationArgumentList {
    pub fn new(n: Node) -> Self {
        if n.named_child_count() == 0 {
            return Self::Nil;
        }

        let key_values = n.try_cs_by_k("annotation_key_value");

        if key_values.is_empty() {
            Self::Value(n.cvalue_by_n("value", source_code()))
        } else {
            let key_values = key_values
                .into_iter()
                .map(AnnotationKeyValue::new)
                .collect();
            Self::KeyValues(key_values)
        }
    }
}

impl<'a> DocBuild<'a> for AnnotationArgumentList {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            Self::Nil => {}
            Self::Value(v) => {
                result.push(b.txt("("));
                result.push(b.txt(v));
                result.push(b.txt(")"));
            }
            Self::KeyValues(vec) => {
                if !vec.is_empty() {
                    let docs = b.build_docs(vec);
                    let single_line_doc = b.pretty_surrounded_single_line(&docs, " ", "(", ")");
                    result.push(single_line_doc);
                }
            }
        }
    }
}
