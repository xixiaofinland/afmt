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
            "field_declaration" => ClassMember::Field(Box::new(FieldDeclaration::new(n))),
            "class_declaration" => ClassMember::NestedClass(Box::new(ClassDeclaration::new(n))),
            "method_declaration" => ClassMember::Method(Box::new(MethodDeclaration::new(n))),
            "block" => ClassMember::Block(Box::new(Block::new(n))),
            _ => panic!("## unknown node: {} in UnnanotatedType ", n.kind().red()),
        }
    }
}

impl<'a> DocBuild<'a> for ClassMember {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            ClassMember::Field(field_decl) => {
                result.push(field_decl.build(b));
            }
            ClassMember::NestedClass(class_decl) => {
                result.push(class_decl.build(b));
            }
            ClassMember::Method(method) => {
                result.push(method.build(b));
            }
            ClassMember::Block(block) => {
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
            "type_identifier" => {
                UnnanotatedType::Simple(SimpleType::Identifier(n.value(source_code())))
            }
            "void_type" => UnnanotatedType::Simple(SimpleType::Void(VoidType::new(n))),
            _ => panic!("## unknown node: {} in UnnanotatedType ", n.kind().red()),
        }
    }
}

impl<'a> DocBuild<'a> for UnnanotatedType {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            UnnanotatedType::Simple(s) => result.push(s.build(b)),
        }
    }
}

#[derive(Debug, Serialize)]
pub enum SimpleType {
    Identifier(String),
    Void(VoidType),
}

impl<'a> DocBuild<'a> for SimpleType {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            SimpleType::Identifier(i) => {
                result.push(b.txt(i));
            }
            SimpleType::Void(v) => {
                result.push(b.txt(&v.value));
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
            VariableInitializer::Expression(exp) => {
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
            "string_literal" => Expression::StringLiteral(n.value(source_code())),
            "binary_expression" => Expression::Binary(Box::new(BinaryExpression::new(n))),
            "method_invocation" => Expression::Primary(Box::new(PrimaryExpression::Method(
                MethodInvocation::new(n),
            ))),
            _ => panic!("## unknown node: {} in Expression", n.kind().red()),
        }
    }
}

impl<'a> DocBuild<'a> for Expression {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            Expression::StringLiteral(s) => {
                result.push(b.txt(s));
            }
            Expression::Binary(binary) => {
                result.push(binary.build(b));
            }
            Expression::Primary(p) => {
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
    Identifier(String),
    Method(MethodInvocation),
}

impl PrimaryExpression {
    pub fn new(n: Node) -> Self {
        match n.kind() {
            "identifier" => PrimaryExpression::Identifier(n.value(source_code())),
            "method_invocation" => PrimaryExpression::Method(MethodInvocation::new(n)),
            _ => panic!("## unknown node: {} in PrimaryExpression", n.kind().red()),
        }
    }
}

impl<'a> DocBuild<'a> for PrimaryExpression {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            PrimaryExpression::Identifier(i) => {
                result.push(b.txt(i));
            }
            PrimaryExpression::Method(m) => {
                result.push(m.build(b));
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
            "public" => Modifier::Public,
            "with_sharing" => Modifier::WithSharing,
            "without_sharing" => Modifier::WithoutSharing,
            "private" => Modifier::Private,
            "override" => Modifier::Override,
            _ => panic!("## unknown node: {} in Modifier", kind),
        }
    }
}

impl<'a> DocBuild<'a> for Modifier {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            Modifier::Abstract => {
                result.push(b.txt("abstract"));
            }
            Modifier::Final => {
                result.push(b.txt("final"));
            }
            Modifier::Global => {
                result.push(b.txt("global"));
            }
            Modifier::InheritedSharing => {
                result.push(b.txt("inherited sharing"));
            }
            Modifier::Override => {
                result.push(b.txt("override"));
            }
            Modifier::Private => {
                result.push(b.txt("private"));
            }
            Modifier::Protected => {
                result.push(b.txt("Protected"));
            }
            Modifier::Public => {
                result.push(b.txt("public"));
            }
            Modifier::Static => {
                result.push(b.txt("static"));
            }
            Modifier::TestMethod => {
                result.push(b.txt("testmethod"));
            }
            Modifier::Transient => {
                result.push(b.txt("transient"));
            }
            Modifier::Virtual => {
                result.push(b.txt("virtual"));
            }
            Modifier::Webservice => {
                result.push(b.txt("webserivce"));
            }
            Modifier::WithSharing => {
                result.push(b.txt("with sharing"));
            }
            Modifier::WithoutSharing => {
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
    Exp(Expression),
}

impl Statement {
    pub fn new(n: Node) -> Self {
        match n.kind() {
            "expression_statement" => Statement::Exp(Expression::new(n.first_c())),
            _ => panic!("## unknown node: {} in Statement", n.kind().red()),
        }
    }
}

impl<'a> DocBuild<'a> for Statement {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            Statement::Exp(exp) => {
                result.push(exp.build(b));
                result.push(b.txt(";"));
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
            "type_identifier" => Type::Unnanotated(UnnanotatedType::Simple(
                SimpleType::Identifier(n.value(source_code())),
            )),
            _ => panic!("## unknown node: {} in Type ", n.kind().red()),
        }
    }
}

impl<'a> DocBuild<'a> for Type {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            Type::Unnanotated(u) => {
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
            PropertyNavigation::SafeNavigationOperator => {
                result.push(b.txt("?."));
            }
            PropertyNavigation::Dot => {
                result.push(b.txt("."));
            }
        }
    }
}
