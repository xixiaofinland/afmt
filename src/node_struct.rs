use crate::config::{Context, Shape};
use crate::utility::{get_modifiers, get_parameters, indent_lines};
use crate::visitor::Visitor;
use crate::{define_node, define_nodes};
use anyhow::{Context as AnyhowContext, Result};
use tree_sitter::Node;

pub trait Rewrite {
    fn rewrite(&self, context: &Context) -> Option<String> {
        self.rewrite_result(context).ok()
    }

    fn rewrite_result(&self, context: &Context) -> Result<String>;
}

define_nodes!(
    ClassDeclaration => "class_declaration",
    FieldDeclaration => "field_declaration",
    MethodDeclaration => "method_declaration"
);

impl<'a, 'b, 'tree> ClassDeclaration<'a, 'b, 'tree> {
    pub fn format_body(&self, context: &Context) -> Result<String> {
        let mut result = String::new();
        let body_node = self
            .as_ast_node()
            .child_by_field_name("body")
            .context("mandatory body field missing")?;

        let visitor = Visitor::default();
        result.push_str(
            &visitor
                .walk(&body_node, context, &self.shape)
                .context("walk() failed")?,
        );
        Ok(result)
    }
}

impl<'a, 'b, 'tree> Rewrite for ClassDeclaration<'a, 'b, 'tree> {
    fn rewrite_result(&self, context: &Context) -> Result<String> {
        let modifier_nodes = get_modifiers(self.as_ast_node());
        let modifiers_doc = modifier_nodes
            .iter()
            .map(|n| {
                n.utf8_text(context.source_code.as_bytes())
                    .ok()
                    .unwrap_or_default()
            })
            .collect::<Vec<&str>>()
            .join(" ");

        let mut result = String::new();
        result.push_str(&modifiers_doc);
        result.push(' ');

        let name_node = self
            .as_ast_node()
            .child_by_field_name("name")
            .context("mandatory name field missing")?;
        let name_node_value = name_node.utf8_text(context.source_code.as_bytes())?;

        result.push_str(name_node_value);
        result.push_str(" {\n");

        result.push_str(&self.format_body(context)?);

        result.push('}');
        let result = indent_lines(&result, self.shape);
        Ok(result)
    }
}

impl<'a, 'b, 'tree> Rewrite for FieldDeclaration<'a, 'b, 'tree> {
    fn rewrite_result(&self, context: &Context) -> Result<String> {
        let modifier_nodes = get_modifiers(self.as_ast_node());
        let modifiers_doc = modifier_nodes
            .iter()
            .map(|n| {
                n.utf8_text(context.source_code.as_bytes())
                    .ok()
                    .unwrap_or_default()
            })
            .collect::<Vec<&str>>()
            .join(" ");

        let mut result = String::new();
        result.push_str(&modifiers_doc);

        result.push(' ');

        let type_node = self
            .as_ast_node()
            .child_by_field_name("type")
            .context("mandatory type field missing")?;
        let type_node_value = type_node.utf8_text(context.source_code.as_bytes())?;
        result.push_str(type_node_value);

        result.push(' ');

        let name_node = self
            .as_ast_node()
            .child_by_field_name("declarator")
            .context("mandatory declarator field missing")?
            .child_by_field_name("name")
            .context("mandatory name field missing")?;
        let name_node_value = name_node.utf8_text(context.source_code.as_bytes())?;
        result.push_str(name_node_value);

        result.push(';');
        result.push('\n');

        let mut result = indent_lines(&result, self.shape);
        result.push('\n');

        Ok(result)
    }
}

impl<'a, 'b, 'tree> Rewrite for MethodDeclaration<'a, 'b, 'tree> {
    fn rewrite_result(&self, context: &Context) -> Result<String> {
        let modifier_nodes = get_modifiers(self.as_ast_node());
        let modifiers_doc = modifier_nodes
            .iter()
            .map(|n| {
                n.utf8_text(context.source_code.as_bytes())
                    .ok()
                    .unwrap_or_default()
            })
            .collect::<Vec<&str>>()
            .join(" ");

        let mut result = String::new();
        result.push_str(&modifiers_doc);

        result.push(' ');

        let type_node = self
            .as_ast_node()
            .child_by_field_name("type")
            .context("mandatory type field missing")?;
        let type_node_value = type_node.utf8_text(context.source_code.as_bytes())?;
        result.push_str(type_node_value);

        result.push(' ');

        let name_node = self
            .as_ast_node()
            .child_by_field_name("name")
            .context("mandatory name field missing")?;
        let name_node_value = name_node.utf8_text(context.source_code.as_bytes())?;
        result.push_str(name_node_value);

        result.push('(');
        let parameters_node = get_parameters(self.as_ast_node());
        let parameters_doc = parameters_node
            .iter()
            .map(|n| {
                let type_node = n.child_by_field_name("type").unwrap();
                let name_node = n.child_by_field_name("name").unwrap();
                let type_str = type_node
                    .utf8_text(context.source_code.as_bytes())
                    .ok()
                    .unwrap();
                let name_str = name_node
                    .utf8_text(context.source_code.as_bytes())
                    .ok()
                    .unwrap();
                let r = format!("{} {}", type_str, name_str);
                r
            })
            .collect::<Vec<String>>()
            .join(", ");

        result.push_str(&parameters_doc);

        result.push(')');

        result.push(';');
        result.push('\n');

        let mut result = indent_lines(&result, self.shape);
        result.push('\n');

        Ok(result)
    }
}
