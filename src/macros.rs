#[macro_export]
macro_rules! define_struct {
    ( $( $name:ident ),+ ) => {
        $(
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
                ) -> (&'a Node<'tree>, String, &'b str, &'b $crate::config::Config) {
                    let node = self.node();
                    let result = String::new();
                    let source_code = context.source_code;
                    let config = context.config;
                    (node, result, source_code, config)
                }
            }
        )+
    };
}

#[macro_export]
macro_rules! define_routing {
    ( $node:ident, $result:ident, $context:ident, $shape:ident, $route_name:ident;
      $( $kind:literal => $struct_name:ident ),* ) => {
        match $node.kind() {
            $(
                $kind => {
                    $result.push_str(&$struct_name::new(&$node).rewrite($context, $shape));
                }
            )*
            _ => {
                println!(
                    "{} {}",
                    format!("### {} Routing - unknown child: ", $route_name).yellow(),
                    $node.kind().red()
                );
                panic!();
            }
        }
    };
}
