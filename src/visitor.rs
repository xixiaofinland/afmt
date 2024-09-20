use crate::{
    context::FmtContext,
    node_ext::*,
    node_struct::*,
    shape::{Indent, Shape},
};
use colored::Colorize;
use tree_sitter::Node;

pub struct Visitor {
    //pub block_indent: Indent,
    //pub buffer: String,
}

impl Visitor {
    pub fn default() -> Self {
        Visitor::new(Indent::default())
    }

    //pub fn new(parent_context: Option<&'a FmtContext<'_>>, block_indent: Indent) -> Self {
    pub fn new(block_indent: Indent) -> Self {
        Self {
            //block_indent,
            //buffer: String::new(),
        }
    }

    pub fn visit_root(&mut self, context: &FmtContext) -> String {
        let mut result = visit_root_children(&context.ast_tree.root_node(), context);

        // remove the extra "\n" introduced by the top-level class declaration
        result.truncate(result.trim_end_matches('\n').len());
        result
    }
}

pub fn visit_node(node: &Node, context: &FmtContext, shape: &mut Shape) -> String {
    if node.is_named() && node.grammar_name() == "operator" {
        return node.v(context.source_code).to_string();
    }

    let kind = NodeKind::from_kind(node.kind());
    match kind {
        NodeKind::ClassDeclaration => {
            let n = ClassDeclaration::new(node);
            n.rewrite(context, shape)
        }
        NodeKind::MethodDeclaration => {
            let n = MethodDeclaration::new(node);
            n.rewrite(context, shape)
        }
        NodeKind::FieldDeclaration => {
            let n = FieldDeclaration::new(node);
            n.rewrite(context, shape)
        }
        NodeKind::WhileStatement => {
            let n = WhileStatement::new(node);
            n.rewrite(context, shape)
        }
        NodeKind::Statement => {
            let n = Statement::new(node);
            n.rewrite(context, shape)
        }
        NodeKind::EmptyNode => visit_standalone_children(node, context, shape),
        NodeKind::Block => {
            let n = Block::new(node);
            n.rewrite(context, shape)
        }
        NodeKind::Interfaces => {
            let n = Interfaces::new(node);
            n.rewrite(context, shape)
        }
        NodeKind::SuperClass => {
            let n = SuperClass::new(node);
            n.rewrite(context, shape)
        }
        NodeKind::Expression => {
            let n = Expression::new(node);
            n.rewrite(context, shape)
        }
        NodeKind::Value => {
            let n = Value::new(node);
            n.rewrite(context, shape)
        }
        NodeKind::ValueSpace => {
            let n = ValueSpace::new(node);
            n.rewrite(context, shape)
        }
        NodeKind::SpaceValueSpace => {
            let n = SpaceValueSpace::new(node);
            n.rewrite(context, shape)
        }
        NodeKind::LocalVariableDeclaration => {
            let n = LocalVariableDeclaration::new(node);
            n.rewrite(context, shape)
        }
        NodeKind::VariableDeclarator => {
            let n = VariableDeclarator::new(node);
            n.rewrite(context, shape)
        }
        NodeKind::ArgumentList => {
            let n = ArgumentList::new(node);
            n.rewrite(context, shape)
        }
        NodeKind::ArrayInitializer => {
            let n = ArrayInitializer::new(node);
            n.rewrite(context, shape)
        }
        NodeKind::ForStatement => {
            let n = ForStatement::new(node);
            n.rewrite(context, shape)
        }
        NodeKind::EnhancedForStatement => {
            let n = EnhancedForStatement::new(node);
            n.rewrite(context, shape)
        }
        NodeKind::IfStatement => {
            let n = IfStatement::new(node);
            n.rewrite(context, shape)
        }
        NodeKind::ParenthesizedExpression => {
            let n = ParenthesizedExpression::new(node);
            n.rewrite(context, shape)
        }
        NodeKind::LineComment => {
            let n = LineComment::new(node);
            n.rewrite(context, shape)
        }
        NodeKind::ReturnStatement => {
            let n = ReturnStatement::new(node);
            n.rewrite(context, shape)
        }
        NodeKind::TypeArguments => {
            let n = TypeArguments::new(node);
            n.rewrite(context, shape)
        }
        NodeKind::GenericType => {
            let n = GenericType::new(node);
            n.rewrite(context, shape)
        }
        NodeKind::DimensionsExpr => {
            let n = DimensionsExpr::new(node);
            n.rewrite(context, shape)
        }
        NodeKind::ArrayType => {
            let n = ArrayType::new(node);
            n.rewrite(context, shape)
        }
        NodeKind::MapInitializer => {
            let n = MapInitializer::new(node);
            n.rewrite(context, shape)
        }
        NodeKind::Annotation => {
            let n = Annotation::new(node);
            n.rewrite(context, shape)
        }
        NodeKind::AnnotationArgumentList => {
            let n = AnnotationArgumentList::new(node);
            n.rewrite(context, shape)
        }
        NodeKind::AnnotationKeyValue => {
            let n = AnnotationKeyValue::new(node);
            n.rewrite(context, shape)
        }
        NodeKind::Modifiers => {
            let n = Modifiers::new(node);
            n.rewrite(context, shape)
        }
        NodeKind::ConstructorDeclaration => {
            let n = ConstructorDeclaration::new(node);
            n.rewrite(context, shape)
        }
        NodeKind::ConstructorBody => {
            let n = ConstructorBody::new(node);
            n.rewrite(context, shape)
        }
        NodeKind::ExplicitConstructorInvocation => {
            let n = ExplicitConstructorInvocation::new(node);
            n.rewrite(context, shape)
        }
        NodeKind::AssignmentExpression => {
            let n = AssignmentExpression::new(node);
            n.rewrite(context, shape)
        }
        NodeKind::DmlExpression => {
            let n = DmlExpression::new(node);
            n.rewrite(context, shape)
        }
        NodeKind::DmlType => {
            let n = DmlType::new(node);
            n.rewrite(context, shape)
        }
        NodeKind::RunAsStatement => {
            let n = RunAsStatement::new(node);
            n.rewrite(context, shape)
        }
        NodeKind::PrimaryExpression => {
            let n = PrimaryExpression::new(node);
            n.rewrite(context, shape)
        }
        NodeKind::ArrayAccess => {
            let n = ArrayAccess::new(node);
            n.rewrite(context, shape)
        }
        NodeKind::ScopedTypeIdentifier => {
            let n = ScopedTypeIdentifier::new(node);
            n.rewrite(context, shape)
        }
        NodeKind::DmlSecurityMode => {
            let n = DmlSecurityMode::new(node);
            n.rewrite(context, shape)
        }
        NodeKind::EnumDeclaration => {
            let n = EnumDeclaration::new(node);
            n.rewrite(context, shape)
        }
        NodeKind::EnumConstant => {
            let n = EnumConstant::new(node);
            n.rewrite(context, shape)
        }
        NodeKind::TryStatement => {
            let n = TryStatement::new(node);
            n.rewrite(context, shape)
        }
        NodeKind::FieldAccess => {
            let n = FieldAccess::new(node);
            n.rewrite(context, shape)
        }
        NodeKind::InstanceOfExpression => {
            let n = InstanceOfExpression::new(node);
            n.rewrite(context, shape)
        }
        NodeKind::TernaryExpression => {
            let n = TernaryExpression::new(node);
            n.rewrite(context, shape)
        }
        _ => {
            println!(
                "{} {}",
                "### Visitor: unknown node: ".yellow(),
                node.kind().red()
            );
            panic!();
        }
    }
}

pub fn visit_root_children(root: &Node, context: &FmtContext) -> String {
    let mut result = String::new();
    let shape = Shape::empty(context.config);

    let mut cursor = root.walk();
    let children = root
        .named_children(&mut cursor)
        .map(|child| {
            let mut child_shape = shape.clone_with_stand_alone(true);
            visit_node(&child, context, &mut child_shape)
        })
        .collect::<Vec<_>>()
        .join("\n");

    result.push_str(&children);
    result
}

pub fn visit_standalone_children(node: &Node, context: &FmtContext, shape: &Shape) -> String {
    let mut result = String::new();
    let shape = shape.copy_with_indent_block_plus(context.config);

    let mut cursor = node.walk();
    let children = node
        .named_children(&mut cursor)
        .map(|child| {
            let mut child_shape = shape.clone_with_stand_alone(true);
            visit_node(&child, context, &mut child_shape)
        })
        .collect::<Vec<_>>()
        .join("\n");

    if !children.is_empty() {
        result.push_str(&children);
        result.push('\n');
    }
    result
}
