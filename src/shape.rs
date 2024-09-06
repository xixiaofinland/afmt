use crate::config::Config;

#[derive(Clone)]
pub struct Shape {
    pub indent: Indent,
    pub width: usize, // width = max_width - indent_width;
    pub offset: usize,
}

impl Shape {
    pub fn new(indent: Indent, config: &Config) -> Self {
        Self {
            indent,
            width: config
                .max_width()
                .saturating_sub(indent.block_indent * config.indent_size()),
            offset: 0,
        }
    }

    pub fn empty(config: &Config) -> Self {
        Self {
            indent: Indent::default(),
            width: config.max_width(),
            offset: 0,
        }
    }

    pub fn copy_with_indent_block_plus(&self, config: &Config) -> Self {
        let indent = self.indent.copy_with_increased_block_indent();
        let offset = indent.block_indent * config.indent_size();
        let width = config.max_width().saturating_sub(offset);

        Self {
            indent,
            width,
            offset,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Indent {
    pub block_indent: usize,
    //pub alignment: usize,
}

impl Indent {
    pub fn default() -> Indent {
        Indent::new(0)
    }

    pub fn new(block_indent: usize) -> Indent {
        Indent { block_indent }
    }

    pub fn copy_with_increased_block_indent(&self) -> Self {
        Self {
            block_indent: self.block_indent + 1,
        }
    }

    pub fn to_string(&self) -> String {
        "  ".repeat(self.block_indent)
    }
}
