use crate::context::FmtContext;
use crate::node_ext::*;
use crate::shape::Shape;
use crate::utility::*;
use crate::visitor::{visit_children_in_same_line, visit_node, visit_standalone_children};
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
        "object_creation_expression" | "array_creation_expression" | "string_literal" | "map_creation_expression" |
        "assignment_expression",
    true; AssignmentExpression => "assignment_expression",
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
    true; MapInitializer => "map_initializer",
    true; Annotation => "annotation",
    true; AnnotationArgumentList => "annotation_argument_list",
    true; AnnotationKeyValue => "annotation_key_value",
    true; Modifiers => "modifiers",
    true; ConstructorDeclaration => "constructor_declaration",
    true; ConstructorBody => "constructor_body",
    true; ExplicitConstructorInvocation => "explicit_constructor_invocation"
);

impl<'a, 'tree> Rewrite for ClassDeclaration<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let node = self.node();
        let mut result = String::new();
        try_add_standalone_prefix(&mut result, shape, context);

        if let Some(ref a) = node.try_c_by_k("modifiers") {
            result.push_str(&Modifiers::new(a).rewrite(context, shape));
        }

        result.push_str(" class ");

        let name_node_value = node.cv_by_n("name", context.source_code);
        result.push_str(name_node_value);

        if let Some(c) = node.try_c_by_n("superclass") {
            let n = SuperClass::new(&c);
            result.push_str(&n.rewrite(context, &mut shape.clone_with_stand_alone(false)));
        }

        if let Some(c) = node.try_c_by_n("interfaces") {
            let n = Interfaces::new(&c);
            result.push_str(&n.rewrite(context, &mut shape.clone_with_stand_alone(false)));
        }

        result.push_str(" {\n");

        let body_node = node.c_by_n("body");
        result.push_str(&visit_standalone_children(&body_node, context, shape));
        result.push_str(&format!("{}}}", shape.indent.as_string(context.config)));

        result
    }
}

impl<'a, 'tree> Rewrite for MethodDeclaration<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let node = self.node();
        let source_code = context.source_code;
        let config = context.config;
        let mut result = String::new();
        try_add_standalone_prefix(&mut result, shape, context);

        if let Some(ref a) = node.try_c_by_k("modifiers") {
            result.push_str(&Modifiers::new(a).rewrite(context, shape));
        }

        result.push(' ');

        let type_node_value = node.cv_by_n("type", source_code);
        result.push_str(type_node_value);
        result.push(' ');

        let name_node_value = node.cv_by_n("name", source_code);
        result.push_str(name_node_value);

        result.push('(');

        let parameters_node = node
            .try_c_by_n("parameters")
            .map(|n| n.try_cs_by_k("formal_parameter"))
            .unwrap_or_default();

        let parameters_value: Vec<String> = parameters_node
            .iter()
            .map(|n| {
                let type_str = n.cv_by_n("type", source_code);
                let name_str = n.cv_by_n("name", source_code);
                format!("{} {}", type_str, name_str)
            })
            .collect();

        let params_single_line = parameters_value.join(", ");

        shape.offset = result.len() + 3; // add trailing `) {` size

        if shape.offset + params_single_line.len() <= shape.width {
            result.push_str(&params_single_line);
        } else {
            let param_shape = shape.copy_with_indent_block_plus(config);
            result.push('\n');
            for (i, param) in parameters_value.iter().enumerate() {
                result.push_str(&param_shape.indent.as_string(config));
                result.push_str(param);

                if i < parameters_value.len() - 1 {
                    result.push(',');
                }
                result.push('\n');
            }
            result.push_str(&shape.indent.as_string(config));
        }

        result.push_str(") {\n");

        let body_node = self.node().c_by_n("body");
        result.push_str(&visit_standalone_children(&body_node, context, shape));
        result.push_str(&format!("{}}}", shape.indent.as_string(config)));

        result
    }
}

impl<'a, 'tree> Rewrite for FieldDeclaration<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let node = self.node();
        let source_code = context.source_code;
        let mut result = String::new();
        try_add_standalone_prefix(&mut result, shape, context);

        if let Some(ref a) = node.try_c_by_k("modifiers") {
            result.push_str(&Modifiers::new(a).rewrite(context, shape));
            result.push(' ');
        }

        let type_node_value = node.cv_by_n("type", source_code);
        result.push_str(type_node_value);

        result.push(' ');

        let variable_declarator = node.c_by_k("variable_declarator");
        let n = VariableDeclarator::new(&variable_declarator);
        result.push_str(&n.rewrite(context, shape));

        try_add_standalone_suffix(&mut result, shape);

        result
    }
}

