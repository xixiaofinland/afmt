use tree_sitter::Node;

#[derive(Debug)]
struct RichNode<'a, 'tree> {
    pub content: String,
    pub comments: CommentBuckets,
    pub children: Vec<RichNode<'a, 'tree>>,
    pub formatting_info: FormattingInfo,
    pub inner: &'a Node<'tree>,
    //pub field_name: Option<String>,
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

impl RichNode<'a, 'tree> {
    pub fn new(inner: &'a Node<'tree>) -> Self {
        Self {
            inner,
            ..Default::default()
        }
    }

    fn enrich(&mut self) {
        //self.enrich_comments(source_code);
        //self.enrich_formatting();

        for c in self.inner.named_children() {
            //c.enrich();
        }
    }

    fn enrich_comments(&mut self) {
        let mut prev = self.inner.prev_sibling();
        while let Some(node) = prev {
            if node.kind() == "line_comment" {
                self.comments.pre_comments.push(Comment::from_node(&node));
            }
            prev = node.prev_sibling();
        }
    }

    fn enrich_formatting(&mut self) {
        self.formatting_info = match self.inner.kind() {
            "class_declaration" => FormattingInfo {
                ..Default::default()
            },

            "method_declaration" => FormattingInfo {
                wrappable: true,
                ..Default::default()
            },
        }
    }

    fn rewrite(&self) -> String {
        let mut result = String::new();

        // Handle pre-comments
        for comment in &self.comments.pre_comments {
            //result.push_str(&comment.format(shape));
        }

        // Add content
        result.push_str(&self.content);

        // Handle inline comments...
        // Handle children...
        // Handle post comments...

        result
    }
}
