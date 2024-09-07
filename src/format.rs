use crate::node_ext::*;
use crate::shape::Shape;
use crate::visitor::*;
use crate::{context::FmtContext, node_struct::*, utility::*};
use anyhow::{bail, Context, Result};
use tree_sitter::Node;

impl Visitor {
    pub fn format_class(&mut self, node: &Node, context: &FmtContext, shape: &mut Shape) {
        let n = ClassDeclaration::new(&node);
        self.push_rewritten(n.rewrite(context, shape), &node);

        let body_node = node
            .child_by_field_name("body")
            .expect("mandatory body node missing");
        self.visit_named_children(&body_node, context, shape);

        self.push_block_close(shape);
    }

    pub fn format_method(&mut self, node: &Node, context: &FmtContext, shape: &mut Shape) {
        let n = MethodDeclaration::new(&node);
        self.push_rewritten(n.rewrite(context, shape), &node);

        let body_node = node
            .child_by_field_name("body")
            .expect("mandatory body node missing");
        self.visit_named_children(&body_node, context, shape);

        self.push_block_close(shape);
    }

    pub fn format_expression_statement(
        &mut self,
        node: &Node,
        context: &FmtContext,
        shape: &mut Shape,
    ) {
        let child = node
            .named_child(0)
            .expect("ExpressionStatement mandatory child missing.");
        self.visit_item(&child, context, shape);
    }

    pub fn format_binary_expression(
        &mut self,
        node: &Node,
        context: &FmtContext,
        shape: &mut Shape,
    ) {
        self.visit_children_in_same_line(node, context, shape);
    }

    pub fn format_variable_declaration(
        &mut self,
        node: &Node,
        context: &FmtContext,
        shape: &mut Shape,
    ) {
        self.visit_children_in_same_line(node, context, shape);

        match node.next_named_sibling() {
            Some(sibling) if sibling.kind() == "variable_declarator" => self.push_str(", "),
            _ => {}
        }
    }

    fn push_block_open_line(&mut self) {
        self.push_str(" {\n");
    }

    fn push_block_close(&mut self, shape: &mut Shape) {
        self.push_str(&format!("{}}}", get_indent_string(&shape.indent)));
    }

    pub fn format_if_statement(
        &mut self,
        node: &Node,
        context: &FmtContext,
        shape: &mut Shape,
    ) -> Result<()> {
        self.push_str("if");
        let condition = node.get_mandatory_child_by_name("condition");
        self.visit_item(&condition, context, shape);

        self.push_block_open_line();

        let consequence = node.get_mandatory_child_by_name("consequence");
        self.visit_item(&consequence, context, shape);

        self.push_block_close(shape);

        //let condition_node = get_mandatory_child_by_name("condition", node)?;
        //self.visit_item(&condition_node, context, shape);

        Ok(())
    }
}
