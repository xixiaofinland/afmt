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
    true; Value => "boolean" | "identifier" | "operator" | "type_identifier",
    true; ValueSpace => "N/A",
    true; SpaceValueSpace => "assignment_operator",
    true; SuperClass => "superclass",
    true; Expression => "binary_expression" | "int" | "method_invocation" | "unary_expression" |
        "object_creation_expression" | "array_creation_expression" | "string_literal" | "map_creation_expression",
    true; LocalVariableDeclaration => "local_variable_declaration",
    true; VariableDeclarator => "variable_declarator",
    true; IfStatement => "if_statement",
    true; ParenthesizedExpression => "parenthesized_expression",
    true; Interfaces => "interfaces",
    true; LineComment => "line_comment",
    true; ReturnStatement => "return_statement",
    true; ArgumentList => "argument_list",
    true; TypeArguments => "type_arguments",
    true; GenericType => "generic_type",
    true; ArrayInitializer => "array_initializer",
    true; DimensionsExpr => "dimensions_expr",
    true; ArrayType => "array_type",
    true; MapInitializer => "map_initializer"
);

impl<'a, 'tree> Rewrite for ClassDeclaration<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let node = self.node();
        let mut result = String::new();
        add_standalone_prefix(&mut result, shape, context);

        let modifiers_value = node
            .try_get_child_by_kind("modifiers")
            .and_then(|n| Some(n.get_children_value_by_kind("modifier", context.source_code)))
            .unwrap_or_else(Vec::new)
            .join(" ");
        result.push_str(&modifiers_value);
        result.push_str(" class ");

        let name_node_value = node.get_child_value_by_name("name", context.source_code);
        result.push_str(name_node_value);

        if let Some(c) = node.try_get_child_by_name("superclass") {
            let n = SuperClass::new(&c);
            result.push_str(&n.rewrite(context, &mut shape.clone_with_stand_alone(false)));
        }

        if let Some(c) = node.try_get_child_by_name("interfaces") {
            let n = Interfaces::new(&c);
            result.push_str(&n.rewrite(context, &mut shape.clone_with_stand_alone(false)));
        }

        result.push_str(" {\n");

        let body_node = node.get_child_by_name("body");
        result.push_str(&visit_standalone_named_children(&body_node, context, shape));
        result.push_str(&format!("{}}}", shape.indent.to_string(context.config)));

        result
    }
}

impl<'a, 'tree> Rewrite for MethodDeclaration<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let node = self.node();
        let source_code = context.source_code;
        let config = context.config;
        let mut result = String::new();
        add_standalone_prefix(&mut result, shape, context);

        let modifiers_value = node
            .try_get_child_by_kind("modifiers")
            .and_then(|n| Some(n.get_children_value_by_kind("modifier", source_code)))
            .unwrap_or_else(Vec::new)
            .join(" ");

        result.push_str(&modifiers_value);
        result.push(' ');

        let type_node_value = node.get_child_value_by_name("type", source_code);
        result.push_str(type_node_value);
        result.push(' ');

        let name_node_value = node.get_child_value_by_name("name", source_code);
        result.push_str(name_node_value);

        result.push('(');

        let parameters_node = node
            .child_by_field_name("parameters")
            .and_then(|n| Some(n.try_get_children_by_kind("formal_parameter")))
            .unwrap_or_else(Vec::new);

        let parameters_value = parameters_node
            .iter()
            .map(|n| {
                let type_str = n.get_child_value_by_name("type", source_code);
                let name_str = n.get_child_value_by_name("name", source_code);
                format!("{} {}", type_str, name_str)
            })
            .collect::<Vec<String>>();

        let params_single_line = parameters_value.join(", ");

        shape.offset = result.len() + 3; // add trailing `) {` size

        if shape.offset + params_single_line.len() <= shape.width {
            result.push_str(&params_single_line);
        } else {
            let param_shape = shape.copy_with_indent_block_plus(config);
            result.push('\n');
            for (i, param) in parameters_value.iter().enumerate() {
                result.push_str(&param_shape.indent.to_string(config));
                result.push_str(param);

                if i < parameters_value.len() - 1 {
                    result.push(',');
                }
                result.push('\n');
            }
            result.push_str(&shape.indent.to_string(config));
        }

        result.push_str(") {\n");

        let body_node = self.node().get_child_by_name("body");
        result.push_str(&visit_standalone_named_children(&body_node, context, shape));
        result.push_str(&format!("{}}}", shape.indent.to_string(config)));

        result
    }
}

