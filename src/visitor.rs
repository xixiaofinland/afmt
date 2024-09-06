use crate::{
    context::FmtContext,
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

    //pub fn from_current(shape: &Shape) -> Visitor {
    //    let block_indent = Indent::new(shape.indent.block_indent, 0);
    //    Visitor::new(block_indent)
    //}

    pub fn push_rewritten(&mut self, rewritten: Option<String>, node: &Node) {
        if let Some(r) = rewritten {
            self.push_str(&r);
        } else {
        }
    }

    pub fn push(&mut self, s: char) {
        self.buffer.push(s);
    }

    pub fn push_str(&mut self, s: &str) {
        self.buffer.push_str(s);
    }

    pub fn visit_root(&mut self, context: &FmtContext, parent_shape: &Shape) {
        self.visit_named_children(&context.ast_tree.root_node(), context, parent_shape);

        // remove the extra "\n" introduced by the top-level class declaration
        self.buffer
            .truncate(self.buffer.trim_end_matches('\n').len());
    }

    pub fn visit_children_in_same_line(
        &mut self,
        node: &Node,
        context: &FmtContext,
        shape: &Shape,
    ) {
        let mut cursor = node.walk();
        for child in node.named_children(&mut cursor) {
            self.visit_item(&child, context, &shape);
        }
    }

    pub fn visit_named_children(
        &mut self,
        node: &Node,
        context: &FmtContext,
        parent_shape: &Shape,
    ) {
        let is_root_node = node.kind() == "parser_output";
        let child_shape = if is_root_node {
            Shape::empty(context.config)
        } else {
            parent_shape.copy_with_indent_block_plus(context.config)
        };

        //println!("shape: {}, {}", node.kind(), shape.indent.block_indent);

        let mut cursor = node.walk();
        for child in node.named_children(&mut cursor) {
            let is_standalone = is_standalone(&child);
            if is_standalone {
                let child_shape = child_shape.clone(); // standalone node should use its own shape;
                self.push_str(&child_shape.indent.to_string());
            }

            self.visit_item(&child, context, &child_shape);

            if is_standalone {
                if has_body_node(&child) {
                    self.push_str("\n");
                } else {
                    self.push_str(";\n");
                }
            }
        }
    }

    pub fn visit_item(&mut self, node: &Node, context: &FmtContext, shape: &Shape) {
        if node.is_named() {
            match node.grammar_name() {
                "operator" => {
                    self.push_str(&format!(" {} ", get_value(node, context.source_code)));
                    return;
                }
                _ => {}
            }
        }

        let kind = NodeKind::from_kind(node.kind());
        //println!("node:{}", node.kind());
        match kind {
            NodeKind::ClassDeclaration => {
                self.format_class(&node, context, &shape);
            }
            NodeKind::MethodDeclaration => {
                self.format_method(&node, context, &shape);
            }
            NodeKind::FieldDeclaration => {
                let n = FieldDeclaration::new(&node);
                self.push_rewritten(n.rewrite(context, &shape), &node);
            }
            NodeKind::ExpressionStatement => {
                self.format_expression_statement(&node, context, &shape);
            }
            NodeKind::EmptyNode => {
                self.visit_named_children(node, context, &shape);
            }
            NodeKind::BinaryExpression => {
                self.format_binary_expression(&node, context, &shape);
            }
            NodeKind::Value => {
                let n = Value::new(&node);
                self.push_rewritten(n.rewrite(context, &shape), &node);
                //println!("kind check: {}:{}", node.kind(), is_standalone);
                //if is_standalone {
                //    self.push_str(";");
                //}
            }
            NodeKind::ValueSpace => {
                let n = ValueSpace::new(&node);
                self.push_rewritten(n.rewrite(context, &shape), &node);
            }
            NodeKind::SpaceValueSpace => {
                let n = SpaceValueSpace::new(&node);
                self.push_rewritten(n.rewrite(context, &shape), &node);
            }
            NodeKind::LocalVariableDeclaration => {
                self.format_local_variable_declaration(&node, context, &shape);
            }
            NodeKind::VariableDeclarator => {
                self.format_variable_declaration(&node, context, &shape)
            }
            NodeKind::IfStatement => {
                self.format_if_statement(&node, context, &shape);
                //let n = FieldDeclaration::new(&node);
                //self.push_rewritten(n.rewrite(context, &shape), &node);
            }
            NodeKind::ParenthesizedExpression => {
                self.push('(');
                self.visit_children_in_same_line(node, context, &shape);
                self.push(')');
            }
            _ => {
                println!("### Unknow node: {}", node.kind());
            }
        }
    }
}
