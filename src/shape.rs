#[derive(Default, Clone)]
pub struct Shape {
    pub block_indent: usize,
}

impl Shape {
    pub fn new(block_indent: usize) -> Self {
        Self { block_indent }
    }
}