impl<'a, 'tree> Rewrite for FieldDeclaration<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let node = self.node();
        let source_code = context.source_code;
        let mut result = String::new();
        add_standalone_prefix(&mut result, shape, context);

        let modifiers_value = node
            .try_get_child_by_kind("modifiers")
            .and_then(|n| Some(n.get_children_value_by_kind("modifier", source_code)))
            .unwrap_or_else(Vec::new)
            .join(" ");

        result.push_str(&modifiers_value);

        result.push(' ');

        let type_node_value = node.get_child_value_by_name("type", source_code);
        result.push_str(type_node_value);

        result.push(' ');

        let name_node_value = node
            .get_child_by_name("declarator")
            .get_child_value_by_name("name", source_code);
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
            .get_child_value_by_kind("type_identifier", context.source_code);
        result.push_str(&value);

        result
    }
}

impl<'a, 'tree> Rewrite for Interfaces<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let node = self.node();
        let mut result = String::new();
        result.push_str(" implements ");

        let type_list = node.get_child_by_kind("type_list");

        let type_lists =
            type_list.get_children_value_by_kind("type_identifier", context.source_code);
        result.push_str(&type_lists.join(", "));

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

impl<'a, 'tree> Rewrite for LocalVariableDeclaration<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let mut result = String::new();
        add_standalone_prefix(&mut result, shape, context);

        let t = self.node().get_child_by_name("type");
        result.push_str(&visit_node(
            &t,
            context,
            &mut shape.clone_with_stand_alone(false),
        ));

        result.push(' ');

        let declarator_nodes = self.node().get_mandatory_children_by_name("declarator");
        let declarator_values: Vec<String> = declarator_nodes
            .iter()
            .map(|d| {
                let n = VariableDeclarator::new(&d);
                n.rewrite(context, shape)
            })
            .collect();

        result.push_str(&declarator_values.join(", "));

        add_standalone_suffix(&mut result, shape);
        result
    }
}

impl<'a, 'tree> Rewrite for Statement<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let node = self.node();
        let mut result = String::new();
        add_standalone_prefix(&mut result, shape, context);

        match node.kind() {
            "expression_statement" => {
                let child = node
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
        let node = self.node();
        let source_code = context.source_code;
        let mut result = String::new();

        let name = node.get_child_value_by_name("name", source_code);
        result.push_str(name);

        if let Some(v) = node.try_get_child_by_name("value") {
            result.push_str(" = ");
            result.push_str(&visit_node(
                &v,
                context,
                &mut shape.clone_with_stand_alone(false),
            ));
        }

        result
    }
}

impl<'a, 'tree> Rewrite for IfStatement<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let mut result = String::new();
        add_standalone_prefix(&mut result, shape, context);

        result.push_str("if ");
        let condition = self.node().get_child_by_kind("parenthesized_expression");
        result.push_str(&visit_node(&condition, context, shape));

        let consequence = self.node().get_child_by_kind("block");
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
        format!(
            "({})",
            &visit_named_children_in_same_line(self.node(), context, shape)
        )
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
        let node = self.node();
        let source_code = context.source_code;
        let mut result = String::new();

        match node.kind() {
            "unary_expression" => {
                let operator_value = node.get_child_value_by_name("operator", source_code);
                result.push_str(operator_value);

                let operand = node.get_child_by_name("operand");
                result.push_str(&visit_node(
                    &operand,
                    context,
                    &mut shape.clone_with_stand_alone(false),
                ));
                result
            }
            "binary_expression" => {
                let left = node.get_child_value_by_name("left", source_code);
                let op = node.get_child_value_by_name("operator", source_code);
                let right = node.get_child_value_by_name("right", source_code);
                result = format!("{} {} {}", left, op, right);
                result
            }
            "int" => node.get_value(source_code).to_string(),
            "method_invocation" => {
                let object = &node
                    .try_get_child_value_by_name("object", source_code)
                    .map(|v| format!("{}.", v))
                    .unwrap_or("".to_string());
                result.push_str(object);

                let name = node.get_child_value_by_name("name", source_code);
                result.push_str(name);
                result.push('(');

                let arguments = node.get_child_by_name("arguments");
                let mut cursor = arguments.walk();
                let arguments_value = arguments
                    .named_children(&mut cursor)
                    .map(|n| visit_node(&n, context, shape))
                    .collect::<Vec<String>>()
                    .join(", ");

                result.push_str(&arguments_value);
                result.push(')');

                result
            }
            "object_creation_expression" => {
                result.push_str("new ");
                let t = node.get_child_by_name("type");
                result.push_str(&visit_node(
                    &t,
                    context,
                    &mut shape.clone_with_stand_alone(false),
                ));

                let arguments = node.get_child_by_name("arguments");
                result.push_str(&visit_node(
                    &arguments,
                    context,
                    &mut shape.clone_with_stand_alone(false),
                ));
                result
            }
            "array_creation_expression" => {
                result.push_str("new ");
                let t = self.node().get_child_by_name("type");
                result.push_str(&visit_node(
                    &t,
                    context,
                    &mut shape.clone_with_stand_alone(false),
                ));

                if let Some(v) = node.try_get_child_by_name("value") {
                    result.push_str(&visit_node(
                        &v,
                        context,
                        &mut shape.clone_with_stand_alone(false),
                    ));
                }

                if let Some(v) = node.try_get_child_by_name("dimensions") {
                    result.push_str(&visit_node(
                        &v,
                        context,
                        &mut shape.clone_with_stand_alone(false),
                    ));
                }
                result
            }
            "map_creation_expression" => {
                result.push_str("new ");

                let t = node.get_child_by_name("type");
                result.push_str(&visit_node(
                    &t,
                    context,
                    &mut shape.clone_with_stand_alone(false),
                ));

                let value = node.get_child_by_name("value");
                let n = MapInitializer::new(&value);
                result.push_str(&n.rewrite(context, shape));

                result
            }
            "string_literal" => {
                result.push_str(node.get_value(source_code));
                result
            }

            v => {
                eprintln!("### Unknow Expression node: {}", v);
                unreachable!();
            }
        }
    }
}

