use crate::{
    accessor::Accessor,
    context::{NodeContext, Punctuation},
    doc::DocRef,
    doc_builder::{DocBuilder, Insertable},
    enum_def::*,
    message_helper::red,
    utility::*,
};
use std::fmt::Debug;
use tree_sitter::Node;

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
    pub members: Vec<BodyMember<RootMember>>,
    pub node_context: NodeContext,
}

impl Root {
    pub fn new(node: Node) -> Self {
        assert_check(node, "parser_output");

        let members: Vec<_> = node
            .children_vec()
            .into_iter()
            .map(|n| BodyMember::new(&n, RootMember::new(n)))
            .collect();

        let node_context = NodeContext::with_punctuation(&node);

        Self {
            members,
            node_context,
        }
    }
}

impl<'a> DocBuild<'a> for Root {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        let bucket = get_comment_bucket(&self.node_context.id);
        if !bucket.dangling_comments.is_empty() {
            let docs: Vec<_> = bucket
                .dangling_comments
                .iter()
                .map(|n| n.build(b))
                .collect();
            return result.push(b.concat(docs));
        }

        let doc = b.intersperse_body_members(&self.members);
        result.push(doc);
        result.push(b.nl());
    }
}

#[derive(Debug)]
pub struct ClassDeclaration {
    pub modifiers: Option<Modifiers>,
    pub name: ValueNode,
    pub type_parameters: Option<TypeParameters>,
    pub superclass: Option<SuperClass>,
    pub interface: Option<Interface>,
    pub body: ClassBody,
    pub node_context: NodeContext,
}

impl ClassDeclaration {
    pub fn new(node: Node) -> Self {
        assert_check(node, "class_declaration");

        Self {
            modifiers: node.try_c_by_k("modifiers").map(|n| Modifiers::new(n)),
            name: ValueNode::new(node.c_by_n("name")),
            type_parameters: node
                .try_c_by_k("type_parameters")
                .map(|n| TypeParameters::new(n)),
            superclass: node.try_c_by_k("superclass").map(|n| SuperClass::new(n)),
            interface: node.try_c_by_k("interfaces").map(|n| Interface::new(n)),
            body: ClassBody::new(node.c_by_n("body")),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for ClassDeclaration {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            if let Some(ref n) = self.modifiers {
                result.push(n.build(b));
            }

            let mut docs = vec![];

            docs.push(b.txt_("class"));
            docs.push(self.name.build(b));

            if let Some(ref n) = self.type_parameters {
                docs.push(n.build(b));
            }

            if self.superclass.is_some() || self.interface.is_some() {
                docs.push(b.softline());
            }

            if let Some(ref n) = self.superclass {
                docs.push(n.build(b));
                if self.interface.is_some() {
                    docs.push(b.txt(" "));
                }
            }

            if let Some(ref n) = self.interface {
                docs.push(n.build(b));
            }

            docs.push(b.txt(" "));
            result.push(b.group_indent_concat(docs));

            result.push(self.body.build(b));
        });
    }
}

#[derive(Debug)]
pub struct MethodDeclaration {
    pub modifiers: Option<Modifiers>,
    pub type_: UnannotatedType,
    pub name: ValueNode,
    pub formal_parameters: FormalParameters,
    pub body: Option<Block>,
    //pub dimensions
    pub node_context: NodeContext,
}

impl MethodDeclaration {
    pub fn new(node: Node) -> Self {
        assert_check(node, "method_declaration");

        Self {
            modifiers: node.try_c_by_k("modifiers").map(|n| Modifiers::new(n)),
            type_: UnannotatedType::new(node.c_by_n("type")),
            name: ValueNode::new(node.c_by_n("name")),
            formal_parameters: FormalParameters::new(node.c_by_n("parameters")),
            body: node.try_c_by_n("body").map(|n| Block::new(n)),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for MethodDeclaration {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            if let Some(ref n) = self.modifiers {
                result.push(n.build(b));
            }

            result.push(self.type_.build(b));
            result.push(b.txt(" "));
            result.push(self.name.build(b));
            result.push(self.formal_parameters.build(b));

            if let Some(ref n) = self.body {
                result.push(b.txt(" "));
                let body_doc = n.build(b);
                result.push(body_doc);
            }
        });
    }
}

#[derive(Debug)]
pub struct FormalParameters {
    pub formal_parameters: Vec<FormalParameter>,
    pub node_context: NodeContext,
}

impl FormalParameters {
    pub fn new(node: Node) -> Self {
        assert_check(node, "formal_parameters");

        let formal_parameters = node
            .try_cs_by_k("formal_parameter")
            .into_iter()
            .map(FormalParameter::new)
            .collect();

        Self {
            formal_parameters,
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for FormalParameters {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            let parameters_doc = b.to_docs(&self.formal_parameters);

            let sep = Insertable::new::<&str>(None, None, Some(b.softline()));
            let open = Insertable::new(None, Some("("), Some(b.maybeline()));
            let close = Insertable::new(Some(b.maybeline()), Some(")"), None);
            let doc = b.group_surround(&parameters_doc, sep, open, close);
            result.push(doc);
        });
    }
}

#[derive(Debug)]
pub struct FormalParameter {
    pub modifiers: Option<Modifiers>,
    pub type_: UnannotatedType,
    pub name: ValueNode,
    pub dimensions: Option<Dimensions>,
    pub node_context: NodeContext,
}

impl FormalParameter {
    pub fn new(node: Node) -> Self {
        assert_check(node, "formal_parameter");

        Self {
            modifiers: node.try_c_by_k("modifiers").map(Modifiers::new),
            type_: UnannotatedType::new(node.c_by_n("type")),
            name: ValueNode::new(node.c_by_n("name")),
            dimensions: node.try_c_by_k("dimensions").map(Dimensions::new),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for FormalParameter {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            if let Some(ref n) = self.modifiers {
                result.push(n.build(b));
            }
            result.push(self.type_.build(b));
            result.push(b.txt(" "));
            result.push(self.name.build(b));
            if let Some(ref d) = self.dimensions {
                result.push(b.txt(" "));
                result.push(d.build(b));
            }
        });
    }
}

#[derive(Debug)]
pub struct SuperClass {
    pub type_: Type,
    pub node_context: NodeContext,
}

impl SuperClass {
    pub fn new(node: Node) -> Self {
        assert_check(node, "superclass");

        Self {
            type_: Type::new(node.first_c()),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for SuperClass {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(b.txt_("extends"));
            result.push(self.type_.build(b));
        });
    }
}

#[derive(Debug)]
pub struct Modifiers {
    annotations: Vec<Annotation>,
    modifiers: Vec<Modifier>,
    pub node_context: NodeContext,
}

impl Modifiers {
    pub fn new(node: Node) -> Self {
        assert_check(node, "modifiers");

        let annotations = node
            .try_cs_by_k("annotation")
            .into_iter()
            .map(Annotation::new)
            .collect();

        let modifiers = node
            .try_cs_by_k("modifier")
            .into_iter()
            .map(Modifier::new)
            .collect();

        Self {
            annotations,
            modifiers,
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for Modifiers {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments(b, &self.node_context, result, |b, result| {
            result.extend(self.annotations.iter().map(|n| n.build(b)));

            if !self.modifiers.is_empty() {
                let docs = b.to_docs(&self.modifiers);
                let sep = Insertable::new(None, Some(" "), None);
                result.push(b.intersperse(&docs, sep));
                result.push(b.txt(" "));
            }
        });
    }
}

#[derive(Debug)]
pub struct Modifier {
    kind: ModifierKind,
    node_context: NodeContext,
}

impl Modifier {
    pub fn new(node: Node) -> Self {
        assert_check(node, "modifier");

        Self {
            kind: ModifierKind::new(node.first_c()),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for Modifier {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(self.kind.build(b));
        });
    }
}

#[derive(Debug)]
pub struct Annotation {
    pub name: ValueNode,
    pub arguments: Option<AnnotationArgumentList>,
    pub node_context: NodeContext,
}

impl Annotation {
    pub fn new(node: Node) -> Self {
        assert_check(node, "annotation");

        let arguments = node
            .try_c_by_n("arguments")
            .map(AnnotationArgumentList::new);

        Self {
            name: ValueNode::new(node.c_by_n("name")),
            arguments,
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for Annotation {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(b.txt("@"));
            result.push(self.name.build(b));

            if let Some(a) = &self.arguments {
                result.push(a.build(b));
            }
        });
        result.push(b.nl());
    }
}

#[derive(Debug)]
pub struct AnnotationKeyValue {
    key: ValueNode,
    value: ValueNode,
    pub node_context: NodeContext,
}

impl AnnotationKeyValue {
    pub fn new(node: Node) -> Self {
        assert_check(node, "annotation_key_value");

        Self {
            key: ValueNode::new(node.c_by_n("key")),
            value: ValueNode::new(node.c_by_n("value")),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for AnnotationKeyValue {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(self.key.build(b));
            result.push(b.txt("="));
            result.push(self.value.build(b));
        });
    }
}

#[derive(Debug)]
pub struct ClassBody {
    pub class_members: Vec<BodyMember<ClassMember>>,
    pub node_context: NodeContext,
}

impl ClassBody {
    pub fn new(node: Node) -> Self {
        assert_check(node, "class_body");

        let class_members: Vec<_> = node
            .children_vec()
            .into_iter()
            .map(|n| BodyMember::new(&n, ClassMember::new(n)))
            .collect();
        let node_context = NodeContext::with_punctuation(&node);

        Self {
            class_members,
            node_context,
        }
    }
}

impl<'a> DocBuild<'a> for ClassBody {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        let bucket = get_comment_bucket(&self.node_context.id);
        handle_pre_comments(b, bucket, result);

        if bucket.dangling_comments.is_empty() {
            result.push(b.surround_body_members(&self.class_members, "{", "}"));
            handle_post_comments(b, bucket, result);
        } else {
            handle_dangling_comments_in_bracket_surround(b, bucket, result);
        }
    }
}

#[derive(Debug)]
pub struct FieldDeclaration {
    pub modifiers: Option<Modifiers>,
    pub type_: UnannotatedType,
    pub declarators: Vec<VariableDeclarator>,
    pub accessor_list: Option<AccessorList>,
    pub node_context: NodeContext,
}

impl FieldDeclaration {
    pub fn new(node: Node) -> Self {
        assert_check(node, "field_declaration");

        let modifiers = node.try_c_by_k("modifiers").map(|n| Modifiers::new(n));

        let declarators = node
            .cs_by_n("declarator")
            .into_iter()
            .map(|n| VariableDeclarator::new(n))
            .collect();

        let accessor_list = node
            .try_c_by_k("accessor_list")
            .map(|n| AccessorList::new(n));

        Self {
            modifiers,
            type_: UnannotatedType::new(node.c_by_n("type")),
            declarators,
            accessor_list,
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for FieldDeclaration {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            if let Some(ref n) = self.modifiers {
                result.push(n.build(b));
            }

            result.push(self.type_.build(b));
            result.push(b.txt(" "));

            let docs = b.to_docs(&self.declarators);

            // prevent unnessary indentation when only one element;
            let doc = if docs.len() == 1 {
                docs[0]
            } else {
                let sep = Insertable::new::<&str>(None, None, Some(b.softline()));
                b.group(b.indent(b.intersperse(&docs, sep)))
            };
            result.push(doc);

            if let Some(ref n) = self.accessor_list {
                result.push(b.txt(" "));
                result.push(n.build(b));
            }
        });
    }
}

#[derive(Debug)]
pub struct ArrayInitializer {
    initializers: Vec<VariableInitializer>,
    pub node_context: NodeContext,
}

impl ArrayInitializer {
    pub fn new(node: Node) -> Self {
        assert_check(node, "array_initializer");

        let initializers: Vec<_> = node
            .children_vec()
            .into_iter()
            .map(|n| VariableInitializer::new(n))
            .collect();

        Self {
            initializers,
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for ArrayInitializer {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            let docs = b.to_docs(&self.initializers);

            let sep = Insertable::new::<&str>(None, None, Some(b.softline()));
            let open = Insertable::new(None, Some("{"), Some(b.softline()));
            let close = Insertable::new(Some(b.softline()), Some("}"), None);
            let doc = b.group_surround(&docs, sep, open, close);
            result.push(doc);
        });
    }
}

#[derive(Debug)]
pub struct AssignmentExpression {
    pub left: AssignmentLeft,
    pub op: ValueNode,
    pub right: Expression,
    pub is_right_child_a_query_node: bool,
    pub node_context: NodeContext,
}

impl AssignmentExpression {
    pub fn new(node: Node) -> Self {
        assert_check(node, "assignment_expression");

        let right_child = node.c_by_n("right");

        Self {
            left: AssignmentLeft::new(node.c_by_n("left")),
            op: ValueNode::new(node.c_by_n("operator")),
            right: Expression::new(right_child),
            is_right_child_a_query_node: is_query_expression(&right_child),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for AssignmentExpression {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            let mut docs = vec![self.left.build(b), b.txt(" "), self.op.build(b)];
            if self.is_right_child_a_query_node {
                docs.push(b.txt(" "));
                docs.push(self.right.build(b));
                result.push(b.concat(docs));
            } else {
                docs.push(b.softline());
                docs.push(self.right.build(b));
                result.push(b.group_indent_concat(docs));
            }
        });
    }
}

#[derive(Debug)]
pub enum AssignmentLeft {
    Identifier(ValueNode),
    Field(FieldAccess),
    Array(ArrayAccess),
}

impl AssignmentLeft {
    pub fn new(node: Node) -> Self {
        match node.kind() {
            "identifier" => Self::Identifier(ValueNode::new(node)),
            "field_access" => Self::Field(FieldAccess::new(node)),
            "array_access" => Self::Array(ArrayAccess::new(node)),
            _ => panic_unknown_node(node, "AssignmentLeft"),
        }
    }
}

impl<'a> DocBuild<'a> for AssignmentLeft {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            Self::Identifier(n) => {
                result.push(n.build(b));
            }
            Self::Field(n) => {
                result.push(n.build(b));
            }
            Self::Array(n) => {
                result.push(n.build(b));
            }
        }
    }
}

#[derive(Debug)]
pub struct BoolType {
    pub node_context: NodeContext,
}

impl BoolType {
    pub fn new(node: Node) -> Self {
        assert_check(node, "boolean_type");

        Self {
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for BoolType {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(b.txt("boolean"));
        });
    }
}

#[derive(Debug)]
pub struct Block {
    pub statements: Vec<BodyMember<Statement>>,
    pub node_context: NodeContext,
}

impl Block {
    pub fn new(node: Node) -> Self {
        assert_check(node, "block");

        let statements: Vec<BodyMember<Statement>> = node
            .children_vec()
            .into_iter()
            .map(|n| BodyMember::new(&n, Statement::new(n)))
            .collect();
        let node_context = NodeContext::with_punctuation(&node);

        Self {
            statements,
            node_context,
        }
    }
}

impl<'a> DocBuild<'a> for Block {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        let bucket = get_comment_bucket(&self.node_context.id);
        handle_pre_comments(b, bucket, result);

        if bucket.dangling_comments.is_empty() {
            let docs = b.surround_body_members(&self.statements, "{", "}");
            result.push(docs);
        } else {
            handle_dangling_comments_in_bracket_surround(b, bucket, result);
            return;
        }
        handle_post_comments(b, bucket, result);
    }
}

#[derive(Debug)]
pub struct Interface {
    pub type_list: TypeList,
    pub node_context: NodeContext,
}

impl Interface {
    pub fn new(node: Node) -> Self {
        assert_check(node, "interfaces");

        Self {
            type_list: TypeList::new(node.c_by_k("type_list")),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for Interface {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            let doc = self.type_list.build(b);
            let impl_group = b.concat(vec![b.txt_("implements"), doc]);
            result.push(impl_group);
        });
    }
}

#[derive(Debug)]
pub struct TypeList {
    pub types: Vec<Type>,
    pub node_context: NodeContext,
}

impl TypeList {
    pub fn new(node: Node) -> Self {
        assert_check(node, "type_list");

        let types = node
            .children_vec()
            .into_iter()
            .map(|n| Type::new(n))
            .collect();

        Self {
            types,
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for TypeList {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            let docs = b.to_docs(&self.types);
            let sep = Insertable::new(None, Some(" "), None);
            let doc = b.intersperse(&docs, sep);
            result.push(doc);
        });
    }
}

#[derive(Debug)]
pub struct ChainingContext {
    pub is_parent_a_chaining_node: bool,
    pub is_top_most_in_a_chain: bool,
}

#[derive(Debug)]
pub enum ObjectExpression {
    Primary(Box<PrimaryExpression>),
    Super(Super),
}

