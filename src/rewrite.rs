use crate::child::Accessor;
use crate::context::FmtContext;
use crate::match_routing;
use crate::route::EXP_MAP;
use crate::shape::Shape;
use crate::static_routing;
use crate::struct_def::*;
use crate::utility::*;
use crate::visit::Visitor;
use colored::Colorize;
#[allow(unused_imports)]
use log::debug;

pub trait Rewrite {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String;
}

impl<'a, 'tree> Rewrite for ClassDeclaration<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);
        try_add_standalone_prefix(&mut result, shape, context);

        if let Some(ref a) = node.try_c_by_k("modifiers") {
            result.push_str(&rewrite::<Modifiers>(a, shape, context));

            if let Some(_) = a.try_c_by_k("modifier") {
                result.push(' ');
            }
        }

        result.push_str("class ");
        result.push_str(node.cv_by_n("name", source_code));

        if let Some(ref c) = node.try_c_by_n("type_parameters") {
            result.push_str(&rewrite_shape::<TypeParameters>(c, shape, false, context));
        }

        if let Some(ref c) = node.try_c_by_n("superclass") {
            result.push_str(&rewrite_shape::<SuperClass>(c, shape, false, context));
        }

        if let Some(ref c) = node.try_c_by_n("interfaces") {
            result.push_str(&rewrite_shape::<Interfaces>(c, shape, false, context));
        }

        result.push_str(" {\n");

        let body_node = node.c_by_n("body");
        result.push_str(&body_node.apply_to_standalone_children(
            shape,
            context,
            |c, c_shape, c_context| c._visit(c_shape, c_context),
        ));

        result.push_str(&format!("{}}}", shape.indent.as_string(context.config)));
        try_add_standalone_suffix_no_semicolumn(node, &mut result, shape, source_code);
        result
    }
}

impl<'a, 'tree> Rewrite for MethodDeclaration<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, source_code, config) = self.prepare(context);
        try_add_standalone_prefix(&mut result, shape, context);

        if let Some(ref a) = node.try_c_by_k("modifiers") {
            result.push_str(&rewrite::<Modifiers>(a, shape, context));
            if let Some(_) = a.try_c_by_k("modifier") {
                result.push(' ');
            }
        }

        let type_node_value = node.cv_by_n("type", source_code);
        result.push_str(type_node_value);
        result.push(' ');

        let name_node_value = node.cv_by_n("name", source_code);
        result.push_str(name_node_value);

        result.push('(');

        let formal_parameter_nodes = node
            .try_c_by_n("parameters")
            .map(|n| n.try_cs_by_k("formal_parameter"))
            .unwrap_or_default();

        let parameters_value: Vec<String> = formal_parameter_nodes
            .iter()
            .map(|n| {
                if let Some(ref a) = n.try_c_by_k("modifiers") {
                    result.push_str(&rewrite::<Modifiers>(a, shape, context));
                    if let Some(_) = a.try_c_by_k("modifier") {
                        result.push(' ');
                    }
                }
                let type_str = n.cv_by_n("type", source_code);
                let name_str = n.cv_by_n("name", source_code);
                format!("{} {}", type_str, name_str)
            })
            .collect();

        let params_single_line = parameters_value.join(", ");

        shape.offset = result.len() + 3; // add trailing `) {` size

        //if shape.offset + params_single_line.len() <= shape.width {
        if shape.offset + params_single_line.len() <= 1000000 {
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

        result.push_str(")");

        if let Some(body) = node.try_c_by_n("body") {
            result.push_str(&rewrite_shape::<Block>(&body, shape, false, context));
            try_add_standalone_suffix_no_semicolumn(node, &mut result, shape, source_code);
        } else {
            try_add_standalone_suffix(node, &mut result, shape, source_code);
        }
        result
    }
}

impl<'a, 'tree> Rewrite for EnumDeclaration<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);
        try_add_standalone_prefix(&mut result, shape, context);

        if let Some(ref a) = node.try_c_by_k("modifiers") {
            result.push_str(&rewrite::<Modifiers>(a, shape, context));
            result.push(' ');
        }

        result.push_str("enum ");
        result.push_str(node.cv_by_n("name", source_code));

        let body = node.c_by_n("body");
        result.push_str(&rewrite_shape::<EnumBody>(&body, shape, false, context));

        add_standalone_suffix_no_semicolumn(&node, &mut result, source_code);

        result
    }
}

impl<'a, 'tree> Rewrite for EnumConstant<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);
        try_add_standalone_prefix(&mut result, shape, context);
        result.push_str(node.v(source_code));
        try_add_standalone_suffix(node, &mut result, shape, source_code);

        result
    }
}

impl<'a, 'tree> Rewrite for EnumBody<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);

        if shape.standalone {
            add_indent(&mut result, shape, context);
        } else {
            result.push(' ');
        }

        result.push_str("{\n");

        if node.named_child_count() > 0 {
            add_indent(
                &mut result,
                &shape.copy_with_indent_increase(context.config),
                context,
            );
        }
        result.push_str(&node.try_csv_by_k("enum_constant", source_code).join(", "));
        if node.named_child_count() > 0 {
            result.push('\n');
        }

        add_indent(&mut result, shape, context);
        result.push('}');
        result
    }
}

impl<'a, 'tree> Rewrite for FieldDeclaration<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);
        try_add_standalone_prefix(&mut result, shape, context);

        if let Some(ref a) = node.try_c_by_k("modifiers") {
            result.push_str(&rewrite::<Modifiers>(a, shape, context));
            result.push(' ');
        }

        let type_node_value = node.cv_by_n("type", source_code);
        result.push_str(type_node_value);

        result.push(' ');

        let v = node.c_by_k("variable_declarator");
        result.push_str(&rewrite::<VariableDeclarator>(&v, shape, context));

        if let Some(ref a) = node.try_c_by_k("accessor_list") {
            result.push_str(&rewrite::<AccessorList>(a, shape, context));

            // special case: it has no `;` ending with "accessor_list"
            try_add_standalone_suffix_no_semicolumn(node, &mut result, shape, &context.source_code);
        } else {
            try_add_standalone_suffix(node, &mut result, shape, &context.source_code);
        }
        result
    }
}

impl<'a, 'tree> Rewrite for SuperClass<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, _, _) = self.prepare(context);

        result.push_str(" extends ");
        result.push_str(&node.first_c()._visit(shape, context));
        result
    }
}

impl<'a, 'tree> Rewrite for Interfaces<'a, 'tree> {
    fn rewrite(&self, _shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);
        result.push_str(" implements ");

        let type_list = node.c_by_k("type_list");

        let type_lists = type_list.try_csv_by_k("type_identifier", source_code);
        result.push_str(&type_lists.join(", "));

        result
    }
}

