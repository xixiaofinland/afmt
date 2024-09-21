use crate::{rewrite::Rewrite, struct_def::*};
use phf::phf_map;
use tree_sitter::Node;

static ROUTING_MAP: phf::Map<
    &'static str,
    for<'a, 'tree> fn(&'a Node<'tree>) -> Box<dyn Rewrite + 'a>,
> = phf_map! {
    "class_declaration" => |node| Box::new(ClassDeclaration::new(node)),
    "method_declaration" => |node| Box::new(MethodDeclaration::new(node)),
};

// Usage
//define_routing_phf!(ROUTING_MAP, self, result, context, shape);
