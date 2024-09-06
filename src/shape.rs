use crate::config::Config;

#[derive(Clone)]
pub struct Shape {
    pub indent: Indent,
    pub width: usize, // width = max_width - indent_width;
                      //pub offset: usize,
}

impl Shape {
    pub fn new(indent: Indent, width: usize, offset: usize) -> Self {
        Self { indent, width }
    }

    pub fn empty() -> Self {
        Self {
            indent: Indent::new(0, 0),
            width: 1,
        }
    }

    pub fn indented(&self, indent: Indent, config: &Config) -> Shape {
        Shape {
            width: config
                .max_width()
                .saturating_sub(&self.indent.block_indent * config.indent_size()),
            indent,
        }
    }

    pub fn increase_indent(s: &Shape) -> Self {
        Shape::new(Indent::new(s.indent.block_indent + 1, 0), 1, 1)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Indent {
    pub block_indent: usize,
    pub alignment: usize,
}

impl Indent {
    pub fn default() -> Indent {
        Indent::new(0, 0)
    }

    pub fn new(block_indent: usize, alignment: usize) -> Indent {
        Indent {
            block_indent,
            alignment,
        }
    }

    pub fn to_string(&self) -> String {
        "  ".repeat(self.block_indent)
    }
}
