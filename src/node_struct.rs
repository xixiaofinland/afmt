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

//https://github.com/dangmai/prettier-plugin-apex/blob/60db6549a441911a0ef25b0ecc5e61727dc92fbb/packages/prettier-plugin-apex/src/printer.ts#L612
impl<'a, 'tree> Rewrite for Class<'a, 'tree> {
    fn rewrite(&self, shape: &Shape, context: &Context) -> Option<String> {
        let modifier_nodes = self.get_modifiers();
        let value = modifier_nodes[0]
            .utf8_text(context.source_code.as_bytes())
            .ok()?;

        println!("value: {}", value);

        let result = modifier_nodes
            .iter()
            .map(|n| n.to_sexp())
            .collect::<Vec<String>>()
            .join("\n");
        Some(result)
    }
}
