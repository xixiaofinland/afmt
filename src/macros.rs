#[macro_export]
macro_rules! define_node {
    ($name:ident) => {
        pub struct $name<'a, 'tree> {
            inner: &'a Node<'tree>,
        }

        impl<'a, 'tree> $name<'a, 'tree> {
            pub fn new(node: &'a Node<'tree>) -> Self {
                Self { inner: node }
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