impl<'a, 'tree> Rewrite for Value<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);
        try_add_standalone_prefix(&mut result, shape, context);
        result.push_str(node.v(source_code));
        try_add_standalone_suffix(node, &mut result, shape, &context.source_code);
        result
    }
}

impl<'a, 'tree> Rewrite for LocalVariableDeclaration<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);
        try_add_standalone_prefix(&mut result, shape, context);

        if let Some(ref a) = node.try_c_by_k("modifiers") {
            result.push_str(&rewrite::<Modifiers>(a, shape, context));
            result.push(' ');
        }

        let t = node.c_by_n("type"); // _unannotated_type
        result.push_str(&rewrite_shape::<Expression>(&t, shape, false, context));

        result.push(' ');

        let declarator_nodes = node.cs_by_n("declarator");
        let declarator_values: Vec<String> = declarator_nodes
            .iter()
            .map(|d| rewrite::<VariableDeclarator>(d, shape, context))
            .collect();

        result.push_str(&declarator_values.join(", "));

        try_add_standalone_suffix(node, &mut result, shape, source_code);
        result
    }
}

impl<'a, 'tree> Rewrite for Statement<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, _source_code, _) = self.prepare(context);

        result.push_str(&match_routing!(node, context, shape;
            "type_identifier" => Value,
            "identifier" => Value,
            "block" => Block,
            //"break_statement"
            //"continue_statement"
            //"declaration"
            "array_type" => ArrayType,
            "do_statement" => DoStatement,
            "enhanced_for_statement" => EnhancedForStatement,
            "expression_statement" => ExpressionStatement,
            "for_statement" => ForStatement,
            "if_statement" => IfStatement,
            //"labeled_statement"
            "local_variable_declaration" => LocalVariableDeclaration,
            "return_statement" => ReturnStatement,
            "run_as_statement" => RunAsStatement,
            "generic_type" => GenericType,
            //"switch_expression" =>
            //"throw_statement" => Thr
            "try_statement" => TryStatement,
            //"while_statement" => WhileStatement, // NOTE: it conflicts with try_add_standalone_prefix() which adds extra `;` at end
        ));
        result
    }
}

impl<'a, 'tree> Rewrite for ExpressionStatement<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, _, _) = self.prepare(context);
        try_add_standalone_prefix(&mut result, shape, context);
        let c = node.first_c();
        result.push_str(&rewrite_shape::<Expression>(&c, shape, false, context));
        try_add_standalone_suffix(node, &mut result, shape, &context.source_code);
        result
    }
}

impl<'a, 'tree> Rewrite for TryStatement<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);
        try_add_standalone_prefix(&mut result, shape, context);

        result.push_str("try");
        let body = node.c_by_n("body");
        result.push_str(&rewrite_shape::<Block>(&body, shape, false, context));

        let joined_children = node
            .try_cs_by_k("catch_clause")
            .iter()
            .map(|c| rewrite::<CatchClause>(c, shape, context))
            .collect::<Vec<_>>()
            .join("");
        result.push_str(&joined_children);

        if let Some(ref f) = node.try_c_by_k("finally_clause") {
            result.push_str(&rewrite_shape::<FinallyClause>(&f, shape, false, context));
        }
        try_add_standalone_suffix_no_semicolumn(node, &mut result, shape, source_code);

        result
    }
}

impl<'a, 'tree> Rewrite for FinallyClause<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, _, _) = self.prepare(context);

        result.push_str(" finally");
        let block = node.c_by_k("block");
        result.push_str(&rewrite_shape::<Block>(&block, shape, false, context));
        result
    }
}

impl<'a, 'tree> Rewrite for CatchClause<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, _, _) = self.prepare(context);

        result.push_str(" catch ");

        let param = node.c_by_k("catch_formal_parameter");
        result.push_str(&rewrite::<CatchFormalParameter>(&param, shape, context));

        let body = node.c_by_n("body");
        result.push_str(&rewrite_shape::<Block>(&body, shape, false, context));
        result
    }
}

impl<'a, 'tree> Rewrite for CatchFormalParameter<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, _, _) = self.prepare(context);

        result.push('(');
        result.push_str(&node.apply_to_children_in_same_line(
            " ",
            shape,
            context,
            |c, c_shape, c_context| c._visit(c_shape, c_context),
        ));
        result.push(')');
        result
    }
}

impl<'a, 'tree> Rewrite for VariableDeclarator<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);

        let name = node.cv_by_n("name", source_code);
        result.push_str(name);

        if let Some(v) = node.try_c_by_n("value") {
            result.push_str(" = ");
            let mut c_shape = shape.clone_with_standalone(false);
            if v.kind() == "array_initializer" {
                result.push_str(&rewrite::<ArrayInitializer>(&v, &mut c_shape, context));
            } else {
                result.push_str(&rewrite::<Expression>(&v, &mut c_shape, context));
            }
        }
        result
    }
}

impl<'a, 'tree> Rewrite for IfStatement<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, initial_context: &FmtContext) -> String {
        let (node, mut result, source_code, config) = self.prepare(initial_context);

        let mut node = node.clone(); // lifetime challenge
        let updated_context =
            update_source_code_for_if_statement(&node, source_code).map(|updated_source_code| {
                let wrapped_source = format!("class Dummy {{ {{ {} }} }}", updated_source_code);
                FmtContext::new(config, wrapped_source)
            });
        let context = match &updated_context {
            Some(c) => {
                node = c
                    .ast_tree
                    .root_node()
                    .first_c()
                    .c_by_n("body")
                    .c_by_k("block")
                    .c_by_k("if_statement");
                c.clone()
            }
            None => initial_context.clone(), // lifetime challenge
        };
        let source_code = &context.source_code;

        // possible re-structure done;

        try_add_standalone_prefix(&mut result, shape, &context);

        result.push_str("if ");

        let con = node.c_by_n("condition");
        result.push_str(&rewrite::<ParenthesizedExpression>(&con, shape, &context));

        let consequence = node.c_by_n("consequence");
        result.push_str(&rewrite_shape::<Block>(
            &consequence,
            shape,
            false,
            &context,
        ));

        if let Some(ref a) = node.try_c_by_n("alternative") {
            match a.kind() {
                "block" => {
                    result.push_str(" else");
                    result.push_str(&rewrite_shape::<Block>(a, shape, false, &context));
                }
                "if_statement" => {
                    result.push_str(" else ");
                    result.push_str(&rewrite_shape::<IfStatement>(a, shape, false, &context));
                }
                _ => {
                    unreachable!()
                }
            }
        };

        // use original node and context rather than the re-structured
        try_add_standalone_suffix_no_semicolumn(
            &self.node(),
            &mut result,
            shape,
            &initial_context.source_code,
        );
        result
    }
}

