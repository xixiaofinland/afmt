use crate::{
    accessor::Accessor,
    doc::DocRef,
    doc_builder::DocBuilder,
    enum_def::*,
    utility::{assert_check, source_code},
};
use colored::Colorize;
use serde::Serialize;
use std::fmt::Debug;
use tree_sitter::{Node, Point, Range};

pub trait DocBuild<'a> {
    fn build(&self, b: &'a DocBuilder<'a>) -> DocRef<'a> {
        let mut result: Vec<DocRef<'a>> = Vec::new();
        self.build_inner(b, &mut result);
        b.concat(result)
    }

    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>);
}

#[derive(Debug, Default, Serialize)]
pub struct Root {
    pub members: Vec<RootMember>,
}

impl Root {
    pub fn new(node: Node) -> Self {
        assert_check(node, "parser_output");
        let mut root = Root::default();

        for c in node.children_vec() {
            match c.kind() {
                "class_declaration" => root
                    .members
                    .push(RootMember::Class(Box::new(ClassDeclaration::new(c)))),
                _ => panic!("## unknown node: {} in Root ", c.kind().red()),
            }
        }
        root
    }
}

impl<'a> DocBuild<'a> for Root {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        let member_docs = b.build_docs(&self.members);
        let body_doc = b.sep_multi_line(&member_docs, "");
        result.push(body_doc);
        result.push(b.nl());
    }
}

#[derive(Debug, Serialize)]
pub struct ClassDeclaration {
    pub buckets: Option<CommentBuckets>,
    pub modifiers: Option<Modifiers>,
    pub name: String,
    pub superclass: Option<SuperClass>,
    pub interface: Option<Interface>,
    pub body: ClassBody,
    pub range: DataRange,
}

impl ClassDeclaration {
    pub fn new(node: Node) -> Self {
        assert_check(node, "class_declaration");
        let buckets = None;

        let modifiers = node.try_c_by_k("modifiers").map(|n| Modifiers::new(n));
        let name = node.cvalue_by_n("name", source_code());
        let superclass = node.try_c_by_k("superclass").map(|n| SuperClass::new(n));
        let interface = node.try_c_by_k("interfaces").map(|n| Interface::new(n));
        let body = ClassBody::new(node.c_by_n("body"));
        let range = DataRange::from(node.range());

        Self {
            buckets,
            modifiers,
            name,
            superclass,
            interface,
            body,
            range,
        }
    }
}

impl<'a> DocBuild<'a> for ClassDeclaration {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        if let Some(ref n) = self.modifiers {
            result.push(n.build(b));
        }

        result.push(b.txt_("class"));
        result.push(b.txt(&self.name));

        if let Some(ref n) = self.superclass {
            result.push(n.build(b));
        }

        if let Some(ref n) = self.interface {
            result.push(n.build(b));
        }

        result.push(b.txt(" {"));

        if self.body.class_members.is_empty() {
            result.push(b.nl());
        } else {
            result.push(b.add_indent_level(b.nl()));
            let body_doc = self.body.build(b);
            let indented_body = b.add_indent_level(body_doc);
            result.push(indented_body);
            result.push(b.nl());
        }

        result.push(b.txt("}"));
    }
}

#[derive(Debug, Serialize)]
pub struct MethodDeclaration {
    pub modifiers: Option<Modifiers>,
    pub type_: UnnanotatedType,
    pub name: String,
    pub formal_parameters: FormalParameters,
    pub body: Option<Block>,
    //pub dimentions
}

impl MethodDeclaration {
    pub fn new(node: Node) -> Self {
        assert_check(node, "method_declaration");

        let modifiers = node.try_c_by_k("modifiers").map(|n| Modifiers::new(n));
        let type_ = UnnanotatedType::new(node.c_by_n("type"));
        let name = node.cvalue_by_n("name", source_code());
        let formal_parameters = FormalParameters::new(node.c_by_n("parameters"));
        let body = node.try_c_by_n("body").map(|n| Block::new(n));

        Self {
            modifiers,
            type_,
            name,
            formal_parameters,
            body,
        }
    }
}

impl<'a> DocBuild<'a> for MethodDeclaration {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        if let Some(ref n) = self.modifiers {
            result.push(n.build(b));
        }

        result.push(&self.type_.build(b));
        result.push(b._txt(&self.name));
        result.push(self.formal_parameters.build(b));
        result.push(b.txt(" "));

        if let Some(ref n) = self.body {
            let body_doc = n.build(b);
            result.push(body_doc);
        } else {
            result.push(b.txt(";"));
        }
    }
}

