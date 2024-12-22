use std::collections::HashMap;

pub type CommentMap = HashMap<usize, NodeComment>;

#[derive(Debug)]
pub struct NodeComment {
    pub pre_comments: Vec<usize>,
    pub post_comments: Vec<usize>,
    pub dangling_comments: Vec<usize>,
}

impl NodeComment {
    pub fn new() -> Self {
        Self {
            pre_comments: Vec::new(),
            post_comments: Vec::new(),
            dangling_comments: Vec::new(),
        }
    }
}