impl ObjectExpression {
    pub fn new(node: Node) -> Self {
        //TODO: handle incoming comment node
        match node.kind() {
            "super" => Self::Super(Super::new(node)),
            _ => Self::Primary(Box::new(PrimaryExpression::new(node))),
        }
    }
}

impl<'a> DocBuild<'a> for ObjectExpression {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            Self::Primary(n) => {
                result.push(n.build(b));
            }
            Self::Super(n) => {
                result.push(n.build(b));
            }
        }
    }
}

#[derive(Debug)]
pub enum MethodInvocationKind {
    Simple {
        name: ValueNode,
        arguments: ArgumentList,
    },
    Complex {
        object: ObjectExpression,
        property_navigation: PropertyNavigation,
        type_arguments: Option<TypeArguments>,
        name: ValueNode,
        arguments: ArgumentList,
        context: Option<ChainingContext>,
    },
}

impl<'a> DocBuild<'a> for MethodInvocationKind {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            Self::Simple { name, arguments } => {
                result.push(name.build(b));
                result.push(arguments.build(b));
            }
            Self::Complex {
                object,
                property_navigation,
                type_arguments,
                name,
                arguments,
                context,
            } => {
                let mut docs = vec![];
                docs.push(object.build(b));

                // potential chaining scenario
                if let Some(context) = context {
                    if context.is_parent_a_chaining_node || context.is_top_most_in_a_chain {
                        docs.push(b.maybeline());
                    }

                    docs.push(property_navigation.build(b));

                    if let Some(ref n) = type_arguments {
                        docs.push(n.build(b));
                    }

                    docs.push(name.build(b));
                    docs.push(arguments.build(b));

                    if context.is_top_most_in_a_chain {
                        return result.push(b.group_indent_concat(docs));
                    }

                    result.push(b.concat(docs))
                } else {
                    docs.push(property_navigation.build(b));

                    if let Some(ref n) = type_arguments {
                        docs.push(n.build(b));
                    }
                    docs.push(name.build(b));
                    docs.push(arguments.build(b));
                    result.push(b.concat(docs))
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct MethodInvocation {
    pub kind: MethodInvocationKind,
    pub node_context: NodeContext,
}

impl MethodInvocation {
    pub fn new(node: Node) -> Self {
        assert_check(node, "method_invocation");

        let name = ValueNode::new(node.c_by_n("name"));
        let arguments = ArgumentList::new(node.c_by_n("arguments"));

        let kind = if let Some(obj) = node.try_c_by_n("object") {
            let object = ObjectExpression::new(obj);
            let next_named = obj.next_named();
            let property_navigation = if next_named.kind() == "safe_navigation_operator" {
                PropertyNavigation::Safe(SafeNavigationOperator::new(next_named))
            } else {
                PropertyNavigation::Dot
            };

            let type_arguments = node
                .try_c_by_k("type_arguments")
                .map(|n| TypeArguments::new(n));
            let context = build_chaining_context(&node);

            MethodInvocationKind::Complex {
                object,
                property_navigation,
                type_arguments,
                name,
                arguments,
                context,
            }
        } else {
            MethodInvocationKind::Simple { name, arguments }
        };

        Self {
            kind,
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for MethodInvocation {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(self.kind.build(b));
        });
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct TypeArguments {
    pub types: Vec<Type>,
    pub node_context: NodeContext,
}

impl TypeArguments {
    pub fn new(node: Node) -> Self {
        let types = node
            .children_vec()
            .into_iter()
            .map(|n| Type::new(n))
            .collect();

        Self {
            types,
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for TypeArguments {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            let docs = b.to_docs(&self.types);
            let sep = Insertable::new(None, Some(" "), None);
            let open = Insertable::new(None, Some("<"), None);
            let close = Insertable::new(None, Some(">"), None);
            let doc = b.surround(&docs, sep, open, close);
            result.push(doc);
        });
    }
}

#[derive(Debug)]
pub struct ArgumentList {
    pub expressions: Vec<Expression>,
    pub node_context: NodeContext,
}

impl ArgumentList {
    pub fn new(node: Node) -> Self {
        let expressions = node
            .children_vec()
            .into_iter()
            .map(|n| Expression::new(n))
            .collect();

        Self {
            expressions,
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for ArgumentList {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            let docs = b.to_docs(&self.expressions);

            let sep = Insertable::new::<&str>(None, None, Some(b.softline()));
            let open = Insertable::new(None, Some("("), Some(b.maybeline()));
            let close = Insertable::new(Some(b.maybeline()), Some(")"), None);
            let doc = b.group_surround(&docs, sep, open, close);
            result.push(doc);
        });
    }
}

#[derive(Debug)]
pub struct Super {
    pub node_context: NodeContext,
}

impl Super {
    pub fn new(node: Node) -> Self {
        assert_check(node, "super");

        Self {
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for Super {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(b.txt("super"))
        });
    }
}

#[derive(Debug)]
pub struct This {
    pub node_context: NodeContext,
}

impl This {
    pub fn new(node: Node) -> Self {
        assert_check(node, "this");

        Self {
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for This {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(b.txt("this"))
        });
    }
}

#[derive(Debug)]
pub struct BinaryExpressionContext {
    is_a_chaining_inner_node: bool,
    has_parent_same_precedence: bool,
    is_parent_return_statement: bool,
}

#[derive(Debug)]
pub struct BinaryExpression {
    pub left: Expression,
    pub op: String,
    pub right: Expression,
    pub context: BinaryExpressionContext,
    pub node_context: NodeContext,
}

impl BinaryExpression {
    fn build_context(node: &Node) -> BinaryExpressionContext {
        let op = node.c_by_n("operator").kind();
        let precedence = get_precedence(op);
        let parent = node
            .parent()
            .expect("BinaryExpression node should always have a parent");

        let is_a_chaining_inner_node = is_binary_exp(&parent);
        let has_parent_same_precedence = is_binary_exp(&parent)
            && precedence == get_precedence(parent.c_by_n("operator").kind());

        BinaryExpressionContext {
            has_parent_same_precedence,
            is_a_chaining_inner_node,
            is_parent_return_statement: parent.kind() == "return_statement",
        }
    }

    pub fn new(node: Node) -> Self {
        assert_check(node, "binary_expression");

        Self {
            left: Expression::new(node.c_by_n("left")),
            op: node.c_by_n("operator").kind().to_string(),
            right: Expression::new(node.c_by_n("right")),
            context: Self::build_context(&node),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for BinaryExpression {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            let left_doc = self.left.build(b);
            //let op_doc = self.op.build(b);
            let op_doc = b.txt(&self.op);
            let right_doc = self.right.build(b);

            let context = &self.context;

            // chaining case: deligate to the parent to handle group() or align()
            if context.has_parent_same_precedence {
                return result.push(b.concat(vec![
                    left_doc,
                    b.softline(),
                    op_doc,
                    b.txt(" "),
                    right_doc,
                ]));
            }

            // group() using the current line indent level
            if !context.is_a_chaining_inner_node && !context.is_parent_return_statement {
                return result.push(b.group_concat(vec![
                    left_doc,
                    b.softline(),
                    op_doc,
                    b.txt(" "),
                    right_doc,
                ]));
            }

            // otherwise:
            result.push(b.group_indent_concat(vec![
                left_doc,
                b.softline(),
                op_doc,
                b.txt(" "),
                right_doc,
            ]))
        });
    }
}

#[derive(Debug)]
pub struct LocalVariableDeclaration {
    pub modifiers: Option<Modifiers>,
    pub type_: UnannotatedType,
    pub declarators: Vec<VariableDeclarator>,
    //pub is_parent_for_statement: bool,
    pub node_context: NodeContext,
}

impl LocalVariableDeclaration {
    pub fn new(node: Node) -> Self {
        assert_check(node, "local_variable_declaration");

        let modifiers = node.try_c_by_k("modifiers").map(|n| Modifiers::new(n));
        let declarators = node
            .cs_by_n("declarator")
            .into_iter()
            .map(|n| VariableDeclarator::new(n))
            .collect();

        Self {
            modifiers,
            type_: UnannotatedType::new(node.c_by_n("type")),
            declarators,
            //is_parent_for_statement: node.parent().is_some_and(|n| n.kind() == "for_statement"),
            node_context: NodeContext::without_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for LocalVariableDeclaration {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            if let Some(ref n) = self.modifiers {
                result.push(n.build(b));
            }

            result.push(self.type_.build(b));
            result.push(b.txt(" "));

            let docs = b.to_docs(&self.declarators);

            // prevent unnessary indentation when only one element;
            let doc = if docs.len() == 1 {
                docs[0]
            } else {
                let sep = Insertable::new::<&str>(None, None, Some(b.softline()));
                b.group(b.indent(b.intersperse(&docs, sep)))
            };

            result.push(doc);
        });
    }
}

#[derive(Debug)]
pub struct VariableDeclarator {
    pub name: ValueNode,
    //pub dimenssions
    pub op: Option<ValueNode>,
    pub value: Option<VariableInitializer>,
    pub is_value_child_a_query_node: bool,
    pub node_context: NodeContext,
}

impl VariableDeclarator {
    pub fn new(node: Node) -> Self {
        assert_check(node, "variable_declarator");

        let mut is_value_child_a_query_node = false;
        let op = node.try_c_by_k("assignment_operator").map(ValueNode::new);
        let value = node.try_c_by_n("value").map(|n| {
            is_value_child_a_query_node = is_query_expression(&n);
            VariableInitializer::Exp(Expression::new(n))
        });

        Self {
            name: ValueNode::new(node.c_by_n("name")),
            op,
            value,
            is_value_child_a_query_node,
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for VariableDeclarator {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        // TODO: handle dotted expression use-case
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            let mut docs = vec![self.name.build(b)];

            if self.value.is_none() {
                result.push(b.concat(docs));
                return;
            }

            let value = self.value.as_ref().unwrap();
            docs.push(b.txt(" "));
            if let Some(ref n) = self.op {
                docs.push(n.build(b));
            }

            if self.is_value_child_a_query_node {
                docs.push(b.txt(" "));
                docs.push(value.build(b));
                result.push(b.concat(docs));
            } else {
                docs.push(b.softline());
                docs.push(value.build(b));
                result.push(b.group_indent_concat(docs));
            }
        });
    }
}

#[derive(Debug)]
pub struct GenericType {
    pub generic_identifier: GenericIdentifier,
    pub type_arguments: TypeArguments,
    pub node_context: NodeContext,
}

impl GenericType {
    pub fn new(node: Node) -> Self {
        assert_check(node, "generic_type");

        let generic_identifier = if let Some(t) = node.try_c_by_k("type_identifier") {
            GenericIdentifier::Type(t.value())
        } else if let Some(s) = node.try_c_by_k("scoped_type_identifier") {
            GenericIdentifier::Scoped(ScopedTypeIdentifier::new(s))
        } else {
            panic!("## can't build generic_identifier node in GenericType");
        };

        Self {
            generic_identifier,
            type_arguments: TypeArguments::new(node.c_by_k("type_arguments")),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for GenericType {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(self.generic_identifier.build(b));
            result.push(self.type_arguments.build(b));
        });
    }
}

#[derive(Debug)]
pub enum GenericIdentifier {
    Type(String),
    Scoped(ScopedTypeIdentifier),
}

impl<'a> DocBuild<'a> for GenericIdentifier {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            Self::Type(s) => {
                result.push(b.txt(s));
            }
            Self::Scoped(s) => {
                result.push(s.build(b));
            }
        }
    }
}

#[derive(Debug)]
pub struct ElseClause {
    pub else_node: ValueNode, // The 'else' keyword node
    pub statement: Statement, // The actual else-block or nested if
}

#[derive(Debug)]
pub struct IfStatement {
    pub condition: ParenthesizedExpression,
    pub consequence: Statement,
    pub alternative: Option<ElseClause>,
    pub node_context: NodeContext,
}

impl IfStatement {
    pub fn new(node: Node) -> Self {
        assert_check(node, "if_statement");

        let alternative = node.try_c_by_n("alternative").map(|alt_stmt| {
            let else_node = node
                .children(&mut node.walk())
                .find(|n| n.kind() == "else")
                .expect("a mandatory `else` node is missing");

            ElseClause {
                else_node: ValueNode::new(else_node),
                statement: Statement::new(alt_stmt),
            }
        });

        Self {
            condition: ParenthesizedExpression::new(node.c_by_n("condition")),
            consequence: Statement::new(node.c_by_n("consequence")),
            alternative,
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for IfStatement {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(b.txt("if "));
            result.push(self.condition.build(b));

            if self.consequence.is_block() {
                result.push(b.txt(" "));
                result.push(self.consequence.build(b));
            } else {
                result.push(b.indent(b.nl()));
                result.push(b.indent(self.consequence.build(b)));
            }

            // Handle the 'else' part
            if let Some(ref alt) = self.alternative {
                if self.consequence.is_block() {
                    result.push(b.txt(" "));

                    result.push(alt.else_node.build(b));
                    result.push(b.txt(" "));
                } else {
                    result.push(b.nl());

                    result.push(alt.else_node.build(b));

                    if !matches!(alt.statement, Statement::If(_) | Statement::Block(_)) {
                        result.push(b.indent(b.nl()));
                    } else {
                        result.push(b.txt(" "));
                    }
                }
                result.push(alt.statement.build(b));
            }
        });
    }
}

#[derive(Debug)]
pub struct ParenthesizedExpression {
    pub exp: Expression,
    pub node_context: NodeContext,
}

impl ParenthesizedExpression {
    pub fn new(node: Node) -> Self {
        Self {
            exp: Expression::new(node.first_c()),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for ParenthesizedExpression {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            // to align with prettier apex
            result.push(b.txt("("));
            let doc = b.concat(vec![
                b.indent(b.maybeline()),
                b.indent(self.exp.build(b)),
                b.maybeline(),
            ]);
            result.push(b.group(doc));
            result.push(b.txt(")"));
        });
    }
}

#[derive(Debug)]
pub enum ForInitOption {
    Declaration(LocalVariableDeclaration),
    Exps(Vec<Expression>),
}

impl ForInitOption {
    pub fn new(node: Node) -> Self {
        match node.kind() {
            "local_variable_declaration" => Self::Declaration(LocalVariableDeclaration::new(node)),
            _ => Self::Exps(
                node.parent()
                    .expect("node must have parent in ForInitOption")
                    .cs_by_n("init")
                    .into_iter()
                    .map(|n| Expression::new(n))
                    .collect(),
            ),
        }
    }
}

impl<'a> DocBuild<'a> for ForInitOption {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            Self::Declaration(n) => {
                result.push(n.build(b));
            }
            Self::Exps(exps) => {
                let docs = b.to_docs(exps);
                let sep = Insertable::new::<&str>(None, None, Some(b.softline()));
                let doc = b.group(b.intersperse(&docs, sep));
                result.push(doc);
            }
        }
    }
}

#[derive(Debug)]
pub struct ForStatement {
    pub init: Option<ForInitOption>,
    pub condition: Option<Expression>,
    pub update: Option<Expression>,
    pub body: Statement,
    pub semicolons: Vec<Punctuation>,
    pub node_context: NodeContext,
}

impl ForStatement {
    pub fn new(node: Node) -> Self {
        assert_check(node, "for_statement");

        // when init or condition is None, the semicolon is immediate child of
        // ForStatement, for which case we need to collect and print.
        let mut punc_count = 0;

        let init = node.try_c_by_n("init").map(|n| ForInitOption::new(n));
        let condition = node.try_c_by_n("condition").map(|n| Expression::new(n));
        let update = node.try_c_by_n("update").map(|n| Expression::new(n));

        let semicolons = Self::build_semi_colons(&node);

        if init.is_none() {
            punc_count += 1;
        }
        if condition.is_none() {
            punc_count += 1;
        }

        if punc_count > semicolons.len() {
            panic!("## ForStatement missing semicolons.");
        }

        Self {
            init,
            condition,
            update,
            body: Statement::new(node.c_by_n("body")),
            semicolons,
            node_context: NodeContext::with_punctuation(&node),
        }
    }

    fn build_semi_colons(node: &Node) -> Vec<Punctuation> {
        let mut cursor = node.walk();
        node.children(&mut cursor)
            .filter(|c| c.kind() == ";")
            .map(|c| Punctuation::new(c))
            .collect()
    }
}

impl<'a> DocBuild<'a> for ForStatement {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(b.txt("for "));

            let mut semicolons_iter = self.semicolons.iter();

            let init = match &self.init {
                Some(i) => i.build(b),
                None => semicolons_iter.next().unwrap().build(b),
            };

