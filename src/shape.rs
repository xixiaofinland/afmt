use crate::config::Config;

#[derive(Clone, Debug)]
pub struct Shape {
    pub indent: Indent,
    pub width: usize,  // width = max_width - indent_width;
    pub offset: usize, // space already taken in the line;
    pub standalone: bool,
    pub single_only: bool, // is it possible to switch to multi-line mode;
}

impl Shape {
    pub fn clone_with_standalone(&self, stand_alone: bool) -> Self {
        Self {
            indent: self.indent,
            width: self.width,
            offset: self.offset,
            standalone: stand_alone,
            single_only: self.single_only,
        }
    }

    pub fn empty(config: &Config) -> Self {
        Self {
            indent: Indent::default(),
            width: config.max_width(),
            offset: 0,
            standalone: true,
            single_only: true,
        }
    }

    pub fn copy_with_indent_increase(&self, config: &Config) -> Self {
        let indent = self.indent.copy_with_increased_block_indent();
        let offset = indent.block_indent * config.indent_size();
        let width = config.max_width().saturating_sub(offset);
        let standalone = self.standalone;
        let can_split = self.single_only;

        Self {
            indent,
            width,
            offset,
            standalone,
            single_only: can_split,
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

    pub fn as_string(&self, config: &Config) -> String {
        " ".repeat(config.indent_size).repeat(self.block_indent)
    }
}
