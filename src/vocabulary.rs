use wasm_bindgen::prelude::*;
use std::collections::HashMap;
use itertools;

use crate::normalizer::tokenize;

#[wasm_bindgen]
pub struct Vocabulary {
    words: Vec<String>
}

impl Vocabulary {
    pub fn new(docs: Vec<String>) -> Vocabulary {
        let words_vector: Vec<String> = docs.into_iter()
                            .flat_map(tokenize)
                            .collect();
        
        Vocabulary { words: words_vector }
    } 
}