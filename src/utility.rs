use crate::{context::FmtContext, shape::Shape};

pub fn try_add_standalone_prefix(result: &mut String, shape: &Shape, context: &FmtContext) {
    if shape.standalone {
        add_indent(result, shape, context);
    }
}

pub fn try_add_standalone_suffix(result: &mut String, shape: &Shape) {
    if shape.standalone {
        result.push(';');
    }
}

pub fn add_indent(result: &mut String, shape: &Shape, context: &FmtContext) {
    result.push_str(&shape.indent.as_string(context.config));
}
