use crate::{
    config::{Indent, Shape},
    context::FmtContext,
    node_struct::{ClassDeclaration, FieldDeclaration, MethodDeclaration, NodeKind, Rewrite},
};
use tree_sitter::Node;

pub struct Visitor {
    //parent_context: Option<&'a FmtContext<'_>>,
    pub block_indent: Indent,
    pub buffer: String,
}

impl Visitor {
    //pub fn new(parent_context: Option<&'a FmtContext<'_>>, block_indent: Indent) -> Self {
    pub fn new(block_indent: Indent) -> Self {
        Self {
            block_indent,
            buffer: String::new(),
        }
    }

    pub fn from_current(shape: &Shape) -> Visitor {
        let block_indent = Indent::new(shape.indent.block_indent, 0);
        Visitor::new(block_indent)
    }

    pub fn push_rewritten(&mut self, rewritten: Option<String>, node: &Node) {
        if let Some(r) = rewritten {
            self.push_str(&r);
        } else {
        }
    }

    pub fn push_str(&mut self, s: &str) {
        self.buffer.push_str(s);
    }

    pub fn visit_root(&mut self, context: &FmtContext, parent_shape: &Shape) {
        self.visit_direct_children(&context.ast_tree.root_node(), context, parent_shape)
    }

    pub fn visit_direct_children(
        &mut self,
        node: &Node,
        context: &FmtContext,
        parent_shape: &Shape,
    ) {
        let is_root_node = node.kind() == "parser_output";

        let shape = if is_root_node {
            Shape::empty()
        } else {
            Shape::increase_indent(parent_shape)
        };

        let mut cursor = node.walk();
        for child in node.named_children(&mut cursor) {
            let kind = NodeKind::from_kind(child.kind());

            match kind {
                NodeKind::ClassDeclaration => {
                    self.visit_class(&child, context, &shape);
                }
                NodeKind::FieldDeclaration => {
                    let n = FieldDeclaration::new(&child);
                    self.push_rewritten(n.rewrite(context, &shape), &child);
                }
                NodeKind::MethodDeclaration => {
                    let n = MethodDeclaration::new(&child);
                    self.push_rewritten(n.rewrite(context, &shape), &child);
                }
                //NodeKind::Modifiers => {
                //    self.visit_if_node(node);
                //}
                //NodeKind::ForLoop => {
                //    //self.visit_for_node(node);
                //}
                NodeKind::Unknown => {
                    println!("### Unknow node: {}", child.kind());
                }
                _ => {
                    !unimplemented!();
                }
            }
        }
    }

    pub fn visit_class(&mut self, node: &Node, context: &FmtContext, shape: &Shape) {
        let n = ClassDeclaration::new(&node);
        self.push_rewritten(n.rewrite(context, &shape), &node);

        self.push_block_open_line();

        let mut v = Visitor::from_current(&shape);
        let body_node = node
            .child_by_field_name("body")
            .expect("mandatory body node missing");
        v.visit_direct_children(&body_node, context, &shape);
        self.buffer.push_str(&v.buffer);

        self.push_block_close_line();
    }

    //pub fn visit_block(&mut self, node: &Node, context: &FmtContext, parent_shape: &Shape) {
    //    let mut visitor = Visitor::from_current(parent_shape);
    //    visitor.visit_direct_children(node, context, parent_shape);
    //    visitor.buffer;
    //}

    fn push_block_open_line(&mut self) {
        self.buffer.push_str(" {\n");
    }

    fn push_block_close_line(&mut self) {
        self.buffer.push_str("}\n");
    }
}
