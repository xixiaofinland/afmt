use crate::context::FmtContext;
use crate::node_ext::*;
use crate::shape::Shape;
use crate::utility::*;
use crate::visitor::{visit_named_children, visit_named_children_in_same_line, visit_node};
use crate::{define_struct, define_struct_and_enum};
use anyhow::{Context, Result};
use log::debug;
use tree_sitter::Node;

pub trait Rewrite {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String;
}

define_struct_and_enum!(
    true; ClassDeclaration => "class_declaration",
    true; FieldDeclaration => "field_declaration",
    true; MethodDeclaration => "method_declaration",
    false; EmptyNode => "block" | "class_body",
    true; Statement => "expression_statement",
    true; Value => "boolean" | "int" | "identifier"  |  "string_literal",
    true; ValueSpace => "type_identifier",
    true; SpaceValueSpace => "assignment_operator",
    true; Expression => "binary_expression",
    true; LocalVariableDeclaration => "local_variable_declaration",
    true; VariableDeclarator => "variable_declarator",
    true; IfStatement => "if_statement",
    true; ParenthesizedExpression => "parenthesized_expression"
);

impl<'a, 'tree> Rewrite for ClassDeclaration<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let mut result = String::new();

        let modifiers_value = get_modifiers_value(self.node(), context.source_code);
        result.push_str(&modifiers_value);
        result.push_str(" class ");

        let name_node_value = self
            .node()
            .get_mandatory_child_value_by_name("name", context.source_code);
        result.push_str(name_node_value);
        result.push_str(" {\n");
        shape.offset += result.len();

        let body_node = self.node().get_mandatory_child_by_name("body");
        result.push_str(&visit_named_children(&body_node, context, shape));

        result.push_str(&format!("{}}}\n", shape.indent.to_string()));

        result
    }
}

impl<'a, 'tree> Rewrite for MethodDeclaration<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let mut result = String::new();

        let modifier_nodes = get_modifiers(self.node());
        let modifiers_doc = modifier_nodes
            .iter()
            .map(|n| n.get_value(context.source_code))
            .collect::<Vec<&str>>()
            .join(" ");

        result.push_str(&modifiers_doc);
        result.push(' ');

        let type_node_value = self
            .node()
            .get_mandatory_child_value_by_name("type", context.source_code);
        result.push_str(type_node_value);
        result.push(' ');

        let name_node_value = self
            .node()
            .get_mandatory_child_value_by_name("name", context.source_code);
        result.push_str(name_node_value);

        result.push('(');
        let parameters_node = get_parameters(self.node());
        let parameters_doc = parameters_node
            .iter()
            .map(|n| {
                let type_str = n.get_mandatory_child_value_by_name("type", context.source_code);
                let name_str = n.get_mandatory_child_value_by_name("name", context.source_code);
                format!("{} {}", type_str, name_str)
            })
            .collect::<Vec<String>>()
            .join(", ");

        result.push_str(&parameters_doc);
        result.push(')');
        result.push_str(" {\n");
        shape.offset += result.len();

        let body_node = self.node().get_mandatory_child_by_name("body");
        result.push_str(&visit_named_children(&body_node, context, shape));

        result.push_str(&format!("{}}}\n", shape.indent.to_string()));

        result
    }
}

impl<'a, 'tree> Rewrite for FieldDeclaration<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let mut result = String::new();

        let modifier_nodes = get_modifiers(self.node());
        let modifiers_doc = modifier_nodes
            .iter()
            .map(|n| n.get_value(context.source_code))
            .collect::<Vec<&str>>()
            .join(" ");

        result.push_str(&modifiers_doc);

        result.push(' ');

        let type_node_value = self
            .node()
            .get_mandatory_child_value_by_name("type", context.source_code);
        result.push_str(type_node_value);

        result.push(' ');

        let name_node_value = self
            .node()
            .get_mandatory_child_by_name("declarator")
            .get_mandatory_child_value_by_name("name", context.source_code);
        result.push_str(name_node_value);
        result
    }
}

impl<'a, 'tree> Rewrite for Value<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let mut result = String::new();
        //result.push_str(&get_indent_string(&shape.indent));

        let name_node_value = self.node().get_value(context.source_code);
        result.push_str(name_node_value);
        result
    }
}

impl<'a, 'tree> Rewrite for SpaceValueSpace<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let mut result = String::from(' ');
        let name_node_value = self.node().get_value(context.source_code);
        result.push_str(name_node_value);
        result.push(' ');
        result
    }
}

impl<'a, 'tree> Rewrite for ValueSpace<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let mut result = String::new();
        let name_node_value = self.node().get_value(context.source_code);
        result.push_str(name_node_value);
        result.push(' ');
        result
    }
}

// TODO:
impl<'a, 'tree> Rewrite for LocalVariableDeclaration<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let type_value = self
            .node()
            .get_mandatory_child_value_by_name("type", context.source_code);

        let declarator_nodes = self.node().get_mandatory_children_by_name("declarator");

        let declarator_values: Vec<String> = declarator_nodes
            .iter()
            .map(|d| {
                let name = d.get_mandatory_child_value_by_name("name", context.source_code);
                let value = d
                    .child_by_field_name("value")
                    .map(|n| format!(" = {}", n.get_value(context.source_code)))
                    .unwrap_or_default();

                format!("{}{}", name, value)
            })
            .collect();

        let result = format!("{} {}", type_value, declarator_values.join(", "));

        debug!("LocalVariable: {}", result);

        result
    }
}

impl<'a, 'tree> Rewrite for Statement<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        match self.node().kind() {
            "expression_statement" => {
                let child = self
                    .node()
                    .named_child(0)
                    .unwrap_or_else(|| panic!("mandatory child expression node missing."));
                let exp = Expression::new(&child);
                exp.rewrite(context, shape)
            }
            _ => unreachable!(),
        }
    }
}

impl<'a, 'tree> Rewrite for VariableDeclarator<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let mut result = String::new();
        match self.node().next_named_sibling() {
            Some(sibling) if sibling.kind() == "variable_declarator" => result.push_str(", "),
            _ => {}
        }
        result
    }
}

impl<'a, 'tree> Rewrite for IfStatement<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let mut result = String::new();
        result.push_str("if");
        let condition = self.node().get_mandatory_child_by_name("condition");
        result.push_str(&visit_node(&condition, context, shape));

        result.push_str(" {\n");

        let consequence = self.node().get_mandatory_child_by_name("consequence");
        result.push_str(&visit_node(&consequence, context, shape));
        result.push_str(&format!("{}}}", get_indent_string(&shape.indent)));

        result
    }
}

impl<'a, 'tree> Rewrite for ParenthesizedExpression<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let mut result = String::new();
        result.push('(');
        result.push_str(&visit_named_children_in_same_line(
            self.node(),
            context,
            shape,
        ));
        result.push(')');

        result
    }
}
