mod normalizer;
mod utils;
pub mod vocabulary;
use std::collections::HashMap;

use crate::utils::log;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    console_log!("Test console.log");
    alert("Hello, it's tfidf time once again!");
}

#[wasm_bindgen]
pub fn lower(s: &str) -> String {
    normalizer::lowercase(s)
}

#[wasm_bindgen]
pub fn make_tokens(s: String) -> js_sys::Array {
    let tokens = normalizer::tokenize(s);
    tokens.into_iter().map(JsValue::from).collect()
}

#[wasm_bindgen]
pub fn get_vector() -> JsValue {
    let rust_vec = HashMap::from([
        ("Mercury", 0.4),
        ("Venus", 0.7),
        ("Earth", 1.0),
        ("Mars", 1.5),
    ]);
    console_log!("{:?}", rust_vec);
    serde_wasm_bindgen::to_value(&rust_vec).unwrap()
}