impl<'a, 'tree> Rewrite for ForStatement<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);
        try_add_standalone_prefix(&mut result, shape, context);

        result.push_str("for (");
        if let Some(ref c) = node.try_c_by_n("init") {
            result.push_str(&rewrite_shape::<Expression>(c, shape, false, context));
        };
        result.push(';');

        if let Some(ref c) = node.try_c_by_n("condition") {
            result.push(' ');
            result.push_str(&rewrite_shape::<Expression>(c, shape, false, context));
        };

        result.push(';');

        if let Some(ref c) = node.try_c_by_n("update") {
            result.push(' ');
            result.push_str(&rewrite_shape::<Expression>(c, shape, false, context));
        };
        result.push(')');

        let body = node.c_by_n("body");
        let is_block_node = body.kind() == "block";

        if is_block_node {
            result.push_str(&rewrite_shape::<Block>(&body, shape, false, context));
        } else if body.kind() == ";" {
            result.push(';');
        } else {
            result.push_str(" {\n");
            let mut c_shape = shape
                .copy_with_indent_increase(context.config)
                .clone_with_standalone(true);
            result.push_str(&rewrite::<Statement>(&body, &mut c_shape, context));

            result.push('\n');
            add_indent(&mut result, shape, context);
            result.push_str("}");
        }

        add_standalone_suffix_no_semicolumn(&node, &mut result, source_code);
        result
    }
}

impl<'a, 'tree> Rewrite for EnhancedForStatement<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);
        try_add_standalone_prefix(&mut result, shape, context);

        result.push_str("for (");
        let t = node.c_by_n("type");
        result.push_str(&rewrite_shape::<Statement>(&t, shape, false, context));
        result.push(' ');

        let name = node.c_by_n("name");
        result.push_str(name.v(source_code));
        result.push_str(" : ");

        let value = node.c_by_n("value");
        result.push_str(&rewrite_shape::<Expression>(&value, shape, false, context));
        result.push(')');

        let body = node.c_by_n("body");
        let is_block_node = body.kind() == "block";

        if is_block_node {
            result.push_str(&rewrite_shape::<Block>(&body, shape, false, context));
        } else if body.kind() == ";" {
            result.push(';');
        } else {
            result.push_str(" {\n");
            let mut c_shape = shape
                .copy_with_indent_increase(context.config)
                .clone_with_standalone(true);
            result.push_str(&rewrite::<Statement>(&value, &mut c_shape, context));
            result.push_str(&format!("\n{}}}", shape.indent.as_string(context.config)));
        }

        add_standalone_suffix_no_semicolumn(&node, &mut result, source_code);
        result
    }
}

impl<'a, 'tree> Rewrite for ParenthesizedExpression<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        format!(
            "({})",
            &self.node().apply_to_children_in_same_line(
                ", ",
                shape,
                context,
                |c, c_shape, c_context| c._visit(c_shape, c_context),
            )
        )
    }
}

impl<'a, 'tree> Rewrite for Block<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, _, _) = self.prepare(context);

        if shape.standalone {
            add_indent(&mut result, shape, context);
        } else {
            result.push(' ');
        }

        result.push_str("{\n");

        // TODO: children -> statement
        result.push_str(&node.apply_to_standalone_children(
            &shape.clone_with_standalone(true),
            context,
            |c, c_shape, c_context| c._visit(c_shape, c_context),
        ));

        add_indent(&mut result, shape, context);
        result.push('}');
        result
    }
}

impl<'a, 'tree> Rewrite for Expression<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, _, _) = self.prepare(context);

        result.push_str(&static_routing!(EXP_MAP, node, context, shape));
        result
    }
}

impl<'a, 'tree> Rewrite for LineComment<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);
        add_prefix_for_comment(node, &mut result, shape, context);
        result.push_str(node.v(source_code));
        add_standalone_suffix_no_semicolumn(node, &mut result, source_code);
        result
    }
}

impl<'a, 'tree> Rewrite for BlockComment<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);
        add_prefix_for_comment(node, &mut result, shape, context);
        result.push_str(node.v(source_code));
        add_standalone_suffix_no_semicolumn(node, &mut result, source_code);
        result
    }
}

impl<'a, 'tree> Rewrite for ReturnStatement<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);
        try_add_standalone_prefix(&mut result, shape, context);

        result.push_str("return");
        if node.named_child_count() != 0 {
            let child = node.first_c();
            result.push(' ');
            result.push_str(&rewrite_shape::<Expression>(&child, shape, false, context));
        }

        try_add_standalone_suffix(node, &mut result, shape, source_code);

        result
    }
}

impl<'a, 'tree> Rewrite for GenericType<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);

        let name = node.c_by_k("type_identifier");
        result.push_str(name.v(source_code));

        let arguments = node.c_by_k("type_arguments");
        result.push_str(&rewrite::<TypeArguments>(&arguments, shape, context));
        result
    }
}

impl<'a, 'tree> Rewrite for ArgumentList<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, _, _) = self.prepare(context);

        result.push('(');
        let joined = node
            .children_vec()
            .iter()
            .map(|c| rewrite_shape::<Expression>(c, shape, false, context))
            .collect::<Vec<_>>()
            .join(", ");

        result.push_str(&joined);
        result.push(')');
        result
    }
}

impl<'a, 'tree> Rewrite for TypeArguments<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, _, _) = self.prepare(context);

        result.push('<');
        let joined = node.try_visit_cs(context, shape).join(", ");
        result.push_str(&joined);
        result.push('>');
        result
    }
}

impl<'a, 'tree> Rewrite for ArrayInitializer<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
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
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let child = self.node().first_c();
        format!("[{}]", &rewrite::<Expression>(&child, shape, context))
    }
}

impl<'a, 'tree> Rewrite for ArrayType<'a, 'tree> {
    fn rewrite(&self, _shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);

        let element_value = node.cv_by_n("element", source_code);
        result.push_str(element_value);
        let element_value = node.cv_by_n("dimensions", source_code);
        result.push_str(element_value);
        result
    }
}

impl<'a, 'tree> Rewrite for MapInitializer<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, _, _) = self.prepare(context);

        let children = node
            .children_vec()
            .iter()
            .map(|c| rewrite::<Expression>(c, shape, context))
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
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);

        result.push('@');
        let name = node.c_by_n("name");
        result.push_str(name.v(source_code));

        if let Some(a) = node.try_c_by_n("arguments") {
            result.push('(');
            result.push_str(&rewrite::<AnnotationArgumentList>(&a, shape, context));
            result.push(')');
        }

        result.push('\n');
        add_indent(&mut result, shape, context);
        result
    }
}

