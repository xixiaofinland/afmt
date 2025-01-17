mod accessor;
pub mod args;
mod context;
mod data_model;
mod doc;
mod doc_builder;
mod enum_def;
pub mod formatter;
pub mod message_helper;
mod utility;
use formatter::Formatter;

pub fn format(f: Formatter) -> Vec<Result<String, String>> {
    f.format()
}

//#[wasm_bindgen]
//pub fn greet(source_code: &str) -> String {
//    let config = Config::default();
//    Formatter::format_one(source_code, config)
//}

//#[wasm_bindgen]
//pub fn greet(source_code: &str) -> String {
//    "hello".to_string()
//}

//#[wasm_bindgen]
//pub fn greet() -> Result<String, JsValue> {
//    Ok("hello world!".to_string())
//}
