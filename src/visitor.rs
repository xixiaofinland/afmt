use crate::{
    context::FmtContext,
    node_ext::*,
    node_struct::*,
    shape::{Indent, Shape},
    utility::*,
};
use anyhow::{bail, Context, Result};
use tree_sitter::Node;

pub struct Visitor {
    //parent_context: Option<&'a FmtContext<'_>>,
    pub block_indent: Indent,
    pub buffer: String,
}

impl Visitor {
    pub fn default() -> Self {
        Visitor::new(Indent::default())
    }

    //pub fn new(parent_context: Option<&'a FmtContext<'_>>, block_indent: Indent) -> Self {
    pub fn new(block_indent: Indent) -> Self {
        Self {
            block_indent,
            buffer: String::new(),
        }
    }

    pub fn visit_root(&mut self, context: &FmtContext) -> String {
        let shape = Shape::empty(context.config);
        let mut result = visit_named_children(&context.ast_tree.root_node(), context, &shape);

        // remove the extra "\n" introduced by the top-level class declaration
        result.truncate(result.trim_end_matches('\n').len());
        result
    }
}

pub fn visit_node(node: &Node, context: &FmtContext, shape: &mut Shape) -> String {
    if node.is_named() {
        match node.grammar_name() {
            "operator" => {
                return node.get_value(context.source_code).to_string();
            }
            _ => {}
        }
    }

    let kind = NodeKind::from_kind(node.kind());
    match kind {
        NodeKind::ClassDeclaration => {
            let n = ClassDeclaration::new(&node);
            n.rewrite(context, shape)
        }
        NodeKind::MethodDeclaration => {
            let n = MethodDeclaration::new(&node);
            n.rewrite(context, shape)
        }
        NodeKind::FieldDeclaration => {
            let n = FieldDeclaration::new(&node);
            n.rewrite(context, shape)
        }
        NodeKind::Statement => {
            let n = Statement::new(&node);
            n.rewrite(context, shape)
        }
        NodeKind::EmptyNode => visit_named_children(node, context, shape),
        NodeKind::Expression => {
            let n = Expression::new(&node);
            n.rewrite(context, shape)
        }
        NodeKind::Value => {
            let n = Value::new(&node);
            n.rewrite(context, shape)
        }
        NodeKind::ValueSpace => {
            let n = ValueSpace::new(&node);
            n.rewrite(context, shape)
        }
        NodeKind::SpaceValueSpace => {
            let n = SpaceValueSpace::new(&node);
            n.rewrite(context, shape)
        }
        NodeKind::LocalVariableDeclaration => {
            let n = LocalVariableDeclaration::new(&node);
            n.rewrite(context, shape)
        }
        NodeKind::VariableDeclarator => {
            let n = VariableDeclarator::new(&node);
            n.rewrite(context, shape)
        }
        NodeKind::IfStatement => {
            let n = IfStatement::new(&node);
            n.rewrite(context, shape)
        }
        NodeKind::ParenthesizedExpression => {
            let n = ParenthesizedExpression::new(&node);
            n.rewrite(context, shape)
        }
        _ => {
            panic!("### Unknow node: {}", node.kind());
        }
    }
}

pub fn visit_named_children(node: &Node, context: &FmtContext, shape: &Shape) -> String {
    let mut result = String::new();

    let is_root_node = node.kind() == "parser_output";
    let mut shape = if is_root_node {
        Shape::empty(context.config)
    } else {
        shape.copy_with_indent_block_plus(context.config)
    };

    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        let is_standalone = is_standalone(&child);
        if is_standalone {
            let child_shape = shape.clone(); // standalone node should use its own shape;
            result.push_str(&child_shape.indent.to_string(context.config));
        }

        result.push_str(&visit_node(&child, context, &mut shape));

        if is_standalone {
            if has_body_node(&child) {
                //result.push_str("\n");
            } else {
                result.push_str(";\n");
            }
        }
    }
    result
}

pub fn visit_named_children_in_same_line(
    node: &Node,
    context: &FmtContext,
    shape: &mut Shape,
) -> String {
    let mut result = String::new();
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        result.push_str(&visit_node(&child, context, shape));
    }
    result
}
