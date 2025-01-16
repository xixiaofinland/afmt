use crate::{
    accessor::Accessor,
    context::NodeInfo,
    data_model::*,
    doc::DocRef,
    doc_builder::{DocBuilder, Insertable},
    utility::{assert_check, build_with_comments_and_punc, get_comment_bucket, panic_unknown_node},
};
use tree_sitter::Node;

#[derive(Debug)]
pub enum RootMember {
    Class(Box<ClassDeclaration>),
    Enum(Box<EnumDeclaration>),
    Interface(Box<InterfaceDeclaration>),
    Trigger(Box<TriggerDeclaration>),
}

impl RootMember {
    pub fn new(n: Node) -> Self {
        match n.kind() {
            "class_declaration" => Self::Class(Box::new(ClassDeclaration::new(n))),
            "enum_declaration" => Self::Enum(Box::new(EnumDeclaration::new(n))),
            "trigger_declaration" => Self::Trigger(Box::new(TriggerDeclaration::new(n))),
            "interface_declaration" => Self::Interface(Box::new(InterfaceDeclaration::new(n))),
            _ => panic_unknown_node(n, "Root"),
        }
    }
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
            RootMember::Trigger(n) => {
                result.push(n.build(b));
            }
        }
    }
}

#[derive(Debug)]
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
            _ => panic_unknown_node(n, "ClassMember"),
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

#[derive(Debug)]
pub enum UnannotatedType {
    Simple(SimpleType),
    Array(Box<ArrayType>),
}

impl UnannotatedType {
    pub fn new(n: Node) -> Self {
        match n.kind() {
            "type_identifier"
            | "void_type"
            | "boolean_type"
            | "generic_type"
            | "java_type"
            | "scoped_type_identifier" => Self::Simple(SimpleType::new(n)),
            "array_type" => Self::Array(Box::new(ArrayType::new(n))),
            _ => panic_unknown_node(n, "UnnanotatedType"),
        }
    }
}

impl<'a> DocBuild<'a> for UnannotatedType {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            Self::Simple(n) => result.push(n.build(b)),
            Self::Array(n) => result.push(n.build(b)),
        }
    }
}

#[derive(Debug)]
pub enum SimpleType {
    Identifier(ValueNode),
    Void(ValueNode),
    Bool(BoolType),
    Generic(GenericType),
    Scoped(ScopedTypeIdentifier),
    Java(JavaType),
}

impl SimpleType {
    pub fn new(n: Node) -> Self {
        match n.kind() {
            "type_identifier" => Self::Identifier(ValueNode::new(n)),
            "void_type" => Self::Void(ValueNode::new(n)),
            "boolean_type" => Self::Bool(BoolType::new(n)),
            "java_type" => Self::Java(JavaType::new(n)),
            "generic_type" => Self::Generic(GenericType::new(n)),
            "scoped_type_identifier" => Self::Scoped(ScopedTypeIdentifier::new(n)),
            _ => panic_unknown_node(n, "SimpleType"),
        }
    }
}

impl<'a> DocBuild<'a> for SimpleType {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            Self::Identifier(n) => {
                result.push(n.build(b));
            }
            Self::Java(n) => {
                result.push(n.build(b));
            }
            Self::Void(n) => {
                result.push(n.build(b));
            }
            Self::Bool(n) => {
                result.push(n.build(b));
            }
            Self::Generic(n) => {
                result.push(n.build(b));
            }
            Self::Scoped(n) => {
                result.push(n.build(b));
            }
        }
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum Expression {
    Assignment(Box<AssignmentExpression>),
    Binary(Box<BinaryExpression>),
    Primary(Box<PrimaryExpression>),
    Update(UpdateExpression),
    Unary(UnaryExpression),
    Dml(Box<DmlExpression>),
    Te(Box<TernaryExpression>),
    Cast(Box<CastExpression>),
    Instance(Box<InstanceOfExpression>),
}

impl Expression {
    pub fn new(n: Node) -> Self {
        match n.kind() {
            "assignment_expression" => Self::Assignment(Box::new(AssignmentExpression::new(n))),
            "binary_expression" => Self::Binary(Box::new(BinaryExpression::new(n))),
            "int"
            | "decimal_floating_point_literal"
            | "query_expression"
            | "boolean"
            | "identifier"
            | "null_literal"
            | "class_literal"
            | "method_invocation"
            | "parenthesized_expression"
            | "object_creation_expression"
            | "map_creation_expression"
            | "array_access"
            | "field_access"
            | "string_literal"
            | "version_expression"
            | "java_field_access"
            | "this"
            | "array_creation_expression" => Self::Primary(Box::new(PrimaryExpression::new(n))),
            "update_expression" => Self::Update(UpdateExpression::new(n)),
            "unary_expression" => Self::Unary(UnaryExpression::new(n)),
            "dml_expression" => Self::Dml(Box::new(DmlExpression::new(n))),
            "ternary_expression" => Self::Te(Box::new(TernaryExpression::new(n))),
            "cast_expression" => Self::Cast(Box::new(CastExpression::new(n))),
            "instanceof_expression" => Self::Instance(Box::new(InstanceOfExpression::new(n))),
            _ => panic_unknown_node(n, "Expression"),
        }
    }
}

