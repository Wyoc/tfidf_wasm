use crate::normalizer::{lowercase, tokenize};

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
extern crate serde_json;
use crate::utils::log;
use std::collections::BTreeMap;
use itertools::Itertools;

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Vocabulary {
    words_count: BTreeMap<String, u32>,
    words_in_doc: BTreeMap<String, u32>,
}

#[wasm_bindgen]
impl Vocabulary {
    pub fn new(docs: &JsValue) -> Vocabulary {
        let doc_list: Vec<String> = docs.into_serde().unwrap();

        let processed_doc: Vec<Vec<String>> = doc_list
            .into_iter()
            .map(tokenize)
            .map(|x| x.into_iter().map(|w| lowercase(w.as_str())).collect())
            .collect();

        let words_count = processed_doc.clone().into_iter().flatten().fold(
            BTreeMap::<String, u32>::new(),
            |mut m, x| {
                *m.entry(x).or_default() += 1;
                m
            },
        );
        let words_in_doc =
            processed_doc
                .into_iter()
                .fold(BTreeMap::<String, u32>::new(), |mut m, x| {
                    for w in x.into_iter().unique() {
                        *m.entry(w).or_default() += 1;
                    }
                    m
                });

        Vocabulary {
            words_count,
            words_in_doc,
        }
    }

    pub fn vocabulary(&self) -> JsValue {
        let vocab: Vec<String> = self.words_count.keys().cloned().collect();
        serde_wasm_bindgen::to_value(&vocab).unwrap()
    }

    //#[wasm_bindgen]
    pub fn words_count(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.words_count).unwrap()
    }
    pub fn words_in_doc(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.words_in_doc).unwrap()
    }
}
