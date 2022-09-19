use wasm_bindgen::prelude::*;

use crat::vocabulary::Vocabulary;

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Tfidf {
    vocabulary: Vocabulary,
}

#[wasm_bindgen]
impl Vocabulary {
    pub fn new(docs: &JsValue) -> Tfidf {
        let vocabulary: Vocabulary = Vocabulary::new(docs);
        Tfidf( vocabulary,)
    }
}