impl<'a, 'tree> Rewrite for SuperClass<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let mut result = String::new();
        result.push_str(" extends ");

        let value = self.node().cv_by_k("type_identifier", context.source_code);
        result.push_str(value);

        result
    }
}

impl<'a, 'tree> Rewrite for Interfaces<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let node = self.node();
        let mut result = String::new();
        result.push_str(" implements ");

        let type_list = node.c_by_k("type_list");

        let type_lists = type_list.try_csv_by_k("type_identifier", context.source_code);
        result.push_str(&type_lists.join(", "));

        result
    }
}

impl<'a, 'tree> Rewrite for Value<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let mut result = String::new();
        let name_node_value = self.node().v(context.source_code);
        result.push_str(name_node_value);
        result
    }
}

impl<'a, 'tree> Rewrite for SpaceValueSpace<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let mut result = String::from(' ');
        let name_node_value = self.node().v(context.source_code);
        result.push_str(name_node_value);
        result.push(' ');
        result
    }
}

impl<'a, 'tree> Rewrite for ValueSpace<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let mut result = String::new();
        let name_node_value = self.node().v(context.source_code);
        result.push_str(name_node_value);
        result.push(' ');
        result
    }
}

impl<'a, 'tree> Rewrite for LocalVariableDeclaration<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let mut result = String::new();
        let node = self.node();

        if let Some(ref a) = node.try_c_by_k("modifiers") {
            result.push_str(&Modifiers::new(a).rewrite(context, shape));
            result.push(' ');
        } else {
            try_add_standalone_prefix(&mut result, shape, context);
        }

        let t = node.c_by_n("type");
        result.push_str(&visit_node(
            &t,
            context,
            &mut shape.clone_with_stand_alone(false),
        ));

        result.push(' ');

        let declarator_nodes = node.cs_by_n("declarator");
        let declarator_values: Vec<String> = declarator_nodes
            .iter()
            .map(|d| {
                let n = VariableDeclarator::new(d);
                n.rewrite(context, shape)
            })
            .collect();

        result.push_str(&declarator_values.join(", "));

        try_add_standalone_suffix(&mut result, shape);
        result
    }
}

impl<'a, 'tree> Rewrite for Statement<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let node = self.node();
        let mut result = String::new();
        try_add_standalone_prefix(&mut result, shape, context);

        match node.kind() {
            "expression_statement" => {
                let child = node
                    .named_child(0)
                    .unwrap_or_else(|| panic!("mandatory child expression node missing."));
                let exp = Expression::new(&child);
                result.push_str(&exp.rewrite(context, shape));
            }
            "block" => {
                let n = Block::new(node);
                result.push_str(&n.rewrite(context, shape));
            }
            _ => unreachable!(),
        }
        try_add_standalone_suffix(&mut result, shape);

        result
    }
}

impl<'a, 'tree> Rewrite for VariableDeclarator<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let node = self.node();
        let source_code = context.source_code;
        let mut result = String::new();

        let name = node.cv_by_n("name", source_code);
        result.push_str(name);

        if let Some(v) = node.try_c_by_n("value") {
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
        let node = self.node();
        let mut result = String::new();
        try_add_standalone_prefix(&mut result, shape, context);

        result.push_str("if ");
        let condition = node.c_by_k("parenthesized_expression");
        let n = ParenthesizedExpression::new(&condition);
        result.push_str(&n.rewrite(context, shape));

        let consequence = node.c_by_n("consequence");
        let has_block_node = consequence.kind() == "block";

        if has_block_node {
            let n = Block::new(&consequence);
            result.push_str(&n.rewrite(context, &mut shape.clone_with_stand_alone(false)));
        } else {
            result.push_str(" {\n");
            let mut child_shape = shape.copy_with_indent_block_plus(context.config);
            result.push_str(&visit_node(&consequence, context, &mut child_shape));
            result.push_str(&format!("\n{}}}", shape.indent.as_string(context.config)));
        };

        node.try_c_by_n("alternative").map(|a| {
            result.push_str(" else");

            let has_block_node = a.kind() == "block";
            if has_block_node {
                let n = Block::new(&a);
                result.push_str(&n.rewrite(context, &mut shape.clone_with_stand_alone(false)));
            } else {
                result.push_str(" {\n");
                let mut child_shape = shape.copy_with_indent_block_plus(context.config);
                result.push_str(&visit_node(&a, context, &mut child_shape));
                result.push_str(&format!("\n{}}}", shape.indent.as_string(context.config)));
            };
        });

        result
    }
}

