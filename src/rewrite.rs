use crate::child::Accessor;
use crate::context::FmtContext;
use crate::match_routing;
use crate::shape::Shape;
use crate::struct_def::*;
use crate::utility::*;
use crate::visit::Visitor;
use colored::Colorize;
#[allow(unused_imports)]
use log::debug;

pub trait Rewrite {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String;
}

impl<'a, 'tree> Rewrite for ClassDeclaration<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);
        try_add_standalone_prefix(&mut result, shape, context);

        if let Some(ref a) = node.try_c_by_k("modifiers") {
            result.push_str(&Modifiers::new(a).rewrite(context, shape));
        }

        result.push_str(" class ");

        result.push_str(node.cv_by_n("name", source_code));

        if let Some(c) = node.try_c_by_n("superclass") {
            let n = SuperClass::new(&c);
            result.push_str(&n.rewrite(context, &mut shape.clone_with_standalone(false)));
        }

        if let Some(c) = node.try_c_by_n("interfaces") {
            let n = Interfaces::new(&c);
            result.push_str(&n.rewrite(context, &mut shape.clone_with_standalone(false)));
        }

        result.push_str(" {\n");

        let body_node = node.c_by_n("body");
        result.push_str(&body_node.visit_standalone_children(context, shape));
        result.push_str(&format!("{}}}", shape.indent.as_string(context.config)));

        result
    }
}

impl<'a, 'tree> Rewrite for MethodDeclaration<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let (node, mut result, source_code, config) = self.prepare(context);
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
            let param_shape = shape.copy_with_indent_increase(config);
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

        let body_node = node.c_by_n("body");
        result.push_str(&body_node.visit_standalone_children(context, shape));
        result.push_str(&format!("{}}}", shape.indent.as_string(config)));

        result
    }
}

impl<'a, 'tree> Rewrite for EnumDeclaration<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);
        try_add_standalone_prefix(&mut result, shape, context);

        if let Some(ref a) = node.try_c_by_k("modifiers") {
            result.push_str(&Modifiers::new(a).rewrite(context, shape));
        }

        result.push_str(" enum ");
        result.push_str(node.cv_by_n("name", source_code));

        let body = node.c_by_n("body");
        let n = EnumBody::new(&body);
        result.push_str(&n.rewrite(context, &mut shape.clone_with_standalone(false)));

        result
    }
}

impl<'a, 'tree> Rewrite for EnumConstant<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);
        try_add_standalone_prefix(&mut result, shape, context);
        result.push_str(node.v(source_code));
        try_add_standalone_suffix(node, &mut result, shape, source_code);

        result
    }
}

impl<'a, 'tree> Rewrite for EnumBody<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);

        if shape.standalone {
            add_indent(&mut result, shape, context);
        } else {
            result.push(' ');
        }

        result.push_str("{\n");

        add_indent(
            &mut result,
            &shape.copy_with_indent_increase(context.config),
            context,
        );
        result.push_str(&node.try_csv_by_k("enum_constant", source_code).join(", "));

        result.push('\n');
        add_indent(&mut result, shape, context);
        result.push('}');

        result
    }
}

impl<'a, 'tree> Rewrite for FieldDeclaration<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);
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

        if let Some(a) = node.try_c_by_k("accessor_list") {
            let n = AccessorList::new(&a);
            result.push_str(&n.rewrite(context, shape));

            // special case: it has no `;` ending with "accessor_list"
            try_add_standalone_suffix_no_semicolumn(node, &mut result, shape, context.source_code);
        } else {
            try_add_standalone_suffix(node, &mut result, shape, context.source_code);
        }

        result
    }
}

impl<'a, 'tree> Rewrite for SuperClass<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, _shape: &mut Shape) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);
        result.push_str(" extends ");

        let value = node.cv_by_k("type_identifier", source_code);
        result.push_str(value);

        result
    }
}

impl<'a, 'tree> Rewrite for Interfaces<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, _shape: &mut Shape) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);
        result.push_str(" implements ");

        let type_list = node.c_by_k("type_list");

        let type_lists = type_list.try_csv_by_k("type_identifier", source_code);
        result.push_str(&type_lists.join(", "));

        result
    }
}

impl<'a, 'tree> Rewrite for Value<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);
        try_add_standalone_prefix(&mut result, shape, context);
        result.push_str(node.v(source_code));
        try_add_standalone_suffix(node, &mut result, shape, context.source_code);
        result
    }
}

