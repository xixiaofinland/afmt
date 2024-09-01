use crate::{
    config::{Indent, Shape},
    context::FmtContext,
    node_struct::{
        ClassDeclaration, FieldDeclaration, MethodDeclaration, NodeKind, Rewrite, SimpleStatement,
    },
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
        self.visit_named_children(&context.ast_tree.root_node(), context, parent_shape)
            self.buffer.trim_end_matches('\n').to_string();
    }

    pub fn visit_named_children(
        &mut self,
        node: &Node,
        context: &FmtContext,
        parent_shape: &Shape,
    ) {
        let is_root_node = node.kind() == "parser_output";
        let shape = if is_root_node {
            Shape::empty()
        } else {
            Shape::increase_indent(parent_shape)
        };

        //println!("shape: {}, {}", node.kind(), shape.indent.block_indent);

        let mut cursor = node.walk();
        for child in node.named_children(&mut cursor) {
            self.visit_item(&child, context, &shape);
        }
    }

    pub fn visit_item(&mut self, node: &Node, context: &FmtContext, shape: &Shape) {
        let is_standalone = is_standalone(node);
        //println!("standalone? {}, {}", node.kind(), is_standalone);

        if is_standalone {
            self.push_str(&get_indent_string(&shape.indent));
        }

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
            NodeKind::BinaryExpression => {
                self.format_binary_expression(&node, context, &shape);
            }
            NodeKind::SimpleStatement => {
                let n = SimpleStatement::new(&node);
                self.push_rewritten(n.rewrite(context, &shape), &node);
                //println!("kind check: {}:{}", node.kind(), is_standalone);
                if is_standalone {
                    self.push_str(";");
                }
            }
            //NodeKind::Modifiers => {
            //    self.visit_if_node(node);
            //}
            //NodeKind::ForLoop => {
            //    //self.visit_for_node(node);
            //}
            _ => {
                println!("### Unknow node: {}", node.kind());
            }
        }

        if is_standalone {
            if has_body_node(node) {
                self.push_str("\n");
            } else {
                self.push_str(";\n");
            }
        }
    }

    pub fn format_class(&mut self, node: &Node, context: &FmtContext, shape: &Shape) {
        let n = ClassDeclaration::new(&node);
        self.push_rewritten(n.rewrite(context, &shape), &node);

        self.push_block_open_line();

        let body_node = node
            .child_by_field_name("body")
            .expect("mandatory body node missing");
        self.visit_named_children(&body_node, context, &shape);

        self.push_block_close(shape);
    }

    pub fn format_method(&mut self, node: &Node, context: &FmtContext, shape: &Shape) {
        let n = MethodDeclaration::new(&node);
        self.push_rewritten(n.rewrite(context, &shape), &node);

        self.push_block_open_line();

        let body_node = node
            .child_by_field_name("body")
            .expect("mandatory body node missing");
        self.visit_named_children(&body_node, context, &shape);

        self.push_block_close(shape);
    }

    pub fn format_expression_statement(
        &mut self,
        node: &Node,
        context: &FmtContext,
        shape: &Shape,
    ) {
        let child = node
            .named_child(0)
            .expect("ExpressionStatement mandatory child missing.");
        self.visit_item(&child, context, &shape);
    }

    pub fn format_binary_expression(&mut self, node: &Node, context: &FmtContext, shape: &Shape) {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.visit_item(&child, context, &shape);
        }
    }

    fn push_block_open_line(&mut self) {
        self.push_str(" {\n");
    }

    fn push_block_close(&mut self, shape: &Shape) {
        //println!("|{:?}|", &self.block_indent);

        self.push_str(&format!("{}}}", get_indent_string(&shape.indent)));
    }
}
