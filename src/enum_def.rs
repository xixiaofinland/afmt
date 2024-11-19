use colored::Colorize;
use serde::Serialize;
use tree_sitter::Node;

use crate::{
    accessor::Accessor, data_model::*, doc::DocRef, doc_builder::DocBuilder, utility::source_code,
};

#[derive(Debug, Serialize)]
pub enum RootMember {
    Class(Box<ClassDeclaration>),
    Enum(Box<EnumDeclaration>),
    Interface(Box<InterfaceDeclaration>),
}

impl<'a> DocBuild<'a> for RootMember {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            RootMember::Class(n) => {
                result.push(n.build(b));
            }
            RootMember::Enum(n) => {
                result.push(n.build(b));
            }
            RootMember::Interface(n) => {
                result.push(n.build(b));
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
    Interface(Box<InterfaceDeclaration>),
    Enum(EnumDeclaration),
    Static(StaticInitializer),
    Constructor(ConstructorDeclaration),
    //SemiColumn,
}

impl ClassMember {
    pub fn new(n: Node) -> Self {
        match n.kind() {
            "field_declaration" => Self::Field(Box::new(FieldDeclaration::new(n))),
            "class_declaration" => Self::NestedClass(Box::new(ClassDeclaration::new(n))),
            "method_declaration" => Self::Method(Box::new(MethodDeclaration::new(n))),
            "interface_declaration" => Self::Interface(Box::new(InterfaceDeclaration::new(n))),
            "block" => Self::Block(Box::new(Block::new(n))),
            "constructor_declaration" => Self::Constructor(ConstructorDeclaration::new(n)),
            "enum_declaration" => Self::Enum(EnumDeclaration::new(n)),
            "static_initializer" => Self::Static(StaticInitializer::new(n)),
            _ => panic!("## unknown node: {} in UnannotatedType ", n.kind().red()),
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
            Self::Interface(i) => {
                result.push(i.build(b));
            }
            Self::Block(block) => {
                result.push(block.build(b));
            }
            Self::Constructor(c) => {
                result.push(c.build(b));
            }
            Self::Enum(en) => {
                result.push(en.build(b));
            }
            Self::Static(s) => {
                result.push(s.build(b));
            }
        }
    }
}

#[derive(Debug, Serialize)]
pub enum UnannotatedType {
    Simple(SimpleType),
    //Array(ArrayType),
}

impl UnannotatedType {
    pub fn new(n: Node) -> Self {
        match n.kind() {
            "type_identifier" | "void_type" | "generic_type" | "scoped_type_identifier" => {
                Self::Simple(SimpleType::new(n))
            }
            _ => panic!("## unknown node: {} in UnannotatedType ", n.kind().red()),
        }
    }
}

impl<'a> DocBuild<'a> for UnannotatedType {
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
    Scoped(ScopedTypeIdentifier),
}

impl SimpleType {
    pub fn new(n: Node) -> Self {
        match n.kind() {
            "type_identifier" => Self::Identifier(n.value(source_code())),
            "void_type" => Self::Void(VoidType::new(n)),
            "generic_type" => Self::Generic(GenericType::new(n)),
            "scoped_type_identifier" => Self::Scoped(ScopedTypeIdentifier::new(n)),
            _ => panic!("## unknown node: {} in SimpleType", n.kind().red()),
        }
    }
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
            Self::Scoped(g) => {
                result.push(g.build(b));
            }
        }
    }
}

#[derive(Debug, Serialize)]
pub enum VariableInitializer {
    Exp(Expression),
    Array(Box<ArrayInitializer>),
}

impl VariableInitializer {
    pub fn new(n: Node) -> Self {
        match n.kind() {
            "array_initializer" => Self::Array(Box::new(ArrayInitializer::new(n))),
            _ => Self::Exp(Expression::new(n)),
        }
    }
}

