use crate::context::FmtContext;
use crate::node_ext::*;
use crate::node_struct::{BinaryExpression, Rewrite};
use crate::shape::Shape;
use crate::utility::*;
use anyhow::{Context, Result};
use log::debug;

impl<'a, 'tree> Rewrite for BinaryExpression<'a, 'tree> {
    fn rewrite_result(&self, context: &FmtContext, shape: &mut Shape) -> Result<String> {
        let left = self
            .as_ast_node()
            .get_mandatory_child_value_by_name("left", context.source_code);

        let op = self
            .as_ast_node()
            .get_mandatory_child_value_by_name("operator", context.source_code);

        let right = self
            .as_ast_node()
            .get_mandatory_child_value_by_name("right", context.source_code);

        let result = format!("{} {} {}", left, op, right);
        Ok(result)
    }
}