impl<'a> DocBuild<'a> for Expression {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            Self::Assignment(n) => {
                result.push(n.build(b));
            }
            Self::Binary(n) => {
                result.push(n.build(b));
            }
            Self::Primary(n) => {
                result.push(n.build(b));
            }
            Self::Update(n) => {
                result.push(n.build(b));
            }
            Self::Unary(n) => {
                result.push(n.build(b));
            }
            Self::Dml(n) => {
                result.push(n.build(b));
            }
            Self::Te(n) => {
                result.push(n.build(b));
            }
            Self::Cast(n) => {
                result.push(n.build(b));
            }
            Self::Instance(n) => {
                result.push(n.build(b));
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

#[derive(Debug)]
pub enum PrimaryExpression {
    Literal(Literal_),
    Identifier(ValueNode),
    Class(ClassLiteral),
    Method(MethodInvocation),
    Parenth(ParenthesizedExpression),
    Obj(ObjectCreationExpression),
    Map(MapCreationExpression),
    Field(FieldAccess),
    Array(Box<ArrayAccess>),
    ArrayCreation(ArrayCreationExpression),
    Version(VersionExpression),
    Query(QueryExpression),
    This(This),
    Java(JavaFieldAccess),
}

impl PrimaryExpression {
    pub fn new(n: Node) -> Self {
        match n.kind() {
            "int"
            | "decimal_floating_point_literal"
            | "boolean"
            | "null_literal"
            | "string_literal" => Self::Literal(Literal_::new(n)),
            "identifier" => Self::Identifier(ValueNode::new(n)),
            "class_literal" => Self::Class(ClassLiteral::new(n)),
            "method_invocation" => Self::Method(MethodInvocation::new(n)),
            "parenthesized_expression" => Self::Parenth(ParenthesizedExpression::new(n)),
            "object_creation_expression" => Self::Obj(ObjectCreationExpression::new(n)),
            "map_creation_expression" => Self::Map(MapCreationExpression::new(n)),
            "field_access" => Self::Field(FieldAccess::new(n)),
            "array_access" => Self::Array(Box::new(ArrayAccess::new(n))),
            "array_creation_expression" => Self::ArrayCreation(ArrayCreationExpression::new(n)),
            "version_expression" => Self::Version(VersionExpression::new(n)),
            "query_expression" => Self::Query(QueryExpression::new(n)),
            "java_field_access" => Self::Java(JavaFieldAccess::new(n)),
            "this" => Self::This(This::new(n)),
            _ => panic_unknown_node(n, "PrimaryExpression"),
        }
    }
}

impl<'a> DocBuild<'a> for PrimaryExpression {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            Self::Literal(n) => {
                result.push(n.build(b));
            }
            Self::Identifier(n) => {
                result.push(n.build(b));
            }
            Self::Class(n) => {
                result.push(n.build(b));
            }
            Self::Method(n) => {
                result.push(n.build(b));
            }
            Self::Parenth(n) => {
                result.push(n.build(b));
            }
            Self::Obj(n) => {
                result.push(n.build(b));
            }
            Self::Map(n) => {
                result.push(n.build(b));
            }
            Self::Field(n) => {
                result.push(n.build(b));
            }
            Self::Array(n) => {
                result.push(n.build(b));
            }
            Self::ArrayCreation(n) => {
                result.push(n.build(b));
            }
            Self::Version(n) => {
                result.push(n.build(b));
            }
            Self::Query(n) => {
                result.push(n.build(b));
            }
            Self::Java(n) => {
                result.push(n.build(b));
            }
            Self::This(n) => {
                result.push(n.build(b));
            }
        }
    }
}

#[derive(Debug)]
pub struct ClassLiteral {
    pub type_: UnannotatedType,
    pub node_info: NodeInfo,
}