impl<'a, 'tree> Rewrite for LocalVariableDeclaration<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);

        if let Some(ref a) = node.try_c_by_k("modifiers") {
            result.push_str(&Modifiers::new(a).rewrite(context, shape));
            result.push(' ');
        } else {
            try_add_standalone_prefix(&mut result, shape, context);
        }

        let t = node.c_by_n("type");
        result.push_str(&t.visit(context, &mut shape.clone_with_standalone(false)));

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

        try_add_standalone_suffix(node, &mut result, shape, source_code);
        result
    }
}

impl<'a, 'tree> Rewrite for Statement<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);

        match_routing!(node, result, context, shape;
            "block" => Block,
            //"break_statement"
            //"continue_statement"
            //"declaration"
            "do_statement" => DoStatement,
            "enhanced_for_statement" => EnhancedForStatement,
            "expression_statement" => ExpressionStatement,
            "for_statement" => ForStatement,
            "if_statement" => IfStatement,
            //"labeled_statement"
            "local_variable_declaration" => LocalVariableDeclaration,
            "return_statement" => ReturnStatement,
            "run_as_statement" => RunAsStatement,
            //"switch_expression" =>
            //"throw_statement" => Thr
            "try_statement" => TryStatement,
            //"while_statement" => WhileStatement, // NOTE: it conflicts with try_add_standalone_prefix() which adds extra `;` at end
        );
        result
    }
}

impl<'a, 'tree> Rewrite for ExpressionStatement<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let (node, mut result, _, _) = self.prepare(context);
        let c = node.first_c();
        result.push_str(&Expression::new(&c).rewrite(context, shape));
        result
    }
}

impl<'a, 'tree> Rewrite for TryStatement<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let (node, mut result, _, _) = self.prepare(context);
        try_add_standalone_prefix(&mut result, shape, context);

        result.push_str("try");
        let body = node.c_by_n("body");
        let n = Block::new(&body);
        result.push_str(&n.rewrite(context, &mut shape.clone_with_standalone(false)));

        let joined_children = node
            .try_cs_by_k("catch_clause")
            .iter()
            .map(|c| CatchClause::new(c).rewrite(context, shape))
            .collect::<Vec<_>>()
            .join("");
        result.push_str(&joined_children);

        if let Some(f) = node.try_c_by_k("finally_clause") {
            let n = FinallyClause::new(&f);
            result.push_str(&n.rewrite(context, &mut shape.clone_with_standalone(false)));
        }

        result
    }
}

impl<'a, 'tree> Rewrite for FinallyClause<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let (node, mut result, _, _) = self.prepare(context);

        result.push_str(" finally");
        let block = node.c_by_k("block");
        let n = Block::new(&block);
        result.push_str(&n.rewrite(context, &mut shape.clone_with_standalone(false)));
        result
    }
}

impl<'a, 'tree> Rewrite for CatchClause<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let (node, mut result, _, _) = self.prepare(context);

        result.push_str(" catch ");

        let param = node.c_by_k("catch_formal_parameter");
        let n = CatchFormalParameter::new(&param);
        result.push_str(&n.rewrite(context, shape));

        let body = node.c_by_n("body");
        let n = Block::new(&body);
        result.push_str(&n.rewrite(context, &mut shape.clone_with_standalone(false)));
        result
    }
}

impl<'a, 'tree> Rewrite for CatchFormalParameter<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let (node, mut result, _, _) = self.prepare(context);

        result.push('(');
        result.push_str(&node.visit_children_in_same_line(" ", context, shape));
        result.push(')');
        result
    }
}

impl<'a, 'tree> Rewrite for VariableDeclarator<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);

        let name = node.cv_by_n("name", source_code);
        result.push_str(name);

        if let Some(v) = node.try_c_by_n("value") {
            result.push_str(" = ");
            let mut c_shape = shape.clone_with_standalone(false);
            if v.kind() == "array_initializer" {
                result.push_str(&ArrayInitializer::new(&v).rewrite(context, &mut c_shape));
            } else {
                result.push_str(&Expression::new(&v).rewrite(context, &mut c_shape));
            }
        }
        result
    }
}

