//#[macro_export]
//macro_rules! rich_struct {
//    ( $( $name:ident ),+ $(,)? ) => {
//        $(
//            // Define each struct
//            #[derive(Debug)]
//            pub struct $name<'t> {
//                pub inner: Node<'t>,
//                pub content: String,
//                pub buckets: CommentBuckets,
//                pub children: Vec<ASTNode<'t>>,
//                pub format_info: FormatInfo,
//            }
//
//            // Implement the RichNode trait for each struct
//            impl<'t> RichNode for $name<'t> {
//                fn enrich(&mut self, shape: &mut EShape, context: &EContext) {
//                    self.enrich_comments(shape, context);
//                    self.enrich_data(shape, context);
//                }
//            }
//
//            // Implement struct methods for each struct
//            impl<'t> $name<'t> {
//                pub fn build(inner: Node<'t>, shape: &mut EShape, context: &EContext) -> Self {
//                    let mut n = Self {
//                        inner,
//                        content: String::new(),
//                        buckets: CommentBuckets::default(),
//                        children: Vec::new(),
//                        format_info: FormatInfo::default(),
//                    };
//                    n.enrich(shape, context);
//                    n
//                }
//
//                fn enrich_comments(&mut self, shape: &mut EShape, context: &EContext) {
//                    let mut prev_sibling = self.inner.prev_sibling();
//                    while let Some(node) = prev_sibling {
//                        if node.is_comment() {
//                            let comment_id = node.id();
//                            if let Some(comment) = shape.comments.iter_mut().find(|c| c.id == comment_id) {
//                                if !comment.is_processed {
//                                    self.buckets
//                                        .pre_comments
//                                        .push(Comment::from_node(&node, context));
//                                    comment.is_processed = true;
//                                }
//                            } else {
//                                self.buckets
//                                    .pre_comments
//                                    .push(Comment::from_node(&node, context));
//                            }
//                        }
//                        prev_sibling = node.prev_sibling();
//                    }
//
//                    let mut next_sibling = self.inner.next_sibling();
//                    while let Some(node) = next_sibling {
//                        if node.is_comment() {
//                            let comment_id = node.id();
//                            if let Some(comment) = shape.comments.iter_mut().find(|c| c.id == comment_id) {
//                                if !comment.is_processed {
//                                    self.buckets
//                                        .post_comments
//                                        .push(Comment::from_node(&node, context));
//                                    comment.is_processed = true;
//                                }
//                            } else {
//                                self.buckets
//                                    .post_comments
//                                    .push(Comment::from_node(&node, context));
//                            }
//                        }
//                        next_sibling = node.next_sibling();
//                    }
//                }
//
//                pub fn prepare<'a>(
//                    &mut self,
//                    context: &'a EContext,
//                ) -> (
//                    &Node<'t>,
//                    String,
//                    &'a str,
//                    &'a Config,
//                    &mut Vec<ASTNode<'t>>,
//                ) {
//                    let node = &self.inner;
//                    let result = String::new();
//                    let source_code = context.source_code.as_str();
//                    let config = &context.config;
//                    (node, result, source_code, config, &mut self.children)
//                }
//            }
//
//            // Add each struct as a variant in the ASTNode enum
//            impl<'t> From<$name<'t>> for ASTNode<'t> {
//                fn from(value: $name<'t>) -> Self {
//                    ASTNode::$name(value)
//                }
//            }
//        )*
//    };
//}
