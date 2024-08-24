use crate::extension::NodeUtilities;
use crate::shape::Shape;
use crate::utility::{get_indent, get_source_code, indent_lines};
use crate::visitor::walk;
use tree_sitter::Node;

#[derive(Debug)]
pub enum NodeKind {
    ClassDeclaration,
    FieldDeclaration,
    MethodDeclaration,
    IfStatement,
    ForLoop,
    Unknown,
}

impl NodeKind {
    pub fn from_kind(kind: &str) -> NodeKind {
        match kind {
            "class_declaration" => NodeKind::ClassDeclaration,
            "field_declaration" => NodeKind::FieldDeclaration,
            "method_declaration" => NodeKind::MethodDeclaration,
            "if_statement" => NodeKind::IfStatement,
            "for_statement" => NodeKind::ForLoop,
            _ => {
                println!("Unknown node kind: {}", kind);
                NodeKind::Unknown
            }
        }
    }
}

pub trait Rewrite {
    fn rewrite(&self) -> Option<String>;

    //fn rewrite_result(&self) -> RewriteResult {
    //    self.rewrite(context, shape).unknown_error()
    //}
}

pub struct Class<'a, 'b, 'tree> {
    inner: &'a Node<'tree>,
    shape: &'b Shape,
}

impl<'a, 'b, 'tree> Class<'a, 'b, 'tree> {
    pub fn new(node: &'a Node<'tree>, shape: &'b Shape) -> Self {
        Self { inner: node, shape }
    }

    pub fn as_ast_node(&self) -> &'a Node<'tree> {
        self.inner
    }

    pub fn get_modifiers(&self) -> Vec<Node<'tree>> {
        if let Some(n) = self.as_ast_node().get_child_by_kind("modifiers") {
            n.get_children_by_kind("modifier")
        } else {
            Vec::new()
        }
    }

    pub fn format_body(&self) -> Option<String> {
        let mut result = String::new();
        let body_node = self.as_ast_node().child_by_field_name("body")?;
        result.push_str(&walk(&body_node, &self.shape)?);
        Some(result)
    }
}

impl<'a, 'b, 'tree> Rewrite for Class<'a, 'b, 'tree> {
    fn rewrite(&self) -> Option<String> {
        let modifier_nodes = self.get_modifiers();
        let modifiers_doc = modifier_nodes
            .iter()
            .map(|n| {
                n.utf8_text(get_source_code().as_bytes())
                    .ok()
                    .unwrap_or_default()
            })
            .collect::<Vec<&str>>()
            .join(" ");

        let mut result = String::new();
        result.push_str(&modifiers_doc);
        result.push(' ');

        let name_node = self.as_ast_node().child_by_field_name("name")?;
        let name_node_value = name_node.utf8_text(get_source_code().as_bytes()).ok()?;

        result.push_str(name_node_value);
        result.push_str(" {\n");

        result.push_str(&self.format_body()?);

        result.push('}');

        let result = indent_lines(&result, self.shape);

        println!("class result:\n{}", result);
        Some(result)
    }
}

pub struct FieldDeclaration<'a, 'b, 'tree> {
    inner: &'a Node<'tree>,
    shape: &'b Shape,
}

impl<'a, 'b, 'tree> FieldDeclaration<'a, 'b, 'tree> {
    pub fn new(node: &'a Node<'tree>, shape: &'b Shape) -> Self {
        Self { inner: node, shape }
    }

    pub fn as_ast_node(&self) -> &'a Node<'tree> {
        self.inner
    }

    pub fn get_modifiers(&self) -> Vec<Node<'tree>> {
        if let Some(n) = self.as_ast_node().get_child_by_kind("modifiers") {
            n.get_children_by_kind("modifier")
        } else {
            Vec::new()
        }
    }
}

impl<'a, 'b, 'tree> Rewrite for FieldDeclaration<'a, 'b, 'tree> {
    fn rewrite(&self) -> Option<String> {
        let modifier_nodes = self.get_modifiers();
        let modifiers_doc = modifier_nodes
            .iter()
            .map(|n| {
                n.utf8_text(get_source_code().as_bytes())
                    .ok()
                    .unwrap_or_default()
            })
            .collect::<Vec<&str>>()
            .join(" ");

        let mut result = String::new();
        result.push_str(&modifiers_doc);

        result.push(' ');

        let type_node = self.as_ast_node().child_by_field_name("type")?;
        let type_node_value = type_node.utf8_text(get_source_code().as_bytes()).ok()?;
        result.push_str(type_node_value);

        result.push(' ');

        let name_node = self
            .as_ast_node()
            .child_by_field_name("declarator")?
            .child_by_field_name("name")?;
        let name_node_value = name_node.utf8_text(get_source_code().as_bytes()).ok()?;
        result.push_str(name_node_value);

        result.push('\n');

        let mut result = indent_lines(&result, self.shape);
        result.push('\n');

        Some(result)
    }
}
