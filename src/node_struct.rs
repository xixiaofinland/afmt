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
    fn rewrite(&self, shape: &Shape) -> Option<String>;

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
    fn rewrite(&self, shape: &Shape) -> Option<String> {
        let modifier_nodes = self.get_modifiers();
        println!("t: {}", modifier_nodes.len());
        let result = modifier_nodes
            .iter()
            .map(|n| n.to_sexp())
            .collect::<Vec<String>>()
            .join("\n");
        Some(result)
    }
}