impl<'a, 'tree> Rewrite for IfStatement<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let (node, mut result, _, _) = self.prepare(context);
        try_add_standalone_prefix(&mut result, shape, context);

        result.push_str("if ");
        let condition = node.c_by_n("condition");
        let n = ParenthesizedExpression::new(&condition);
        result.push_str(&n.rewrite(context, shape));

        let consequence = node.c_by_n("consequence");
        let is_block_node = consequence.kind() == "block";

        if is_block_node {
            let n = Block::new(&consequence);
            result.push_str(&n.rewrite(context, &mut shape.clone_with_standalone(false)));
        } else {
            result.push_str(" {\n");
            let mut child_shape = shape
                .copy_with_indent_increase(context.config)
                .clone_with_standalone(true);
            result.push_str(&consequence.visit(context, &mut child_shape));
            result.push_str(&format!("\n{}}}", shape.indent.as_string(context.config)));
        };

        if let Some(a) = node.try_c_by_n("alternative") {
            match a.kind() {
                "block" => {
                    result.push_str(" else");
                    let n = Block::new(&a);
                    result.push_str(&n.rewrite(context, &mut shape.clone_with_standalone(false)));
                }
                "if_statement" => {
                    result.push_str(" else ");
                    let n = IfStatement::new(&a);
                    result.push_str(&n.rewrite(context, &mut shape.clone_with_standalone(false)));
                }
                _ => {
                    result.push_str(" else {\n");
                    let mut child_shape = shape
                        .copy_with_indent_increase(context.config)
                        .clone_with_standalone(true);
                    result.push_str(&a.visit(context, &mut child_shape));
                    result.push_str(&format!("\n{}}}", shape.indent.as_string(context.config)));
                }
            }
        };

        result
    }
}

impl<'a, 'tree> Rewrite for ForStatement<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let (node, mut result, _, _) = self.prepare(context);
        try_add_standalone_prefix(&mut result, shape, context);

        result.push_str("for (");
        if let Some(c) = node.try_c_by_n("init") {
            let n = Expression::new(&c);
            result.push_str(&n.rewrite(context, &mut shape.clone_with_standalone(false)));
        };
        result.push(';');

        if let Some(c) = node.try_c_by_n("condition") {
            result.push(' ');
            let n = Expression::new(&c);
            result.push_str(&n.rewrite(context, &mut shape.clone_with_standalone(false)));
        };

        result.push(';');

        if let Some(c) = node.try_c_by_n("update") {
            result.push(' ');
            let n = Expression::new(&c);
            result.push_str(&n.rewrite(context, &mut shape.clone_with_standalone(false)));
        };
        result.push(')');

        let body = node.c_by_n("body");
        let is_block_node = body.kind() == "block";

        if is_block_node {
            let n = Block::new(&body);
            result.push_str(&n.rewrite(context, &mut shape.clone_with_standalone(false)));
        } else {
            result.push('\n');
            let mut child_shape = shape
                .copy_with_indent_increase(context.config)
                .clone_with_standalone(true);
            result.push_str(&body.visit(context, &mut child_shape));
            //result.push_str(&format!("\n{}}}", shape.indent.as_string(context.config)));
        };

        result
    }
}

impl<'a, 'tree> Rewrite for EnhancedForStatement<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let (node, mut result, _, _) = self.prepare(context);
        try_add_standalone_prefix(&mut result, shape, context);

        result.push_str("for (");
        let t = node.c_by_n("type");
        result.push_str(&t.visit(context, &mut shape.clone_with_standalone(false)));
        result.push(' ');

        let name = node.c_by_n("name");
        result.push_str(&name.visit(context, &mut shape.clone_with_standalone(false)));
        result.push_str(" : ");

        let value = node.c_by_n("value");
        result.push_str(&value.visit(context, &mut shape.clone_with_standalone(false)));
        result.push(')');

        let body = node.c_by_n("body");
        let is_block_node = body.kind() == "block";

        if is_block_node {
            let n = Block::new(&body);
            result.push_str(&n.rewrite(context, &mut shape.clone_with_standalone(false)));
        } else {
            result.push_str(" {\n");
            let mut child_shape = shape
                .copy_with_indent_increase(context.config)
                .clone_with_standalone(true);
            result.push_str(&body.visit(context, &mut child_shape));
            result.push_str(&format!("\n{}}}", shape.indent.as_string(context.config)));
        };

        result
    }
}

impl<'a, 'tree> Rewrite for ParenthesizedExpression<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        format!(
            "({})",
            &self
                .node()
                .visit_children_in_same_line(", ", context, shape)
        )
    }
}