impl<'a, 'tree> Rewrite for AnnotationArgumentList<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);

        let joined = node
            .try_visit_cs(context, &mut shape.clone_with_standalone(false))
            .join(" ");
        result.push_str(&joined);
        result

        //if let Some(c) = node.try_c_by_n("value") {
        //    result.push_str(c.v(source_code));
        //}

        //let joined_children = node
        //    .try_cs_by_k("annotation_key_value")
        //    .iter()
        //    .map(|c| rewrite_shape::<AnnotationKeyValue>(c, shape, false, context))
        //    .collect::<Vec<_>>()
        //    .join(" ");

        //if let Some(ref a) = node
        //    .try_c_by_k("modifiers")
        //    .and_then(|n| n.try_c_by_k("annotation"))
        //{
        //    result.push_str(&rewrite::<Annotation>(a, shape, context));
        //}
    }
}

impl<'a, 'tree> Rewrite for AnnotationKeyValue<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);

        let key = node.c_by_n("key");
        result.push_str(key.v(source_code));

        result.push('=');

        let value = node.c_by_n("value");
        result.push_str(&rewrite::<Expression>(&value, shape, context));

        result
    }
}

impl<'a, 'tree> Rewrite for Modifiers<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);

        node.try_cs_by_k("annotation").iter().for_each(|c| {
            result.push_str(&rewrite_shape::<Annotation>(c, shape, true, context));
        });

        let joined = node
            .try_cs_by_k("modifier")
            .iter()
            .map(|c| {
                if c.first_c().kind() == "testMethod" {
                    // old style test method
                    "testMethod".to_string()
                } else {
                    c.v(source_code).to_string()
                }
            })
            .collect::<Vec<_>>()
            .join(" ");
        result.push_str(&joined);
        result
    }
}

impl<'a, 'tree> Rewrite for ConstructorDeclaration<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);

        try_add_standalone_prefix(&mut result, shape, context);

        if let Some(ref c) = node.try_c_by_k("modifiers") {
            result.push_str(&rewrite::<Modifiers>(c, shape, context));
            result.push(' ');
        }

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
        result.push_str(&rewrite::<ConstructorBody>(
            &constructor_body,
            shape,
            context,
        ));

        try_add_standalone_suffix_no_semicolumn(node, &mut result, shape, source_code);
        result
    }
}

impl<'a, 'tree> Rewrite for ConstructorBody<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, _, _) = self.prepare(context);

        result.push_str(" {\n");
        result.push_str(&node.apply_to_standalone_children(
            shape,
            context,
            |c, c_shape, c_context| c._visit(c_shape, c_context),
        ));
        result.push_str(&format!("{}}}", shape.indent.as_string(context.config)));
        result
    }
}

impl<'a, 'tree> Rewrite for ExplicitConstructorInvocation<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);
        try_add_standalone_prefix(&mut result, shape, context);

        let constructor = node.c_by_n("constructor");
        result.push_str(constructor.v(source_code));

        let arguments = node.c_by_n("arguments");
        result.push_str(&rewrite::<ArgumentList>(&arguments, shape, context));
        try_add_standalone_suffix(node, &mut result, shape, source_code);

        result
    }
}

impl<'a, 'tree> Rewrite for AssignmentExpression<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);
        try_add_standalone_prefix(&mut result, shape, context);

        let left = node.c_by_n("left");
        let left_value = match_routing!(left, context, shape;
            "array_access" => ArrayAccess,
            "field_access" => FieldAccess,
            "identifier" => Value,
        );

        let op = node.cv_by_n("operator", source_code);

        let right = node.c_by_n("right");
        let right_value = rewrite_shape::<Expression>(&right, shape, false, context);

        result.push_str(&format!("{} {} {}", left_value, op, right_value));
        try_add_standalone_suffix(node, &mut result, shape, &context.source_code);
        result
    }
}

impl<'a, 'tree> Rewrite for DoStatement<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, _, _) = self.prepare(context);
        try_add_standalone_prefix(&mut result, shape, context);

        result.push_str("do");
        let body = node.c_by_n("body");
        result.push_str(&rewrite_shape::<Block>(&body, shape, false, context));

        result.push_str(" while ");
        let condition = node.c_by_n("condition");
        result.push_str(&rewrite_shape::<ParenthesizedExpression>(
            &condition, shape, false, context,
        ));

        try_add_standalone_suffix(node, &mut result, shape, &context.source_code);
        result
    }
}

impl<'a, 'tree> Rewrite for WhileStatement<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);
        try_add_standalone_prefix(&mut result, shape, context);

        result.push_str("while ");
        let condition = node.c_by_n("condition");
        result.push_str(&rewrite_shape::<ParenthesizedExpression>(
            &condition, shape, false, context,
        ));

        let body = node.c_by_n("body");
        let is_block_node = body.kind() == "block";

        if is_block_node {
            result.push_str(&rewrite_shape::<Block>(&body, shape, false, context));
        } else if body.kind() == ";" {
            result.push(';');
        } else {
            result.push_str(" {\n");
            let mut c_shape = shape
                .copy_with_indent_increase(context.config)
                .clone_with_standalone(true);
            result.push_str(&rewrite::<Statement>(&body, &mut c_shape, context));

            result.push('\n');
            add_indent(&mut result, shape, context);
            result.push_str("}");
        }

        add_standalone_suffix_no_semicolumn(&node, &mut result, source_code);
        result
    }
}

impl<'a, 'tree> Rewrite for ArrayAccess<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, _, _) = self.prepare(context);

        let array = &node.c_by_n("array");
        result.push_str(&rewrite::<Expression>(&array, shape, context));

        let index = &node.c_by_n("index");
        result.push('[');
        result.push_str(&rewrite::<Expression>(&index, shape, context));
        result.push(']');

        result
    }
}
impl<'a, 'tree> Rewrite for PrimaryExpression<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);

        if node.named_child_count() != 0 {
            result.push_str(&node.apply_to_children_in_same_line(
                " ",
                shape,
                context,
                |c, c_shape, c_context| c._visit(c_shape, c_context),
            ));
            return result;
        }

        match node.kind() {
            "this" => {
                result.push_str(node.v(source_code));
                result
            }
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
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, _, _) = self.prepare(context);
        try_add_standalone_prefix(&mut result, shape, context);
        result.push_str(&node.apply_to_children_in_same_line(
            " ",
            shape,
            context,
            |c, c_shape, c_context| c._visit(c_shape, c_context),
        ));
        try_add_standalone_suffix(node, &mut result, shape, &context.source_code);
        result
    }
}

impl<'a, 'tree> Rewrite for DmlSecurityMode<'a, 'tree> {
    fn rewrite(&self, _shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);

        result.push_str("as ");
        result.push_str(node.v(source_code));
        result
    }
}

impl<'a, 'tree> Rewrite for DmlType<'a, 'tree> {
    fn rewrite(&self, _shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);
        result.push_str(node.v(source_code));
        result
    }
}

