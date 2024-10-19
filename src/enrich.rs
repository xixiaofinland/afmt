use std::fmt::Debug;
use tree_sitter::Node;

use crate::config::Config;

trait RichNode: Debug {
    fn enrich(&mut self);
    //fn enrich_comments(&mut self);
    //fn enrich_data(&mut self);
    //fn rewrite(&mut self) -> String;
}

#[derive(Debug, Default)]
struct FormattingInfo {
    pub wrappable: bool,
    pub indent_level: usize,
    pub force_break_before: bool,
    pub force_break_after: bool,
    pub offset: usize,
}

#[derive(Debug, Default)]
struct CommentBuckets {
    pub pre_comments: Vec<Comment>,
    pub inline_comments: Vec<Comment>,
    pub post_comments: Vec<Comment>,
}

#[derive(Debug)]
struct Comment {
    pub content: String,
    pub comment_type: CommentType,
}

#[derive(Debug)]
enum CommentType {
    Line,
    Block,
}

#[derive(Debug)]
pub struct EContext {
    pub config: Config,
    pub source_code: String,
}

impl EContext {
    pub fn new(config: &Config, source_code: &str) -> Self {
        let config = config.clone();
        let source_code = String::from(source_code);
        Self {
            config,
            source_code,
        }
    }
}

#[derive(Debug)]
pub struct ClassDNode<'a, 'tree> {
    pub inner: &'a Node<'tree>,
    pub content: String,
    pub comments: CommentBuckets,
    pub children: Vec<Box<dyn RichNode>>,
    pub field_name: Option<String>,
    pub formatting_info: FormattingInfo,
}

impl<'a, 'tree> ClassDNode<'a, 'tree> {
    pub fn new(inner: &'a Node<'tree>) -> Self {
        Self {
            inner,
            content: String::new(),
            comments: CommentBuckets::default(),
            children: Vec::new(),
            field_name: None,
            formatting_info: FormattingInfo::default(),
        }
    }
}

impl<'a, 'tree> RichNode for ClassDNode<'a, 'tree> {
    fn enrich(&mut self) {}
}

//impl Comment<'a, 'tree> {
//    pub fn from_node(node: &'a Node<'tree>) -> Self {
//        let content = node.v();
//    }
//}
//
//impl RichNode<'a, 'tree> {
//    pub fn new(inner: &'a Node<'tree>) -> Self {
//        Self {
//            inner,
//            ..Default::default()
//        }
//    }
//
//    fn enrich(&mut self) {
//        //self.enrich_comments(source_code);
//        self.enrich_data();
//
//        for c in self.inner.named_children() {
//            //c.enrich();
//        }
//    }
//
//    fn enrich_comments(&mut self) {
//        //let mut prev = self.inner.prev_sibling();
//        //while let Some(c) = prev {
//        //    if c.kind() == "line_comment" {
//        //        self.comments.pre_comments.push(Comment::from_node(&c));
//        //    }
//        //    prev = c.prev_sibling();
//        //}
//    }
//
//    fn enrich_data(&mut self) {
//        self.formatting_info = match self.inner.kind() {
//            "class_declaration" => FormattingInfo {
//                ..Default::default()
//            },
//
//            "method_declaration" => FormattingInfo {
//                wrappable: true,
//                ..Default::default()
//            },
//        }
//    }
//
//    fn rewrite(&self) -> String {
//        let mut result = String::new();
//
//        // Handle pre-comments
//        for comment in &self.comments.pre_comments {
//            //result.push_str(&comment.format(shape));
//        }
//
//        // Add content
//        result.push_str(&self.content);
//
//        // Handle inline comments...
//        // Handle children...
//        // Handle post comments...
//
//        result
//    }
//}