impl<'a, 'tree> Rewrite for Block<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let (node, mut result, _, _) = self.prepare(context);

        if shape.standalone {
            add_indent(&mut result, shape, context);
        } else {
            result.push(' ');
        }

        result.push_str("{\n");

        result
            .push_str(&node.visit_standalone_children(context, &shape.clone_with_standalone(true)));

        add_indent(&mut result, shape, context);
        result.push('}');
        result
    }
}

impl<'a, 'tree> Rewrite for Expression<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let (node, mut result, _, _) = self.prepare(context);

        match_routing!(node, result, context, shape;
            "field_access" => FieldAccess,
            "array_creation_expression" => ArrayCreationExpression,
            "assignment_expression" => AssignmentExpression,
            "binary_expression" => BinaryExpression,
            "cast_expression" => CastExpression,
            "dml_expression" => DmlExpression,
            "instanceof_expression" => InstanceOfExpression,
            "primary_expression" => PrimaryExpression,
            "ternary_expression" => TernaryExpression,
            "unary_expression" => UnaryExpression,
            "update_expression" => UpdateExpression,
            "local_variable_declaration" => LocalVariableDeclaration,
            "map_creation_expression" => MapCreationExpression,
            "object_creation_expression" => ObjectCreationExpression,
            "method_invocation" => MethodInvocation,
            "string_literal" => Value,
            "identifier" => Value,
            "int" => Value,
            "boolean" => Value
        );
        result
    }
}

impl<'a, 'tree> Rewrite for LineComment<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);
        try_add_standalone_prefix(&mut result, shape, context);
        result.push_str(node.v(source_code));

        result
    }
}

impl<'a, 'tree> Rewrite for ReturnStatement<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);
        try_add_standalone_prefix(&mut result, shape, context);

        result.push_str("return");
        if node.named_child_count() != 0 {
            let child = node.first_c();
            result.push(' ');
            result.push_str(
                &Expression::new(&child).rewrite(context, &mut shape.clone_with_standalone(false)),
            );
        }

        try_add_standalone_suffix(node, &mut result, shape, source_code);

        result
    }
}

impl<'a, 'tree> Rewrite for GenericType<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);

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
        let (node, mut result, _, _) = self.prepare(context);

        result.push('(');
        let mut cursor = node.walk();
        let joined = node
            .named_children(&mut cursor)
            .map(|c| {
                let n = Expression::new(&c);
                n.rewrite(context, &mut shape.clone_with_standalone(false))
            })
            .collect::<Vec<_>>()
            .join(", ");

        result.push_str(&joined);
        result.push(')');
        result
    }
}

impl<'a, 'tree> Rewrite for TypeArguments<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let (node, mut result, _, _) = self.prepare(context);

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
        let child = self.node().first_c();
        let exp = Expression::new(&child);
        format!("[{}]", &exp.rewrite(context, shape))
    }
}

impl<'a, 'tree> Rewrite for ArrayType<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, _shape: &mut Shape) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);

        let element_value = node.cv_by_n("element", source_code);
        result.push_str(element_value);
        let element_value = node.cv_by_n("dimensions", source_code);
        result.push_str(element_value);
        result
    }
}

impl<'a, 'tree> Rewrite for MapInitializer<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let (node, mut result, _, _) = self.prepare(context);

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
        let (node, mut result, source_code, _) = self.prepare(context);

        try_add_standalone_prefix(&mut result, shape, context);
        result.push('@');

        let name = node.c_by_n("name");
        result.push_str(name.v(source_code));

        if let Some(a) = node.try_c_by_n("arguments") {
            result.push('(');
            result.push_str(&a.visit(context, shape));
            result.push(')');
        }

        result.push('\n');
        add_indent(&mut result, shape, context);
        result
    }
}

impl<'a, 'tree> Rewrite for AnnotationArgumentList<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);

        if let Some(c) = node.try_c_by_n("value") {
            result.push_str(c.v(source_code));
        }

        let joined_children = node
            .try_cs_by_k("annotation_key_value")
            .iter()
            .map(|c| {
                AnnotationKeyValue::new(c).rewrite(context, &mut shape.clone_with_standalone(false))
            })
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
        let (node, mut result, source_code, _) = self.prepare(context);

        let key = node.c_by_n("key");
        result.push_str(key.v(source_code));

        result.push('=');

        let value = node.c_by_n("value");
        result.push_str(&value.visit(context, shape));

        result
    }
}

