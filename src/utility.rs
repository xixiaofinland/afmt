use crate::shape::Shape;

pub fn get_indent(shape: &Shape) -> String {
    let indent = "  ".repeat(shape.block_indent);
    indent
}