            let condition = match &self.condition {
                Some(c) => b.concat(vec![b.txt(" "), c.build(b)]),
                None => semicolons_iter.next().unwrap().build(b),
            };

            let update = match &self.update {
                Some(u) => b.concat(vec![b.txt(" "), u.build(b)]),
                None => b.nil(),
            };

            let docs = vec![init, condition, update];

            let sep = Insertable::new::<&str>(None, None, Some(b.maybeline()));
            let open = Insertable::new(None, Some("("), Some(b.maybeline()));
            let close = Insertable::new(Some(b.maybeline()), Some(")"), None);
            let doc = b.group_surround(&docs, sep, open, close);
            result.push(doc);

            match self.body {
                Statement::SemiColumn => result.push(b.txt(";")),
                _ => {
                    result.push(b.txt(" "));
                    result.push(self.body.build(b));
                }
            }
        });
    }
}

#[derive(Debug)]
pub struct EnhancedForStatement {
    //pub modifiers: Option<Modifiers>,
    pub type_: UnannotatedType,
    pub name: ValueNode,
    //pub dimension
    pub value: Expression,
    pub body: Statement,
    pub node_context: NodeContext,
}

impl EnhancedForStatement {
    pub fn new(node: Node) -> Self {
        assert_check(node, "enhanced_for_statement");

        //let modifiers = node.try_c_by_k("modifiers").map(|n| Modifiers::new(n));

        Self {
            //modifiers,
            type_: UnannotatedType::new(node.c_by_n("type")),
            name: ValueNode::new(node.c_by_n("name")),
            value: Expression::new(node.c_by_n("value")),
            body: Statement::new(node.c_by_n("body")),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for EnhancedForStatement {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(b.txt("for ("));
            result.push(self.type_.build(b));
            result.push(b.txt(" "));
            result.push(self.name.build(b));
            result.push(b._txt_(":"));
            result.push(self.value.build(b));
            result.push(b.txt(")"));
            match self.body {
                Statement::SemiColumn => result.push(b.txt(";")),
                _ => {
                    result.push(b.txt(" "));
                    result.push(self.body.build(b));
                }
            }
        });
    }
}
#[derive(Debug)]
pub enum UpdateExpressionVariant {
    Pre {
        operator: String,
        operand: Box<Expression>,
    },
    Post {
        operand: Box<Expression>,
        operator: String,
    },
}

impl UpdateExpressionVariant {
    pub fn new(node: Node) -> Self {
        assert_check(node, "update_expression");

        let operator_node = node.c_by_n("operator");
        let operand_node = node.c_by_n("operand");

        if operator_node.start_byte() < operand_node.start_byte() {
            Self::Pre {
                operator: operator_node.value(),
                operand: Box::new(Expression::new(operand_node)),
            }
        } else {
            Self::Post {
                operand: Box::new(Expression::new(operand_node)),
                operator: operator_node.value(),
            }
        }
    }
}

impl<'a> DocBuild<'a> for UpdateExpressionVariant {
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

#[derive(Debug)]
pub struct ScopedTypeIdentifier {
    pub scoped_choice: ScopedChoice,
    pub annotations: Vec<Annotation>,
    pub type_identifier: String,
    pub node_context: NodeContext,
}

impl ScopedTypeIdentifier {
    pub fn new(node: Node) -> Self {
        assert_check(node, "scoped_type_identifier");

        let prefix_node = node.first_c();
        let scoped_choice = match prefix_node.kind() {
            "type_identifier" => ScopedChoice::TypeIdentifier(prefix_node.value()),
            "scoped_type_identifier" => ScopedChoice::Scoped(Box::new(Self::new(prefix_node))),
            "generic_type" => ScopedChoice::Generic(Box::new(GenericType::new(prefix_node))),
            _ => panic_unknown_node(prefix_node, "ScopedTypeIdentifier"),
        };

        let annotations: Vec<_> = node
            .try_cs_by_k("annotation")
            .into_iter()
            .map(|n| Annotation::new(n))
            .collect();

        let type_identifier_node = node
            .cs_by_k("type_identifier")
            .pop()
            .expect("## mandatory node type_identifier missing in ScopedTypeIdentifier");

        Self {
            scoped_choice,
            annotations,
            type_identifier: type_identifier_node.value(),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for ScopedTypeIdentifier {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(self.scoped_choice.build(b));
            result.push(b.txt("."));
            if !self.annotations.is_empty() {
                let docs = b.to_docs(&self.annotations);
                let sep = Insertable::new(None, Some(" "), None);
                result.push(b.intersperse(&docs, sep));
                result.push(b.txt(" "));
            }
            result.push(b.txt(&self.type_identifier));
        });
    }
}

#[derive(Debug)]
pub enum ScopedChoice {
    TypeIdentifier(String),
    Scoped(Box<ScopedTypeIdentifier>),
    Generic(Box<GenericType>),
}

impl<'a> DocBuild<'a> for ScopedChoice {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            Self::TypeIdentifier(t) => {
                result.push(b.txt(t));
            }
            Self::Scoped(s) => {
                result.push(s.build(b));
            }
            Self::Generic(g) => {
                result.push(g.build(b));
            }
        }
    }
}

#[derive(Debug)]
pub struct ConstructorDeclaration {
    pub modifiers: Option<Modifiers>,
    pub type_parameters: Option<TypeParameters>,
    pub name: ValueNode,
    pub parameters: FormalParameters,
    pub body: ConstructorBody,
    pub node_context: NodeContext,
}

impl ConstructorDeclaration {
    pub fn new(node: Node) -> Self {
        let modifiers = node.try_c_by_k("modifiers").map(|n| Modifiers::new(n));
        let type_parameters = node
            .try_c_by_k("type_parameters")
            .map(|n| TypeParameters::new(n));

        Self {
            modifiers,
            type_parameters,
            name: ValueNode::new(node.c_by_n("name")),
            parameters: FormalParameters::new(node.c_by_n("parameters")),
            body: ConstructorBody::new(node.c_by_n("body")),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for ConstructorDeclaration {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            if let Some(ref n) = self.modifiers {
                result.push(n.build(b));
            }
            if let Some(ref n) = self.type_parameters {
                result.push(n.build(b));
            }

            result.push(self.name.build(b));
            result.push(self.parameters.build(b));
            result.push(b.txt(" "));
            result.push(self.body.build(b));
        });
    }
}

#[derive(Debug)]
pub struct ConstructorBody {
    pub constructor_invocation: Option<BodyMember<ConstructInvocation>>,
    pub statements: Vec<BodyMember<Statement>>,
    pub node_context: NodeContext,
}

impl ConstructorBody {
    pub fn new(node: Node) -> Self {
        let mut constructor_invocation = None;
        let mut statements: Vec<BodyMember<Statement>> = Vec::new();

        for (i, c) in node.children_vec().into_iter().enumerate() {
            if i == 0 && c.kind() == "explicit_constructor_invocation" {
                constructor_invocation = Some(BodyMember::new(&c, ConstructInvocation::new(c)));
            } else {
                statements.push(BodyMember::new(&c, Statement::new(c)));
            }
        }

        Self {
            constructor_invocation,
            statements,
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for ConstructorBody {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        let bucket = get_comment_bucket(&self.node_context.id);
        handle_pre_comments(b, bucket, result);

        if bucket.dangling_comments.is_empty() {
            if self.constructor_invocation.is_none() && self.statements.is_empty() {
                return result.push(b.concat(vec![b.txt("{"), b.nl(), b.txt("}")]));
            }

            result.push(b.txt("{"));

            if let Some(c) = &self.constructor_invocation {
                result.push(b.indent(b.concat(vec![b.nl(), c.member.build(b)])));

                if !self.statements.is_empty() {
                    if c.has_trailing_newline {
                        result.push(b.empty_new_line());
                    } else {
                        result.push(b.indent(b.nl()));
                    }
                }
            } else {
                result.push(b.indent(b.nl()));
            }

            result.push(b.indent(b.intersperse_body_members(&self.statements)));
            result.push(b.nl());
            result.push(b.txt("}"));
        } else {
            handle_dangling_comments_in_bracket_surround(b, bucket, result);
        }
    }
}

#[derive(Debug)]
pub struct ConstructInvocation {
    pub object: Option<Box<PrimaryExpression>>,
    pub type_arguments: Option<TypeArguments>,
    pub constructor: Option<Constructor>,
    pub arguments: ArgumentList,
    pub node_context: NodeContext,
}

impl ConstructInvocation {
    pub fn new(node: Node) -> Self {
        let object = node
            .try_c_by_n("object")
            .map(|n| Box::new(PrimaryExpression::new(n)));

        let type_arguments = node
            .try_c_by_k("type_arguments")
            .map(|n| TypeArguments::new(n));

        let constructor = node.try_c_by_n("constructor").map(|n| match n.kind() {
            "this" => Constructor::This,
            "super" => Constructor::Super,
            _ => panic_unknown_node(n, "Constructor"),
        });

        Self {
            object,
            type_arguments,
            constructor,
            arguments: ArgumentList::new(node.c_by_n("arguments")),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for ConstructInvocation {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            if let Some(ref o) = self.object {
                result.push(o.build(b));
            }

            if let Some(ref t) = self.type_arguments {
                result.push(t.build(b));
            }

            if let Some(ref c) = self.constructor {
                result.push(c.build(b));
            }

            result.push(self.arguments.build(b));
        });
    }
}

#[derive(Debug)]
pub enum Constructor {
    This,
    Super,
}

impl<'a> DocBuild<'a> for Constructor {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            Self::This => result.push(b.txt("this")),
            Self::Super => result.push(b.txt("super")),
        }
    }
}

#[derive(Debug)]
pub struct TypeParameters {
    pub type_parameters: Vec<TypeParameter>,
    pub node_context: NodeContext,
}

impl TypeParameters {
    pub fn new(node: Node) -> Self {
        assert_check(node, "type_parameters");

        let type_parameters: Vec<_> = node
            .cs_by_k("type_parameter")
            .into_iter()
            .map(|n| TypeParameter::new(n))
            .collect();

        Self {
            type_parameters,
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for TypeParameters {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            let docs = b.to_docs(&self.type_parameters);

            let sep = Insertable::new::<&str>(None, None, Some(b.softline()));
            let open = Insertable::new(None, Some("<"), Some(b.maybeline()));
            let close = Insertable::new(Some(b.maybeline()), Some(">"), None);
            let doc = b.group_surround(&docs, sep, open, close);
            result.push(doc);
        });
    }
}

#[derive(Debug)]
pub struct TypeParameter {
    annotations: Vec<Annotation>,
    pub type_identifier: String,
    pub node_context: NodeContext,
}

impl TypeParameter {
    pub fn new(node: Node) -> Self {
        let annotations: Vec<_> = node
            .try_cs_by_k("annotation")
            .into_iter()
            .map(|n| Annotation::new(n))
            .collect();

        Self {
            annotations,
            type_identifier: node.cvalue_by_k("type_identifier"),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for TypeParameter {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            if !self.annotations.is_empty() {
                let docs = b.to_docs(&self.annotations);
                let sep = Insertable::new(None, Some(" "), None);
                result.push(b.intersperse(&docs, sep));
                result.push(b.txt(" "));
            }
            result.push(b.txt(&self.type_identifier));
        });
    }
}

#[derive(Debug)]
pub struct ObjectCreationExpression {
    pub type_arguments: Option<TypeArguments>,
    pub type_: UnannotatedType,
    pub arguments: ArgumentList,
    pub class_body: Option<ClassBody>,
    pub node_context: NodeContext,
}

impl ObjectCreationExpression {
    pub fn new(node: Node) -> Self {
        assert_check(node, "object_creation_expression");

        let type_arguments = node
            .try_c_by_k("type_arguments")
            .map(|n| TypeArguments::new(n));
        let class_body = node.try_c_by_k("class_body").map(|n| ClassBody::new(n));

        Self {
            type_arguments,
            type_: UnannotatedType::new(node.c_by_n("type")),
            arguments: ArgumentList::new(node.c_by_n("arguments")),
            class_body,
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for ObjectCreationExpression {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(b.txt("new"));
            if let Some(t) = &self.type_arguments {
                result.push(t.build(b));
            }

            result.push(b.txt(" "));
            result.push(self.type_.build(b));
            result.push(self.arguments.build(b));

            if let Some(c) = &self.class_body {
                result.push(c.build(b));
            }
        });
    }
}

#[derive(Debug)]
pub struct RunAsStatement {
    pub user: ParenthesizedExpression,
    pub block: Block,
    pub node_context: NodeContext,
}

impl RunAsStatement {
    pub fn new(node: Node) -> Self {
        assert_check(node, "run_as_statement");

        Self {
            user: ParenthesizedExpression::new(node.c_by_n("user")),
            block: Block::new(node.c_by_k("block")),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for RunAsStatement {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(b.txt("System.runAs"));
            result.push(self.user.build(b));
            result.push(b.txt(" "));
            result.push(self.block.build(b));
        });
    }
}

#[derive(Debug)]
pub struct DoStatement {
    pub body: Block,
    pub condition: ParenthesizedExpression,
    pub node_context: NodeContext,
}

impl DoStatement {
    pub fn new(node: Node) -> Self {
        assert_check(node, "do_statement");

        Self {
            body: Block::new(node.c_by_n("body")),
            condition: ParenthesizedExpression::new(node.c_by_n("condition")),

            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for DoStatement {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(b.txt_("do"));
            result.push(self.body.build(b));
            result.push(b._txt_("while"));
            result.push(self.condition.build(b));
        });
    }
}

#[derive(Debug)]
pub struct WhileStatement {
    pub condition: ParenthesizedExpression,
    pub body: Statement,
    pub node_context: NodeContext,
}

impl WhileStatement {
    pub fn new(node: Node) -> Self {
        assert_check(node, "while_statement");

        Self {
            condition: ParenthesizedExpression::new(node.c_by_n("condition")),
            body: Statement::new(node.c_by_n("body")),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for WhileStatement {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments(b, &self.node_context, result, |b, result| {
            result.push(b.txt_("while"));
            result.push(self.condition.build(b));

            match self.body {
                Statement::SemiColumn => {}
                _ => {
                    result.push(b.txt(" "));
                    result.push(self.body.build(b));
                }
            }
        });
    }
}

#[derive(Debug)]
pub struct UnaryExpression {
    pub operator: String,
    pub operand: Box<Expression>,
    pub node_context: NodeContext,
}

impl UnaryExpression {
    pub fn new(node: Node) -> Self {
        assert_check(node, "unary_expression");

        let operator = node.cvalue_by_n("operator");
        Self {
            operator,
            operand: Box::new(Expression::new(node.c_by_n("operand"))),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for UnaryExpression {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(b.txt(&self.operator));
            result.push(self.operand.build(b));
        });
    }
}

#[derive(Debug)]
pub struct FieldAccess {
    pub object: MethodObject,
    pub property_navigation: PropertyNavigation,
    pub field: FieldOption,
    pub context: Option<ChainingContext>,
    pub node_context: NodeContext,
}

impl FieldAccess {
    pub fn new(node: Node) -> Self {
        assert_check(node, "field_access");

        let obj_node = node.c_by_n("object");
        let object = if obj_node.kind() == "super" {
            MethodObject::Super(Super::new(obj_node))
        } else {
            MethodObject::Primary(Box::new(PrimaryExpression::new(obj_node)))
        };

        Self {
            object,
            property_navigation: Self::get_property_navigation(&node),
            field: FieldOption::new(node.c_by_n("field")),
            context: build_chaining_context(&node),
            node_context: NodeContext::with_punctuation(&node),
        }
    }

    fn get_property_navigation(parent_node: &Node) -> PropertyNavigation {
        let property_navigation =
            if let Some(n) = parent_node.try_c_by_k("safe_navigation_operator") {
                PropertyNavigation::Safe(SafeNavigationOperator::new(n))
            } else {
                PropertyNavigation::Dot
            };
        property_navigation
    }
}

impl<'a> DocBuild<'a> for FieldAccess {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            let mut docs = vec![];

            docs.push(self.object.build(b));

            if let Some(ref context) = self.context {
                if context.is_parent_a_chaining_node || context.is_top_most_in_a_chain {
                    docs.push(b.maybeline());
                }
            }

            docs.push(self.property_navigation.build(b));
            docs.push(self.field.build(b));

            if self
                .context
                .as_ref()
                .is_some_and(|context| context.is_top_most_in_a_chain)
            {
                result.push(b.group_indent_concat(docs));
            } else {
                result.push(b.concat(docs));
            }
        });
    }
}

#[derive(Debug)]
pub enum FieldOption {
    This(This),
    Identifier(ValueNode),
}