impl<'a, 'tree> Rewrite for Modifiers<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);

        node.try_cs_by_k("annotation").iter().for_each(|c| {
            result.push_str(
                &Annotation::new(c).rewrite(context, &mut shape.clone_with_standalone(true)),
            );
        });

        result.push_str(&node.try_csv_by_k("modifier", source_code).join(" "));

        result
    }
}

impl<'a, 'tree> Rewrite for ConstructorDeclaration<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);

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
        let (node, mut result, _, _) = self.prepare(context);

        result.push_str(" {\n");
        result.push_str(&node.visit_standalone_children(context, shape));
        result.push_str(&format!("{}}}", shape.indent.as_string(context.config)));
        result
    }
}

impl<'a, 'tree> Rewrite for ExplicitConstructorInvocation<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);
        try_add_standalone_prefix(&mut result, shape, context);

        let constructor = node.c_by_n("constructor");
        result.push_str(constructor.v(source_code));

        let arguments = node.c_by_n("arguments");
        let n = ArgumentList::new(&arguments);
        result.push_str(&n.rewrite(context, shape));
        try_add_standalone_suffix(node, &mut result, shape, source_code);

        result
    }
}

impl<'a, 'tree> Rewrite for AssignmentExpression<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);
        try_add_standalone_prefix(&mut result, shape, context);

        let left_value = node.cv_by_n("left", source_code);
        let op = node.cv_by_n("operator", source_code);

        let right = node.c_by_n("right");
        let n = Expression::new(&right);
        let right_value = &n.rewrite(context, &mut shape.clone_with_standalone(false));

        result.push_str(&format!("{} {} {}", left_value, op, right_value));
        try_add_standalone_suffix(node, &mut result, shape, context.source_code);
        result
    }
}

impl<'a, 'tree> Rewrite for DoStatement<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let (node, mut result, _, _) = self.prepare(context);
        try_add_standalone_prefix(&mut result, shape, context);

        result.push_str("do");
        let body = node.c_by_n("body");
        let n = Block::new(&body);
        result.push_str(&n.rewrite(context, &mut shape.clone_with_standalone(false)));

        result.push_str(" while ");
        let condition = node.c_by_n("condition");
        let n = ParenthesizedExpression::new(&condition);
        result.push_str(&n.rewrite(context, &mut shape.clone_with_standalone(false)));

        try_add_standalone_suffix(node, &mut result, shape, context.source_code);
        result
    }
}

impl<'a, 'tree> Rewrite for WhileStatement<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let (node, mut result, _, _) = self.prepare(context);
        try_add_standalone_prefix(&mut result, shape, context);

        result.push_str("while ");
        let condition = node.c_by_n("condition");
        let n = ParenthesizedExpression::new(&condition);
        result.push_str(&n.rewrite(context, &mut shape.clone_with_standalone(false)));

        let body = node.c_by_n("body");
        let n = Block::new(&body);
        result.push_str(&n.rewrite(context, &mut shape.clone_with_standalone(false)));

        result
    }
}

impl<'a, 'tree> Rewrite for ArrayAccess<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let (node, mut result, _, _) = self.prepare(context);

        let array = &node.c_by_n("array");
        let n = Expression::new(array);
        result.push_str(&n.rewrite(context, shape));

        let index = &node.c_by_n("index");
        let n = Expression::new(index);
        result.push('[');
        result.push_str(&n.rewrite(context, shape));
        result.push(']');

        result
    }
}
impl<'a, 'tree> Rewrite for PrimaryExpression<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);

        if node.named_child_count() != 0 {
            result.push_str(&node.visit_children_in_same_line(" ", context, shape));
            return result;
        }

        match node.kind() {
            "identifier" => {
                result.push_str(node.v(source_code));
                result
            }
            _ => {
                println!(
                    "{} {}",
                    "### PrimaryExpression: unknown node: ".yellow(),
                    node.kind().red()
                );
                unreachable!();
            }
        }
    }
}

impl<'a, 'tree> Rewrite for DmlExpression<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let (node, mut result, _, _) = self.prepare(context);
        try_add_standalone_prefix(&mut result, shape, context);
        result.push_str(&node.visit_children_in_same_line(" ", context, shape));
        try_add_standalone_suffix(node, &mut result, shape, context.source_code);
        result
    }
}

impl<'a, 'tree> Rewrite for DmlSecurityMode<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, _shape: &mut Shape) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);

        result.push_str("as ");
        result.push_str(node.v(source_code));
        result
    }
}