#[derive(Debug, Serialize)]
pub struct FormalParameters {
    pub formal_parameters: Vec<FormalParameter>,
}

impl FormalParameters {
    pub fn new(node: Node) -> Self {
        let formal_parameters = node
            .try_cs_by_k("formal_parameter")
            .into_iter()
            .map(|n| FormalParameter::new(n))
            .collect();

        Self { formal_parameters }
    }
}

impl<'a> DocBuild<'a> for FormalParameters {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        let modifiers_doc = b.build_docs(&self.formal_parameters);
        result.push(b.pretty_surrounded(&modifiers_doc, ", ", ",", "(", ")"));
    }
}

#[derive(Debug, Serialize)]
pub struct FormalParameter {
    pub modifiers: Option<Modifiers>,
    pub type_: UnnanotatedType,
    pub name: String,
    //pub dimenssions
}

impl FormalParameter {
    pub fn new(node: Node) -> Self {
        let modifiers = node.try_c_by_k("modifiers").map(|n| Modifiers::new(n));
        let type_ = UnnanotatedType::new(node.c_by_n("type"));
        let name = node.cvalue_by_n("name", source_code());

        Self {
            modifiers,
            type_,
            name,
        }
    }
}

impl<'a> DocBuild<'a> for FormalParameter {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        if let Some(ref n) = self.modifiers {
            result.push(n.build(b));
        }

        result.push(self.type_.build(b));
        result.push(b._txt(&self.name));
    }
}

#[derive(Debug, Serialize)]
pub struct SuperClass {
    pub type_: Type,
}

impl SuperClass {
    pub fn new(node: Node) -> Self {
        assert_check(node, "superclass");

        let type_ = Type::new(node.first_c());
        Self { type_ }
    }
}

impl<'a> DocBuild<'a> for SuperClass {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        result.push(b.txt(" extends "));
        result.push(self.type_.build(b));
    }
}

#[derive(Debug, Default, Serialize)]
pub struct Modifiers {
    //pub buckets: CommentBuckets,
    annotation: Option<Annotation>,
    modifiers: Vec<Modifier>,
}

impl Modifiers {
    pub fn new(node: Node) -> Self {
        assert_check(node, "modifiers");
        let mut this = Self::default();

        for c in node.children_vec() {
            match c.kind() {
                "annotation" => {
                    this.annotation = Some(Annotation::new(c));
                }
                "modifier" => this.modifiers.push(Modifier::new(c.first_c())),
                "line_comment" | "block_comment" => continue,
                _ => panic!("## unknown node: {} in Modifiers", c.kind().red()),
            }
        }
        this
    }
}

impl<'a> DocBuild<'a> for Modifiers {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        if let Some(ref n) = self.annotation {
            result.push(n.build(b));
        }

        if !self.modifiers.is_empty() {
            let modifiers_doc = b.build_docs(&self.modifiers);
            result.push(b.sep_single_line(&modifiers_doc, " "));
            result.push(b.txt(" "));
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Annotation {
    pub name: String,
    pub arguments: Option<AnnotationArgumentList>,
}

impl Annotation {
    pub fn new(node: Node) -> Self {
        let name = node.cvalue_by_n("name", source_code());

        let mut arguments = None;
        node.try_c_by_n("arguments").map(|n| {
            arguments = Some(AnnotationArgumentList::new(n));
        });

        Self { name, arguments }
    }
}

impl<'a> DocBuild<'a> for Annotation {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        result.push(b.txt(format!("@{}", self.name)));

        if let Some(a) = &self.arguments {
            result.push(a.build(b));
        }
        result.push(b.nl());
    }
}

#[derive(Debug, Serialize)]
pub struct AnnotationKeyValue {
    key: String,
    value: String,
}

impl AnnotationKeyValue {
    pub fn new(node: Node) -> Self {
        assert_check(node, "annotation_key_value");
        Self {
            key: node.cvalue_by_n("key", source_code()),
            value: node.cvalue_by_n("value", source_code()),
        }
    }
}

impl<'a> DocBuild<'a> for AnnotationKeyValue {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        result.push(b.txt(&self.key));
        result.push(b.txt("="));
        result.push(b.txt(&self.value));
    }
}

#[derive(Debug, Serialize)]
pub struct ClassBody {
    pub class_members: Vec<ClassMember>,
}

