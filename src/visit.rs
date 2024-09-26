use crate::child::Accessor;
use crate::context::FmtContext;
use crate::rewrite::Rewrite;
use crate::route::COMMON_MAP;
use crate::shape::Shape;
use crate::static_routing;
use colored::Colorize;
#[allow(unused_imports)]
use log::debug;
use tree_sitter::Node;

#[allow(dead_code)]
pub trait Visitor<'tree> {
    fn apply_to_standalone_children<F>(&self, context: &FmtContext, shape: &Shape, f: F) -> String
    where
        F: FnMut(&Node<'tree>, &FmtContext, &mut Shape) -> String;

    fn apply_to_children_in_same_line<F>(
        &self,
        delimiter: &str,
        context: &FmtContext,
        shape: &mut Shape,
        f: F,
    ) -> String
    where
        F: FnMut(&Node<'tree>, &FmtContext, &mut Shape) -> String;

    fn try_visit_cs_by_k(&self, kind: &str, context: &FmtContext, shape: &mut Shape)
        -> Vec<String>;
    fn try_visit_cs(&self, context: &FmtContext, shape: &mut Shape) -> Vec<String>;
    fn _visit(&self, context: &FmtContext, shape: &mut Shape) -> String;
}

impl<'tree> Visitor<'tree> for Node<'tree> {
    fn _visit(&self, context: &FmtContext, shape: &mut Shape) -> String {
        let mut result = String::new();
        result.push_str(&static_routing!(COMMON_MAP, self, context, shape));
        result
    }

    fn apply_to_standalone_children<F>(
        &self,
        context: &FmtContext,
        shape: &Shape,
        mut f: F,
    ) -> String
    where
        F: FnMut(&Node<'tree>, &FmtContext, &mut Shape) -> String,
    {
        let mut result = String::new();
        let shape_base = shape.copy_with_indent_increase(context.config);
        let mut cursor = self.walk();

        let children = self
            .named_children(&mut cursor)
            .map(|child| {
                let mut c_shape = shape_base.clone_with_standalone(true);
                f(&child, context, &mut c_shape)
            })
            .collect::<Vec<_>>()
            .join("\n");

        if !children.is_empty() {
            result.push_str(&children);
            result.push('\n');
        }
        result
    }

    fn apply_to_children_in_same_line<F>(
        &self,
        delimiter: &str,
        context: &FmtContext,
        shape: &mut Shape,
        mut f: F,
    ) -> String
    where
        F: FnMut(&Node<'tree>, &FmtContext, &mut Shape) -> String,
    {
        let mut result = String::new();
        let mut cursor = self.walk();

        let fields = self
            .named_children(&mut cursor)
            .map(|child| {
                let mut child_shape = shape.clone_with_standalone(false);
                f(&child, context, &mut child_shape)
            })
            .collect::<Vec<_>>()
            .join(delimiter);

        result.push_str(&fields);
        result
    }

    fn try_visit_cs(&self, context: &FmtContext, shape: &mut Shape) -> Vec<String> {
        let mut cursor = self.walk();
        self.named_children(&mut cursor)
            .map(|n| n._visit(context, shape))
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
            .map(|n| n._visit(context, shape))
            .collect::<Vec<_>>()
    }
}