impl FieldOption {
    pub fn new(node: Node) -> Self {
        match node.kind() {
            "this" => Self::This(This::new(node)),
            _ => Self::Identifier(ValueNode::new(node)),
        }
    }
}

impl<'a> DocBuild<'a> for FieldOption {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            Self::This(n) => {
                result.push(n.build(b));
            }
            Self::Identifier(n) => {
                result.push(n.build(b));
            }
        }
    }
}

#[derive(Debug)]
pub struct EnumDeclaration {
    pub modifiers: Option<Modifiers>,
    pub name: ValueNode,
    pub interface: Option<Interface>,
    pub body: EnumBody,
    pub node_context: NodeContext,
}

impl EnumDeclaration {
    pub fn new(node: Node) -> Self {
        let modifiers = node.try_c_by_k("modifiers").map(|n| Modifiers::new(n));
        let interface = node.try_c_by_k("interfaces").map(|n| Interface::new(n));

        Self {
            modifiers,
            name: ValueNode::new(node.c_by_n("name")),
            interface,
            body: EnumBody::new(node.c_by_n("body")),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for EnumDeclaration {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            if let Some(ref n) = self.modifiers {
                result.push(n.build(b));
            }
            result.push(b.txt_("enum"));
            result.push(self.name.build(b));
            result.push(b.txt(" "));

            if let Some(ref n) = self.interface {
                result.push(n.build(b));
            }
            result.push(self.body.build(b));
        });
    }
}

#[derive(Debug)]
pub struct EnumBody {
    enum_constants: Vec<EnumConstant>,
    pub node_context: NodeContext,
}

impl EnumBody {
    pub fn new(node: Node) -> Self {
        assert_check(node, "enum_body");

        let enum_constants = node
            .try_cs_by_k("enum_constant")
            .into_iter()
            .map(|n| EnumConstant::new(n))
            .collect();

        Self {
            enum_constants,
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for EnumBody {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        let bucket = get_comment_bucket(&self.node_context.id);
        handle_pre_comments(b, bucket, result);

        if bucket.dangling_comments.is_empty() {
            let docs = b.to_docs(&self.enum_constants);

            if docs.is_empty() {
                return result.push(b.concat(vec![b.txt("{"), b.nl(), b.txt("}")]));
            }

            let sep = Insertable::new::<&str>(None, None, Some(b.nl()));
            let open = Insertable::new(None, Some("{"), Some(b.nl()));
            let close = Insertable::new(Some(b.nl()), Some("}"), None);
            let doc = b.group_surround(&docs, sep, open, close);
            result.push(doc);
            handle_post_comments(b, bucket, result);
        } else {
            handle_dangling_comments_in_bracket_surround(b, bucket, result);
        }
    }
}

#[derive(Debug)]
pub struct EnumConstant {
    pub modifiers: Option<Modifiers>,
    pub name: ValueNode,
    pub node_context: NodeContext,
}

impl EnumConstant {
    pub fn new(node: Node) -> Self {
        let modifiers = node.try_c_by_k("modifiers").map(|n| Modifiers::new(n));

        Self {
            modifiers,
            name: ValueNode::new(node.c_by_n("name")),

            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for EnumConstant {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            if let Some(ref n) = self.modifiers {
                result.push(n.build(b));
            }
            result.push(self.name.build(b));
        });
    }
}

#[derive(Debug)]
pub enum DmlExpressionVariant {
    Basic {
        dml_type: DmlType,
        security_mode: Option<DmlSecurityMode>,
        target: Expression,
    },
    Upsert {
        dml_type: DmlType,
        security_mode: Option<DmlSecurityMode>,
        target: Expression,
        unannotated: Option<Box<UnannotatedType>>,
    },
    Merge {
        dml_type: DmlType,
        security_mode: Option<DmlSecurityMode>,
        target: Expression,
        merge_with: Expression,
    },
}

impl DmlExpressionVariant {
    pub fn new(node: Node) -> Self {
        let security_mode = node
            .try_c_by_k("dml_security_mode")
            .map(|n| DmlSecurityMode::new(n));
        let target = Expression::new(node.c_by_n("target"));

        let dml_type = DmlType::new(node.c_by_k("dml_type"));
        match dml_type.variant {
            DmlTypeVariant::Merge => Self::Merge {
                dml_type,
                security_mode,
                target,
                merge_with: Expression::new(node.c_by_n("merge_with")),
            },
            DmlTypeVariant::Upsert => {
                let unannotated = node
                    .try_c_by_n("upsert_key")
                    .map(|n| Box::new(UnannotatedType::new(n)));
                Self::Upsert {
                    dml_type,
                    security_mode,
                    target,
                    unannotated,
                }
            }
            _ => Self::Basic {
                dml_type,
                security_mode,
                target,
            },
        }
    }
}

impl<'a> DocBuild<'a> for DmlExpressionVariant {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            Self::Basic {
                dml_type,
                security_mode,
                target: exp,
            } => {
                result.push(dml_type.build(b));
                result.push(b.txt(" "));
                if let Some(ref s) = security_mode {
                    result.push(s.build(b));
                    result.push(b.txt(" "));
                }
                result.push(exp.build(b));
            }
            Self::Merge {
                dml_type,
                security_mode,
                target: exp,
                merge_with: exp_extra,
            } => {
                result.push(dml_type.build(b));
                result.push(b.txt(" "));
                if let Some(ref s) = security_mode {
                    result.push(s.build(b));
                    result.push(b.txt(" "));
                }

                let docs = b.to_docs(vec![exp, exp_extra]);
                let sep = Insertable::new::<&str>(None, None, Some(b.softline()));
                let doc = b.group(b.indent(b.intersperse(&docs, sep)));
                result.push(doc);
            }
            Self::Upsert {
                dml_type,
                security_mode,
                target: exp,
                unannotated,
            } => {
                result.push(dml_type.build(b));
                result.push(b.txt(" "));

                let mut docs = vec![];
                if let Some(ref s) = security_mode {
                    docs.push(s.build(b));
                }

                docs.push(exp.build(b));

                if let Some(ref u) = unannotated {
                    docs.push(u.build(b));
                }

                let sep = Insertable::new::<&str>(None, None, Some(b.softline()));
                let doc = b.group(b.indent(b.intersperse(&docs, sep)));
                result.push(doc);
            }
        }
        result.push(b.nil());
    }
}

#[derive(Debug)]
pub enum DmlSecurityMode {
    User(String),
    System(String),
}

impl DmlSecurityMode {
    pub fn new(n: Node) -> Self {
        let child = n.first_c();
        match child.kind() {
            "user" => Self::User(child.value()),
            "system" => Self::System(child.value()),
            _ => panic_unknown_node(n, "DmlSecurityMode"),
        }
    }
}

impl<'a> DocBuild<'a> for DmlSecurityMode {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        result.push(b.txt_("as"));
        match self {
            Self::User(v) => result.push(b.txt(v)),
            Self::System(v) => result.push(b.txt(v)),
        }
    }
}

#[derive(Debug)]
pub enum DmlTypeVariant {
    Insert,
    Update,
    Delete,
    Undelete,
    Merge,
    Upsert,
}

impl DmlTypeVariant {
    pub fn new(node: Node) -> Self {
        let k = node.kind();
        match k {
            "insert" => Self::Insert,
            "update" => Self::Update,
            "delete" => Self::Delete,
            "undelete" => Self::Undelete,
            "merge" => Self::Merge,
            "upsert" => Self::Upsert,
            _ => panic!("## unknown node: {} in DmlTypeVariant ", red(k)),
        }
    }
}

impl<'a> DocBuild<'a> for DmlTypeVariant {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        let txt = match self {
            Self::Insert => "insert",
            Self::Update => "update",
            Self::Delete => "delete",
            Self::Undelete => "undelete",
            Self::Merge => "merge",
            Self::Upsert => "upsert",
        };
        result.push(b.txt(txt));
    }
}

#[derive(Debug)]
pub struct ArrayAccess {
    pub array: PrimaryExpression,
    pub index: Expression,
    pub node_context: NodeContext,
}

impl ArrayAccess {
    pub fn new(node: Node) -> Self {
        assert_check(node, "array_access");

        Self {
            array: PrimaryExpression::new(node.c_by_n("array")),
            index: Expression::new(node.c_by_n("index")),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for ArrayAccess {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(self.array.build(b));
            result.push(b.txt("["));
            result.push(self.index.build(b));
            result.push(b.txt("]"));
        });
    }
}

#[derive(Debug)]
pub struct ArrayCreationExpression {
    pub type_: SimpleType,
    pub variant: ArrayCreationVariant,
    pub node_context: NodeContext,
}

impl ArrayCreationExpression {
    pub fn new(node: Node) -> Self {
        assert_check(node, "array_creation_expression");

        let value_node = node.try_c_by_n("value");
        let dimensions_node = node.try_c_by_n("dimensions");

        let variant = if value_node.is_none() {
            // DD
            let dimensions_exprs = node
                .cs_by_k("dimensions_expr")
                .into_iter()
                .map(|n| DimensionsExpr::new(n))
                .collect();
            let dimensions = node.try_c_by_k("dimensions").map(|n| Dimensions::new(n));
            ArrayCreationVariant::DD {
                dimensions_exprs,
                dimensions,
            }
        } else if dimensions_node.is_none() {
            //OnlyV
            let value = ArrayInitializer::new(node.c_by_n("value"));
            ArrayCreationVariant::OnlyV { value }
        } else {
            //DV
            ArrayCreationVariant::DV {
                value: ArrayInitializer::new(value_node.unwrap()),
                dimensions: Dimensions::new(dimensions_node.unwrap()),
            }
        };

        Self {
            type_: SimpleType::new(node.c_by_n("type")),
            variant,
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for ArrayCreationExpression {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(b.txt_("new"));
            result.push(self.type_.build(b));
            result.push(self.variant.build(b));
        });
    }
}

#[derive(Debug)]
pub enum ArrayCreationVariant {
    DD {
        dimensions_exprs: Vec<DimensionsExpr>,
        dimensions: Option<Dimensions>,
    },
    DV {
        dimensions: Dimensions,
        value: ArrayInitializer,
    },
    OnlyV {
        value: ArrayInitializer,
    },
}

impl<'a> DocBuild<'a> for ArrayCreationVariant {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            Self::OnlyV { value } => {
                result.push(value.build(b));
            }
            Self::DD {
                dimensions_exprs,
                dimensions,
            } => {
                dimensions_exprs
                    .iter()
                    .for_each(|n| result.push(n.build(b)));

                if let Some(ref n) = dimensions {
                    result.push(b.txt(" "));
                    result.push(n.build(b));
                }
            }
            Self::DV { dimensions, value } => {
                result.push(dimensions.build(b));
                result.push(value.build(b));
            }
        }
    }
}

#[derive(Debug)]
pub struct Dimensions {
    pub node_context: NodeContext,
}

impl Dimensions {
    pub fn new(node: Node) -> Self {
        assert_check(node, "dimensions");

        Self {
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for Dimensions {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(b.txt("[]"));
        });
    }
}

#[derive(Debug)]
pub struct DimensionsExpr {
    pub exp: Expression,
    pub node_context: NodeContext,
}

impl DimensionsExpr {
    pub fn new(node: Node) -> Self {
        assert_check(node, "dimensions_expr");

        Self {
            exp: Expression::new(node.first_c()),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for DimensionsExpr {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(b.txt("["));
            result.push(self.exp.build(b));
            result.push(b.txt("]"));
        });
    }
}

#[derive(Debug)]
pub struct ReturnStatement {
    pub exp: Option<Expression>,
    pub node_context: NodeContext,
}

impl ReturnStatement {
    pub fn new(node: Node) -> Self {
        assert_check(node, "return_statement");
        let exp = node.try_first_c().map(|n| Expression::new(n));
        let node_context = if exp.is_none() {
            NodeContext::with_inner_punctuation(&node)
        } else {
            NodeContext::with_punctuation(&node)
        };

        Self { exp, node_context }
    }
}

impl<'a> DocBuild<'a> for ReturnStatement {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        if self.exp.is_none() {
            build_with_comments_and_punc_attached(b, &self.node_context, result, |b, result| {
                result.push(b.txt("return"));
            });
            return;
        }

        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(b.txt("return"));
            result.push(b.txt(" "));
            result.push(self.exp.as_ref().unwrap().build(b));
        });
    }
}

#[derive(Debug)]
pub struct TernaryExpression {
    pub condition: Expression,
    pub consequence: Expression,
    pub alternative: Expression,
    pub node_context: NodeContext,
}

impl TernaryExpression {
    pub fn new(node: Node) -> Self {
        assert_check(node, "ternary_expression");

        Self {
            condition: Expression::new(node.c_by_n("condition")),
            consequence: Expression::new(node.c_by_n("consequence")),
            alternative: Expression::new(node.c_by_n("alternative")),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for TernaryExpression {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            let docs = vec![
                self.condition.build(b),
                b.softline(),
                b.txt_("?"),
                self.consequence.build(b),
                b.softline(),
                b.txt_(":"),
                self.alternative.build(b),
            ];
            result.push(b.group_concat(docs));
        });
    }
}

#[derive(Debug)]
pub struct TryStatement {
    pub body: Block,
    pub tail: TryStatementTail,
    pub node_context: NodeContext,
}

impl TryStatement {
    pub fn new(node: Node) -> Self {
        assert_check(node, "try_statement");

        let tail = if node.try_c_by_k("finally_clause").is_some() {
            TryStatementTail::CatchesFinally(
                node.try_cs_by_k("catch_clause")
                    .into_iter()
                    .map(|n| CatchClause::new(n))
                    .collect(),
                FinallyClause::new(node.c_by_k("finally_clause")),
            )
        } else {
            TryStatementTail::Catches(
                node.cs_by_k("catch_clause")
                    .into_iter()
                    .map(|n| CatchClause::new(n))
                    .collect(),
            )
        };
        Self {
            body: Block::new(node.c_by_n("body")),
            tail,
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for TryStatement {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(b.txt_("try"));
            result.push(self.body.build(b));
            result.push(self.tail.build(b));
        });
    }
}

#[derive(Debug)]
pub enum TryStatementTail {
    Catches(Vec<CatchClause>),
    CatchesFinally(Vec<CatchClause>, FinallyClause),
}

impl<'a> DocBuild<'a> for TryStatementTail {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            Self::Catches(v) => {
                let docs = b.to_docs(v);
                let catches_doc = b.concat(docs);
                result.push(catches_doc);
            }
            Self::CatchesFinally(v, f) => {
                let docs = b.to_docs(v);
                let catches_doc = b.concat(docs);
                result.push(catches_doc);
                result.push(f.build(b));
            }
        }
    }
}

#[derive(Debug)]
pub struct CatchClause {
    pub formal_parameter: FormalParameter,
    pub body: Block,
    pub node_context: NodeContext,
}

impl CatchClause {
    pub fn new(node: Node) -> Self {
        assert_check(node, "catch_clause");

        Self {
            formal_parameter: FormalParameter::new(node.c_by_k("formal_parameter")),
            body: Block::new(node.c_by_n("body")),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for CatchClause {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(b._txt_("catch"));

            result.push(b.txt("("));
            result.push(self.formal_parameter.build(b));
            result.push(b.txt_(")"));
            result.push(self.body.build(b));
        });
    }
}

#[derive(Debug)]
pub struct FinallyClause {
    pub body: Block,
    pub node_context: NodeContext,
}

impl FinallyClause {
    pub fn new(node: Node) -> Self {
        assert_check(node, "finally_clause");

        Self {
            body: Block::new(node.c_by_k("block")),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for FinallyClause {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(b._txt_("finally"));
            result.push(self.body.build(b));
        });
    }
}

#[derive(Debug)]
pub struct StaticInitializer {
    pub block: Block,
    pub node_context: NodeContext,
}

impl StaticInitializer {
    pub fn new(node: Node) -> Self {
        Self {
            block: Block::new(node.c_by_k("block")),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for StaticInitializer {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(b.txt_("static"));
            result.push(self.block.build(b));
        });
    }
}

#[derive(Debug)]
pub struct InterfaceDeclaration {
    pub modifiers: Option<Modifiers>,
    pub name: ValueNode,
    pub type_parameters: Option<TypeParameters>,
    pub extends: Option<ExtendsInterface>,
    pub body: InterfaceBody,
    pub node_context: NodeContext,
}