impl ClassBody {
    pub fn new(node: Node) -> Self {
        assert_check(node, "class_body");
        let mut class_members: Vec<ClassMember> = Vec::new();

        for c in node.children_vec() {
            match c.kind() {
                "line_comment" | "block_comment" => continue,
                _ => class_members.push(ClassMember::new(c)),
            }
        }

        Self { class_members }
    }
}

impl<'a> DocBuild<'a> for ClassBody {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        let member_docs = b.build_docs(&self.class_members);
        let body_doc = b.sep_multi_line(&member_docs, "");
        result.push(body_doc);
    }
}

#[derive(Debug, Serialize)]
pub struct FieldDeclaration {
    pub buckets: Option<CommentBuckets>,
    pub modifiers: Option<Modifiers>,
    pub type_: UnnanotatedType,
    pub declarators: Vec<VariableDeclarator>,
    pub range: DataRange,
}

impl FieldDeclaration {
    pub fn new(node: Node) -> Self {
        assert_check(node, "field_declaration");
        let buckets = None;

        let modifiers = node.try_c_by_k("modifiers").map(|n| Modifiers::new(n));

        let type_node = node.c_by_n("type");
        let type_ = UnnanotatedType::new(type_node);

        let declarators = node
            .cs_by_n("declarator")
            .into_iter()
            .map(|n| VariableDeclarator::new(n))
            .collect();

        Self {
            buckets,
            modifiers,
            type_,
            declarators,
            range: DataRange::from(node.range()),
        }
    }
}

impl<'a> DocBuild<'a> for FieldDeclaration {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        if let Some(ref n) = self.modifiers {
            result.push(n.build(b));
        }

        result.push(self.type_.build(b));
        result.push(b.txt(" "));

        let decl_docs = b.build_docs(&self.declarators);

        let declarators_doc = b.separated_choice(&decl_docs, ", ", ", ");
        result.push(declarators_doc);

        result.push(b.txt(";"));
    }
}

#[derive(Debug, Default, Serialize)]
pub struct ArrayInitializer {
    variable_initializers: Vec<VariableInitializer>,
}

impl ArrayInitializer {
    pub fn new(node: Node, indent: usize) -> Self {
        assert_check(node, "array_initializer");
        ArrayInitializer::default()
    }
}

#[derive(Debug, Default, Serialize)]
pub struct AssignmentExpression {
    pub left: String,
    pub op: String,
    pub right: String,
}

impl AssignmentExpression {
    pub fn new(node: Node) -> Self {
        assert_check(node, "assignment_expression");

        let left = node.cvalue_by_n("left", source_code());
        let op = node.cvalue_by_n("operator", source_code());
        let right = node.cvalue_by_n("right", source_code());
        Self { left, op, right }
    }
}

impl<'a> DocBuild<'a> for AssignmentExpression {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        result.push(b.txt(format!("{} {} {}", self.left, self.op, self.right)));
    }
}

//#[derive(Debug, Serialize)]
//pub struct Identifier {
//    pub value: String,
//}
//
//impl Identifier {
//    pub fn new(node: Node) -> Self {
//        Self {
//            value: node.value(source_code()),
//        }
//    }
//}

#[derive(Debug, Serialize)]
pub struct VoidType {
    pub value: String,
}

