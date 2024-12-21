use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main_js() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub struct Formatter2 {}

#[wasm_bindgen]
impl Formatter2 {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Formatter2 {}
    }

    #[wasm_bindgen]
    pub fn format_one(source_code: String) -> String {
        println!("{}", source_code);
        source_code
    }
}