impl<'a, 'tree> Rewrite for ParenthesizedExpression<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        format!(
            "({})",
            &visit_children_in_same_line(self.node(), context, shape)
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

        result.push_str(&visit_standalone_children(self.node(), context, shape));

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
                let operator_value = node.cv_by_n("operator", source_code);
                result.push_str(operator_value);

                let operand = node.c_by_n("operand");
                result.push_str(&visit_node(
                    &operand,
                    context,
                    &mut shape.clone_with_stand_alone(false),
                ));
                result
            }
            "binary_expression" => {
                let left = node.cv_by_n("left", source_code);
                let op = node.cv_by_n("operator", source_code);
                let right = node.cv_by_n("right", source_code);
                result = format!("{} {} {}", left, op, right);
                result
            }
            "int" => node.v(source_code).to_string(),
            "method_invocation" => {
                let object = &node
                    .try_cv_by_n("object", source_code)
                    .map(|v| format!("{}.", v))
                    .unwrap_or_default();
                result.push_str(object);

                let name = node.cv_by_n("name", source_code);
                result.push_str(name);
                result.push('(');

                let arguments = node.c_by_n("arguments");
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
                let t = node.c_by_n("type");
                result.push_str(&visit_node(
                    &t,
                    context,
                    &mut shape.clone_with_stand_alone(false),
                ));

                let arguments = node.c_by_n("arguments");
                result.push_str(&visit_node(
                    &arguments,
                    context,
                    &mut shape.clone_with_stand_alone(false),
                ));
                result
            }
            "array_creation_expression" => {
                result.push_str("new ");
                let t = self.node().c_by_n("type");
                result.push_str(&visit_node(
                    &t,
                    context,
                    &mut shape.clone_with_stand_alone(false),
                ));

                if let Some(v) = node.try_c_by_n("value") {
                    result.push_str(&visit_node(
                        &v,
                        context,
                        &mut shape.clone_with_stand_alone(false),
                    ));
                }

                if let Some(v) = node.try_c_by_n("dimensions") {
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

                let t = node.c_by_n("type");
                result.push_str(&visit_node(
                    &t,
                    context,
                    &mut shape.clone_with_stand_alone(false),
                ));

                let value = node.c_by_n("value");
                let n = MapInitializer::new(&value);
                result.push_str(&n.rewrite(context, shape));

                result
            }
            "string_literal" => {
                result.push_str(node.v(source_code));
                result
            }
            "assignment_expression" => {
                let n = AssignmentExpression::new(node);
                result.push_str(&n.rewrite(context, shape));
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
        try_add_standalone_prefix(&mut result, shape, context);

        result.push_str(self.node().v(context.source_code));

        result
    }
}

impl<'a, 'tree> Rewrite for ReturnStatement<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let node = self.node();
        let mut result = String::new();
        try_add_standalone_prefix(&mut result, shape, context);

        result.push_str("return");
        if node.named_child_count() != 0 {
            let child = node.named_child(0).unwrap();
            result.push(' ');
            result.push_str(&visit_node(&child, context, shape));
        }

        try_add_standalone_suffix(&mut result, shape);

        result
    }
}

impl<'a, 'tree> Rewrite for GenericType<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let node = self.node();
        let source_code = context.source_code;
        let mut result = String::new();

        let name = node.c_by_k("type_identifier");
        result.push_str(name.v(source_code));

        let arguments = node.c_by_k("type_arguments");
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

        let joined = node.try_visit_cs(context, shape).join(", ");

        result.push_str(&joined);
        result.push(')');
        result
    }
}

impl<'a, 'tree> Rewrite for TypeArguments<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let node = self.node();
        let mut result = String::new();
        result.push('<');

        let joined = node.try_visit_cs(context, shape).join(", ");
        result.push_str(&joined);

        result.push('>');
        result
    }
}