impl InterfaceDeclaration {
    pub fn new(node: Node) -> Self {
        assert_check(node, "interface_declaration");

        let modifiers = node.try_c_by_k("modifiers").map(|n| Modifiers::new(n));
        let type_parameters = node
            .try_c_by_k("type_parameters")
            .map(|n| TypeParameters::new(n));
        let extends = node
            .try_c_by_k("extends_interfaces")
            .map(|n| ExtendsInterface::new(n));

        Self {
            modifiers,
            name: ValueNode::new(node.c_by_n("name")),
            type_parameters,
            extends,
            body: InterfaceBody::new(node.c_by_n("body")),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for InterfaceDeclaration {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            if let Some(ref n) = self.modifiers {
                result.push(n.build(b));
            }

            result.push(b.txt_("interface"));
            result.push(self.name.build(b));
            if let Some(ref n) = self.type_parameters {
                result.push(n.build(b));
            }

            if let Some(ref n) = self.extends {
                result.push(n.build(b));
            }

            result.push(b.txt(" "));
            result.push(self.body.build(b));
        });
    }
}

#[derive(Debug)]
pub struct ExtendsInterface {
    pub type_list: TypeList,
    pub node_context: NodeContext,
}

impl ExtendsInterface {
    pub fn new(node: Node) -> Self {
        Self {
            type_list: TypeList::new(node.c_by_k("type_list")),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for ExtendsInterface {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            let doc = self.type_list.build(b);
            let extends_group = b.concat(vec![b._txt_("extends"), doc]);
            result.push(extends_group);
        });
    }
}

#[derive(Debug)]
pub struct InterfaceBody {
    members: Vec<BodyMember<InterfaceMember>>,
    pub node_context: NodeContext,
}

impl InterfaceBody {
    pub fn new(node: Node) -> Self {
        assert_check(node, "interface_body");

        let members: Vec<_> = node
            .children_vec()
            .into_iter()
            .map(|n| {
                let member = match n.kind() {
                    "constant_declaration" => {
                        InterfaceMember::Constant(ConstantDeclaration::new(n))
                    }
                    "enum_declaration" => InterfaceMember::EnumD(EnumDeclaration::new(n)),
                    "method_declaration" => InterfaceMember::Method(MethodDeclaration::new(n)),
                    "class_declaration" => InterfaceMember::Class(ClassDeclaration::new(n)),
                    "interface_declaration" => {
                        InterfaceMember::Interface(InterfaceDeclaration::new(n))
                    }
                    _ => panic_unknown_node(n, "InterfaceBody"),
                };

                BodyMember::new(&n, member)
            })
            .collect();

        Self {
            members,
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for InterfaceBody {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        let bucket = get_comment_bucket(&self.node_context.id);
        handle_pre_comments(b, bucket, result);

        if bucket.dangling_comments.is_empty() {
            result.push(b.surround_body_members(&self.members, "{", "}"));
            handle_post_comments(b, bucket, result);
        } else {
            handle_dangling_comments_in_bracket_surround(b, bucket, result);
        }
    }
}

#[derive(Debug)]
pub enum InterfaceMember {
    Constant(ConstantDeclaration),
    EnumD(EnumDeclaration),
    Method(MethodDeclaration),
    Class(ClassDeclaration),
    Interface(InterfaceDeclaration),
    //Semicolon,
}

impl<'a> DocBuild<'a> for InterfaceMember {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            Self::Constant(n) => {
                result.push(n.build(b));
            }
            Self::EnumD(n) => {
                result.push(n.build(b));
            }
            Self::Method(n) => {
                result.push(n.build(b));
            }
            Self::Class(n) => {
                result.push(n.build(b));
            }
            Self::Interface(n) => {
                result.push(n.build(b));
            }
        }
    }
}

#[derive(Debug)]
pub struct ConstantDeclaration {
    pub modifiers: Option<Modifiers>,
    pub type_: UnannotatedType,
    pub declarators: Vec<VariableDeclarator>,
    pub node_context: NodeContext,
}

impl ConstantDeclaration {
    pub fn new(node: Node) -> Self {
        let modifiers = node.try_c_by_k("modifiers").map(|n| Modifiers::new(n));
        let declarators = node
            .cs_by_n("declarator")
            .into_iter()
            .map(|n| VariableDeclarator::new(n))
            .collect();

        Self {
            modifiers,
            type_: UnannotatedType::new(node.c_by_n("type")),
            declarators,
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for ConstantDeclaration {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            if let Some(ref n) = self.modifiers {
                result.push(n.build(b));
            }

            result.push(self.type_.build(b));
            result.push(b.txt(" "));

            let docs = b.to_docs(&self.declarators);
            let sep = Insertable::new::<&str>(None, None, Some(b.softline()));
            let doc = b.group(b.intersperse(&docs, sep));
            result.push(doc);
        });
    }
}

#[derive(Debug)]
pub struct AccessorList {
    pub accessor_declarations: Vec<AccessorDeclaration>,
    pub child_has_body_section: bool,
    pub node_context: NodeContext,
}

impl AccessorList {
    pub fn new(node: Node) -> Self {
        assert_check(node, "accessor_list");

        let accessor_declarations: Vec<_> = node
            .cs_by_k("accessor_declaration")
            .into_iter()
            .map(|n| AccessorDeclaration::new(n))
            .collect();
        let child_has_body_section = accessor_declarations.iter().any(|n| n.body.is_some());

        Self {
            accessor_declarations,
            child_has_body_section,
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for AccessorList {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            // to align with prettier apex;
            if self.child_has_body_section {
                // NOTE: group does NOT work with b.nl() so can't use group_surround()
                let docs = b.to_docs(&self.accessor_declarations);
                let sep = Insertable::new::<&str>(None, None, Some(b.nl()));
                let joined = vec![
                    b.txt("{"),
                    b.indent(b.nl()),
                    b.indent(b.intersperse(&docs, sep)),
                    b.nl(),
                    b.txt("}"),
                ];

                result.push(b.concat(joined));
            } else {
                let docs = b.to_docs(&self.accessor_declarations);
                let sep = Insertable::new::<&str>(None, None, Some(b.softline()));
                let open = Insertable::new(None, Some("{"), Some(b.softline()));
                let close = Insertable::new(Some(b.softline()), Some("}"), None);
                let doc = b.group_surround(&docs, sep, open, close);
                result.push(doc);
            }
        });
    }
}

#[derive(Debug)]
pub struct AccessorDeclaration {
    pub modifiers: Option<Modifiers>,
    pub accessor: String,
    pub body: Option<Block>,
    pub node_context: NodeContext,
}

impl AccessorDeclaration {
    pub fn new(node: Node) -> Self {
        assert_check(node, "accessor_declaration");

        Self {
            modifiers: node.try_c_by_k("modifiers").map(|n| Modifiers::new(n)),
            accessor: node.cvalue_by_n("accessor"),
            body: node.try_c_by_n("body").map(|n| Block::new(n)),
            node_context: NodeContext::with_inner_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for AccessorDeclaration {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            if let Some(ref n) = self.modifiers {
                result.push(n.build(b));
            }
            result.push(b.txt(&self.accessor));

            if let Some(ref n) = self.body {
                result.push(b.txt(" "));
                result.push(n.build(b));
            }
        });
    }
}

#[derive(Debug)]
pub struct CastExpression {
    pub type_: Type,
    pub value: Expression,
    pub node_context: NodeContext,
}

impl CastExpression {
    pub fn new(node: Node) -> Self {
        assert_check(node, "cast_expression");

        Self {
            type_: Type::new(node.c_by_n("type")),
            value: Expression::new(node.c_by_n("value")),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for CastExpression {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(b.txt("("));
            result.push(self.type_.build(b));
            result.push(b.txt_(")"));
            result.push(self.value.build(b));
        });
    }
}

#[derive(Debug)]
pub struct ThrowStatement {
    pub exp: Expression,
    pub node_context: NodeContext,
}

impl ThrowStatement {
    pub fn new(node: Node) -> Self {
        assert_check(node, "throw_statement");

        Self {
            exp: Expression::new(node.first_c()),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for ThrowStatement {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(b.txt("throw "));
            result.push(self.exp.build(b));
        });
    }
}

#[derive(Debug)]
pub struct BreakStatement {
    pub identifier: Option<ValueNode>,
    pub node_context: NodeContext,
}

impl BreakStatement {
    pub fn new(node: Node) -> Self {
        assert_check(node, "break_statement");

        let identifier = node.try_c_by_k("identifier").map(|n| ValueNode::new(n));
        let node_context = if identifier.is_none() {
            NodeContext::with_inner_punctuation(&node)
        } else {
            NodeContext::with_punctuation(&node)
        };

        Self {
            identifier,
            node_context,
        }
    }
}

impl<'a> DocBuild<'a> for BreakStatement {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        if self.identifier.is_none() {
            build_with_comments_and_punc_attached(b, &self.node_context, result, |b, result| {
                result.push(b.txt("break"));
            });
            return;
        }

        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(b.txt("break"));
            result.push(b.txt(" "));
            result.push(self.identifier.as_ref().unwrap().build(b));
        });
    }
}

#[derive(Debug)]
pub struct ContinueStatement {
    pub identifier: Option<ValueNode>,
    pub node_context: NodeContext,
}

impl ContinueStatement {
    pub fn new(node: Node) -> Self {
        assert_check(node, "continue_statement");

        let identifier = node.try_c_by_k("identifier").map(|n| ValueNode::new(n));
        let node_context = if identifier.is_none() {
            NodeContext::with_inner_punctuation(&node)
        } else {
            NodeContext::with_punctuation(&node)
        };

        Self {
            identifier,
            node_context,
        }
    }
}

impl<'a> DocBuild<'a> for ContinueStatement {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        if self.identifier.is_none() {
            build_with_comments_and_punc_attached(b, &self.node_context, result, |b, result| {
                result.push(b.txt("continue"));
            });
            return;
        }

        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(b.txt("continue"));
            result.push(b.txt(" "));
            result.push(self.identifier.as_ref().unwrap().build(b));
        });
    }
}

#[derive(Debug)]
pub struct SwitchExpression {
    pub condition: Expression,
    pub body: SwitchBlock,
    pub node_context: NodeContext,
}

impl SwitchExpression {
    pub fn new(node: Node) -> Self {
        assert_check(node, "switch_expression");

        Self {
            condition: Expression::new(node.c_by_n("condition")),
            body: SwitchBlock::new(node.c_by_n("body")),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for SwitchExpression {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            let docs = vec![b.txt("switch on"), b.softline(), self.condition.build(b)];
            let doc = b.group_indent_concat(docs);
            result.push(doc);
            result.push(b.txt(" "));
            result.push(self.body.build(b));
        });
    }
}

#[derive(Debug)]
pub struct SwitchBlock {
    pub rules: Vec<SwitchRule>,
    pub node_context: NodeContext,
}

impl SwitchBlock {
    pub fn new(node: Node) -> Self {
        assert_check(node, "switch_block");

        let rules = node
            .cs_by_k("switch_rule")
            .into_iter()
            .map(|n| SwitchRule::new(n))
            .collect();

        Self {
            rules,
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for SwitchBlock {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            let docs = b.to_docs(&self.rules);

            let sep = Insertable::new(None, Some(""), Some(b.nl()));
            let open = Insertable::new(None, Some("{"), Some(b.nl()));
            let close = Insertable::new(Some(b.nl()), Some("}"), None);
            let doc = b.surround(&docs, sep, open, close);
            result.push(doc);
        });
    }
}

#[derive(Debug)]
pub struct SwitchRule {
    pub label: SwitchLabel,
    pub block: Block,
    pub node_context: NodeContext,
}

impl SwitchRule {
    pub fn new(node: Node) -> Self {
        assert_check(node, "switch_rule");

        Self {
            label: SwitchLabel::new(node.c_by_k("switch_label")),
            block: Block::new(node.c_by_k("block")),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for SwitchRule {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(self.label.build(b));
            result.push(b.txt(" "));
            result.push(self.block.build(b));
        });
    }
}

#[derive(Debug)]
pub enum SwitchLabel {
    WhenSObject(WhenSObjectType),
    Expressions(Vec<Expression>),
    Else,
}

impl SwitchLabel {
    pub fn new(node: Node) -> Self {
        assert_check(node, "switch_label");

        if node.children_vec().is_empty() {
            Self::Else
        } else if let Some(when_node) = node.try_c_by_k("when_sobject_type") {
            Self::WhenSObject(WhenSObjectType::new(when_node))
        } else {
            let expressions = node
                .children_vec()
                .into_iter()
                .map(Expression::new)
                .collect();
            Self::Expressions(expressions)
        }
    }
}

impl<'a> DocBuild<'a> for SwitchLabel {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        result.push(b.txt_("when"));
        match self {
            Self::WhenSObject(n) => {
                result.push(n.build(b));
            }
            Self::Expressions(vec) => {
                let docs = b.to_docs(vec);
                let sep = Insertable::new::<&str>(None, None, Some(b.softline()));
                let doc = b.group(b.indent(b.intersperse(&docs, sep)));
                result.push(doc);
            }
            Self::Else => {
                result.push(b.txt("else"));
            }
        }
    }
}

#[derive(Debug)]
pub struct WhenSObjectType {
    pub unannotated_type: UnannotatedType,
    pub identifier: String,
    pub node_context: NodeContext,
}

impl WhenSObjectType {
    pub fn new(node: Node) -> Self {
        let mut unannotated_type = None;
        let mut identifier = None;

        for child in node.children_vec() {
            match child.kind() {
                "identifier" => {
                    identifier = Some(child.value());
                }
                _ => {
                    unannotated_type = Some(UnannotatedType::new(child));
                }
            }
        }

        Self {
            unannotated_type: unannotated_type
                .expect("Missing unannotated_type in WhenSObjectType"),
            identifier: identifier.expect("Missing identifier in WhenSObjectType"),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for WhenSObjectType {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(self.unannotated_type.build(b));
            result.push(b.txt(" "));
            result.push(b.txt(&self.identifier));
        });
    }
}

#[derive(Debug)]
pub struct InstanceOfExpression {
    pub left: Expression,
    pub right: Type,
    pub node_context: NodeContext,
}

impl InstanceOfExpression {
    pub fn new(node: Node) -> Self {
        assert_check(node, "instanceof_expression");

        Self {
            left: Expression::new(node.c_by_n("left")),
            right: Type::new(node.c_by_n("right")),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for InstanceOfExpression {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(self.left.build(b));
            result.push(b._txt_("instanceof"));
            result.push(self.right.build(b));
        });
    }
}

#[derive(Debug)]
pub struct VersionExpression {
    version_number: Option<ValueNode>,
    pub node_context: NodeContext,
}

impl VersionExpression {
    pub fn new(node: Node) -> Self {
        assert_check(node, "version_expression");

        let version_number = node.try_c_by_n("version_num").map(|n| ValueNode::new(n));
        Self {
            version_number,
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for VersionExpression {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(b.txt("Package.Version."));
            if let Some(ref n) = self.version_number {
                result.push(n.build(b));
            } else {
                result.push(b.txt("Request"));
            }
        });
    }
}

#[derive(Debug)]
pub struct JavaFieldAccess {
    pub field_access: FieldAccess,
    pub node_context: NodeContext,
}

impl JavaFieldAccess {
    pub fn new(node: Node) -> Self {
        assert_check(node, "java_field_access");

        Self {
            field_access: FieldAccess::new(node.c_by_k("field_access")),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for JavaFieldAccess {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(b.txt("java:"));
            result.push(self.field_access.build(b));
        });
    }
}

#[derive(Debug)]
pub struct JavaType {
    pub scoped_type_identifier: ScopedTypeIdentifier,
    pub node_context: NodeContext,
}

impl JavaType {
    pub fn new(node: Node) -> Self {
        let scoped_type_identifier =
            ScopedTypeIdentifier::new(node.c_by_k("scoped_type_identifier"));

        Self {
            scoped_type_identifier,
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for JavaType {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(b.txt("java:"));
            result.push(self.scoped_type_identifier.build(b));
        });
    }
}

#[derive(Debug)]
pub struct ArrayType {
    pub element: UnannotatedType,
    pub dimensions: Dimensions,
    pub node_context: NodeContext,
}

impl ArrayType {
    pub fn new(node: Node) -> Self {
        assert_check(node, "array_type");

        Self {
            element: UnannotatedType::new(node.c_by_n("element")),
            dimensions: Dimensions::new(node.c_by_n("dimensions")),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for ArrayType {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(self.element.build(b));
            result.push(self.dimensions.build(b));
        });
    }
}

#[derive(Debug)]
pub struct TriggerDeclaration {
    pub name: ValueNode,
    pub events: Vec<TriggerEvent>,
    pub object: ValueNode,
    pub body: TriggerBody,
    pub node_context: NodeContext,
}

