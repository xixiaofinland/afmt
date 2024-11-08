use colored::Colorize;
use serde::Serialize;
use tree_sitter::Node;

use crate::{data_model::*, doc::DocRef, doc_builder::DocBuilder};

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
    Block(Box<Block>),
    //Method(MethodDeclaration<'a>),
    //Interface(InterfaceDeclaration<'a>),
    //Enum(EnumDeclaration<'a>),
    //StaticInitializer(StaticInitializer<'a>),
    //Constructor(ConstructorDeclaration<'a>),
    //EmptyStatement, // Represents the ";" case
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
                UnnanotatedType::Simple(SimpleType::Identifier(Identifier::new(n)))
            }
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
    Identifier(Identifier),
}

impl<'a> DocBuild<'a> for SimpleType {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            SimpleType::Identifier(i) => {
                result.push(b.txt(&i.value));
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
    //Assignment(AssignmentExpression),
    Primary(PrimaryExpression),
}

impl<'a> DocBuild<'a> for Expression {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            Expression::Primary(p) => {
                result.push(p.build(b));
            }
        }
    }
}

#[derive(Debug, Serialize)]
pub enum PrimaryExpression {
    Identifier(Identifier),
}

impl<'a> DocBuild<'a> for PrimaryExpression {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            PrimaryExpression::Identifier(i) => {
                result.push(b.txt(&i.value));
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
                result.push(b.txt("overwrite"));
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

#[derive(Debug, Serialize)]
pub enum Statement {
    //Identifier(Identifier),
}

impl<'a> DocBuild<'a> for Statement {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        //match self {
        //    PrimaryExpression::Identifier(i) => {
        //        result.push(b.txt(&i.value));
        //    }
        //}
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
                SimpleType::Identifier(Identifier::new(n)),
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
