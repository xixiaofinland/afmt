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
    ($($name:ident => $str_repr:expr),*) => {
        $(
            define_node!($name);
        )*

        #[derive(Debug)]
        pub enum NodeKind {
            $($name,)*
            Unknown,
        }

        impl NodeKind {
            pub fn from_kind(kind: &str) -> NodeKind {
                match kind {
                    $(
                        $str_repr => NodeKind::$name,
                    )*
                    _ => NodeKind::Unknown,
                }
            }
        }
    };
}