impl<'a, 'tree> Rewrite for UpdateExpression<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);

        // Needs to travsers un-named children
        // AST can't tell `i++` v.s. `++i` OR `i++` v.s. `i--`
        node.all_children_vec().iter().for_each(|c| {
            if c.is_named() {
                result.push_str(&rewrite::<Expression>(&c, shape, context));
            } else {
                result.push_str(c.v(source_code));
            }
        });
        result
    }
}

impl<'a, 'tree> Rewrite for RunAsStatement<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);
        try_add_standalone_prefix(&mut result, shape, context);

        result.push_str("System.runAs");
        let user = &node.c_by_n("user");
        result.push_str(&rewrite_shape::<ParenthesizedExpression>(
            &user, shape, false, context,
        ));

        let user = &node.c_by_k("block");
        result.push_str(&rewrite_shape::<Block>(&user, shape, false, context));

        try_add_standalone_suffix_no_semicolumn(node, &mut result, shape, source_code);
        result
    }
}

impl<'a, 'tree> Rewrite for ScopedTypeIdentifier<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, _, _) = self.prepare(context);
        result.push_str(&node.apply_to_children_in_same_line(
            ".",
            shape,
            context,
            |c, c_shape, c_context| c._visit(c_shape, c_context),
        ));
        result
    }
}

impl<'a, 'tree> Rewrite for ObjectCreationExpression<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, _source_code, _) = self.prepare(context);

        result.push_str("new ");
        let t = node.c_by_n("type"); // _simple_type, send to Exp for simplicity for now
        result.push_str(&rewrite_shape::<Expression>(&t, shape, false, context));

        let arguments = node.c_by_n("arguments");
        result.push_str(&rewrite_shape::<ArgumentList>(
            &arguments, shape, false, context,
        ));
        result
    }
}

impl<'a, 'tree> Rewrite for FieldAccess<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);

        let o = node.c_by_n("object");
        result.push_str(&match o.kind() {
            "super" => o.v(source_code).to_string(),
            "identifier" => o.v(source_code).to_string(),
            "field_access" => rewrite::<FieldAccess>(&o, shape, context),
            "array_access" => rewrite::<ArrayAccess>(&o, shape, context),
            _ => rewrite::<PrimaryExpression>(&o, shape, context),
        });

        // FIXME: parser updated already -> `?.` need to traverse unnamed node;
        let mut current_node = o.next_sibling();
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
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);

        let left = node.c_by_n("left");
        result.push_str(&rewrite::<Expression>(&left, shape, context));

        result.push_str(" instanceof ");

        result.push_str(node.cv_by_n("right", source_code));
        result
    }
}

impl<'a, 'tree> Rewrite for CastExpression<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);

        result.push('(');
        result.push_str(node.cv_by_n("type", source_code));
        result.push_str(") ");

        let value = node.c_by_n("value");
        result.push_str(&rewrite::<Expression>(&value, shape, context));
        result
    }
}

impl<'a, 'tree> Rewrite for AccessorList<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, _, _) = self.prepare(context);

        result.push_str(" { ");
        let joined = node
            .cs_by_k("accessor_declaration")
            .iter()
            .map(|c| rewrite::<AccessorDeclaration>(c, shape, context))
            .collect::<Vec<_>>()
            .join(" ");

        result.push_str(&joined);
        result.push_str(" }");

        result
    }
}

impl<'a, 'tree> Rewrite for AccessorDeclaration<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);

        if let Some(ref a) = node.try_c_by_k("modifiers") {
            result.push_str(&rewrite::<Modifiers>(a, shape, context));
            result.push(' ');
        }

        // it travsers un-named children
        node.all_children_vec().iter().for_each(|c| {
            if !c.is_named() {
                result.push_str(c.v(source_code));
            }
        });

        // FIXME: implements max-width logic
        if let Some(ref b) = node.try_c_by_k("block") {
            result.push_str(&rewrite_shape::<Block>(&b, shape, false, context));
            result.push(' ');
        }
        result
    }
}

//impl<'a, 'tree> Rewrite for Boolean<'a, 'tree> {
//    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
//        let (node, mut result, source_code, _) = self.prepare(context);
//
//        result.push('(');
//        result.push_str(node.cv_by_n("type", source_code));
//        result.push_str(") ");
//
//        let value = node.c_by_n("value");
//        result.push_str(&rewrite::<Expression>(&value, shape, context));
//        result
//    }
//}

impl<'a, 'tree> Rewrite for TernaryExpression<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, _, _) = self.prepare(context);

        let condition = node.c_by_n("condition");
        result.push_str(&rewrite::<Expression>(&condition, shape, context));

        result.push_str(" ? ");

        let consequence = node.c_by_n("consequence");
        result.push_str(&rewrite::<Expression>(&consequence, shape, context));

        result.push_str(" : ");

        let alternative = node.c_by_n("alternative");
        result.push_str(&rewrite::<Expression>(&alternative, shape, context));
        result
    }
}

impl<'a, 'tree> Rewrite for MethodInvocation<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
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

        if let Some(a) = node.try_c_by_n("arguments") {
            result.push_str(&rewrite::<ArgumentList>(&a, shape, context));
        }
        try_add_standalone_suffix(node, &mut result, shape, &context.source_code);
        result
    }
}

impl<'a, 'tree> Rewrite for QueryExpression<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, _, _) = self.prepare(context);

        let c = node.first_c().first_c(); // skip SoslQuery and SoqlQuery container node;
        result.push_str(&match_routing!(c, context, shape;
            "sosl_query_body" => SoslQueryBody,
            "soql_query_body" => SoqlQueryBody,
        ));
        result
    }
}

impl<'a, 'tree> Rewrite for SoqlQuery<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, _, _) = self.prepare(context);
        let c = node.first_c();
        result.push_str(&rewrite::<SoqlQueryBody>(&c, shape, context));
        result
    }
}

impl<'a, 'tree> Rewrite for SoqlQueryBody<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, _, _) = self.prepare(context);

        result.push_str("[");

        let joined_children = node
            .children_vec()
            .iter()
            .map(|c| {
                match_routing!(c, context, shape;
                "select_clause" => SelectClause,
                "from_clause" => FromClause,
                "where_clause" => WhereCluase,
                "limit_clause" => LimitClause,
                "offset_clause" => OffsetClause,
                "all_rows_clause" => AllRowClause,
                "order_by_clause" => OrderByClause,
                )
            })
            .collect::<Vec<_>>()
            .join(" ");
        result.push_str(&joined_children);
        result.push_str("]");
        result

        //all_rows_clause
        //for_clause
        //group_by_clause
        //limit_clause
        //order_by_clause
        //update_clause
        //using_clause
        //with_clause
    }
}