impl ClassLiteral {
    pub fn new(node: Node) -> Self {
        assert_check(node, "class_literal");

        Self {
            type_: UnannotatedType::new(node.first_c()),
            node_info: NodeInfo::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for ClassLiteral {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_info, result, |b, result| {
            result.push(self.type_.build(b));
            result.push(b.txt("class"));
        });
    }
}

#[derive(Debug)]
pub enum Literal_ {
    Bool(ValueNodeLowerCase),
    Null(ValueNodeLowerCase),
    Int(ValueNode),
    Decimal(ValueNodeLowerCase),
    Str(ValueNode),
}

impl Literal_ {
    pub fn new(node: Node) -> Self {
        match node.kind() {
            "boolean" => Self::Bool(ValueNodeLowerCase::new(node)),
            "null_literal" => Self::Null(ValueNodeLowerCase::new(node)),
            "int" => Self::Int(ValueNode::new(node)),
            "string_literal" => Self::Str(ValueNode::new(node)),
            "decimal_floating_point_literal" => Self::Decimal(ValueNodeLowerCase::new(node)),
            _ => panic_unknown_node(node, "Literal_"),
        }
    }
}

impl<'a> DocBuild<'a> for Literal_ {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            Self::Bool(n) => {
                result.push(n.build(b));
            }
            Self::Null(n) => {
                result.push(n.build(b));
            }
            Self::Int(n) => {
                result.push(n.build(b));
            }
            Self::Decimal(n) => {
                result.push(n.build(b));
            }
            Self::Str(n) => {
                result.push(n.build(b));
            }
        }
    }
}

#[derive(Debug)]
pub enum ModifierKind {
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

impl ModifierKind {
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
            "inherited_sharing" => Self::InheritedSharing,
            "protected" => Self::Protected,
            "testMethod" => Self::TestMethod,
            "transient" => Self::Transient,
            "webservice" => Self::Webservice,
            _ => panic_unknown_node(n, "Modifier"),
        }
    }
}

impl<'a> DocBuild<'a> for ModifierKind {
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
                result.push(b.txt("protected"));
            }
            Self::Public => {
                result.push(b.txt("public"));
            }
            Self::Static => {
                result.push(b.txt("static"));
            }
            Self::TestMethod => {
                result.push(b.txt("testMethod"));
            }
            Self::Transient => {
                result.push(b.txt("transient"));
            }
            Self::Virtual => {
                result.push(b.txt("virtual"));
            }
            Self::Webservice => {
                result.push(b.txt("webService"));
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

#[derive(Debug)]
pub enum Statement {
    If(Box<IfStatement>),
    Exp(ExpressionStatement),
    Local(LocalVariableDeclaration),
    Block(Box<Block>),
    For(Box<ForStatement>),
    EnhancedFor(Box<EnhancedForStatement>),
    Run(RunAsStatement),
    Do(Box<DoStatement>),
    While(Box<WhileStatement>),
    Return(ReturnStatement),
    Try(Box<TryStatement>),
    Throw(ThrowStatement),
    Break(BreakStatement),
    Continue(ContinueStatement),
    Switch(Box<SwitchExpression>),
    SemiColumn,
}

impl Statement {
    pub fn new(n: Node) -> Self {
        match n.kind() {
            "if_statement" => Self::If(Box::new(IfStatement::new(n))),
            "expression_statement" => Self::Exp(ExpressionStatement::new(n)),
            "local_variable_declaration" => Self::Local(LocalVariableDeclaration::new(n)),
            "block" => Self::Block(Box::new(Block::new(n))),
            "for_statement" => Self::For(Box::new(ForStatement::new(n))),
            "enhanced_for_statement" => Self::EnhancedFor(Box::new(EnhancedForStatement::new(n))),
            "run_as_statement" => Self::Run(RunAsStatement::new(n)),
            "do_statement" => Self::Do(Box::new(DoStatement::new(n))),
            "while_statement" => Self::While(Box::new(WhileStatement::new(n))),
            "return_statement" => Self::Return(ReturnStatement::new(n)),
            "try_statement" => Self::Try(Box::new(TryStatement::new(n))),
            "throw_statement" => Self::Throw(ThrowStatement::new(n)),
            "break_statement" => Self::Break(BreakStatement::new(n)),
            "continue_statement" => Self::Continue(ContinueStatement::new(n)),
            "switch_expression" => Self::Switch(Box::new(SwitchExpression::new(n))),
            ";" => Self::SemiColumn,
            _ => panic_unknown_node(n, "Statement"),
        }
    }
    pub fn is_block(&self) -> bool {
        matches!(self, Statement::Block(_))
    }
}

impl<'a> DocBuild<'a> for Statement {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            Self::If(n) => {
                result.push(n.build(b));
            }
            Self::Exp(n) => {
                result.push(n.build(b));
            }
            Self::Local(n) => {
                result.push(n.build(b));
            }
            Self::Block(n) => {
                result.push(n.build(b));
            }
            Self::For(n) => {
                result.push(n.build(b));
            }
            Self::EnhancedFor(n) => {
                result.push(n.build(b));
            }
            Self::Run(n) => {
                result.push(n.build(b));
            }
            Self::Do(n) => {
                result.push(n.build(b));
            }
            Self::While(n) => {
                result.push(n.build(b));
            }
            Self::Return(n) => {
                result.push(n.build(b));
            }
            Self::Try(n) => {
                result.push(n.build(b));
            }
            Self::Throw(n) => {
                result.push(n.build(b));
            }
            Self::Break(n) => {
                result.push(n.build(b));
            }
            Self::Continue(n) => {
                result.push(n.build(b));
            }
            Self::Switch(n) => {
                result.push(n.build(b));
            }
            Self::SemiColumn => {
                result.push(b.txt(";"));
            }
        }
    }
}

