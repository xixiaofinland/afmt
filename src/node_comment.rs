use std::collections::HashMap;
use tree_sitter::Node;

pub type CommentMap = HashMap<usize, NodeComment>;

#[derive(Debug)]
pub struct NodeComment {
    pub node_id: usize,
    pub pre_comments: Vec<usize>,
    pub post_comments: Vec<usize>,
    pub dangling_comments: Vec<usize>,
}

impl NodeComment {
    pub fn new(node_id: usize) -> Self {
        Self {
            node_id,
            pre_comments: vec![],
            post_comments: vec![],
            dangling_comments: vec![],
        }
    }
}

