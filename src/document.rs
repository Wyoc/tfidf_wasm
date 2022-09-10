use wasm_bindgen::prelude::*;
use std::collections::HashMap;
use itertools;

use crate::normalizer::{tokenize, lowercase};


pub struct Document {
    words : Vec<String>,
    length : i32,
    id : String,
}

impl Document {
    pub fn new(text: String) -> Document {
        let words_vector: Vec<String> = docs.into_iter()
                            .flat_map(tokenize)
                            .collect();
        
        Vocabulary { words: words_vector }
    }

    pub fn lowercase(&self) {
        self.words = self.words.into_iter()
                                .map(|x| lowercase(x.to_string))
    }

    
}