#[derive(Debug)]
pub enum Type {
    Unannotated(UnannotatedType),
}

impl Type {
    pub fn new(n: Node) -> Self {
        match n.kind() {
            "type_identifier"
            | "void_type"
            | "boolean_type"
            | "generic_type"
            | "scoped_type_identifier"
            | "java_type" => Self::Unannotated(UnannotatedType::Simple(SimpleType::new(n))),
            "array_type" => Self::Unannotated(UnannotatedType::Array(Box::new(ArrayType::new(n)))),
            _ => panic_unknown_node(n, "Type"),
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

#[derive(Debug)]
pub enum PropertyNavigation {
    Safe(SafeNavigationOperator),
    Dot,
}

impl<'a> DocBuild<'a> for PropertyNavigation {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            Self::Safe(n) => {
                result.push(n.build(b));
            }
            Self::Dot => {
                result.push(b.txt("."));
            }
        }
    }
}

#[derive(Debug)]
pub enum AnnotationArgumentList {
    Nil,
    Value(ValueNode),
    KeyValues(Vec<AnnotationKeyValue>),
}

impl AnnotationArgumentList {
    pub fn new(n: Node) -> Self {
        if n.named_child_count() == 0 {
            return Self::Nil;
        }

        let key_values = n.try_cs_by_k("annotation_key_value");

        if key_values.is_empty() {
            Self::Value(ValueNode::new(n.c_by_n("value")))
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
                result.push(v.build(b));
                result.push(b.txt(")"));
            }
            Self::KeyValues(vec) => {
                if !vec.is_empty() {
                    let docs = b.to_docs(vec);
                    let sep = Insertable::new::<&str>(None, None, Some(b.softline()));
                    let open = Insertable::new(None, Some("("), Some(b.maybeline()));
                    let close = Insertable::new(Some(b.maybeline()), Some(")"), None);
                    let doc = b.group_surround(&docs, sep, open, close);
                    result.push(doc);
                }
            }
        }
    }
}

// Generic struct to associate a body member with trailing newline information
#[derive(Debug)]
pub struct BodyMember<M> {
    pub member: M,
    pub has_trailing_newline: bool, // already take comment nodes into consideration
}

impl<M> BodyMember<M> {
    pub fn new(node: &Node, member: M) -> Self {
        Self {
            member,
            has_trailing_newline: Self::has_trailing_newline(node),
        }
    }

    // take comment nodes into consideration
    fn has_trailing_newline(node: &Node) -> bool {
        let node_id = &node.id();
        let bucket = get_comment_bucket(node_id);

        // we assume post_comments are always inline
        // because this method is not called by last element of BodyMember
        if let Some(last_post_comment) = bucket.post_comments.last() {
            return last_post_comment.has_newline_below();
        }

        node.next_named_sibling()
            .is_some_and(|n| node.end_position().row < n.start_position().row - 1)
    }
}

#[derive(Debug)]
pub enum TriggerEventVariant {
    BeforeInsert,
    BeforeUpdate,
    BeforeDelete,
    AfterInsert,
    AfterUpdate,
    AfterDelete,
    AfterUndelete,
}

impl TriggerEventVariant {
    pub fn new(n: Node) -> Self {
        match n.kind() {
            "before_insert" => Self::BeforeInsert,
            "before_update" => Self::BeforeUpdate,
            "before_delete" => Self::BeforeDelete,
            "after_insert" => Self::AfterInsert,
            "after_update" => Self::AfterUpdate,
            "after_delete" => Self::AfterDelete,
            "after_undelete" => Self::AfterUndelete,
            _ => panic_unknown_node(n, "TriggerEvent"),
        }
    }
}

impl<'a> DocBuild<'a> for TriggerEventVariant {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            Self::BeforeInsert => {
                result.push(b.txt("before insert"));
            }
            Self::BeforeUpdate => {
                result.push(b.txt("before update"));
            }
            Self::BeforeDelete => {
                result.push(b.txt("before delete"));
            }
            Self::AfterInsert => {
                result.push(b.txt("after insert"));
            }
            Self::AfterUpdate => {
                result.push(b.txt("after update"));
            }
            Self::AfterDelete => {
                result.push(b.txt("after delete"));
            }
            Self::AfterUndelete => {
                result.push(b.txt("after undelete"));
            }
        }
    }
}

#[derive(Debug)]
pub enum SelectClauseVariant {
    Count(CountExpression),
    Selectable(Vec<SelectableExpression>),
}

impl SelectClauseVariant {
    pub fn new(node: Node) -> Self {
        assert_check(node, "select_clause");

        if let Some(count_node) = node.try_c_by_k("count_expression") {
            Self::Count(CountExpression::new(count_node))
        } else {
            Self::Selectable(
                node.children_vec()
                    .into_iter()
                    .map(|n| SelectableExpression::new(n))
                    .collect(),
            )
        }
    }
}

