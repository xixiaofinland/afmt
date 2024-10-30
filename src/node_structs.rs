pub struct ClassNode {
    name: String,
    fields: Vec<FieldNode>,
    methods: Vec<FunctionNode>,
    comments: CommentBuckets,
    start_line: usize,
    end_line: usize,
    indent_level: usize,
}