impl TriggerDeclaration {
    pub fn new(node: Node) -> Self {
        assert_check(node, "trigger_declaration");

        let events = node
            .cs_by_k("trigger_event")
            .into_iter()
            .map(|n| TriggerEvent::new(n))
            .collect();

        Self {
            name: ValueNode::new(node.c_by_n("name")),
            object: ValueNode::new(node.c_by_n("object")),
            events,
            body: TriggerBody::new(node.c_by_n("body")),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for TriggerDeclaration {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(b.txt_("trigger"));
            result.push(self.name.build(b));
            result.push(b._txt_("on"));
            result.push(self.object.build(b));

            let docs = b.to_docs(&self.events);
            let sep = Insertable::new::<&str>(None, None, Some(b.softline()));
            let open = Insertable::new(None, Some("("), Some(b.maybeline()));
            let close = Insertable::new(Some(b.maybeline()), Some(")"), None);
            let doc = b.group_surround(&docs, sep, open, close);
            result.push(doc);

            result.push(b.txt(" "));
            result.push(self.body.build(b));
        });
    }
}

#[derive(Debug)]
pub struct TriggerEvent {
    pub event: TriggerEventVariant,
    pub node_context: NodeContext,
}

impl TriggerEvent {
    pub fn new(node: Node) -> Self {
        assert_check(node, "trigger_event");

        Self {
            event: TriggerEventVariant::new(node.first_c()),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for TriggerEvent {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(self.event.build(b));
        });
    }
}

#[derive(Debug)]
pub struct TriggerBody {
    pub block: Block,
    pub node_context: NodeContext,
}

impl TriggerBody {
    pub fn new(node: Node) -> Self {
        assert_check(node, "trigger_body");

        Self {
            block: Block::new(node.c_by_k("block")),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for TriggerBody {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(self.block.build(b));
        });
    }
}

#[derive(Debug)]
pub struct QueryExpression {
    pub query_body: QueryBody,
    pub context: Option<ChainingContext>,
    pub node_context: NodeContext,
}

impl QueryExpression {
    pub fn new(node: Node) -> Self {
        assert_check(node, "query_expression");

        let query_body = if let Some(soql_node) = node.try_c_by_k("soql_query_body") {
            QueryBody::Soql(SoqlQueryBody::new(soql_node))
        } else {
            QueryBody::Sosl(SoslQueryBody::new(node.c_by_k("sosl_query_body")))
        };

        Self {
            query_body,
            context: build_chaining_context(&node),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for QueryExpression {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            if self.context.is_some() {
                let mut docs = vec![];
                docs.push(b.txt("["));
                docs.push(b.maybeline());

                let body_docs = vec![self.query_body.build(b)];
                let sep = Insertable::new::<&str>(None, None, Some(b.softline()));
                docs.push(b.intersperse(&body_docs, sep));

                // TODO: Why dedent() is needed here? This is the only place in the code place.
                docs.push(b.dedent(b.maybeline()));
                docs.push(b.txt("]"));

                result.push(b.group_concat(docs));
            } else {
                let docs = vec![self.query_body.build(b)];
                let sep = Insertable::new::<&str>(None, None, Some(b.softline()));
                let open = Insertable::new(None, Some("["), Some(b.maybeline()));
                let close = Insertable::new(Some(b.maybeline()), Some("]"), None);
                let doc = b.group_surround(&docs, sep, open, close);
                result.push(doc);
            }
        });
    }
}

#[derive(Debug)]
pub enum QueryBody {
    Soql(SoqlQueryBody),
    Sosl(SoslQueryBody),
}

impl<'a> DocBuild<'a> for QueryBody {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            Self::Soql(n) => {
                result.push(n.build(b));
            }
            Self::Sosl(n) => {
                result.push(n.build(b));
            }
        }
    }
}

#[derive(Debug)]
pub struct SoslQueryBody {
    pub find_clause: FindClause,
    pub in_clause: Option<InClause>,
    pub returning_clause: Option<ReturningClause>,
    pub with_clauses: Vec<SoslWithClause>,
    pub using_clause: Option<SoslUsingClause>,
    pub limit_clause: Option<LimitClause>,
    //pub offset_clause: Option<OffsetClause>,
    pub update_clause: Option<UpdateClause>,
    pub node_context: NodeContext,
}

impl SoslQueryBody {
    pub fn new(node: Node) -> Self {
        assert_check(node, "sosl_query_body");

        let find_clause = FindClause::new(node.c_by_k("find_clause"));
        let in_clause = node.try_c_by_k("in_clause").map(|n| InClause::new(n));
        let returning_clause = node
            .try_c_by_k("returning_clause")
            .map(|n| ReturningClause::new(n));
        let with_clauses = node
            .try_cs_by_k("with_clause")
            .into_iter()
            .map(|n| SoslWithClause::new(n))
            .collect();
        let using_clause = node
            .try_c_by_k("sosl_using_clause")
            .map(|n| SoslUsingClause::new(n));
        let limit_clause = node.try_c_by_k("limit_clause").map(|n| LimitClause::new(n));
        let update_clause = node
            .try_c_by_k("update_clause")
            .map(|n| UpdateClause::new(n));

        Self {
            find_clause,
            in_clause,
            returning_clause,
            with_clauses,
            using_clause,
            limit_clause,
            update_clause,
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for SoslQueryBody {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            let mut docs = vec![];
            docs.push(self.find_clause.build(b));

            if let Some(ref n) = self.in_clause {
                docs.push(n.build(b));
            }
            if let Some(ref n) = self.returning_clause {
                docs.push(n.build(b));
            }
            if !self.with_clauses.is_empty() {
                let with_clauses_docs = b.to_docs(&self.with_clauses);
                let sep = Insertable::new::<&str>(None, None, Some(b.softline()));
                let doc = b.intersperse(&with_clauses_docs, sep);
                docs.push(doc);
            }
            if let Some(ref n) = self.using_clause {
                docs.push(n.build(b));
            }
            if let Some(ref n) = self.limit_clause {
                docs.push(n.build(b));
            }
            if let Some(ref n) = self.update_clause {
                docs.push(n.build(b));
            }

            let sep = Insertable::new::<&str>(None, None, Some(b.softline()));
            let doc = b.intersperse(&docs, sep);
            result.push(doc);
        });
    }
}

#[derive(Debug)]
pub enum FindClause {
    Bound(BoundApexExpression),
    Term(String),
}

impl FindClause {
    pub fn new(node: Node) -> Self {
        assert_check(node, "find_clause");

        if let Some(bound_node) = node.try_c_by_k("bound_apex_expression") {
            Self::Bound(BoundApexExpression::new(bound_node))
        } else {
            Self::Term(node.cvalue_by_k("term"))
        }
    }
}

impl<'a> DocBuild<'a> for FindClause {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        result.push(b.txt_("FIND"));
        match self {
            Self::Bound(n) => {
                result.push(n.build(b));
            }
            Self::Term(n) => {
                result.push(b.txt(format!("'{}'", n)));
            }
        }
    }
}

#[derive(Debug)]
pub struct InClause {
    in_type: ValueNode,
    pub node_context: NodeContext,
}

impl InClause {
    pub fn new(node: Node) -> Self {
        assert_check(node, "in_clause");

        Self {
            in_type: ValueNode::new(node.c_by_k("in_type")),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for InClause {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(b.txt_("IN"));
            result.push(self.in_type.build(b));
            result.push(b.txt(" "));
            result.push(b.txt("FIELDS"));
        });
    }
}

#[derive(Debug)]
pub struct ReturningClause {
    sobject_returns: Vec<SObjectReturn>,
    pub node_context: NodeContext,
}

impl ReturningClause {
    pub fn new(node: Node) -> Self {
        let sobject_returns = node
            .cs_by_k("sobject_return")
            .into_iter()
            .map(|n| SObjectReturn::new(n))
            .collect();

        Self {
            sobject_returns,
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for ReturningClause {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(b.txt("RETURNING"));

            let docs = b.to_docs(&self.sobject_returns);
            let sep = Insertable::new::<&str>(None, None, Some(b.softline()));
            let doc = b.group_indent(b.concat(vec![b.softline(), b.intersperse(&docs, sep)]));
            result.push(doc);
        });
    }
}

// TODO:

#[derive(Debug)]
pub struct SObjectReturn {
    pub identifier: ValueNode,
    pub sobject_return_query: Option<SObjectReturnQuery>,
    pub node_context: NodeContext,
}

impl SObjectReturn {
    pub fn new(node: Node) -> Self {
        assert_check(node, "sobject_return");

        let sobject_return_query = node
            .try_c_by_k("selected_fields")
            .map(|n| SObjectReturnQuery {
                selected_fields: n
                    .children_vec()
                    .into_iter()
                    .map(|selectable_node| SelectableExpression::new(selectable_node))
                    .collect(),
                using_clause: node.try_c_by_k("using_clause").map(|n| UsingClause::new(n)),
                where_clause: node.try_c_by_k("where_clause").map(|n| WhereClause::new(n)),
                order_by_clause: node
                    .try_c_by_k("order_by_clause")
                    .map(|n| OrderByClause::new(n)),
                limit_clause: node.try_c_by_k("limit_clause").map(|n| LimitClause::new(n)),
                offset_clause: node
                    .try_c_by_k("offset_clause")
                    .map(|n| OffsetClause::new(n)),
                node_context: NodeContext::with_punctuation(&n),
            });

        Self {
            identifier: ValueNode::new(node.c_by_k("identifier")),
            sobject_return_query,
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for SObjectReturn {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(self.identifier.build(b));

            if let Some(ref n) = self.sobject_return_query {
                result.push(n.build(b));
            }
        });
    }
}

#[derive(Debug)]
pub struct SObjectReturnQuery {
    pub selected_fields: Vec<SelectableExpression>,
    pub using_clause: Option<UsingClause>,
    pub where_clause: Option<WhereClause>,
    pub order_by_clause: Option<OrderByClause>,
    pub limit_clause: Option<LimitClause>,
    pub offset_clause: Option<OffsetClause>,
    pub node_context: NodeContext,
}

impl<'a> DocBuild<'a> for SObjectReturnQuery {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            let mut docs = vec![];

            let selected_fields_docs = b.to_docs(&self.selected_fields);
            let sep = Insertable::new::<&str>(None, None, Some(b.softline()));
            let doc = b.intersperse(&selected_fields_docs, sep);
            docs.push(doc);

            if let Some(ref n) = self.using_clause {
                docs.push(n.build(b));
            }
            if let Some(ref n) = self.where_clause {
                docs.push(n.build(b));
            }
            if let Some(ref n) = self.order_by_clause {
                docs.push(n.build(b));
            }
            if let Some(ref n) = self.limit_clause {
                docs.push(n.build(b));
            }
            if let Some(ref n) = self.offset_clause {
                docs.push(n.build(b));
            }

            let sep = Insertable::new::<&str>(None, None, Some(b.softline()));
            let open = Insertable::new(None, Some("("), Some(b.maybeline()));
            let close = Insertable::new(None, Some(")"), None);
            let doc = b.group_surround(&docs, sep, open, close);
            result.push(doc);
        });
    }
}

#[derive(Debug)]
pub struct SoqlQueryBody {
    pub select_clause: SelectClause,
    pub from_clause: FromClause,
    //using_clause;
    pub where_clause: Option<WhereClause>,
    pub with_clause: Option<SoqlWithClause>,
    pub group_by_clause: Option<GroupByClause>,
    pub order_by_clause: Option<OrderByClause>,
    pub limit_clause: Option<LimitClause>,
    pub offset_clause: Option<OffsetClause>,
    pub for_clause: Vec<ForClause>,
    //update_c;
    pub all_rows_clause: Option<AllRowsClause>,
    pub node_context: NodeContext,
}

impl SoqlQueryBody {
    pub fn new(node: Node) -> Self {
        assert_check(node, "soql_query_body");

        let where_clause = node.try_c_by_n("where_clause").map(|n| WhereClause::new(n));
        let with_clause = node
            .try_c_by_n("with_clause")
            .map(|n| SoqlWithClause::new(n));
        let group_by_clause = node
            .try_c_by_n("group_by_clause")
            .map(|n| GroupByClause::new(n));
        let order_by_clause = node
            .try_c_by_n("order_by_clause")
            .map(|n| OrderByClause::new(n));
        let limit_clause = node.try_c_by_n("limit_clause").map(|n| LimitClause::new(n));
        let offset_clause = node
            .try_c_by_n("offset_clause")
            .map(|n| OffsetClause::new(n));
        let all_rows_clause = node
            .try_c_by_n("all_rows_clause")
            .map(|n| AllRowsClause::new(n));
        let for_clause = node
            .try_cs_by_k("for_clause")
            .into_iter()
            .map(|n| ForClause::new(n))
            .collect();

        Self {
            select_clause: SelectClause::new(node.c_by_n("select_clause")),
            from_clause: FromClause::new(node.c_by_n("from_clause")),
            where_clause,
            with_clause,
            group_by_clause,
            order_by_clause,
            limit_clause,
            offset_clause,
            for_clause,
            all_rows_clause,
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for SoqlQueryBody {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            let mut docs = vec![];
            docs.push(self.select_clause.build(b));
            docs.push(self.from_clause.build(b));

            if let Some(ref n) = self.where_clause {
                docs.push(n.build(b));
            }
            if let Some(ref n) = self.with_clause {
                docs.push(n.build(b));
            }
            if let Some(ref n) = self.group_by_clause {
                docs.push(n.build(b));
            }
            if let Some(ref n) = self.order_by_clause {
                docs.push(n.build(b));
            }
            if let Some(ref n) = self.limit_clause {
                docs.push(n.build(b));
            }
            if let Some(ref n) = self.offset_clause {
                docs.push(n.build(b));
            }
            if let Some(ref n) = self.all_rows_clause {
                docs.push(n.build(b));
            }
            if !self.for_clause.is_empty() {
                let for_types: Vec<DocRef<'_>> =
                    self.for_clause.iter().map(|n| n.build(b)).collect();
                let sep = Insertable::new(None, Some(" "), None);
                let for_types_doc = b.intersperse(&for_types, sep);

                let for_clause_doc = b.concat(vec![b.txt_("FOR"), for_types_doc]);
                docs.push(for_clause_doc);
            }

            let sep = Insertable::new::<&str>(None, None, Some(b.softline()));
            let doc = b.intersperse(&docs, sep);
            result.push(doc);
        });
    }
}

#[derive(Debug)]
pub struct FromClause {
    pub content: StorageVariant,
    pub node_context: NodeContext,
}

impl FromClause {
    pub fn new(node: Node) -> Self {
        assert_check(node, "from_clause");

        Self {
            content: StorageVariant::new(node.first_c()),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for FromClause {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(b.txt_("FROM"));
            result.push(self.content.build(b));
        });
    }
}

#[derive(Debug)]
pub struct StorageAlias {
    pub storage_identifier: StorageIdentifier,
    pub identifier: ValueNode,
    pub node_context: NodeContext,
}

impl StorageAlias {
    pub fn new(node: Node) -> Self {
        assert_check(node, "storage_alias");

        Self {
            storage_identifier: StorageIdentifier::new(node.c_by_k("storage_identifier")),
            identifier: ValueNode::new(node.c_by_k("identifier")),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for StorageAlias {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(self.storage_identifier.build(b));
            result.push(b.txt(" "));
            result.push(self.identifier.build(b));
        });
    }
}

#[derive(Debug)]
pub struct LimitClause {
    pub limit_value: LimitValue,
    pub node_context: NodeContext,
}

impl LimitClause {
    pub fn new(node: Node) -> Self {
        assert_check(node, "limit_clause");

        Self {
            limit_value: LimitValue::new(node.first_c()),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for LimitClause {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(b.txt_("LIMIT"));
            result.push(self.limit_value.build(b));
        });
    }
}

#[derive(Debug)]
pub struct UpdateClause {
    pub update_types: Vec<ValueNode>,
    pub node_context: NodeContext,
}

impl UpdateClause {
    pub fn new(node: Node) -> Self {
        let update_types = node
            .cs_by_k("update_type")
            .into_iter()
            .map(|n| ValueNode::new(n))
            .collect();

        Self {
            update_types,
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for UpdateClause {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(b.txt_("UPDATE"));

            let docs: Vec<DocRef<'a>> = self.update_types.iter().map(|n| n.build(b)).collect();
            let sep = Insertable::new(None, Some(" "), None);
            let doc = b.intersperse(&docs, sep);
            result.push(doc);
        });
    }
}

#[derive(Debug)]
pub struct BoundApexExpression {
    pub exp: Box<Expression>,
    pub node_context: NodeContext,
}

impl BoundApexExpression {
    pub fn new(node: Node) -> Self {
        assert_check(node, "bound_apex_expression");

        Self {
            exp: Box::new(Expression::new(node.first_c())),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for BoundApexExpression {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(b.txt(":"));
            result.push(self.exp.build(b));
        });
    }
}