impl<'a> DocBuild<'a> for SelectClauseVariant {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        let mut doc_vec = Vec::new();
        doc_vec.push(b.txt("SELECT"));
        doc_vec.push(b.indent(b.softline()));

        match self {
            Self::Count(n) => {
                doc_vec.push(n.build(b));
            }
            Self::Selectable(vec) => {
                let docs = b.to_docs(vec);
                let sep = Insertable::new::<&str>(None, None, Some(b.softline()));
                let doc = b.intersperse(&docs, sep);

                let indented_join = b.indent(doc);
                doc_vec.push(indented_join);
            }
        }
        result.push(b.group_concat(doc_vec));
    }
}

#[derive(Debug)]
pub enum SelectableExpression {
    Value(ValueExpression),
    Alias(AliasExpression),
    //Type(TypeOfClause),
    Fields(FieldsExpression),
    Sub(SubQuery),
}

impl SelectableExpression {
    pub fn new(node: Node) -> Self {
        match node.kind() {
            "field_identifier" => Self::Value(ValueExpression::Field(FieldIdentifier::new(node))),
            "function_expression" => Self::Value(ValueExpression::Function(Box::new(
                FunctionExpression::new(node),
            ))),
            "alias_expression" => Self::Alias(AliasExpression::new(node)),
            "fields_expression" => Self::Fields(FieldsExpression::new(node)),
            "subquery" => Self::Sub(SubQuery::new(node)),
            _ => panic_unknown_node(node, "SelectableExpression"),
        }
    }
}

impl<'a> DocBuild<'a> for SelectableExpression {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            Self::Value(n) => {
                result.push(n.build(b));
            }
            Self::Alias(n) => {
                result.push(n.build(b));
            }
            Self::Fields(n) => {
                result.push(n.build(b));
            }
            Self::Sub(n) => {
                result.push(n.build(b));
            }
        }
    }
}

#[derive(Debug)]
pub struct FieldsExpression {
    fields_type: ValueNodeUpperCase,
    pub node_info: NodeInfo,
}

impl FieldsExpression {
    pub fn new(node: Node) -> Self {
        assert_check(node, "fields_expression");

        Self {
            fields_type: ValueNodeUpperCase::new(node.c_by_k("fields_type")),
            node_info: NodeInfo::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for FieldsExpression {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_info, result, |b, result| {
            result.push(b.txt("FIELDS("));
            result.push(self.fields_type.build(b));
            result.push(b.txt(")"));
        });
    }
}

#[derive(Debug)]
pub struct AliasExpression {
    value_exp: ValueExpression,
    identifier: ValueNode,
    pub node_info: NodeInfo,
}

impl AliasExpression {
    pub fn new(node: Node) -> Self {
        assert_check(node, "alias_expression");

        Self {
            value_exp: ValueExpression::new(node.first_c()),
            identifier: ValueNode::new(node.c_by_k("identifier")),
            node_info: NodeInfo::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for AliasExpression {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_info, result, |b, result| {
            result.push(self.value_exp.build(b));
            result.push(b.txt(" "));
            result.push(self.identifier.build(b));
        });
    }
}

#[derive(Debug)]
pub enum FieldIdentifierVariant {
    Identifier(ValueNode),
    Dotted(DottedIdentifier),
}

impl FieldIdentifierVariant {
    pub fn new(node: Node) -> Self {
        assert_check(node, "field_identifier");

        let c = node.first_c();
        match c.kind() {
            "identifier" => Self::Identifier(ValueNode::new(c)),
            "dotted_identifier" => Self::Dotted(DottedIdentifier::new(c)),
            _ => panic_unknown_node(c, "FieldIdentifier"),
        }
    }
}

impl<'a> DocBuild<'a> for FieldIdentifierVariant {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            Self::Identifier(n) => {
                result.push(n.build(b));
            }
            Self::Dotted(n) => {
                result.push(n.build(b));
            }
        }
    }
}

#[derive(Debug)]
pub enum StorageVariant {
    Identifier(StorageIdentifier),
    Alias(StorageAlias),
}

impl StorageVariant {
    pub fn new(node: Node) -> StorageVariant {
        match node.kind() {
            "storage_alias" => Self::Alias(StorageAlias::new(node)),
            "storage_identifier" => Self::Identifier(StorageIdentifier::new(node)),
            _ => panic_unknown_node(node, "StorageVariant"),
        }
    }
}

impl<'a> DocBuild<'a> for StorageVariant {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            Self::Identifier(n) => {
                result.push(n.build(b));
            }
            Self::Alias(n) => {
                result.push(n.build(b));
            }
        }
    }
}

#[derive(Debug)]
pub enum StorageIdentifierVariant {
    Identifier(ValueNode),
    Dotted(Vec<ValueNode>),
}

