#[macro_export]
macro_rules! define_node {
    ($name:ident) => {
        pub struct $name<'a, 'b, 'tree> {
            inner: &'a Node<'tree>,
            shape: &'b Shape,
        }

        impl<'a, 'b, 'tree> $name<'a, 'b, 'tree> {
            pub fn new(node: &'a Node<'tree>, shape: &'b Shape) -> Self {
                Self { inner: node, shape }
            }

            pub fn as_ast_node(&self) -> &'a Node<'tree> {
                self.inner
            }
        }
    };
}

#[macro_export]
macro_rules! define_nodes {
    ($($name:ident),*) => {
        $(define_node!($name);)*
    };
}
