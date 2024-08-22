use crate::utility::NodeUtilities;
use crate::utility::*;
use anyhow::{anyhow, Context, Result};
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
    fn rewrite(&self) -> Option<String>;

    //fn rewrite_result(&self) -> RewriteResult {
    //    self.rewrite(context, shape).unknown_error()
    //}
}

pub struct Class<'a> {
    inner: &'a Node<'a>,
}

impl<'a> Class<'a> {
    pub fn new(node: &'a Node) -> Self {
        Class { inner: node }
    }

    pub fn as_ast_node(&self) -> &'a Node {
        self.inner
    }

    fn get_modifiers(&self) -> Result<()> {
        let modifiers_node = self
            .inner
            .get_child_by_kind("modifiers")
            .ok_or(anyhow!("no modifiers node found."))?;

        let modifiers = self.inner.get_children_by_kind("modifier");
        println!("modifiers: {:?}", modifiers);
        Ok(())
    }
}

impl<'a> Rewrite for Class<'a> {
    fn rewrite(&self) -> Option<String> {
        let node = self.inner;
        let t = self.inner.get_child_by_kind("modifiers");
        println!("test: {:?}", t);

        Some(String::new())
    }
}

//pub struct Method<'a> {
//    inner: &'a Node<'a>,
//}
//
//impl<'a> Method<'a> {
//    pub fn new(node: &'a Node) -> Self {
//        Method { inner: node }
//    }
//}
//
//impl<'a> Rewrite for Method<'a> {
//    fn rewrite(&self) -> Option<String> {
//        Some(String::new())
//    }
//}