impl<'a> DocBuild<'a> for VariableInitializer {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            Self::Exp(exp) => {
                result.push(exp.build(b));
            }
            Self::Array(a) => {
                result.push(a.build(b));
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
    Assignment(Box<AssignmentExpression>),
    StringLiteral(String),
    Binary(Box<BinaryExpression>),
    Primary(Box<PrimaryExpression>),
    Update(UpdateExpression),
    Unary(UnaryExpression),
    Dml(Box<DmlExpression>),
    Te(Box<TernaryExpression>),
}

impl Expression {
    pub fn new(n: Node) -> Self {
        match n.kind() {
            "assignment_expression" => Self::Assignment(Box::new(AssignmentExpression::new(n))),
            "string_literal" => Self::StringLiteral(n.value(source_code())),
            "binary_expression" => Self::Binary(Box::new(BinaryExpression::new(n))),
            "int"
            | "boolean"
            | "identifier"
            | "null_literal"
            | "method_invocation"
            | "parenthesized_expression"
            | "object_creation_expression"
            | "array_access"
            | "field_access"
            | "array_creation_expression" => Self::Primary(Box::new(PrimaryExpression::new(n))),
            "update_expression" => Self::Update(UpdateExpression::new(n)),
            "unary_expression" => Self::Unary(UnaryExpression::new(n)),
            "dml_expression" => Self::Dml(Box::new(DmlExpression::new(n))),
            "ternary_expression" => Self::Te(Box::new(TernaryExpression::new(n))),
            _ => panic!("## unknown node: {} in Expression", n.kind().red()),
        }
    }
}

impl<'a> DocBuild<'a> for Expression {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            Self::Assignment(a) => {
                result.push(a.build(b));
            }
            Self::StringLiteral(s) => {
                result.push(b.txt(s));
            }
            Self::Binary(binary) => {
                result.push(binary.build(b));
            }
            Self::Primary(p) => {
                result.push(p.build(b));
            }
            Self::Update(u) => {
                result.push(u.build(b));
            }
            Self::Unary(u) => {
                result.push(u.build(b));
            }
            Self::Dml(d) => {
                result.push(d.build(b));
            }
            Self::Te(t) => {
                result.push(t.build(b));
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
    Parenth(ParenthesizedExpression),
    Obj(ObjectCreationExpression),
    Field(FieldAccess),
    Array(Box<ArrayAccess>),
    ArrayCreation(ArrayCreationExpression),
}

impl PrimaryExpression {
    pub fn new(n: Node) -> Self {
        match n.kind() {
            "int" | "boolean" | "null_literal" => Self::Literal(Literal_::new(n)),
            "identifier" => Self::Identifier(n.value(source_code())),
            "method_invocation" => Self::Method(MethodInvocation::new(n)),
            "parenthesized_expression" => Self::Parenth(ParenthesizedExpression::new(n)),
            "object_creation_expression" => Self::Obj(ObjectCreationExpression::new(n)),
            "field_access" => Self::Field(FieldAccess::new(n)),
            "array_access" => Self::Array(Box::new(ArrayAccess::new(n))),
            "array_creation_expression" => Self::ArrayCreation(ArrayCreationExpression::new(n)),
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
            Self::Parenth(p) => {
                result.push(p.build(b));
            }
            Self::Obj(o) => {
                result.push(o.build(b));
            }
            Self::Field(f) => {
                result.push(f.build(b));
            }
            Self::Array(a) => {
                result.push(a.build(b));
            }
            Self::ArrayCreation(a) => {
                result.push(a.build(b));
            }
        }
    }
}

#[derive(Debug, Serialize)]
pub enum Literal_ {
    Bool(String),
    Null,
    Int(String),
    //Decimal(String),
    //Str(String),
}

impl Literal_ {
    pub fn new(n: Node) -> Self {
        match n.kind() {
            "boolean" => Self::Bool(n.value(source_code()).to_lowercase()),
            "null_literal" => Self::Null,
            "int" => Self::Int(n.value(source_code())),
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
            Self::Int(s) => {
                result.push(b.txt(s));
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
            "global" => Self::Global,
            "public" => Self::Public,
            "with_sharing" => Self::WithSharing,
            "without_sharing" => Self::WithoutSharing,
            "private" => Self::Private,
            "override" => Self::Override,
            "static" => Self::Static,
            "final" => Self::Final,
            "virtual" => Self::Virtual,
            "abstract" => Self::Abstract,
            "inherited sharing" => Self::InheritedSharing,
            "protected" => Self::Protected,
            "test method" => Self::TestMethod,
            "transient" => Self::Transient,
            "webservice" => Self::Webservice,
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
    For(Box<ForStatement>),
    EnhancedFor(Box<EnhancedForStatement>),
    Run(RunAsStatement),
    Do(DoStatement),
    While(WhileStatement),
    Return(ReturnStatement),
    Try(TryStatement),
}

impl Statement {
    pub fn new(n: Node) -> Self {
        match n.kind() {
            "if_statement" => Self::If(Box::new(IfStatement::new(n))),
            "expression_statement" => Self::Exp(Expression::new(n.first_c())),
            "local_variable_declaration" => Self::Local(LocalVariableDeclaration::new(n)),
            "block" => Self::Block(Block::new(n)),
            "for_statement" => Self::For(Box::new(ForStatement::new(n))),
            "enhanced_for_statement" => Self::EnhancedFor(Box::new(EnhancedForStatement::new(n))),
            "run_as_statement" => Self::Run(RunAsStatement::new(n)),
            "do_statement" => Self::Do(DoStatement::new(n)),
            "while_statement" => Self::While(WhileStatement::new(n)),
            "return_statement" => Self::Return(ReturnStatement::new(n)),
            "try_statement" => Self::Try(TryStatement::new(n)),
            _ => panic!("## unknown node: {} in Statement", n.kind().red()),
        }
    }

    pub fn is_block(&self) -> bool {
        matches!(self, Statement::Block(_))
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
            Self::For(f) => {
                result.push(f.build(b));
            }
            Self::EnhancedFor(f) => {
                result.push(f.build(b));
            }
            Self::Run(r) => {
                result.push(r.build(b));
            }
            Self::Do(d) => {
                result.push(d.build(b));
            }
            Self::While(w) => {
                result.push(w.build(b));
            }
            Self::Return(r) => {
                result.push(r.build(b));
            }
            Self::Try(t) => {
                result.push(t.build(b));
            }
        }
    }
}

#[derive(Debug, Serialize)]
pub enum Type {
    Unannotated(UnannotatedType),
}

impl Type {
    pub fn new(n: Node) -> Self {
        match n.kind() {
            "type_identifier" | "void_type" | "generic_type" | "scoped_type_identifier" => {
                Self::Unannotated(UnannotatedType::Simple(SimpleType::new(n)))
            }
            _ => panic!("## unknown node: {} in Type ", n.kind().red()),
        }
    }
}

impl<'a> DocBuild<'a> for Type {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            Self::Unannotated(u) => {
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
                    let docs = b.to_docs(vec);

                    let doc = b.concat(vec![
                        b.txt("("),
                        b.intersperse_single_line(&docs, " "),
                        b.txt(")"),
                    ]);

                    result.push(doc);
                }
            }
        }
    }
}

// Generic struct to associate a body member with trailing newline information
#[derive(Debug, Serialize)]
pub struct BodyMember<M> {
    pub member: M,
    pub has_trailing_newline: bool,
}
