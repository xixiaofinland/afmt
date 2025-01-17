#[cfg(not(target_arch = "wasm32"))]
pub fn red(text: &str) -> String {
    format!("\x1b[31m{}\x1b[0m", text) // ANSI red
}

#[cfg(target_arch = "wasm32")]
pub fn red(text: &str) -> &str {
    text
    //format!("<strong style=\"color: red;\">{}</strong>", text) // HTML bold red
}

#[cfg(not(target_arch = "wasm32"))]
pub fn yellow(text: &str) -> String {
    format!("\x1b[33m{}\x1b[0m", text) // ANSI yellow
}

#[cfg(target_arch = "wasm32")]
pub fn yellow(text: &str) -> &str {
    text
    //format!("<strong style=\"color: yellow;\">{}</strong>", text) // HTML bold yellow
}