impl<'a, 'tree> Rewrite for SelectClause<'a, 'tree> {
    fn rewrite(&self, _shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);

        result.push_str("SELECT ");
        let joined_children = node
            .children_vec()
            .iter()
            .map(|c| {
                //let mut c_shape = shape.clone_with_standalone(false);
                c.v(source_code)
            })
            .collect::<Vec<_>>()
            .join(", ");

        result.push_str(&joined_children);

        //"type": "alias_expression",
        //"type": "count_expression",
        //"type": "field_identifier",
        //"type": "fields_expression",
        //"type": "function_expression",
        //"type": "subquery",
        //"type": "type_of_clause",
        result
    }
}

impl<'a, 'tree> Rewrite for FromClause<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, _, _) = self.prepare(context);
        result.push_str("FROM ");

        let joined_children = node
            .children_vec()
            .iter()
            .map(|c| {
                match_routing!(c, context, shape;
                "storage_alias" => StorageAlias,
                "storage_identifier" => StorageIdentifier,
                )
            })
            .collect::<Vec<_>>()
            .join(" ");
        result.push_str(&joined_children);

        result
    }
}

impl<'a, 'tree> Rewrite for OffsetClause<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);

        result.push_str("OFFSET ");
        let c = node.first_c();
        if c.kind() == "bound_apex_expression" {
            result.push_str(&rewrite::<BoundApexExpression>(&c, shape, context));
        } else {
            result.push_str(c.v(source_code));
        }
        result
    }
}

impl<'a, 'tree> Rewrite for AllRowClause<'a, 'tree> {
    fn rewrite(&self, _shape: &mut Shape, _context: &FmtContext) -> String {
        "ALL ROWS".to_string()
    }
}

impl<'a, 'tree> Rewrite for OrderByClause<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, _source_code, _) = self.prepare(context);

        result.push_str("ORDER BY ");

        let joined_c: String = node
            .children_vec()
            .iter()
            .map(|c| rewrite::<OrderExpression>(c, shape, context))
            .collect::<Vec<_>>()
            .join(" ");
        result.push_str(&joined_c);
        result
    }
}

impl<'a, 'tree> Rewrite for OrderExpression<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, _source_code, _) = self.prepare(context);

        let joined_c: String = node
            .children_vec()
            .iter()
            .map(|c| {
                match_routing!(c, context, shape;
                    "field_identifier" => FieldIdentifier,
                    //"type": "function_expression",
                    //"type": "order_direction",
                    //"type": "order_null_direction",
                )
            })
            .collect::<Vec<_>>()
            .join(" ");
        result.push_str(&joined_c);
        result
    }
}

impl<'a, 'tree> Rewrite for StorageAlias<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);

        if node.kind() == "storage_identifier" {
            result.push_str(&rewrite::<StorageIdentifier>(&node, shape, context));
        } else {
            result.push_str(&node.v(source_code));
        }
        result
    }
}

impl<'a, 'tree> Rewrite for StorageIdentifier<'a, 'tree> {
    fn rewrite(&self, _shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);

        let c = node.first_c();
        if c.kind() == "dotted_identifier" {
            let joined = c
                .children_vec()
                .iter()
                .map(|child| child.v(source_code))
                .collect::<Vec<_>>()
                .join(".");
            result.push_str(&joined);
        } else {
            result.push_str(&node.v(source_code));
        }
        result
    }
}

impl<'a, 'tree> Rewrite for WhereCluase<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, _source_code, _) = self.prepare(context);

        result.push_str("WHERE ");
        let c = node.first_c();
        result.push_str(&match_routing!(c, context, shape;
            "comparison_expression" => ComparisonExpression,
            "and_expression" => AndExpression,
            //"not_expression" => StorageIdentifier,
            //"or_expression" => StorageIdentifier,
        ));

        result
    }
}

impl<'a, 'tree> Rewrite for AndExpression<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);

        let joined_children = node
            .children_vec()
            .iter()
            .map(|c| {
                match_routing!(c, context, shape;
                    "comparison_expression" => ComparisonExpression,
                )
            })
            .collect::<Vec<_>>()
            .join(" AND ");
        result.push_str(&joined_children);
        result
    }
}

impl<'a, 'tree> Rewrite for LimitClause<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);

        result.push_str("LIMIT ");
        let c = node.first_c();
        if c.kind() == "bound_apex_expression" {
            result.push_str(&rewrite::<BoundApexExpression>(&c, shape, context));
        } else {
            result.push_str(c.v(source_code));
        }
        result
    }
}

impl<'a, 'tree> Rewrite for ComparisonExpression<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, _source_code, _) = self.prepare(context);

        let joined_children = node
            .children_vec()
            .iter()
            .map(|child| {
                match_routing!(child, context, shape;
                    "field_identifier" => FieldIdentifier,
                    "bound_apex_expression" => BoundApexExpression,
                    "value_comparison_operator" => Value,
                    "string_literal" => Value,
                    "boolean" => Value,
                    "set_comparison_operator" => Value,
                    "null_literal" => Value,
                    "decimal" => Value,
                    "date_literal_with_param" => DateLiteralWithParam,
                    //"storage_identifier" => StorageIdentifier,
                )
            })
            .collect::<Vec<_>>()
            .join(" ");
        result.push_str(&joined_children);
        result
    }
}

impl<'a, 'tree> Rewrite for DateLiteralWithParam<'a, 'tree> {
    fn rewrite(&self, _shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);

        let joined_c: String = node
            .children_vec()
            .iter()
            .map(|c| c.v(source_code))
            .collect::<Vec<_>>()
            .join(":");
        result.push_str(&joined_c);
        result
    }
}

impl<'a, 'tree> Rewrite for FieldIdentifier<'a, 'tree> {
    fn rewrite(&self, _shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);

        let c = node.first_c();
        if c.kind() == "dotted_identifier" {
            let joined = c
                .children_vec()
                .iter()
                .map(|child| child.v(source_code))
                .collect::<Vec<_>>()
                .join(".");
            result.push_str(&joined);
        } else {
            result.push_str(&node.v(source_code));
        }
        result
    }
}

impl<'a, 'tree> Rewrite for BoundApexExpression<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, _, _) = self.prepare(context);

        // special case:
        let after_bound_apex = node.prev_named_sibling().map_or(false, |prev_node| {
            prev_node.kind() == "set_comparison_operator"
        });

        if after_bound_apex {
            result.push('(');
        }

        result.push(':');
        let c = node.first_c();
        result.push_str(&rewrite::<Expression>(&c, shape, context));

        if after_bound_apex {
            result.push(')');
        }
        result
    }
}

impl<'a, 'tree> Rewrite for SoslQuery<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, _, _) = self.prepare(context);
        let c = node.first_c();
        result.push_str(&rewrite::<SoqlQuery>(&c, shape, context));
        result
    }
}

