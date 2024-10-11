use crate::config::Config;

#[derive(Clone, Debug)]
pub struct Shape {
    pub indent: Indent,
    pub width: usize, // width = max_width - indent_width;
    pub offset: usize,
    pub standalone: bool,
    pub can_split: bool, // is it possible to switch to multi-line mode;
}

impl Shape {
    //pub fn new(indent: Indent, config: &Config) -> Self {
    //    Self {
    //        indent,
    //        width: config
    //            .max_width()
    //            .saturating_sub(indent.block_indent * config.indent_size()),
    //        offset: 0,
    //        standalone: false,
    //    }
    //}

    pub fn clone_with_standalone(&self, stand_alone: bool) -> Self {
        Self {
            indent: self.indent,
            width: self.width,
            offset: self.offset,
            standalone: stand_alone,
            can_split: self.can_split,
        }
    }

    pub fn empty(config: &Config) -> Self {
        Self {
            indent: Indent::default(),
            width: config.max_width(),
            offset: 0,
            standalone: true,
            can_split: true,
        }
    }

    //pub fn stand_alone(&mut self, flag: bool) {
    //    self.standalone = flag;
    //}

    pub fn copy_with_indent_increase(&self, config: &Config) -> Self {
        let indent = self.indent.copy_with_increased_block_indent();
        let offset = indent.block_indent * config.indent_size();
        let width = config.max_width().saturating_sub(offset);
        let standalone = self.standalone;
        let can_split = self.can_split;

        Self {
            indent,
            width,
            offset,
            standalone,
            can_split,
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