impl<'a, 'tree> Rewrite for ArrayInitializer<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let node = self.node();

        let joined = node.try_visit_cs(context, shape).join(", ");
        if joined.is_empty() {
            "{}".to_string()
        } else {
            format!("{{ {} }}", joined)
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

        let element_value = self.node().cv_by_n("element", context.source_code);
        result.push_str(element_value);
        let element_value = self.node().cv_by_n("dimensions", context.source_code);
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
                .map(|chunk| {
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

impl<'a, 'tree> Rewrite for Annotation<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let node = self.node();
        let mut result = String::new();

        try_add_standalone_prefix(&mut result, shape, context);
        result.push('@');

        let name = node.c_by_n("name");
        result.push_str(&visit_node(&name, context, shape));

        if let Some(a) = node.try_c_by_n("arguments") {
            result.push('(');
            result.push_str(&visit_node(&a, context, shape));
            result.push(')');
        }

        result.push('\n');
        add_indent(&mut result, shape, context);
        result
    }
}

impl<'a, 'tree> Rewrite for AnnotationArgumentList<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let node = self.node();
        let mut result = String::new();

        if let Some(c) = node.try_c_by_n("value") {
            result.push_str(c.v(context.source_code));
        }

        let joined_children = node
            .try_cs_by_k("annotation_key_value")
            .iter()
            .map(|c| AnnotationKeyValue::new(c).rewrite(context, shape))
            .collect::<Vec<_>>()
            .join(" ");

        result.push_str(&joined_children);

        if let Some(ref a) = node
            .try_c_by_k("modifiers")
            .and_then(|n| n.try_c_by_k("annotation"))
        {
            result.push_str(&Annotation::new(a).rewrite(context, shape));
        }

        result
    }
}

impl<'a, 'tree> Rewrite for AnnotationKeyValue<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let node = self.node();
        let mut result = String::new();

        let key = node.c_by_n("key");
        result.push_str(key.v(context.source_code));

        result.push('=');

        let value = node.c_by_n("value");
        result.push_str(&visit_node(&value, context, shape));

        result
    }
}

impl<'a, 'tree> Rewrite for Modifiers<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let node = self.node();
        let mut result = String::new();

        node.try_cs_by_k("annotation").iter().for_each(|c| {
            result.push_str(
                &Annotation::new(c).rewrite(context, &mut shape.clone_with_stand_alone(true)),
            );
        });

        result.push_str(&node.try_csv_by_k("modifier", context.source_code).join(" "));

        result
    }
}

impl<'a, 'tree> Rewrite for ConstructorDeclaration<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let node = self.node();
        let source_code = context.source_code;
        let mut result = String::new();

        try_add_standalone_prefix(&mut result, shape, context);

        if let Some(ref c) = node.try_c_by_k("modifiers") {
            let n = Modifiers::new(c);
            result.push_str(&n.rewrite(context, shape));
        }

        result.push(' ');
        result.push_str(node.c_by_n("name").v(source_code));

        result.push('(');
        let parameters_node = node
            .try_c_by_n("parameters")
            .map(|n| n.try_cs_by_k("formal_parameter"))
            .unwrap_or_default();

        let parameters_value: Vec<String> = parameters_node
            .iter()
            .map(|n| {
                let type_str = n.cv_by_n("type", source_code);
                let name_str = n.cv_by_n("name", source_code);
                format!("{} {}", type_str, name_str)
            })
            .collect();
        let params_single_line = parameters_value.join(", ");
        result.push_str(&params_single_line);
        result.push(')');

        let constructor_body = node.c_by_n("body");
        let n = ConstructorBody::new(&constructor_body);
        result.push_str(&n.rewrite(context, shape));

        result
    }
}

impl<'a, 'tree> Rewrite for ConstructorBody<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let node = self.node();
        let mut result = String::new();

        result.push_str(" {\n");
        result.push_str(&visit_standalone_children(node, context, shape));
        result.push_str(&format!("{}}}", shape.indent.as_string(context.config)));
        result
    }
}

impl<'a, 'tree> Rewrite for ExplicitConstructorInvocation<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let node = self.node();
        let source_code = context.source_code;
        let mut result = String::new();
        try_add_standalone_prefix(&mut result, shape, context);

        let constructor = node.c_by_n("constructor");
        result.push_str(constructor.v(source_code));

        let arguments = node.c_by_n("arguments");
        let n = ArgumentList::new(&arguments);
        result.push_str(&n.rewrite(context, shape));
        try_add_standalone_suffix(&mut result, shape);

        result
    }
}

impl<'a, 'tree> Rewrite for AssignmentExpression<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let node = self.node();
        let source_code = context.source_code;
        let mut result = String::new();

        let left = node.cv_by_n("left", source_code);
        let op = node.cv_by_n("operator", source_code);
        let right = node.cv_by_n("right", source_code);
        result.push_str(&format!("{} {} {}", left, op, right));
        result
    }
}