impl<'a, 'tree> Rewrite for SoslQueryBody<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, _, _) = self.prepare(context);

        result.push('[');

        let joined_c: String = node
            .children_vec()
            .iter()
            .map(|c| {
                match_routing!(c, context, shape;
                    "find_clause" => FindClause,
                    "returning_clause" => ReturningClause,
                    "in_clause" => InClause,
                    "with_clause" => WithClause,
                    "limit_clause" => LimitClause,
                )
            })
            .collect::<Vec<_>>()
            .join(" ");
        result.push_str(&joined_c);

        //let f = node.c_by_k("find_clause");
        //result.push_str(&rewrite::<FindClause>(&f, shape, context));
        //
        //if let Some(r) = node.try_c_by_k("returning_clause");
        //result.push_str(&rewrite::<FindClause>(&f, shape, context));
        result.push(']');
        result

        //"type": "find_clause",
        //"type": "in_clause",
        //"type": "limit_clause",
        //"type": "offset_clause",
        //"type": "returning_clause",
        //"type": "update_clause",
        //"type": "with_clause",
    }
}

impl<'a, 'tree> Rewrite for FindClause<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);
        result.push_str("FIND ");

        let c = node.first_c();
        if c.kind() == "bound_apex_expression" {
            result.push_str(&rewrite::<BoundApexExpression>(&c, shape, context));
        } else {
            let joined_c = node
                .children_vec()
                .iter()
                .map(|c| c.v(source_code))
                .collect::<Vec<_>>()
                .join("");
            result.push_str(&joined_c);
        }
        result
    }
}

impl<'a, 'tree> Rewrite for WithDivisionExpression<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);
        result.push_str("WITH DIVISION = ");

        let c = node.first_c();
        if c.kind() == "bound_apex_expression" {
            result.push_str(&rewrite::<BoundApexExpression>(&c, shape, context));
        } else {
            let joined_c = node
                .children_vec()
                .iter()
                .map(|c| c.v(source_code))
                .collect::<Vec<_>>()
                .join("");
            result.push_str(&joined_c);
        }
        result
    }
}

impl<'a, 'tree> Rewrite for ReturningClause<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, _, _) = self.prepare(context);
        result.push_str("RETURNING ");

        let joined_c = node
            .children_vec()
            .iter()
            .map(|c| rewrite::<SobjectReturn>(c, shape, context))
            .collect::<Vec<_>>()
            .join(", ");
        result.push_str(&joined_c);
        result
    }
}

impl<'a, 'tree> Rewrite for InClause<'a, 'tree> {
    fn rewrite(&self, _shape: &mut Shape, context: &FmtContext) -> String {
        let (node, _, source_code, _) = self.prepare(context);
        format!("IN {} FIELDS", node.first_c().v(source_code))
    }
}

impl<'a, 'tree> Rewrite for WithClause<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);

        //"with_highlight" //
        //"with_pricebook_expression" // c: string_literal
        //"with_snippet_expression" //o-c: int
        //"with_spell_correction_expression" //c: boolean
        //"with_data_cat_expression"
        //"with_division_expression"
        //"with_metadata_expression"
        //"with_network_expression"
        //"with_record_visibility_expression"
        //"with_user_id_type" //c: string_literal

        let with_type = node.first_c();
        let joined: String = with_type
            .children_vec()
            .iter()
            .map(|c| match c.kind() {
                // NOTE: use Cow to avoid converting?
                "with_highlight" => c.v(source_code).to_string(),
                "with_snippet_expression" => c.v(source_code).to_string(),
                "with_pricebook_expression" => c.first_c().v(source_code).to_string(),
                "with_spell_correction_expression" => c.first_c().v(source_code).to_string(),
                "with_user_id_type" => c.first_c().v(source_code).to_string(),
                _ => {
                    match_routing!(c, context, shape;
                        "with_division_expression" => WithDivisionExpression,
                    )
                }
            })
            .collect::<Vec<_>>()
            .join(" ");
        result.push_str(&joined);
        result
    }
}

impl<'a, 'tree> Rewrite for SobjectReturn<'a, 'tree> {
    fn rewrite(&self, _shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);

        result.push_str(&node.first_c().v(source_code));

        if node.named_child_count() > 1 {
            result.push('(');

            let joined_c = node
                .children_vec()
                .iter()
                .skip(1)
                .map(|c| c.v(source_code))
                .collect::<Vec<_>>()
                .join(" ");
            result.push_str(&joined_c);

            result.push(')');
        }
        result
    }
}

impl<'a, 'tree> Rewrite for BinaryExpression<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);
        try_add_standalone_prefix(&mut result, shape, context);

        let left = node.c_by_n("left");
        let left_v = rewrite::<Expression>(&left, shape, context);

        // `operator`is a hidden/un-named node, but has field_name so `cv_by_n()` works
        let op = node.cv_by_n("operator", source_code);

        let right = node.c_by_n("right");
        let right_v = rewrite::<Expression>(&right, shape, context);

        result.push_str(&format!("{} {} {}", left_v, op, right_v));
        try_add_standalone_suffix(node, &mut result, shape, &context.source_code);
        result
    }
}

impl<'a, 'tree> Rewrite for ArrayCreationExpression<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);
        result.push_str("new ");

        // special case: both styles are supported.
        // a: Integer[] a = new Integer[]{1, 2, 3, 4};
        // b: Integer[] a = new List<Integer>{ 1, 2, 3, 4 };
        let t = node.c_by_n("type"); // _simple_type, send to Exp for simplicity for now
        result.push_str(&rewrite_shape::<Expression>(&t, shape, false, context));

        if let Some(ref v) = node.try_c_by_n("dimensions") {
            if v.kind() == "dimensions" {
                result.push_str(v.v(source_code));
            } else {
                result.push_str(&rewrite_shape::<Expression>(v, shape, false, context));
            }
        }

        if let Some(ref v) = node.try_c_by_n("value") {
            result.push_str(&rewrite_shape::<ArrayInitializer>(v, shape, false, context));
        }
        result
    }
}

impl<'a, 'tree> Rewrite for MapCreationExpression<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, _, _) = self.prepare(context);

        result.push_str("new ");
        let t = node.c_by_n("type"); // _simple_type, send to Exp for simplicity for now
        result.push_str(&rewrite_shape::<Expression>(&t, shape, false, context));

        let value = node.c_by_n("value");
        result.push_str(&rewrite::<MapInitializer>(&value, shape, context));
        result
    }
}

impl<'a, 'tree> Rewrite for UnaryExpression<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);

        let operator_value = node.cv_by_n("operator", source_code);
        result.push_str(operator_value);

        let operand = node.c_by_n("operand");
        result.push_str(&rewrite_shape::<Expression>(
            &operand, shape, false, context,
        ));
        result
    }
}

