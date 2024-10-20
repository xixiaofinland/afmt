use crate::accessor::Accessor;
use crate::context::FmtContext;
use crate::rewrite::Rewrite;
use crate::route::COMMON_MAP;
use crate::shape::Shape;
use crate::static_routing;
use crate::struct_def::{BlockComment, LineComment};
use crate::utility::rewrite;
use colored::Colorize;
#[allow(unused_imports)]
use log::debug;
use tree_sitter::Node;

#[allow(dead_code)]
pub trait Visitor<'tree> {
    fn apply_to_standalone_children<F>(&self, shape: &Shape, context: &FmtContext, f: F) -> String
    where
        F: FnMut(&Node<'tree>, &mut Shape, &FmtContext) -> String;

    fn apply_to_children_in_same_line<F>(
        &self,
        delimiter: &str,
        shape: &mut Shape,
        context: &FmtContext,
        f: F,
    ) -> String
    where
        F: FnMut(&Node<'tree>, &mut Shape, &FmtContext) -> String;

    fn try_visit_cs_by_k(&self, kind: &str, context: &FmtContext, shape: &mut Shape)
        -> Vec<String>;
    fn try_visit_cs(&self, context: &FmtContext, shape: &mut Shape) -> Vec<String>;
    fn _visit(&self, shape: &mut Shape, context: &FmtContext) -> String;
}

impl<'tree> Visitor<'tree> for Node<'tree> {
    fn _visit(&self, shape: &mut Shape, context: &FmtContext) -> String {
        let mut result = String::new();
        result.push_str(&static_routing!(COMMON_MAP, self, context, shape));
        result
    }

    fn apply_to_standalone_children<F>(
        &self,
        shape: &Shape,
        context: &FmtContext,
        mut f: F,
    ) -> String
    where
        F: FnMut(&Node<'tree>, &mut Shape, &FmtContext) -> String,
    {
        let mut result = String::new();
        let shape_base = shape.clone_with_indent_increase(context.config);
        let mut cursor = self.walk();

        let children = self
            .named_children(&mut cursor)
            .map(|child| {
                let mut c_shape = shape_base.clone_with_standalone(true);
                match child.kind() {
                    "line_comment" => rewrite::<LineComment>(&child, &mut c_shape, context),
                    "block_comment" => rewrite::<BlockComment>(&child, &mut c_shape, context),
                    _ => f(&child, &mut c_shape, context),
                }
            })
            .collect::<Vec<_>>()
            .join("");

        if !children.is_empty() {
            result.push_str(&children);
            result.push('\n');
        }
        result
    }

    fn apply_to_children_in_same_line<F>(
        &self,
        delimiter: &str,
        shape: &mut Shape,
        context: &FmtContext,
        mut f: F,
    ) -> String
    where
        F: FnMut(&Node<'tree>, &mut Shape, &FmtContext) -> String,
    {
        let mut result = String::new();
        let mut cursor = self.walk();

        let fields = self
            .named_children(&mut cursor)
            .map(|child| {
                let mut c_shape = shape.clone_with_standalone(false);
                match child.kind() {
                    "line_comment" => rewrite::<LineComment>(&child, &mut c_shape, context),
                    "block_comment" => rewrite::<BlockComment>(&child, &mut c_shape, context),
                    _ => f(&child, &mut c_shape, context),
                }
            })
            .collect::<Vec<_>>()
            .join(delimiter);

        result.push_str(&fields);
        result
    }

    fn try_visit_cs(&self, context: &FmtContext, shape: &mut Shape) -> Vec<String> {
        let mut cursor = self.walk();
        self.named_children(&mut cursor)
            .map(|n| n._visit(shape, context))
            .collect::<Vec<_>>()
    }

    fn try_visit_cs_by_k(
        &self,
        kind: &str,
        context: &FmtContext,
        shape: &mut Shape,
    ) -> Vec<String> {
        self.try_cs_by_k(kind)
            .iter()
            .map(|n| n._visit(shape, context))
            .collect::<Vec<_>>()
    }
}
