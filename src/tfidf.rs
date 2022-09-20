use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

use crate::vocabulary::Vocabulary;

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Tfidf {
    vocabulary: Vocabulary,
}

#[wasm_bindgen]
impl Vocabulary {
    // pub fn new(docs: &JsValue) -> Tfidf {

    // }

    fn _tf(&self) {

    }

    fn _idf(&self) {

    }

    pub fn tfidf(&self){

    }


    pub fn fit(&self) {

    }

    pub fn transform(&self) {

    }

    pub fn fit_transform(&self) {

    }
}