use crate::context::FmtContext;
use crate::node_ext::*;
use crate::shape::Shape;
use crate::utility::*;
use crate::visitor::{
    visit_named_children_in_same_line, visit_node, visit_standalone_named_children,
};
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
    false; EmptyNode => "class_body",
    true; Block => "block",
    true; Statement => "expression_statement",
    true; Value => "boolean" | "int" | "identifier"  |  "string_literal",
    true; SuperClass => "superclass",
    true; ValueSpace => "type_identifier",
    true; SpaceValueSpace => "assignment_operator",
    true; Expression => "binary_expression" | "int" | "method_invocation",
    true; LocalVariableDeclaration => "local_variable_declaration",
    true; VariableDeclarator => "variable_declarator",
    true; IfStatement => "if_statement",
    true; ParenthesizedExpression => "parenthesized_expression"
);

impl<'a, 'tree> Rewrite for ClassDeclaration<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let node = self.node();
        let mut result = String::new();
        add_standalone_prefix(&mut result, shape, context);

        let modifiers_value = get_modifiers_value(node, context.source_code);
        result.push_str(&modifiers_value);
        result.push_str(" class ");

        let name_node_value = node.get_mandatory_child_value_by_name("name", context.source_code);
        result.push_str(name_node_value);

        if let Some(n) = node.get_child_by_name("superclass") {
            result.push_str(&visit_node(
                &n,
                context,
                &mut shape.clone_with_stand_alone(false),
            ));
        }

        result.push_str(" {\n");

        let body_node = node.get_mandatory_child_by_name("body");
        result.push_str(&visit_standalone_named_children(&body_node, context, shape));
        result.push_str(&format!("{}}}", shape.indent.to_string(context.config)));

        result
    }
}

impl<'a, 'tree> Rewrite for MethodDeclaration<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let mut result = String::new();
        add_standalone_prefix(&mut result, shape, context);

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
            .collect::<Vec<String>>();

        let params_single_line = parameters_doc.join(", ");

        shape.offset = result.len() + 3; // add trailing `) {` size

        if shape.offset + params_single_line.len() <= shape.width {
            result.push_str(&params_single_line);
        } else {
            let param_shape = shape.copy_with_indent_block_plus(context.config);
            result.push('\n');
            for (i, param) in parameters_doc.iter().enumerate() {
                result.push_str(&param_shape.indent.to_string(context.config));
                result.push_str(param);

                if i < parameters_doc.len() - 1 {
                    result.push(',');
                }
                result.push('\n');
            }
            result.push_str(&shape.indent.to_string(context.config));
        }

        result.push_str(") {\n");

        let body_node = self.node().get_mandatory_child_by_name("body");
        result.push_str(&visit_standalone_named_children(&body_node, context, shape));
        result.push_str(&format!("{}}}", shape.indent.to_string(context.config)));

        result
    }
}

impl<'a, 'tree> Rewrite for FieldDeclaration<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let mut result = String::new();
        add_standalone_prefix(&mut result, shape, context);

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

        add_standalone_suffix(&mut result, shape);

        result
    }
}

impl<'a, 'tree> Rewrite for SuperClass<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let mut result = String::new();
        result.push_str(" extends ");

        let value = self
            .node()
            .get_mandatory_child_value_by_kind("type_identifier", context.source_code);
        result.push_str(&value);

        result
    }
}

impl<'a, 'tree> Rewrite for Value<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let mut result = String::new();
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
        let mut result = String::new();
        add_standalone_prefix(&mut result, shape, context);

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

        result.push_str(&format!("{} {}", type_value, declarator_values.join(", ")));

        add_standalone_suffix(&mut result, shape);
        result
    }
}

impl<'a, 'tree> Rewrite for Statement<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let mut result = String::new();
        add_standalone_prefix(&mut result, shape, context);

        match self.node().kind() {
            "expression_statement" => {
                let child = self
                    .node()
                    .named_child(0)
                    .unwrap_or_else(|| panic!("mandatory child expression node missing."));
                let exp = Expression::new(&child);
                result.push_str(&exp.rewrite(context, shape));
            }
            _ => unreachable!(),
        }
        add_standalone_suffix(&mut result, shape);

        result
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
        add_standalone_prefix(&mut result, shape, context);

        result.push_str("if ");
        let condition = self
            .node()
            .get_mandatory_child_by_kind("parenthesized_expression");
        result.push_str(&visit_node(&condition, context, shape));

        let consequence = self.node().get_mandatory_child_by_kind("block");
        result.push_str(&visit_node(
            &consequence,
            context,
            &mut shape.clone_with_stand_alone(false),
        ));

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

impl<'a, 'tree> Rewrite for Block<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let mut result = String::new();

        if shape.standalone {
            add_indent(&mut result, shape, context);
        } else {
            result.push(' ');
        }

        result.push_str("{\n");

        result.push_str(&visit_standalone_named_children(
            self.node(),
            context,
            shape,
        ));

        add_indent(&mut result, shape, context);
        result.push('}');

        result
    }
}

impl<'a, 'tree> Rewrite for Expression<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let n = self.node();
        let source_code = context.source_code;
        match n.kind() {
            "binary_expression" => {
                let left = n.get_mandatory_child_value_by_name("left", source_code);
                let op = n.get_mandatory_child_value_by_name("operator", source_code);
                let right = n.get_mandatory_child_value_by_name("right", source_code);
                let result = format!("{} {} {}", left, op, right);
                result
            }
            "int" => n.get_value(source_code).to_string(),
            "method_invocation" => {
                let mut result = String::new();

                let object = &n
                    .get_child_value_by_name("object", source_code)
                    .map(|v| format!("{}.", v))
                    .unwrap_or("".to_string());
                result.push_str(object);

                let name = n.get_mandatory_child_value_by_name("name", source_code);
                result.push_str(name);
                result.push('(');

                let arguments = n.get_mandatory_child_by_name("arguments");
                let mut cursor = arguments.walk();
                let arguments_doc = arguments
                    .named_children(&mut cursor)
                    .map(|n| visit_node(&n, context, shape))
                    .collect::<Vec<String>>()
                    .join(", ");

                result.push_str(&arguments_doc);
                result.push(')');

                result
            }

            v => {
                eprintln!("### Unknow Expression node: {}", v);
                unreachable!();
            }
        }
    }
}
