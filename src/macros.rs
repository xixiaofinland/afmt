#[macro_export]
macro_rules! define_struct {
    ($name:ident) => {
        pub struct $name<'a, 'tree> {
            inner: &'a Node<'tree>,
        }

        impl<'a, 'tree> $name<'a, 'tree> {
            pub fn new(node: &'a Node<'tree>) -> Self {
                Self { inner: node }
            }

            pub fn node(&self) -> &'a Node<'tree> {
                self.inner
            }
        }
    };
}

#[macro_export]
macro_rules! define_struct_and_enum {
    ($( $create_struct:ident; $name:ident => $($str_repr:tt)|+ ),* ) => {
        $(
            $crate::conditional_struct_creation!($create_struct, $name);
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
                        $(
                            $str_repr => NodeKind::$name,
                        )+
                    )*
                    _ => NodeKind::Unknown,
                }
            }
        }
    };
}

#[macro_export]
macro_rules! conditional_struct_creation {
    (true, $name:ident) => {
        define_struct!($name);
    };
    (false, $name:ident) => {
        // No struct is created when the flag is false
    };
}