impl StorageIdentifierVariant {
    pub fn new(node: Node) -> Self {
        assert_check(node, "storage_identifier");
        let c = node.first_c();

        match c.kind() {
            "identifier" => Self::Identifier(ValueNode::new(c)),
            "dotted_identifier" => Self::Dotted(
                c.cs_by_k("identifier")
                    .into_iter()
                    .map(|n| ValueNode::new(n))
                    .collect(),
            ),
            _ => panic_unknown_node(c, "StorageIdentifier"),
        }
    }
}

impl<'a> DocBuild<'a> for StorageIdentifierVariant {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            Self::Identifier(n) => {
                result.push(n.build(b));
            }
            Self::Dotted(vec) => {
                let docs: Vec<_> = vec.iter().map(|n| n.build(b)).collect();
                let sep = Insertable::new(None, Some("."), None);
                let doc = b.intersperse(&docs, sep);
                result.push(doc);
            }
        }
    }
}

#[derive(Debug)]
pub enum LimitValue {
    Int(ValueNode),
    Bound(BoundApexExpression),
}

impl LimitValue {
    pub fn new(n: Node) -> Self {
        match n.kind() {
            "int" => Self::Int(ValueNode::new(n)),
            "bound_apex_expression" => Self::Bound(BoundApexExpression::new(n)),
            _ => panic_unknown_node(n, "LimitValue"),
        }
    }
}

impl<'a> DocBuild<'a> for LimitValue {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            Self::Int(n) => {
                result.push(n.build(b));
            }
            Self::Bound(n) => {
                result.push(n.build(b));
            }
        }
    }
}

#[derive(Debug)]
pub enum BooleanExpression {
    And(AndExpression),
    Or(OrExpression),
    Not(NotExpression),
    Condition(Box<ConditionExpression>),
}

impl BooleanExpression {
    pub fn new(node: Node) -> Self {
        match node.kind() {
            "and_expression" => Self::And(AndExpression::new(node)),
            "or_expression" => Self::Or(OrExpression::new(node)),
            "not_expression" => Self::Not(NotExpression::new(node)),
            _ => Self::Condition(Box::new(ConditionExpression::new(node))),
        }
    }

    fn operator(&self) -> Option<&str> {
        match self {
            Self::And(_) => Some("AND"),
            Self::Or(_) => Some("OR"),
            Self::Not(_) => Some("NOT"),
            Self::Condition(_) => None,
        }
    }

    pub fn build_with_parent<'a>(
        &self,
        b: &'a DocBuilder<'a>,
        parent_op: Option<&str>,
    ) -> DocRef<'a> {
        match self {
            Self::And(n) => n.build(b),
            Self::Or(n) => n.build(b),
            Self::Not(n) => n.build(b),
            Self::Condition(expr) => expr.build_with_parent(b, parent_op),
        }
    }
}

impl<'a> DocBuild<'a> for BooleanExpression {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        let doc = self.build_with_parent(b, None);
        result.push(doc);
    }
}

#[derive(Debug)]
pub enum ConditionExpression {
    Bool(Box<BooleanExpression>),
    Comparison(ComparisonExpression),
}

impl ConditionExpression {
    pub fn new(node: Node) -> Self {
        match node.kind() {
            "comparison_expression" => Self::Comparison(ComparisonExpression::new(node)),
            _ => Self::Bool(Box::new(BooleanExpression::new(node))),
        }
    }

    pub fn build_with_parent<'a>(
        &self,
        b: &'a DocBuilder<'a>,
        parent_op: Option<&str>,
    ) -> DocRef<'a> {
        match self {
            Self::Comparison(n) => n.build(b),
            Self::Bool(n) => {
                let child_op = n.operator();
                let doc = n.build_with_parent(b, child_op);

                if Self::should_parenthesize(parent_op, child_op) {
                    b.concat(vec![b.txt("("), doc, b.txt(")")])
                } else {
                    doc
                }
            }
        }
    }

    fn should_parenthesize(parent_op: Option<&str>, child_op: Option<&str>) -> bool {
        // Parentheses are NOT needed if both parent and child operators are the same.
        // Otherwise, parentheses are needed to maintain correct logical grouping.
        !matches!((parent_op, child_op), (Some(parent), Some(child)) if parent == child)
    }
}

impl<'a> DocBuild<'a> for ConditionExpression {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        let doc = self.build_with_parent(b, None);
        result.push(doc);
    }
}

#[derive(Debug)]
pub enum ValueExpression {
    Field(FieldIdentifier),
    Function(Box<FunctionExpression>),
}

impl ValueExpression {
    pub fn new(n: Node) -> Self {
        match n.kind() {
            "field_identifier" => Self::Field(FieldIdentifier::new(n)),
            "function_expression" => Self::Function(Box::new(FunctionExpression::new(n))),
            _ => panic_unknown_node(n, "ValueExpression"),
        }
    }
}

