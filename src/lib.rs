pub mod keywords;
mod normalizer;
pub mod stopwords;
pub mod tfidf;
mod utils;
pub mod vocabulary;
use std::collections::BTreeMap;
use std::collections::HashMap;

use crate::utils::log;
use normalizer::tokenize;
use rake::*;
extern crate nalgebra as na;
use na::{dmatrix, Matrix3, Vector3};
extern crate serde_json;

use wasm_bindgen::prelude::*;

use voca_rs::strip::strip_tags;

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

#[wasm_bindgen]
pub fn matmul() {
    // A vector with three components.
    let v = Vector3::new(1, 2, 3);

    // A matrix with three lines and four columns.
    // We chose values such that, for example, 23 is at the row 2 and column 3.
    let m = Matrix3::new(11, 12, 13, 21, 22, 23, 31, 32, 33);
    let vm = v.transpose() * m;
    let d = dmatrix![1,2,3;
                                                           4,5,6];
    console_log!("Matrix multiplication");
    console_log!("{:?}", vm);
    console_log!("{:?}", d)
    //serde_wasm_bindgen::to_value(vm).unwrap()
}

pub fn stopwords_from_vec(stopwords_vector: &JsValue) -> StopWords {
    let sw_vec: Vec<String> = stopwords_vector.into_serde().unwrap();
    let mut sw = StopWords::new();
    for w in sw_vec {
        sw.insert(w);
    }
    sw
}

#[wasm_bindgen]
pub fn strip_html(html_doc: String) -> String{
    console_log!("{:?}", html_doc);
    strip_tags(&html_doc)
}


#[wasm_bindgen]
pub fn get_keywords(input_doc: String, input_sw_vec: &JsValue, min_length: usize, max_length: usize) -> JsValue {
    utils::set_panic_hook();
    //let doc: String = input_doc.into_serde().unwrap();
    let sw = stopwords_from_vec(input_sw_vec);
    let r = Rake::new(sw);
    let keywords = r.run(&input_doc);
    let mut candidats = HashMap::<String, f64>::new();
    for kw in keywords.into_iter(){
        candidats.insert(kw.keyword,kw.score);
    }
    candidats.retain(|w, _| tokenize(w.to_string()).len() <= max_length);
    candidats.retain(|w, _| tokenize(w.to_string()).len() >= min_length);
    //console_log!("{:?}", candidats);

    serde_wasm_bindgen::to_value(&candidats).unwrap()
}


