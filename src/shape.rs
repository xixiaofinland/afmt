use crate::config::{Config, Indent};

#[derive(Clone)]
pub struct Shape {
    pub indent: Indent,
    pub width: usize,  //TODO
    pub offset: usize, //TODO
}

impl Shape {
    pub fn new(indent: Indent) -> Self {
        Self {
            indent,
            width: 1,
            offset: 1,
        }
    }

    pub fn empty() -> Self {
        Self {
            indent: Indent::new(0, 0),
            width: 1,
            offset: 1,
        }
    }

    pub fn indented(indent: Indent, config: &Config) -> Shape {
        Shape {
            width: config.max_width,
            indent,
            offset: indent.alignment,
        }
    }

    pub fn increase_indent(s: &Shape) -> Self {
        Shape::new(Indent::new(s.indent.block_indent + 1, 0))
    }
}