impl<'a, 'tree> Rewrite for DmlType<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, _shape: &mut Shape) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);
        result.push_str(node.v(source_code));
        result
    }
}

impl<'a, 'tree> Rewrite for UpdateExpression<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);

        // use unnamed node as parser can't tell `i++` v.s. `++i` OR `i++` v.s. `i--`
        let mut cursor = node.walk();
        node.children(&mut cursor).for_each(|c| {
            if c.is_named() {
                let n = Expression::new(&c);
                result.push_str(&n.rewrite(context, shape));
            } else {
                result.push_str(c.v(source_code));
            }
        });
        result
    }
}

impl<'a, 'tree> Rewrite for RunAsStatement<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let (node, mut result, _, _) = self.prepare(context);
        try_add_standalone_prefix(&mut result, shape, context);

        result.push_str("System.runAs");
        let user = &node.c_by_n("user");
        let n = ParenthesizedExpression::new(user);
        result.push_str(&n.rewrite(context, &mut shape.clone_with_standalone(false)));

        let user = &node.c_by_k("block");
        let n = Block::new(user);
        result.push_str(&n.rewrite(context, &mut shape.clone_with_standalone(false)));

        result
    }
}

impl<'a, 'tree> Rewrite for ScopedTypeIdentifier<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let (node, mut result, _, _) = self.prepare(context);
        result.push_str(&node.visit_children_in_same_line(".", context, shape));
        result
    }
}

impl<'a, 'tree> Rewrite for ObjectCreationExpression<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let (node, mut result, _, _) = self.prepare(context);

        result.push_str("new ");
        let t = node.c_by_n("type");
        result.push_str(&t.visit(context, &mut shape.clone_with_standalone(false)));

        let arguments = node.c_by_n("arguments");
        let n = ArgumentList::new(&arguments);

        result.push_str(&n.rewrite(context, &mut shape.clone_with_standalone(false)));
        result
    }
}

impl<'a, 'tree> Rewrite for FieldAccess<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);

        let object = node.c_by_n("object");
        let n = PrimaryExpression::new(&object);
        result.push_str(&n.rewrite(context, shape));

        // `?.` need to traverse unnamed node;
        let mut current_node = object.next_sibling();
        while let Some(cur) = current_node {
            if cur.is_named() {
                break;
            } else {
                result.push_str(cur.v(source_code));
                current_node = cur.next_sibling();
            }
        }

        result.push_str(node.cv_by_n("field", source_code));
        result
    }
}

impl<'a, 'tree> Rewrite for InstanceOfExpression<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);

        let left = node.c_by_n("left");
        let n = Expression::new(&left);
        result.push_str(&n.rewrite(context, shape));

        result.push_str(" instanceof ");

        result.push_str(node.cv_by_n("right", source_code));
        result
    }
}

impl<'a, 'tree> Rewrite for CastExpression<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);

        result.push('(');
        result.push_str(node.cv_by_n("type", source_code));
        result.push_str(") ");

        let value = node.c_by_n("value");
        let n = Expression::new(&value);
        result.push_str(&n.rewrite(context, shape));

        result
    }
}

impl<'a, 'tree> Rewrite for AccessorList<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let (node, mut result, _, _) = self.prepare(context);

        result.push_str(" { ");
        let joined = node
            .cs_by_k("accessor_declaration")
            .iter()
            .map(|c| AccessorDeclaration::new(c).rewrite(context, shape))
            .collect::<Vec<_>>()
            .join(" ");

        result.push_str(&joined);
        result.push_str(" }");

        result
    }
}

impl<'a, 'tree> Rewrite for AccessorDeclaration<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);

        if let Some(ref a) = node.try_c_by_k("modifiers") {
            result.push_str(&Modifiers::new(a).rewrite(context, shape));
            result.push(' ');
        }

        // need to traverse unnamed node;
        let mut cursor = node.walk();
        node.children(&mut cursor).for_each(|c| {
            if !c.is_named() {
                result.push_str(c.v(source_code));
            }
        });

        // FIXME: implements max-width logic
        if let Some(ref b) = node.try_c_by_k("block") {
            result
                .push_str(&Block::new(b).rewrite(context, &mut shape.clone_with_standalone(false)));
            result.push(' ');
        }
        result
    }
}