#[derive(Debug)]
pub struct SoslUsingClause {
    pub search: UsingSearch,
    pub node_context: NodeContext,
}

impl SoslUsingClause {
    pub fn new(node: Node) -> Self {
        assert_check(node, "sosl_using_clause");

        Self {
            search: UsingSearch::new(node.first_c()),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for SoslUsingClause {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(b.txt_("USING"));
            result.push(self.search.build(b));
        });
    }
}

#[derive(Debug)]
pub enum UsingSearch {
    Phrase,
    Advanced,
}

impl UsingSearch {
    pub fn new(node: Node) -> Self {
        match node.kind() {
            "using_phrase_search" => Self::Phrase,
            "using_advanced_search" => Self::Advanced,
            _ => panic_unknown_node(node, "UsingSearch"),
        }
    }
}

impl<'a> DocBuild<'a> for UsingSearch {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            Self::Phrase => {
                result.push(b.txt("PHRASE SEARCH"));
            }
            Self::Advanced => {
                result.push(b.txt("ADVANCED SEARCH"));
            }
        }
    }
}

#[derive(Debug)]
pub struct UsingClause {
    pub option: UsingClauseOption,
    pub node_context: NodeContext,
}

impl UsingClause {
    pub fn new(node: Node) -> Self {
        assert_check(node, "using_clause");

        Self {
            option: UsingClauseOption::new(node.first_c()),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for UsingClause {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(b.txt_("USING"));
            result.push(self.option.build(b));
        });
    }
}

#[derive(Debug)]
pub enum UsingClauseOption {
    Scope(UsingScopeClause),
    Lookup(UsingLookupClause),
    Listview(UsingListviewClause),
}

impl UsingClauseOption {
    pub fn new(node: Node) -> Self {
        match node.kind() {
            "using_scope_clause" => Self::Scope(UsingScopeClause::new(node)),
            "using_lookup_clause" => Self::Lookup(UsingLookupClause::new(node)),
            "using_listview_clause" => Self::Listview(UsingListviewClause::new(node)),
            _ => panic_unknown_node(node, "UsingClauseOption"),
        }
    }
}

impl<'a> DocBuild<'a> for UsingClauseOption {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            Self::Scope(n) => {
                result.push(n.build(b));
            }
            Self::Lookup(n) => {
                result.push(n.build(b));
            }
            Self::Listview(n) => {
                result.push(n.build(b));
            }
        }
    }
}

#[derive(Debug)]
pub struct UsingScopeClause {
    type_: ValueNode,
    pub node_context: NodeContext,
}

impl UsingScopeClause {
    pub fn new(node: Node) -> Self {
        assert_check(node, "using_scope_clause");

        Self {
            type_: ValueNode::new(node.c_by_k("using_scope_type")),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for UsingScopeClause {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(b.txt_("SCOPE"));
            result.push(self.type_.build(b));
        });
    }
}

#[derive(Debug)]
pub struct UsingLookupClause {
    lookup_field: DottedIdentifier,
    bind_clause: Option<UsingLookupBindClause>,
    pub node_context: NodeContext,
}

impl UsingLookupClause {
    pub fn new(node: Node) -> Self {
        assert_check(node, "using_lookup_clause");

        let lookup_field = DottedIdentifier::new(node.c_by_n("using_lookup_clause"));
        let bind_clause = node
            .try_c_by_k("using_lookup_bind_clause")
            .map(|n| UsingLookupBindClause::new(n));

        Self {
            lookup_field,
            bind_clause,
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for UsingLookupClause {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(b.txt_("LOOKUP"));
            result.push(self.lookup_field.build(b));
            result.push(b.txt(" "));
            if let Some(ref n) = self.bind_clause {
                result.push(n.build(b));
            }
        });
    }
}

#[derive(Debug)]
pub struct UsingListviewClause {
    identifier: ValueNode,
    pub node_context: NodeContext,
}

impl UsingListviewClause {
    pub fn new(node: Node) -> Self {
        assert_check(node, "using_listview_clause");

        Self {
            identifier: ValueNode::new(node.c_by_k("identifier")),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for UsingListviewClause {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(b.txt_("ListView ="));
            result.push(self.identifier.build(b));
        });
    }
}

#[derive(Debug)]
pub struct UsingLookupBindClause {
    bind_exps: Vec<UsingLookupBindExpression>,
    pub node_context: NodeContext,
}

impl UsingLookupBindClause {
    pub fn new(node: Node) -> Self {
        assert_check(node, "using_lookup_bind_clause");

        let bind_exps = node
            .try_cs_by_k("using_lookup_bind_expression")
            .into_iter()
            .map(|n| UsingLookupBindExpression::new(n))
            .collect();

        Self {
            bind_exps,

            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for UsingLookupBindClause {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(b.txt_("BIND"));

            let docs = b.to_docs(&self.bind_exps);
            let sep = Insertable::new(None, Some(" "), None);
            let doc = b.intersperse(&docs, sep);
            result.push(doc);
        });
    }
}

#[derive(Debug)]
pub struct UsingLookupBindExpression {
    field: ValueNode,
    bound_value: SoqlLiteral,
    pub node_context: NodeContext,
}

impl UsingLookupBindExpression {
    pub fn new(node: Node) -> Self {
        assert_check(node, "using_lookup_bind_expression");

        Self {
            field: ValueNode::new(node.c_by_k("field")),
            bound_value: SoqlLiteral::new(node.c_by_n("bound_value")),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for UsingLookupBindExpression {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(self.field.build(b));
            result.push(b._txt_("="));
            result.push(self.bound_value.build(b));
        });
    }
}

#[derive(Debug)]
pub struct WhereClause {
    pub boolean_exp: BooleanExpression,
    pub node_context: NodeContext,
}

impl WhereClause {
    pub fn new(node: Node) -> Self {
        assert_check(node, "where_clause");

        Self {
            boolean_exp: BooleanExpression::new(node.first_c()),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for WhereClause {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            let docs = vec![
                b.txt("WHERE"),
                b.softline(),
                self.boolean_exp.build_with_parent(b, None),
            ];
            result.push(b.group_indent_concat(docs));
        });
    }
}

#[derive(Debug)]
pub struct ComparisonExpression {
    pub value: Box<ValueExpression>,
    pub comparison: Comparison,
    pub node_context: NodeContext,
}

impl ComparisonExpression {
    pub fn new(node: Node) -> Self {
        assert_check(node, "comparison_expression");

        Self {
            value: Box::new(ValueExpression::new(node.first_c())),
            comparison: get_comparsion(&node),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for ComparisonExpression {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(self.value.build(b));
            result.push(self.comparison.build(b));
        });
    }
}

#[derive(Debug)]
pub struct ValueComparison {
    pub operator: String,
    pub compared_with: ValueComparedWith,
}

impl<'a> DocBuild<'a> for ValueComparison {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        result.push(b._txt_(&self.operator));
        result.push(self.compared_with.build(b));
    }
}

#[derive(Debug)]
pub struct SetComparison {
    pub operator: String,
    pub set_value: SetValue,
}

impl<'a> DocBuild<'a> for SetComparison {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        result.push(b._txt_(&self.operator));
        result.push(self.set_value.build(b));
    }
}

#[derive(Debug)]
pub struct ComparableList {
    pub values: Vec<ComparableListValue>,
    pub node_context: NodeContext,
}

impl ComparableList {
    pub fn new(node: Node) -> Self {
        let values = node
            .children_vec()
            .into_iter()
            .map(|n| ComparableListValue::new(n))
            .collect();

        Self {
            values,
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for ComparableList {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            let docs = b.to_docs(&self.values);

            let sep = Insertable::new::<&str>(None, None, Some(b.softline()));
            let open = Insertable::new(None, Some("("), Some(b.maybeline()));
            let close = Insertable::new(Some(b.maybeline()), Some(")"), None);
            let doc = b.group_surround(&docs, sep, open, close);
            result.push(doc);
        });
    }
}

#[derive(Debug)]
pub struct OrderByClause {
    pub exps: Vec<OrderExpression>,
    pub node_context: NodeContext,
}

impl OrderByClause {
    pub fn new(node: Node) -> Self {
        assert_check(node, "order_by_clause");

        let exps = node
            .cs_by_k("order_expression")
            .into_iter()
            .map(|n| OrderExpression::new(n))
            .collect();

        Self {
            exps,
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for OrderByClause {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(b.txt_("ORDER BY"));

            let docs = b.to_docs(&self.exps);
            let sep = Insertable::new(None, Some(" "), None);
            let doc = b.intersperse(&docs, sep);
            result.push(doc);
        });
    }
}

#[derive(Debug)]
pub struct OrderExpression {
    pub value_expression: ValueExpression,
    pub direction: Option<ValueNode>,
    pub null_direction: Option<ValueNode>,
    pub node_context: NodeContext,
}

impl OrderExpression {
    pub fn new(node: Node) -> Self {
        assert_check(node, "order_expression");

        let direction = node
            .try_c_by_k("order_direction")
            .map(|n| ValueNode::new(n));
        let null_direction = node
            .try_c_by_k("order_null_direction")
            .map(|n| ValueNode::new(n));

        Self {
            value_expression: ValueExpression::new(node.first_c()),
            direction,
            null_direction,
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for OrderExpression {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(self.value_expression.build(b));

            if let Some(ref n) = self.direction {
                result.push(b.txt(" "));
                result.push(n.build(b));
            }
            if let Some(ref n) = self.null_direction {
                result.push(b.txt(" "));
                result.push(n.build(b));
            }
        });
    }
}

#[derive(Debug)]
pub struct SubQuery {
    pub soql_query_body: Box<SoqlQueryBody>,
    pub node_context: NodeContext,
}

impl SubQuery {
    pub fn new(node: Node) -> Self {
        let soql_query_body = Box::new(SoqlQueryBody::new(node.c_by_k("soql_query_body")));
        Self {
            soql_query_body,
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for SubQuery {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            let docs = vec![self.soql_query_body.build(b)];
            let sep = Insertable::new::<&str>(None, None, Some(b.softline()));
            let open = Insertable::new(None, Some("("), Some(b.maybeline()));
            let close = Insertable::new(Some(b.maybeline()), Some(")"), None);
            let doc = b.group_surround(&docs, sep, open, close);
            result.push(doc);
        });
    }
}

#[derive(Debug)]
pub struct MapCreationExpression {
    type_: SimpleType,
    value: MapInitializer,
    pub node_context: NodeContext,
}

impl MapCreationExpression {
    pub fn new(node: Node) -> Self {
        assert_check(node, "map_creation_expression");

        Self {
            type_: SimpleType::new(node.c_by_n("type")),
            value: MapInitializer::new(node.c_by_n("value")),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for MapCreationExpression {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(b.txt_("new"));
            result.push(self.type_.build(b));
            result.push(self.value.build(b));
        });
    }
}

#[derive(Debug)]
pub struct MapInitializer {
    initializers: Vec<MapKeyInitializer>,
    pub node_context: NodeContext,
}

impl MapInitializer {
    pub fn new(node: Node) -> Self {
        assert_check(node, "map_initializer");

        let initializers: Vec<_> = node
            .children_vec()
            .into_iter()
            .map(|n| MapKeyInitializer::new(n))
            .collect();

        Self {
            initializers,
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for MapInitializer {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            let docs = b.to_docs(&self.initializers);

            let sep = Insertable::new::<&str>(None, None, Some(b.softline()));
            let open = Insertable::new(None, Some("{"), Some(b.softline()));
            let close = Insertable::new(Some(b.softline()), Some("}"), None);
            let doc = b.group_surround(&docs, sep, open, close);
            result.push(doc);
        });
    }
}

#[derive(Debug)]
pub struct MapKeyInitializer {
    pub exp1: Box<Expression>,
    pub exp2: Box<Expression>,
    pub node_context: NodeContext,
}

impl MapKeyInitializer {
    pub fn new(node: Node) -> Self {
        assert_check(node, "map_key_initializer");

        let children = node.children_vec();
        if children.len() != 2 {
            panic!("### must be exactly 2 child nodes in MapKeyInitializer");
        }

        Self {
            exp1: Box::new(Expression::new(children[0])),
            exp2: Box::new(Expression::new(children[1])),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for MapKeyInitializer {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(self.exp1.build(b));
            result.push(b._txt_("=>"));
            result.push(self.exp2.build(b));
        });
    }
}

#[derive(Debug)]
pub struct GroupByClause {
    pub exps: Vec<GroupByExpression>,
    pub have_clause: Option<HavingClause>,
    pub node_context: NodeContext,
}

impl GroupByClause {
    pub fn new(node: Node) -> Self {
        assert_check(node, "group_by_clause");

        let mut exps = Vec::new();
        let mut have_clause = None;

        for child in node.children_vec() {
            match child.kind() {
                "field_identifier" => {
                    exps.push(GroupByExpression::Field(FieldIdentifier::new(child)));
                }
                "function_expression" => {
                    exps.push(GroupByExpression::Func(FunctionExpression::new(child)));
                }
                "having_clause" => {
                    have_clause = Some(HavingClause::new(child));
                }
                other => {
                    panic!("## unknown node: {} in GroupByClause", red(other));
                }
            }
        }
        Self {
            exps,
            have_clause,
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for GroupByClause {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(b.txt_("GROUP BY"));

            let docs = b.to_docs(&self.exps);
            let sep = Insertable::new(None, Some(" "), None);
            let doc = b.intersperse(&docs, sep);
            result.push(doc);

            if let Some(ref n) = self.have_clause {
                result.push(b.softline());
                result.push(n.build(b));
            }
        });
    }
}

#[derive(Debug)]
pub enum GroupByExpression {
    Field(FieldIdentifier),
    Func(FunctionExpression),
}

impl<'a> DocBuild<'a> for GroupByExpression {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            Self::Field(n) => {
                result.push(n.build(b));
            }
            Self::Func(n) => {
                result.push(n.build(b));
            }
        }
    }
}

#[derive(Debug)]
pub struct HavingClause {
    pub boolean_exp: BooleanExpression,
    pub node_context: NodeContext,
}

impl HavingClause {
    pub fn new(node: Node) -> Self {
        assert_check(node, "having_clause");

        Self {
            boolean_exp: BooleanExpression::new(node.first_c()),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for HavingClause {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            let docs = vec![
                b.txt("HAVING"),
                b.softline(),
                self.boolean_exp.build_with_parent(b, None),
            ];
            result.push(b.group_indent_concat(docs));
        });
    }
}

#[derive(Debug)]
pub struct SoslWithClause {
    pub with_type: SoslWithType,
    pub node_context: NodeContext,
}

impl SoslWithClause {
    pub fn new(node: Node) -> Self {
        assert_check(node, "with_clause");

        Self {
            with_type: SoslWithType::new(node.c_by_k("with_type")),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for SoslWithClause {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            let docs = vec![b.txt_("WITH"), self.with_type.build(b)];
            result.push(b.group_concat(docs));
        });
    }
}

#[derive(Debug)]
pub struct SoqlWithClause {
    pub with_type: SoqlWithType,
    pub node_context: NodeContext,
}

impl SoqlWithClause {
    pub fn new(node: Node) -> Self {
        assert_check(node, "with_clause");

        Self {
            with_type: SoqlWithType::new(node.c_by_k("with_type")),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for SoqlWithClause {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(b.txt_("WITH"));
            result.push(self.with_type.build(b));
        });
    }
}

#[derive(Debug)]
pub enum SoqlWithTypeVariant {
    SimpleType(ValueNode), // Security_Enforced, User_Mode, and System_Mode
    //RecordVisibility(WithRecordVisibilityExpression),
    //DataCategory(WithDataCatExpression),
    UserId(ValueNode),
}

impl SoqlWithTypeVariant {
    pub fn new(node: Node) -> Self {
        let with_type = if node.named_child_count() == 0 {
            return Self::SimpleType(ValueNode::new(node));
        } else {
            let child = node.first_c();
            match child.kind() {
                "with_user_id_type" => Self::UserId(ValueNode::new(child.c_by_k("string_literal"))),
                _ => panic_unknown_node(node, "WithType"),
            }
        };
        with_type
    }
}

impl<'a> DocBuild<'a> for SoqlWithTypeVariant {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            Self::SimpleType(n) => {
                result.push(n.build(b));
            }
            Self::UserId(n) => {
                result.push(b.txt_("UserId ="));
                result.push(n.build(b));
            }
        }
    }
}

#[derive(Debug)]
pub enum SoslWithType {
    DataCat(WithDataCatExpression),
    Division(WithDivisionExpression),
    Snippet(WithSnippetExpression),
    Network(WithNetworkExpression),
    Metadata(WithMetadataExpression),
    Highlight,
    Spell(WithSpellCorrectionExpression),
    PriceBook(WithPriceBookExpression),
}

