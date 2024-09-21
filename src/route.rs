use crate::{rewrite::Rewrite, struct_def::*};
use phf::phf_map;
use tree_sitter::Node;

static ROUTING_MAP: phf::Map<
    &'static str,
    for<'a, 'tree> fn(&'a Node<'tree>) -> ClassDeclaration<'a, 'tree>,
> = phf_map! {
    "class_declaration" => ClassDeclaration::new,
    "method_declaration" => MethodDeclaration::new,
};

// Usage
//define_routing_phf!(ROUTING_MAP, self, result, context, shape);