impl VoidType {
    pub fn new(node: Node) -> Self {
        Self {
            value: node.value(source_code()),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct DataRange {
    pub start_byte: usize,
    pub end_byte: usize,
    pub start_point: DataPoint,
    pub end_point: DataPoint,
}

impl From<Range> for DataRange {
    fn from(r: Range) -> Self {
        let start_point = DataPoint::from(r.start_point);
        let end_point = DataPoint::from(r.end_point);

        Self {
            start_byte: r.start_byte,
            end_byte: r.end_byte,
            start_point,
            end_point,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct DataPoint {
    pub row: usize,
    pub column: usize,
}

impl From<Point> for DataPoint {
    fn from(p: Point) -> Self {
        Self {
            row: p.row,
            column: p.column,
        }
    }
}

#[derive(Debug, Default, Serialize)]
pub struct CommentBuckets {
    pub pre_comments: Vec<Comment>,
    pub post_comments: Vec<Comment>,
}

#[derive(Debug, Serialize)]
pub struct Comment {
    pub id: usize,
    pub content: String,
    pub comment_type: CommentType,
    pub is_processed: bool,
    pub range: DataRange,
}

impl Comment {
    pub fn from_node(node: Node) -> Self {
        let id = node.id();
        let content = node.v(source_code()).to_string();
        Self {
            id,
            content,
            is_processed: false,
            comment_type: match node.kind() {
                "line_comment" => CommentType::Line,
                "block_comment" => CommentType::Block,
                _ => panic!("Unexpected comment type"),
            },
            range: DataRange::from(node.range()),
        }
    }
}

#[derive(Debug, Serialize)]
pub enum CommentType {
    Line,
    Block,
}

#[derive(Debug, Default, Serialize)]
pub struct Block {
    pub statements: Vec<Statement>,
}

impl Block {
    pub fn new(node: Node) -> Self {
        assert_check(node, "block");
        let mut this = Block::default();

        for c in node.children_vec() {
            this.statements.push(Statement::new(c));
        }
        this
    }
}

impl<'a> DocBuild<'a> for Block {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        if self.statements.is_empty() {
            return result.push(b.concat(vec![b.txt("{"), b.nl(), b.txt("}")]));
        }

        let statement_docs = b.build_docs(&self.statements);
        let docs = b.pretty_surrounded_multi_line(&statement_docs, "", "{", "}");
        result.push(docs);
    }
}

#[derive(Debug, Default, Serialize)]
pub struct Interface {
    pub types: Vec<Type>,
}

impl Interface {
    pub fn new(node: Node) -> Self {
        assert_check(node, "interfaces");
        let mut interface = Interface::default();

        let type_list = node.c_by_k("type_list");

        for c in type_list.children_vec() {
            interface.types.push(Type::new(c));
        }

        interface
    }
}

impl<'a> DocBuild<'a> for Interface {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        let types_doc = b.build_docs(&self.types);
        let implements_group = b.concat(vec![
            b.txt(" implements "),
            b.sep_single_line(&types_doc, ", "),
        ]);
        result.push(implements_group);

        //let implements_group = b.add_indent_level(b.group(b.concat(vec![
        //    b.softline(),
        //    b.txt("implements "),
        //    b.sep_single_line(&types_doc, ", "),
        //])));
        //result.push(implements_group);
    }
}

#[derive(Debug, Serialize)]
pub struct MethodInvocation {
    pub object: Option<MethodObject>,
    pub property_navigation: Option<PropertyNavigation>,
    pub type_arguments: Option<TypeArguments>,
    pub name: String,
    pub arguments: ArgumentList,
}

impl MethodInvocation {
    pub fn new(node: Node) -> Self {
        let object = node.try_c_by_n("object").map(|n| {
            if n.kind() == "super" {
                MethodObject::Super(Super {})
            } else {
                MethodObject::Primary(Box::new(PrimaryExpression::new(n)))
            }
        });

        let property_navigation = object.as_ref().map(|_| {
            if node.try_c_by_n("safe_navigation_operator").is_some() {
                PropertyNavigation::SafeNavigationOperator
            } else {
                PropertyNavigation::Dot
            }
        });

        let type_arguments = node
            .try_c_by_k("type_arguments")
            .map(|n| TypeArguments::new(n));

        let name = node.cvalue_by_n("name", source_code());
        let arguments = ArgumentList::new(node.c_by_n("arguments"));

        Self {
            object,
            property_navigation,
            type_arguments,
            name,
            arguments,
        }
    }
}

impl<'a> DocBuild<'a> for MethodInvocation {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        if let Some(ref o) = self.object {
            result.push(o.build(b));
        }

        if let Some(ref p) = self.property_navigation {
            result.push(p.build(b));
        }

        result.push(b.txt(&self.name));
        result.push(self.arguments.build(b));
    }
}

#[derive(Debug, Serialize)]
pub enum MethodObject {
    Super(Super),
    Primary(Box<PrimaryExpression>),
}

impl<'a> DocBuild<'a> for MethodObject {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            MethodObject::Super(s) => {
                result.push(s.build(b));
            }
            MethodObject::Primary(p) => {
                result.push(p.build(b));
            }
        }
    }
}

#[derive(Debug, Serialize)]
pub struct TypeArguments {
    pub types: Vec<Type>,
}

impl TypeArguments {
    pub fn new(node: Node) -> Self {
        let mut types = Vec::new();
        for c in node.children_vec() {
            types.push(Type::new(c));
        }
        Self { types }
    }
}

impl<'a> DocBuild<'a> for TypeArguments {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        let types_doc = b.build_docs(&self.types);
        result.push(b.pretty_surrounded(&types_doc, ", ", ",", "<", ">"));
    }
}

#[derive(Debug, Default, Serialize)]
pub struct ArgumentList {
    pub expressions: Vec<Expression>,
}

