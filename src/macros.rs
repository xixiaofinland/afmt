#[macro_export]
macro_rules! define_struct {
    ($name:ident) => {
        pub struct $name<'a, 'tree> {
            inner: &'a Node<'tree>,
        }

        #[allow(dead_code)]
        impl<'a, 'tree> $name<'a, 'tree> {
            pub fn new(node: &'a Node<'tree>) -> Self {
                Self { inner: node }
            }

            pub fn node(&self) -> &'a Node<'tree> {
                self.inner
            }

            pub fn prepare<'b>(
                &self,
                context: &'b FmtContext,
            ) -> (&'a Node<'tree>, String, &'b str, &'b crate::config::Config) {
                let node = self.node();
                let result = String::new();
                let source_code = context.source_code;
                let config = context.config;
                (node, result, source_code, config)
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

#[macro_export]
macro_rules! define_routing {
    ( $c_node:ident, $result:ident, $context:ident, $shape:ident;
      $( $kind:literal => $struct_name:ident ),* ) => {
        match $c_node.kind() {
            $(
                $kind => {
                    $result.push_str(&$struct_name::new(&$c_node).rewrite($context, $shape));
                }
            )*
            _ => {
                let struct_names = stringify!($($struct_name),*);
                println!(
                    "{} {}",
                    format!("### {}: unknown child: ", struct_names).yellow(),
                    $c_node.kind().red()
                );
                panic!();
            }
        }
    };
}
