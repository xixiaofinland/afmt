use crate::shape::Shape;

pub fn get_indent(shape: &Shape) -> String {
    let indent = "  ".repeat(shape.block_indent);
    indent
}

pub fn indent_lines(prepared_code: &str, shape: &Shape) -> String {
    let indent = get_indent(shape);
    //println!("shape:{}|", shape.block_indent);
    //println!("indent:{}|", indent);

    let lines: Vec<&str> = prepared_code.split('\n').collect();
    let indented_lines: Vec<String> = lines
        .iter()
        .map(|line| format!("{}{}", indent, line))
        .collect();
    indented_lines.join("\n")
}