impl<'a, 'tree> Rewrite for SwitchExpression<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, _, _) = self.prepare(context);
        try_add_standalone_prefix(&mut result, shape, context);

        result.push_str("switch on ");
        let con = node.c_by_n("condition");
        result.push_str(&rewrite_shape::<Expression>(&con, shape, false, context));

        let b = node.c_by_n("body");
        result.push_str(&rewrite_shape::<SwitchBlock>(&b, shape, false, context));

        try_add_standalone_suffix_no_semicolumn(node, &mut result, shape, &context.source_code);
        result
    }
}

impl<'a, 'tree> Rewrite for SwitchBlock<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, _, _) = self.prepare(context);

        result.push_str(" {\n");

        result.push_str(&node.apply_to_standalone_children(
            shape,
            context,
            |c, c_shape, c_context| rewrite::<SwitchRule>(&c, c_shape, c_context),
        ));

        add_indent(&mut result, shape, context);
        result.push('}');

        result
    }
}

impl<'a, 'tree> Rewrite for SwitchRule<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, _, _) = self.prepare(context);
        try_add_standalone_prefix(&mut result, shape, context);

        let s = node.c_by_k("switch_label");
        result.push_str(&rewrite::<SwitchLabel>(&s, shape, context));

        let b = node.c_by_k("block");
        result.push_str(&rewrite::<Block>(
            &b,
            &mut shape.clone_with_standalone(false),
            context,
        ));

        try_add_standalone_suffix_no_semicolumn(node, &mut result, shape, &context.source_code);
        result
    }
}

impl<'a, 'tree> Rewrite for SwitchLabel<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, _source_code, _) = self.prepare(context);

        if node.named_child_count() == 0 {
            result.push_str("when else");
        } else {
            result.push_str("when ");
            // NOTE. use has_comma flag as I can't differentiate delimeter `,` or ` `
            let has_comma = node.all_children_vec().iter().any(|c| c.kind() == ",");
            let delimeter = if has_comma { ", " } else { " " };

            // FIXME: I currently don't have brain power to narrow _visit()
            result.push_str(&node.apply_to_children_in_same_line(
                delimeter,
                shape,
                context,
                |c, c_shape, c_context| c._visit(c_shape, c_context),
            ));
        }
        result
    }
}

impl<'a, 'tree> Rewrite for StaticInitializer<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, _source_code, _) = self.prepare(context);
        try_add_standalone_prefix(&mut result, shape, context);
        result.push_str("static");
        result.push_str(&rewrite::<Block>(
            &node.first_c(),
            &mut shape.clone_with_standalone(false),
            context,
        ));
        result
    }
}

impl<'a, 'tree> Rewrite for InterfaceDeclaration<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);
        try_add_standalone_prefix(&mut result, shape, context);

        if let Some(ref a) = node.try_c_by_k("modifiers") {
            result.push_str(&rewrite::<Modifiers>(a, shape, context));

            if let Some(_) = a.try_c_by_k("modifier") {
                result.push(' ');
            }
        }

        result.push_str("interface ");

        let n = node.c_by_n("name");
        result.push_str(n.v(source_code));

        if let Some(ref c) = node.try_c_by_n("type_parameters") {
            result.push_str(&rewrite_shape::<TypeParameters>(c, shape, false, context));
        }

        if let Some(ref c) = node.try_c_by_k("extends_interfaces") {
            result.push_str(" extends ");
            result.push_str(&rewrite_shape::<TypeList>(
                &c.first_c(),
                shape,
                false,
                context,
            ));
        }

        let b = node.c_by_n("body");
        result.push_str(&rewrite::<Block>(
            &b,
            &mut shape.clone_with_standalone(false),
            context,
        ));

        add_standalone_suffix_no_semicolumn(&node, &mut result, source_code);
        result
    }
}

impl<'a, 'tree> Rewrite for ThrowStatement<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);
        try_add_standalone_prefix(&mut result, shape, context);

        result.push_str("throw ");
        result.push_str(&rewrite_shape::<Expression>(
            &node.first_c(),
            shape,
            false,
            context,
        ));

        try_add_standalone_suffix(node, &mut result, shape, source_code);
        result
    }
}

impl<'a, 'tree> Rewrite for BreakStatement<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);
        try_add_standalone_prefix(&mut result, shape, context);

        result.push_str("break");
        if let Some(c) = node.try_c_by_k("identifier") {
            result.push_str(" ");
            result.push_str(&c.v(source_code));
        }

        try_add_standalone_suffix(node, &mut result, shape, source_code);
        result
    }
}

impl<'a, 'tree> Rewrite for ContinueStatement<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);
        try_add_standalone_prefix(&mut result, shape, context);

        result.push_str("continue");
        if let Some(c) = node.try_c_by_k("identifier") {
            result.push_str(" ");
            result.push_str(&c.v(source_code));
        }

        try_add_standalone_suffix(node, &mut result, shape, source_code);
        result
    }
}

impl<'a, 'tree> Rewrite for TypeParameters<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, _, _) = self.prepare(context);

        result.push('<');
        let joined = node
            .cs_by_k("type_parameter")
            .iter()
            .map(|c| rewrite_shape::<TypeParameter>(&c, shape, false, context))
            .collect::<Vec<_>>()
            .join(", ");
        result.push_str(&joined);
        result.push('>');
        result
    }
}

impl<'a, 'tree> Rewrite for TypeParameter<'a, 'tree> {
    fn rewrite(&self, _shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);

        result.push_str(node.v(source_code));
        result
    }
}

impl<'a, 'tree> Rewrite for TypeList<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, _, _) = self.prepare(context);

        result.push_str(&node.first_c()._visit(shape, context));
        result
    }
}

impl<'a, 'tree> Rewrite for SmallCaseValue<'a, 'tree> {
    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let (node, mut result, source_code, _) = self.prepare(context);
        try_add_standalone_prefix(&mut result, shape, context);

        let value = node.v(source_code);
        result.push_str(&value.to_lowercase());

        try_add_standalone_suffix(node, &mut result, shape, &context.source_code);
        result
    }
}

//impl<'a, 'tree> Rewrite for FormalParameter<'a, 'tree> {
//    fn rewrite(&self, shape: &mut Shape, context: &FmtContext) -> String {
//        let (node, mut result, _source_code, _) = self.prepare(context);
//        result.push_str(&node.apply_to_children_in_same_line(
//            " ",
//            shape,
//            context,
//            |c, c_shape, c_context| c._visit(c_shape, c_context),
//        ));
//        result
//        //let type_str = n.cv_by_n("type", source_code);
//        //let name_str = n.cv_by_n("name", source_code);
//        //format!("{} {}", type_str, name_str)
//    }
//}