impl<'a> DocBuild<'a> for ValueExpression {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            Self::Field(n) => {
                result.push(n.build(b));
            }
            Self::Function(n) => {
                result.push(n.build(b));
            }
        }
    }
}

#[derive(Debug)]
pub enum GeoLocationTypeVariant {
    Field(FieldIdentifier),
    Bound(BoundApexExpression),
    Func {
        function_name: ValueNode,
        decimal1: ValueNode,
        decimal2: ValueNode,
    },
}

impl GeoLocationTypeVariant {
    pub fn new(node: Node) -> Self {
        let child = node.first_c();
        match child.kind() {
            "field_identifier" => Self::Field(FieldIdentifier::new(child)),
            "bound_apex_expression" => Self::Bound(BoundApexExpression::new(child)),
            "identifier" => {
                let decimals = node.cs_by_k("decimal");
                if decimals.len() != 2 {
                    panic!(
                        "expect 2 decimal nodes, found {} in GeoLocationType",
                        decimals.len()
                    );
                }

                Self::Func {
                    function_name: ValueNode::new(child),
                    decimal1: ValueNode::new(decimals[0]),
                    decimal2: ValueNode::new(decimals[1]),
                }
            }

            _ => panic_unknown_node(child, "GeoLocationType"),
        }
    }
}

impl<'a> DocBuild<'a> for GeoLocationTypeVariant {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            Self::Field(n) => {
                result.push(n.build(b));
            }
            Self::Bound(n) => {
                result.push(n.build(b));
            }
            Self::Func {
                function_name,
                decimal1,
                decimal2,
            } => {
                result.push(function_name.build(b));
                result.push(b.txt("("));
                result.push(decimal1.build(b));
                result.push(b.txt(" "));
                result.push(decimal2.build(b));
                result.push(b.txt(")"));
            }
        }
    }
}

#[derive(Debug)]
pub enum Comparison {
    Value(ValueComparison),
    Set(SetComparison),
}

impl<'a> DocBuild<'a> for Comparison {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            Self::Value(n) => {
                result.push(n.build(b));
            }
            Self::Set(n) => {
                result.push(n.build(b));
            }
        }
    }
}

#[derive(Debug)]
pub enum ValueComparedWith {
    Literal(SoqlLiteral),
    Bound(BoundApexExpression),
}

impl<'a> DocBuild<'a> for ValueComparedWith {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            Self::Literal(n) => {
                result.push(n.build(b));
            }
            Self::Bound(n) => {
                result.push(n.build(b));
            }
        }
    }
}

#[derive(Debug)]
pub enum SoqlLiteral {
    Int(ValueNode),
    Decimal(ValueNode),
    StringLiteral(ValueNode),
    Date(ValueNode),
    DateTime(ValueNode),
    Boolean(ValueNode),
    DateLiteral(ValueNode),
    DWithParam(DateLiteralWithParam),
    CurrentLiteral(ValueNode),
    NullLiteral(ValueNode),
}

impl SoqlLiteral {
    pub fn new(node: Node) -> Self {
        match node.kind() {
            "decimal" => Self::Decimal(ValueNode::new(node)),
            "int" => Self::Int(ValueNode::new(node)),
            "string_literal" => Self::StringLiteral(ValueNode::new(node)),
            "boolean" => Self::Boolean(ValueNode::new(node)),
            "date" => Self::Boolean(ValueNode::new(node)),
            "date_literal" => Self::DateLiteral(ValueNode::new(node)),
            "date_literal_with_param" => Self::DWithParam(DateLiteralWithParam::new(node)),
            "null_literal" => Self::NullLiteral(ValueNode::new(node)),
            _ => panic_unknown_node(node, "SoqlLiteral"),
        }
    }
}

impl<'a> DocBuild<'a> for SoqlLiteral {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            Self::Decimal(n) => {
                result.push(n.build(b));
            }
            Self::Int(n) => {
                result.push(n.build(b));
            }
            Self::StringLiteral(n) => {
                result.push(n.build(b));
            }
            Self::Date(n) => {
                result.push(n.build(b));
            }
            Self::Boolean(n) => {
                result.push(n.build(b));
            }
            Self::DateLiteral(n) => {
                result.push(n.build(b));
            }
            Self::DWithParam(n) => {
                result.push(n.build(b));
            }
            Self::NullLiteral(n) => {
                result.push(n.build(b));
            }
            _ => {
                unreachable!();
            }
        }
    }
}

#[derive(Debug)]
pub struct DateLiteralWithParam {
    date_literal: String,
    param: String,
    pub node_info: NodeInfo,
}