impl<'a, 'tree> Rewrite for LineComment<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let mut result = String::new();
        add_standalone_prefix(&mut result, shape, context);

        result.push_str(self.node().get_value(context.source_code));

        result
    }
}

impl<'a, 'tree> Rewrite for ReturnStatement<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let node = self.node();
        let mut result = String::new();
        add_standalone_prefix(&mut result, shape, context);

        result.push_str("return");
        if node.named_child_count() != 0 {
            let child = node.named_child(0).unwrap();
            result.push(' ');
            result.push_str(&visit_node(&child, context, shape));
        }

        add_standalone_suffix(&mut result, shape);

        result
    }
}

impl<'a, 'tree> Rewrite for GenericType<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let node = self.node();
        let source_code = context.source_code;
        let mut result = String::new();

        let name = node.get_child_by_kind("type_identifier");
        result.push_str(name.get_value(source_code));

        let arguments = node.get_child_by_kind("type_arguments");
        let n = TypeArguments::new(&arguments);
        result.push_str(&n.rewrite(context, shape));

        result
    }
}

impl<'a, 'tree> Rewrite for ArgumentList<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let node = self.node();
        let mut result = String::new();
        result.push('(');
        let mut cursor = node.walk();
        let arguments_value = node
            .named_children(&mut cursor)
            .map(|n| visit_node(&n, context, shape))
            .collect::<Vec<String>>()
            .join(", ");

        result.push_str(&arguments_value);
        result.push(')');
        result
    }
}

impl<'a, 'tree> Rewrite for TypeArguments<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let node = self.node();
        let mut result = String::new();
        result.push('<');
        let mut cursor = node.walk();
        let arguments_value = node
            .named_children(&mut cursor)
            .map(|n| visit_node(&n, context, shape))
            .collect::<Vec<String>>()
            .join(", ");

        result.push_str(&arguments_value);
        result.push('>');
        result
    }
}

impl<'a, 'tree> Rewrite for ArrayInitializer<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let node = self.node();

        let mut cursor = node.walk();
        let mut joined_children = node
            .named_children(&mut cursor)
            .map(|n| visit_node(&n, context, shape))
            .collect::<Vec<String>>()
            .join(", ");

        if joined_children.is_empty() {
            "{}".to_string()
        } else {
            format!("{{ {} }}", joined_children)
        }
    }
}

impl<'a, 'tree> Rewrite for DimensionsExpr<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let child = self
            .node()
            .named_child(0)
            .unwrap_or_else(|| panic!("mandatory child expression node missing."));
        let exp = Expression::new(&child);
        format!("[{}]", &exp.rewrite(context, shape))
    }
}

impl<'a, 'tree> Rewrite for ArrayType<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let mut result = String::new();

        let element_value = self
            .node()
            .get_child_value_by_name("element", context.source_code);
        result.push_str(element_value);
        let element_value = self
            .node()
            .get_child_value_by_name("dimensions", context.source_code);
        result.push_str(element_value);
        result
    }
}

impl<'a, 'tree> Rewrite for MapInitializer<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let node = self.node();
        let mut result = String::new();

        let mut cursor = node.walk();
        let children = node
            .named_children(&mut cursor)
            .map(|c| {
                let n = Expression::new(&c);
                n.rewrite(context, shape)
            })
            .collect::<Vec<String>>();

        let children_value = if children.is_empty() {
            "{}".to_string()
        } else {
            // Example: ["'hello'", "1", "'world'", "2"] becomes 'hello' => 1, 'world' => 2
            let joined_children = children
                .chunks(2)
                .enumerate()
                .map(|(_, chunk)| {
                    if chunk.len() == 2 {
                        format!("{} => {}", chunk[0], chunk[1])
                    } else {
                        chunk[0].to_string()
                    }
                })
                .collect::<Vec<String>>()
                .join(", ");

            format!("{{ {} }}", joined_children)
        };

        result.push_str(&children_value);
        result
    }
}