impl<'a, 'tree> Rewrite for Boolean<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);

        result.push('(');
        result.push_str(node.cv_by_n("type", source_code));
        result.push_str(") ");

        let value = node.c_by_n("value");
        let n = Expression::new(&value);
        result.push_str(&n.rewrite(context, shape));

        result
    }
}

impl<'a, 'tree> Rewrite for TernaryExpression<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let (node, mut result, _, _) = self.prepare(context);

        let condition = node.c_by_n("condition");
        let n = Expression::new(&condition);
        result.push_str(&n.rewrite(context, shape));

        result.push_str(" ? ");

        let consequence = node.c_by_n("consequence");
        let n = Expression::new(&consequence);
        result.push_str(&n.rewrite(context, shape));

        result.push_str(" : ");

        let alternative = node.c_by_n("alternative");
        let n = Expression::new(&alternative);
        result.push_str(&n.rewrite(context, shape));
        result
    }
}

impl<'a, 'tree> Rewrite for MethodInvocation<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);
        try_add_standalone_prefix(&mut result, shape, context);

        if let Some(c) = node.try_c_by_n("object") {
            result.push_str(c.v(source_code));

            // `?.` need to traverse unnamed node;
            let mut current_node = c.next_sibling();
            while let Some(cur) = current_node {
                if cur.is_named() {
                    break;
                } else {
                    result.push_str(cur.v(source_code));
                    current_node = cur.next_sibling();
                }
            }
        };

        let name = node.cv_by_n("name", source_code);
        result.push_str(name);

        let arguments = node.c_by_n("arguments");
        result.push_str(&ArgumentList::new(&arguments).rewrite(context, shape));
        try_add_standalone_suffix(node, &mut result, shape, context.source_code);
        result
    }
}

impl<'a, 'tree> Rewrite for QueryExpression<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let (node, mut result, _, _) = self.prepare(context);
        let c = node.first_c();
        match_routing!(c, result, context, shape;
            "sosl_query" => SoslQuery,
            "soql_query" => SoqlQuery
        );
        result
    }
}

impl<'a, 'tree> Rewrite for SoqlQuery<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let (node, mut result, _, _) = self.prepare(context);
        result.push_str("[ ");
        let c = node.first_c();

        // FIXME: remove this, too small
        match_routing!(c, result, context, shape;
            "soql_query_body" => SoqlQueryBody
        );

        result.push_str("] ");
        result
    }
}

impl<'a, 'tree> Rewrite for SoqlQueryBody<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);

        result
    }
}

impl<'a, 'tree> Rewrite for SoslQuery<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);

        result
    }
}

impl<'a, 'tree> Rewrite for BinaryExpression<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);
        try_add_standalone_prefix(&mut result, shape, context);

        let left = node.cv_by_n("left", source_code);
        let op = node.cv_by_n("operator", source_code);
        let right = node.cv_by_n("right", source_code);
        result.push_str(&format!("{} {} {}", left, op, right));
        try_add_standalone_suffix(node, &mut result, shape, context.source_code);
        result
    }
}

impl<'a, 'tree> Rewrite for ArrayCreationExpression<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let (node, mut result, _, _) = self.prepare(context);

        result.push_str("new ");
        let t = self.node().c_by_n("type");
        result.push_str(&t.visit(context, &mut shape.clone_with_standalone(false)));

        if let Some(v) = node.try_c_by_n("value") {
            result.push_str(&v.visit(context, &mut shape.clone_with_standalone(false)));
        }

        if let Some(v) = node.try_c_by_n("dimensions") {
            result.push_str(&v.visit(context, &mut shape.clone_with_standalone(false)));
        }
        result
    }
}

impl<'a, 'tree> Rewrite for MapCreationExpression<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let (node, mut result, _, _) = self.prepare(context);

        result.push_str("new ");
        let t = node.c_by_n("type");
        result.push_str(&t.visit(context, &mut shape.clone_with_standalone(false)));

        let value = node.c_by_n("value");
        let n = MapInitializer::new(&value);
        result.push_str(&n.rewrite(context, shape));
        result
    }
}

impl<'a, 'tree> Rewrite for UnaryExpression<'a, 'tree> {
    fn rewrite(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);

        let operator_value = node.cv_by_n("operator", source_code);
        result.push_str(operator_value);

        let operand = node.c_by_n("operand");
        result.push_str(&operand.visit(context, &mut shape.clone_with_standalone(false)));
        result
    }
}
