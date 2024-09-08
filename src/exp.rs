use crate::context::FmtContext;
use crate::node_ext::*;
use crate::node_struct::*;
use crate::shape::Shape;
use crate::utility::*;
use anyhow::{Context, Result};
use log::debug;

// TODO
impl<'a, 'tree> Rewrite for Expression<'a, 'tree> {
    fn rewrite_result(&self, context: &FmtContext, shape: &mut Shape) -> Result<String> {
        let n = self.as_ast_node();
        match n.kind() {
            "binary_expression" => {
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
            "int" => Ok(n.get_value(context.source_code).to_string()),

            v => {
                eprintln!("### Unknow Expression node: {}", v);
                unreachable!();
            }
        }
    }
}