impl ArgumentList {
    pub fn new(node: Node) -> Self {
        let mut this = ArgumentList::default();
        for c in node.children_vec() {
            this.expressions.push(Expression::new(c));
        }
        this
    }
}

impl<'a> DocBuild<'a> for ArgumentList {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        let exp_doc = b.build_docs(&self.expressions);
        result.push(b.pretty_surrounded(&exp_doc, ", ", ",", "(", ")"));
    }
}

#[derive(Debug, Serialize)]
pub struct Super {}

impl<'a> DocBuild<'a> for Super {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        result.push(b.txt("super"))
    }
}

#[derive(Debug, Serialize)]
pub struct This {}

impl<'a> DocBuild<'a> for This {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        result.push(b.txt("this"))
    }
}

#[derive(Debug, Serialize)]
pub struct BinaryExpression {
    pub left: Expression,
    pub op: String,
    pub right: Expression,
}

impl BinaryExpression {
    pub fn new(node: Node) -> Self {
        let left = Expression::new(node.c_by_n("left"));
        let op = node.cvalue_by_n("operator", source_code());
        let right = Expression::new(node.c_by_n("right"));

        Self { left, op, right }
    }
}

impl<'a> DocBuild<'a> for BinaryExpression {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        result.push(self.left.build(b));
        result.push(b._txt_(&self.op));
        result.push(self.right.build(b));
    }
}

#[derive(Debug, Serialize)]
pub struct LocalVariableDeclaration {
    pub modifiers: Option<Modifiers>,
    pub type_: UnnanotatedType,
    pub declarators: Vec<VariableDeclarator>,
}

impl LocalVariableDeclaration {
    pub fn new(node: Node) -> Self {
        assert_check(node, "local_variable_declaration");

        let modifiers = node.try_c_by_k("modifiers").map(|n| Modifiers::new(n));
        let type_ = UnnanotatedType::new(node.c_by_n("type"));
        let declarators = node
            .cs_by_n("declarator")
            .into_iter()
            .map(|n| VariableDeclarator::new(n))
            .collect();

        Self {
            modifiers,
            type_,
            declarators,
        }
    }
}

impl<'a> DocBuild<'a> for LocalVariableDeclaration {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        if let Some(ref n) = self.modifiers {
            result.push(n.build(b));
        }

        result.push(self.type_.build(b));
        result.push(b.txt(" "));

        let docs_vec = b.build_docs(&self.declarators);
        let declarators_doc = b.separated_choice(&docs_vec, ", ", ",");
        result.push(declarators_doc);
    }
}

#[derive(Debug, Serialize)]
pub struct VariableDeclarator {
    pub name: String,
    //pub dimenssions
    pub value: Option<VariableInitializer>,
}

impl VariableDeclarator {
    pub fn new(node: Node) -> Self {
        assert_check(node, "variable_declarator");
        let name = node.cvalue_by_n("name", source_code());

        let value = node.try_c_by_n("value").map(|v| match v.kind() {
            //"array_initializer" => {
            //    VariableInitializer::ArrayInitializer(ArrayInitializer::new(v, source_code, indent))
            //}
            _ => VariableInitializer::Expression(Expression::Primary(Box::new(
                PrimaryExpression::Identifier(v.value(source_code())),
            ))),
        });

        Self { name, value }
    }
}

impl<'a> DocBuild<'a> for VariableDeclarator {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        result.push(b.txt(&self.name));
        if let Some(ref v) = self.value {
            result.push(b.txt(" = "));
            result.push(v.build(b));
        }
    }
}

#[derive(Debug, Serialize)]
pub struct GenericType {
    pub type_identifier: String,
    pub type_arguments: TypeArguments,
}

impl GenericType {
    pub fn new(node: Node) -> Self {
        let type_identifier = node.cvalue_by_k("type_identifier", source_code());
        let type_arguments = TypeArguments::new(node.c_by_k("type_arguments"));

        Self {
            type_identifier,
            type_arguments,
        }
    }
}

impl<'a> DocBuild<'a> for GenericType {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        result.push(b.txt(&self.type_identifier));
        result.push(self.type_arguments.build(b));
    }
}

#[derive(Debug, Serialize)]
pub struct IfStatement {
    pub condition: ParenthesizedExpression,
    pub consequence: Statement,
    pub alternative: Option<Statement>,
}

