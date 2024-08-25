//use tree_sitter::Node;

//pub trait NodeUtilities<'tree> {
//    fn get_child_by_kind(&self, kind: &str) -> Option<Node<'tree>>;
//    fn get_children_by_kind(&self, kind: &str) -> Vec<Node<'tree>>;
//}
//
//impl<'tree> NodeUtilities<'tree> for Node<'tree> {
//    fn get_child_by_kind(&self, kind: &str) -> Option<Node<'tree>> {
//        let mut cursor = self.walk();
//        let node = self.children(&mut cursor).find(|c| c.kind() == kind);
//        node
//    }
//
//    fn get_children_by_kind(&self, kind: &str) -> Vec<Node<'tree>> {
//        let mut cursor = self.walk();
//        self.children(&mut cursor)
//            .filter(|c| c.kind() == kind)
//            .collect()
//    }
//}