impl SoslWithType {
    pub fn new(node: Node) -> Self {
        assert_check(node, "with_type");

        let child = node.first_c();
        match child.kind() {
            "with_data_cat_expression" => Self::DataCat(WithDataCatExpression::new(child)),
            "with_division_expression" => Self::Division(WithDivisionExpression::new(child)),
            "with_snippet_expression" => Self::Snippet(WithSnippetExpression::new(child)),
            "with_network_expression" => Self::Network(WithNetworkExpression::new(child)),
            "with_metadata_expression" => Self::Metadata(WithMetadataExpression::new(child)),
            "with_spell_correction_expression" => {
                Self::Spell(WithSpellCorrectionExpression::new(child))
            }
            "with_highlight" => Self::Highlight,
            "with_pricebook_expression" => Self::PriceBook(WithPriceBookExpression::new(child)),
            _ => panic_unknown_node(child, "SoslWithType"),
        }
    }
}

impl<'a> DocBuild<'a> for SoslWithType {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            Self::DataCat(n) => {
                result.push(n.build(b));
            }
            Self::Division(n) => {
                result.push(n.build(b));
            }
            Self::Snippet(n) => {
                result.push(n.build(b));
            }
            Self::Network(n) => {
                result.push(n.build(b));
            }
            Self::Metadata(n) => {
                result.push(n.build(b));
            }
            Self::Highlight => {
                result.push(b.txt("HIGHLIGHT"));
            }
            Self::Spell(n) => {
                result.push(n.build(b));
            }
            Self::PriceBook(n) => {
                result.push(n.build(b));
            }
        }
    }
}

#[derive(Debug)]
pub struct WithDataCatExpression {
    pub filters: Vec<WithDataCatFilter>,
    pub node_context: NodeContext,
}

impl WithDataCatExpression {
    pub fn new(node: Node) -> Self {
        assert_check(node, "with_data_cat_expression");

        let filters = node
            .cs_by_k("with_data_cat_filter")
            .into_iter()
            .map(|n| WithDataCatFilter::new(n))
            .collect();

        Self {
            filters,
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for WithDataCatExpression {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(b.txt("DATA CATEGORY"));
            result.push(b.indent(b.softline()));

            let docs = b.to_docs(&self.filters);
            let sep = Insertable::new::<&str>(Some(b.softline()), Some("AND "), None);
            let doc = b.indent(b.intersperse(&docs, sep));
            result.push(doc);
        });
    }
}

#[derive(Debug)]
pub struct WithDataCatFilter {
    pub identifier: ValueNode,
    pub filter_type: ValueNodeUpperCase,
    pub identifiers: Vec<ValueNode>,
    pub node_context: NodeContext,
}

impl WithDataCatFilter {
    pub fn new(node: Node) -> Self {
        assert_check(node, "with_data_cat_filter");

        let all_identififers = node.cs_by_k("identifier");
        if all_identififers.len() < 2 {
            panic!("At least 2 identifier nodes should exist in WithDataCatFilter");
        }

        let identifier = ValueNode::new(all_identififers[0]);
        let identifiers: Vec<_> = all_identififers
            .into_iter()
            .skip(1)
            .map(|n| ValueNode::new(n))
            .collect();

        Self {
            identifier,
            filter_type: ValueNodeUpperCase::new(node.c_by_k("with_data_cat_filter_type")),
            identifiers,
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for WithDataCatFilter {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(self.identifier.build(b));
            result.push(b.txt(" "));
            result.push(self.filter_type.build(b));
            result.push(b.txt(" "));

            if self.identifiers.len() == 1 {
                result.push(self.identifiers[0].build(b));
            } else {
                let docs: Vec<DocRef<'a>> = b.to_docs(&self.identifiers);

                let sep = Insertable::new::<&str>(None, None, Some(b.softline()));
                let open = Insertable::new(None, Some("("), Some(b.maybeline()));
                let close = Insertable::new(Some(b.maybeline()), Some(")"), None);
                let doc = b.group_surround(&docs, sep, open, close);
                result.push(doc);
            }
        });
    }
}

#[derive(Debug)]
pub enum WithDivisionExpression {
    Bound(BoundApexExpression),
    StringLiteral(String),
}

impl WithDivisionExpression {
    pub fn new(node: Node) -> Self {
        assert_check(node, "with_division_expression");

        let child = node.first_c();
        match child.kind() {
            "bound_apex_expression" => Self::Bound(BoundApexExpression::new(child)),
            "string_literal" => Self::StringLiteral(child.value()),
            _ => panic_unknown_node(node, "WithDivisionExpression"),
        }
    }
}

impl<'a> DocBuild<'a> for WithDivisionExpression {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        result.push(b.txt("DIVISION = "));
        match self {
            Self::Bound(n) => {
                result.push(n.build(b));
            }
            Self::StringLiteral(n) => {
                result.push(b.txt(n));
            }
        }
    }
}

#[derive(Debug)]
pub struct WithSnippetExpression {
    int: Option<ValueNode>,
    pub node_context: NodeContext,
}

impl WithSnippetExpression {
    pub fn new(node: Node) -> Self {
        assert_check(node, "with_snippet_expression");

        let int = node.try_c_by_k("int").map(|n| ValueNode::new(n));

        Self {
            int,
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for WithSnippetExpression {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(b.txt("SNIPPET"));

            if let Some(ref n) = self.int {
                result.push(b.txt("(TARGET_LENGTH = "));
                result.push(n.build(b));
                result.push(b.txt(")"));
            }
        });
    }
}

#[derive(Debug)]
pub struct WithNetworkExpression {
    comparison: Comparison,
    pub node_context: NodeContext,
}

impl WithNetworkExpression {
    pub fn new(node: Node) -> Self {
        assert_check(node, "with_network_expression");

        Self {
            comparison: get_comparsion(&node),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for WithNetworkExpression {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(b.txt("NETWORK"));
            result.push(self.comparison.build(b));
        });
    }
}

#[derive(Debug)]
pub struct WithMetadataExpression {
    string_literal: ValueNode,
    pub node_context: NodeContext,
}

impl WithMetadataExpression {
    pub fn new(node: Node) -> Self {
        assert_check(node, "with_metadata_expression");

        Self {
            string_literal: ValueNode::new(node.c_by_k("string_literal")),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for WithMetadataExpression {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(b.txt("METADATA = "));
            result.push(self.string_literal.build(b));
        });
    }
}

#[derive(Debug)]
pub struct WithSpellCorrectionExpression {
    boolean: ValueNode,
    pub node_context: NodeContext,
}

impl WithSpellCorrectionExpression {
    pub fn new(node: Node) -> Self {
        assert_check(node, "with_spell_correction_expression");

        Self {
            boolean: ValueNode::new(node.c_by_k("boolean")),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for WithSpellCorrectionExpression {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(b.txt("SPELL_CORRECTION = "));
            result.push(self.boolean.build(b));
        });
    }
}

#[derive(Debug)]
pub struct WithPriceBookExpression {
    string_literal: ValueNode,
    pub node_context: NodeContext,
}

impl WithPriceBookExpression {
    pub fn new(node: Node) -> Self {
        assert_check(node, "with_pricebook_expression");

        Self {
            string_literal: ValueNode::new(node.c_by_k("string_literal")),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for WithPriceBookExpression {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(b.txt_("PriceBookId ="));
            result.push(self.string_literal.build(b));
        });
    }
}

#[derive(Debug)]
pub struct DottedIdentifier {
    identifiers: Vec<ValueNode>,
    pub node_context: NodeContext,
}

impl DottedIdentifier {
    pub fn new(node: Node) -> Self {
        assert_check(node, "dotted_identifier");

        let identifiers = node
            .cs_by_k("identifier")
            .into_iter()
            .map(|n| ValueNode::new(n))
            .collect();

        Self {
            identifiers,
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for DottedIdentifier {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            let docs: Vec<_> = self.identifiers.iter().map(|n| n.build(b)).collect();
            let sep = Insertable::new(None, Some("."), None);
            let doc = b.intersperse(&docs, sep);
            result.push(doc);
        });
    }
}

// a general node to store simple String value only
// it's used for the purpose of handling comment bucket logic
#[derive(Debug)]
pub struct ValueNode {
    pub value: String,
    pub node_context: NodeContext,
}

impl ValueNode {
    pub fn new(node: Node) -> Self {
        Self {
            value: node.value(),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for ValueNode {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(b.txt(&self.value));
        });
    }
}

#[derive(Debug)]
pub struct ValueNodeLowerCase {
    pub value: String,
    pub node_context: NodeContext,
}

impl ValueNodeLowerCase {
    pub fn new(node: Node) -> Self {
        Self {
            value: node.value(),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for ValueNodeLowerCase {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(b.txt(self.value.to_lowercase()));
        });
    }
}

#[derive(Debug)]
pub struct ValueNodeUpperCase {
    pub value: String,
    pub node_context: NodeContext,
}

impl ValueNodeUpperCase {
    pub fn new(node: Node) -> Self {
        Self {
            value: node.value(),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for ValueNodeUpperCase {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(b.txt(self.value.to_uppercase()));
        });
    }
}

#[derive(Debug)]
pub struct ExpressionStatement {
    pub exp: Expression,
    pub node_context: NodeContext,
}

impl ExpressionStatement {
    pub fn new(node: Node) -> Self {
        assert_check(node, "expression_statement");

        Self {
            exp: Expression::new(node.first_c()),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for ExpressionStatement {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(self.exp.build(b));
        });
    }
}

#[derive(Debug)]
pub struct SafeNavigationOperator {
    pub node_context: NodeContext,
}

impl SafeNavigationOperator {
    pub fn new(node: Node) -> Self {
        assert_check(node, "safe_navigation_operator");

        Self {
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for SafeNavigationOperator {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(b.txt("?."));
        });
    }
}

#[derive(Debug)]
pub struct CountExpression {
    pub function_name: ValueNode,
    pub node_context: NodeContext,
}

impl CountExpression {
    pub fn new(node: Node) -> Self {
        assert_check(node, "count_expression");

        Self {
            function_name: ValueNode::new(node.c_by_n("function_name")),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for CountExpression {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(self.function_name.build(b));
            result.push(b.txt("()"));
        });
    }
}

#[derive(Debug)]
pub struct FunctionExpression {
    pub variant: FunctionExpressionVariant,
    pub node_context: NodeContext,
}

impl FunctionExpression {
    pub fn new(node: Node) -> Self {
        assert_check(node, "function_expression");

        Self {
            variant: FunctionExpressionVariant::new(node),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for FunctionExpression {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(self.variant.build(b));
        });
    }
}

#[derive(Debug)]
pub struct FieldIdentifier {
    pub variant: FieldIdentifierVariant,
    pub node_context: NodeContext,
}

impl FieldIdentifier {
    pub fn new(node: Node) -> Self {
        assert_check(node, "field_identifier");

        Self {
            variant: FieldIdentifierVariant::new(node),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for FieldIdentifier {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(self.variant.build(b));
        });
    }
}

#[derive(Debug)]
pub struct GeoLocationType {
    pub variant: GeoLocationTypeVariant,
    pub node_context: NodeContext,
}

impl GeoLocationType {
    pub fn new(node: Node) -> Self {
        assert_check(node, "geo_location_type");

        Self {
            variant: GeoLocationTypeVariant::new(node),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for GeoLocationType {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(self.variant.build(b));
        });
    }
}

#[derive(Debug)]
pub struct SelectClause {
    pub variant: SelectClauseVariant,
    pub node_context: NodeContext,
}

impl SelectClause {
    pub fn new(node: Node) -> Self {
        assert_check(node, "select_clause");

        Self {
            variant: SelectClauseVariant::new(node),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for SelectClause {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(self.variant.build(b));
        });
    }
}

#[derive(Debug)]
pub struct StorageIdentifier {
    pub variant: StorageIdentifierVariant,
    pub node_context: NodeContext,
}

impl StorageIdentifier {
    pub fn new(node: Node) -> Self {
        assert_check(node, "storage_identifier");

        Self {
            variant: StorageIdentifierVariant::new(node),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for StorageIdentifier {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(self.variant.build(b));
        });
    }
}

#[derive(Debug)]
pub struct AndExpression {
    pub condition_exps: Vec<ConditionExpression>,
    pub node_context: NodeContext,
}

impl AndExpression {
    pub fn new(node: Node) -> Self {
        assert_check(node, "and_expression");

        let condition_exps = node
            .children_vec()
            .into_iter()
            .map(|n| ConditionExpression::new(n))
            .collect();

        Self {
            condition_exps,
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for AndExpression {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            let docs: Vec<DocRef> = self
                .condition_exps
                .iter()
                .map(|expr| expr.build_with_parent(b, Some("AND")))
                .collect();
            let sep = Insertable::new(Some(b.softline()), Some("AND "), None);
            result.push(b.intersperse(&docs, sep));
        });
    }
}

#[derive(Debug)]
pub struct OrExpression {
    pub condition_exps: Vec<ConditionExpression>,
    pub node_context: NodeContext,
}

impl OrExpression {
    pub fn new(node: Node) -> Self {
        assert_check(node, "or_expression");

        let condition_exps = node
            .children_vec()
            .into_iter()
            .map(|n| ConditionExpression::new(n))
            .collect();

        Self {
            condition_exps,
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for OrExpression {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            let docs: Vec<DocRef> = self
                .condition_exps
                .iter()
                .map(|expr| expr.build_with_parent(b, Some("OR")))
                .collect();
            let sep = Insertable::new(Some(b.softline()), Some("OR "), None);
            result.push(b.intersperse(&docs, sep));
        });
    }
}

#[derive(Debug)]
pub struct NotExpression {
    pub condition_exp: ConditionExpression,
    pub node_context: NodeContext,
}

impl NotExpression {
    pub fn new(node: Node) -> Self {
        assert_check(node, "not_expression");

        Self {
            condition_exp: ConditionExpression::new(node.first_c()),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for NotExpression {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            let expr_doc = self.condition_exp.build_with_parent(b, Some("NOT"));
            let doc = b.concat(vec![b.txt_("NOT"), expr_doc]);
            result.push(doc);
        });
    }
}

#[derive(Debug)]
pub struct SoqlWithType {
    pub variant: SoqlWithTypeVariant,
    pub node_context: NodeContext,
}

impl SoqlWithType {
    pub fn new(node: Node) -> Self {
        assert_check(node, "with_type");

        Self {
            variant: SoqlWithTypeVariant::new(node),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for SoqlWithType {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(self.variant.build(b));
        });
    }
}

#[derive(Debug)]
pub struct ForClause {
    pub for_type: ValueNode,
    pub node_context: NodeContext,
}

impl ForClause {
    pub fn new(node: Node) -> Self {
        assert_check(node, "for_clause");

        Self {
            for_type: ValueNode::new(node.c_by_k("for_type")),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for ForClause {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(self.for_type.build(b));
        });
    }
}

#[derive(Debug)]
pub struct AllRowsClause {
    pub node_context: NodeContext,
}

impl AllRowsClause {
    pub fn new(node: Node) -> Self {
        assert_check(node, "all_rows_clause");

        Self {
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for AllRowsClause {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(b.txt("ALL ROWS"));
        });
    }
}

#[derive(Debug)]
pub struct UpdateExpression {
    pub variant: UpdateExpressionVariant,
    pub node_context: NodeContext,
}

impl UpdateExpression {
    pub fn new(node: Node) -> Self {
        assert_check(node, "update_expression");

        Self {
            variant: UpdateExpressionVariant::new(node),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for UpdateExpression {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(self.variant.build(b));
        });
    }
}

#[derive(Debug)]
pub struct DmlExpression {
    pub variant: DmlExpressionVariant,
    pub node_context: NodeContext,
}

impl DmlExpression {
    pub fn new(node: Node) -> Self {
        assert_check(node, "dml_expression");

        Self {
            variant: DmlExpressionVariant::new(node),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for DmlExpression {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(self.variant.build(b));
        });
    }
}

#[derive(Debug)]
pub struct DmlType {
    pub variant: DmlTypeVariant,
    pub node_context: NodeContext,
}

impl DmlType {
    pub fn new(node: Node) -> Self {
        assert_check(node, "dml_type");

        Self {
            variant: DmlTypeVariant::new(node.first_c()),
            node_context: NodeContext::with_punctuation(&node),
        }
    }
}

impl<'a> DocBuild<'a> for DmlType {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        build_with_comments_and_punc(b, &self.node_context, result, |b, result| {
            result.push(self.variant.build(b));
        });
    }
}