impl DateLiteralWithParam {
    pub fn new(node: Node) -> Self {
        assert_check(node, "date_literal_with_param");

        Self {
            date_literal: node.cvalue_by_k("date_literal").to_uppercase(),
            param: node.cvalue_by_k("int"),
            node_info: NodeInfo::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for DateLiteralWithParam {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_info, result, |b, result| {
            result.push(b.txt(format!("{}:{}", &self.date_literal, &self.param)));
        });
    }
}

#[derive(Debug)]
pub enum SetValue {
    Sub(SubQuery),
    List(ComparableList),
    Bound(BoundApexExpression),
}

impl SetValue {
    pub fn new(node: Node) -> Self {
        match node.kind() {
            "subquery" => Self::Sub(SubQuery::new(node)),
            "comparable_list" => Self::List(ComparableList::new(node)),
            "bound_apex_expression" => Self::Bound(BoundApexExpression::new(node)),
            _ => panic_unknown_node(node, "SetValue"),
        }
    }
}

impl<'a> DocBuild<'a> for SetValue {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            Self::Sub(n) => {
                result.push(n.build(b));
            }
            Self::List(n) => {
                result.push(n.build(b));
            }
            Self::Bound(n) => {
                result.push(n.build(b));
            }
        }
    }
}

#[derive(Debug)]
pub enum ComparableListValue {
    Literal(SoqlLiteral),
    Bound(BoundApexExpression),
}

impl ComparableListValue {
    pub fn new(node: Node) -> Self {
        match node.kind() {
            "bound_apex_expression" => Self::Bound(BoundApexExpression::new(node)),
            _ => Self::Literal(SoqlLiteral::new(node)),
        }
    }
}

impl<'a> DocBuild<'a> for ComparableListValue {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            Self::Bound(n) => {
                result.push(n.build(b));
            }
            Self::Literal(n) => {
                result.push(n.build(b));
            }
        }
    }
}

#[derive(Debug)]
pub enum OffsetClause {
    Int(ValueNode),
    Bound(BoundApexExpression),
}

impl OffsetClause {
    pub fn new(node: Node) -> Self {
        assert_check(node, "offset_clause");

        let first_c = node.first_c();
        match first_c.kind() {
            "int" => Self::Int(ValueNode::new(first_c)),
            "bound_apex_expression" => Self::Bound(BoundApexExpression::new(first_c)),
            _ => panic_unknown_node(first_c, "OffsetClause"),
        }
    }
}

impl<'a> DocBuild<'a> for OffsetClause {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        result.push(b.txt_("OFFSET"));

        match self {
            Self::Int(n) => {
                result.push(n.build(b));
            }
            Self::Bound(n) => {
                result.push(n.build(b));
            }
        }
    }
}

#[derive(Debug)]
pub enum FunctionExpressionVariant {
    WithGEO {
        function_name: ValueNode,
        field: Option<FieldIdentifier>,
        bound: Option<BoundApexExpression>,
        geo: GeoLocationType,
        string_literal: ValueNode,
    },
    WithoutGEO {
        function_name: ValueNode,
        value_exps: Vec<ValueExpression>,
    },
}

impl FunctionExpressionVariant {
    pub fn new(node: Node) -> Self {
        assert_check(node, "function_expression");

        let function_expression = if node.try_c_by_k("geo_location_type").is_some() {
            Self::WithGEO {
                function_name: ValueNode::new(node.c_by_n("function_name")),
                field: node
                    .try_c_by_k("field_identifier")
                    .map(|n| FieldIdentifier::new(n)),
                bound: node
                    .try_c_by_k("bound_apex_expression")
                    .map(|n| BoundApexExpression::new(n)),
                geo: GeoLocationType::new(node.c_by_k("geo_location_type")),
                string_literal: ValueNode::new(node.c_by_k("string_literal")),
            }
        } else {
            Self::WithoutGEO {
                function_name: ValueNode::new(node.c_by_n("function_name")),
                value_exps: node
                    .children_vec()
                    .into_iter()
                    .skip(1)
                    .map(|n| ValueExpression::new(n))
                    .collect(),
            }
        };

        function_expression
    }
}

impl<'a> DocBuild<'a> for FunctionExpressionVariant {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            Self::WithGEO {
                function_name,
                field,
                bound,
                geo,
                string_literal,
            } => {
                result.push(function_name.build(b));
                result.push(b.txt("("));
                if let Some(ref n) = field {
                    result.push(n.build(b));
                }
                if let Some(ref n) = bound {
                    result.push(n.build(b));
                }
                result.push(b.txt(" "));
                result.push(geo.build(b));
                result.push(b.txt(" "));
                result.push(string_literal.build(b));
                result.push(b.txt(")"));
            }
            Self::WithoutGEO {
                function_name,
                value_exps,
            } => {
                result.push(function_name.build(b));

                let doc = b.to_docs(value_exps);
                let sep = Insertable::new::<&str>(None, None, Some(b.softline()));
                let open = Insertable::new(None, Some("("), Some(b.maybeline()));
                let close = Insertable::new(Some(b.maybeline()), Some(")"), None);
                let doc = b.group_surround(&doc, sep, open, close);
                result.push(doc);
            }
        }
    }
}
