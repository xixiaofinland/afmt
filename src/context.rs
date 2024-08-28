//use std::sync::OnceLock;
//
//#[derive(Debug, Clone)]
//pub struct Context<'code> {
//    pub source_code: &'code str,
//}
//
//impl<'code> Context<'code> {
//    pub fn new(source_code: &'code str) -> Self {
//        Self { source_code }
//    }
//}
//
//pub static CONTEXT: OnceLock<Context> = OnceLock::new();
