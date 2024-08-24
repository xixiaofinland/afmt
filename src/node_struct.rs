use crate::context::Context;
use crate::shape::Shape;
use crate::utility::NodeUtilities;
use tree_sitter::Node;

#[derive(Debug)]
pub enum NodeKind {
    ClassDeclaration,
    MethodDeclaration,
    IfStatement,
    ForLoop,
    Unknown,
}

impl NodeKind {
    pub fn from_kind(kind: &str) -> NodeKind {
        match kind {
            "class_declaration" => NodeKind::ClassDeclaration,
            "method_declaration" => NodeKind::MethodDeclaration,
            "if_statement" => NodeKind::IfStatement,
            "for_statement" => NodeKind::ForLoop,
            _ => NodeKind::Unknown,
        }
    }
}

pub trait Rewrite {
    fn rewrite(&self, shape: &Shape, context: &Context) -> Option<String>;

    //fn rewrite_result(&self) -> RewriteResult {
    //    self.rewrite(context, shape).unknown_error()
    //}
}

pub struct Class<'a, 'tree> {
    inner: &'a Node<'tree>,
}

impl<'a, 'tree> Class<'a, 'tree> {
    pub fn new(node: &'a Node<'tree>) -> Self {
        Class { inner: node }
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

impl<'a, 'tree> Rewrite for Class<'a, 'tree> {
    fn rewrite(&self, shape: &Shape, context: &Context) -> Option<String> {
        let mut result = String::new();

        let modifier_nodes = self.get_modifiers();
        let value = modifier_nodes[1]
            .utf8_text(context.source_code.as_bytes())
            .ok()?;

        let modifiers_doc = modifier_nodes
            .iter()
            .map(|n| {
                n.utf8_text(context.source_code.as_bytes())
                    .ok()
                    .unwrap_or_default()
            })
            .collect::<Vec<&str>>()
            .join(" ");

        result.push_str(&modifiers_doc);

        result.push(' ');

        let name_node = self.as_ast_node().child_by_field_name("name")?;
        let name_node_value = name_node.utf8_text(context.source_code.as_bytes()).ok()?;

        result.push_str(name_node_value);
        result.push_str(" {\n");
        result.push('}');

        println!("result: {}", result);
        Some(result)
    }
}
