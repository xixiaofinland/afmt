use crate::config::Config;

#[derive(Clone, Debug)]
pub struct Shape {
    pub indent: Indent,
    pub offset: usize, // space already taken in the line;
    pub standalone: bool,
    pub single_line_only: bool, // no need to do multi-line split check anymore;
                                //pub width: usize,  // width = max_width - indent_width;
}

impl Shape {
    pub fn clone_with_standalone(&self, stand_alone: bool) -> Self {
        Self {
            indent: self.indent,
            offset: self.offset,
            standalone: stand_alone,
            single_line_only: self.single_line_only,
            //width: self.width,
        }
    }

    pub fn empty() -> Self {
        Self {
            indent: Indent::default(),
            offset: 0,
            standalone: true,
            single_line_only: false,
            //width: config.max_width(),
        }
    }

    pub fn clone_with_indent_increase(&self) -> Self {
        let indent = self.indent.copy_with_increased_block_indent();
        let offset = self.offset;
        //let offset = indent.block_indent * config.indent_size();
        let standalone = self.standalone;
        let can_split = self.single_line_only;
        //let width = config.max_width().saturating_sub(offset);

        Self {
            indent,
            offset,
            standalone,
            single_line_only: can_split,
            //width,
        }
    }

    pub fn single_line_only(mut self, flag: bool) -> Self {
        self.single_line_only = flag;
        self
    }

    pub fn standalone(mut self, flag: bool) -> Self {
        self.standalone = flag;
        self
    }

    pub fn indent_len(&self, config: &Config) -> usize {
        self.indent.block_indent * config.indent_size()
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
