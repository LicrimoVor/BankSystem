use wasm_bindgen::prelude::*;

// Указываем, что эту функцию можно вызывать из JS
#[wasm_bindgen]
pub fn greet(name: &str, count: i32) -> String {
    (0..count)
        .map(|i| format!("{i}. Привет, {name}! Rust говорит тебе: добро пожаловать в WebAssembly."))
        .collect::<Vec<String>>()
        .join("\n")
}