impl IfStatement {
    pub fn new(node: Node) -> Self {
        let condition = ParenthesizedExpression::new(node.c_by_n("condition"));
        let consequence = Statement::new(node.c_by_n("consequence"));
        let alternative = node.try_c_by_n("alternative").map(|a| Statement::new(a));
        Self {
            condition,
            consequence,
            alternative,
        }
    }
}

impl<'a> DocBuild<'a> for IfStatement {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        result.push(b.txt("if "));
        result.push(self.condition.build(b));

        if self.consequence.is_block() {
            result.push(b.txt(" "));
            result.push(self.consequence.build(b));
        } else {
            result.push(b.add_indent_level(b.nl()));
            result.push(b.add_indent_level(self.consequence.build(b)));
        }

        // Handle the 'else' part
        if let Some(ref a) = self.alternative {
            match a {
                Statement::If(_) => {
                    if self.consequence.is_block() {
                        result.push(b.txt(" else "));
                    } else {
                        result.push(b.nl());
                        result.push(b.txt("else "));
                    }
                    result.push(a.build(b)); // Recursively build the nested 'else if' statement
                }
                Statement::Block(_) => {
                    if self.consequence.is_block() {
                        result.push(b.txt(" else "));
                    } else {
                        result.push(b.nl());
                        result.push(b.txt("else "));
                    }
                    result.push(a.build(b));
                }
                // Handle "else" with a single statement
                _ => {
                    if self.consequence.is_block() {
                        result.push(b.txt(" else "));
                    } else {
                        result.push(b.nl());
                        result.push(b.txt("else"));
                        result.push(b.add_indent_level(b.nl()));
                    }
                    result.push(a.build(b)); // Build the else statement
                }
            }
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ParenthesizedExpression {
    pub exp: Expression,
}

impl ParenthesizedExpression {
    pub fn new(node: Node) -> Self {
        let exp = Expression::new(node.first_c());
        Self { exp }
    }
}

impl<'a> DocBuild<'a> for ParenthesizedExpression {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        result.push(b.txt("("));
        result.push(self.exp.build(b));
        result.push(b.txt(")"));
    }
}

#[derive(Debug, Serialize)]
pub struct ForStatement {
    pub init: Option<LocalVariableDeclaration>,
    pub condition: Option<Expression>,
    pub update: Option<Expression>,
    pub body: Statement,
}

impl ForStatement {
    pub fn new(node: Node) -> Self {
        let init = node
            .try_c_by_n("init")
            .map(|n| LocalVariableDeclaration::new(n));
        let condition = node.try_c_by_n("condition").map(|n| Expression::new(n));
        let update = node.try_c_by_n("update").map(|n| Expression::new(n));
        let body = Statement::new(node.c_by_n("body"));
        Self {
            init,
            condition,
            update,
            body,
        }
    }
}

impl<'a> DocBuild<'a> for ForStatement {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        result.push(b.txt("for "));
        let init = match &self.init {
            Some(i) => i.build(b),
            None => b.nil(),
        };
        let condition = match &self.condition {
            Some(c) => b.concat(vec![b.txt(" "), c.build(b)]),
            None => b.nil(),
        };
        let update = match &self.update {
            Some(u) => b.concat(vec![b.txt(" "), u.build(b)]),
            None => b.nil(),
        };
        let docs = vec![init, condition, update];

        result.push(b.pretty_surrounded(&docs, ";", ";", "(", ")"));
        result.push(b.txt(" "));
        result.push(self.body.build(b));
    }
}

#[derive(Debug, Serialize)]
pub enum UpdateExpression {
    Pre {
        operator: String,
        operand: Box<Expression>,
    },
    Post {
        operand: Box<Expression>,
        operator: String,
    },
}

impl UpdateExpression {
    pub fn new(node: Node) -> Self {
        assert_check(node, "update_expression");

        let operator_node = node.c_by_n("operator");
        let operand_node = node.c_by_n("operand");

        if operator_node.start_byte() < operand_node.start_byte() {
            Self::Pre {
                operator: operator_node.value(source_code()),
                operand: Box::new(Expression::new(operand_node)),
            }
        } else {
            Self::Post {
                operand: Box::new(Expression::new(operand_node)),
                operator: operator_node.value(source_code()),
            }
        }
    }
}

impl<'a> DocBuild<'a> for UpdateExpression {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            Self::Pre { operator, operand } => {
                result.push(b.txt(operator));
                result.push(operand.build(b));
            }
            Self::Post { operand, operator } => {
                result.push(operand.build(b));
                result.push(b.txt(operator));
            }
        }
    }
}
