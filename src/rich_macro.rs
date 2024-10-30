#[macro_export]
macro_rules! def_rich_struct {
    ( $( $name:ident ),+ $(,)? ) => {
        $(
            pub struct $name<'t> {
                inner: Node<'t>,
            }

            #[allow(dead_code)]
            impl<'t> $name<'t> {
                pub fn node(&self) -> Node<'t> { self.inner }

                pub fn prepare<'b>(
                    &self,
                    context: &'b FmtContext,
                ) -> (&Node<'t>, String, &str, &'b Config) {
                    let node = &self.node();
                    let result = String::new();
                    let source_code = &context.source_code;
                    let config = context.config;
                    (node, result, source_code, config)
                }
            }

            impl<'a, 'tree> FromNode<'a, 'tree> for $name<'a, 'tree> {
                fn new(node: &'a Node<'tree>) -> Self {
                    Self { inner: node }
                }
            }
        )+
    };
}
