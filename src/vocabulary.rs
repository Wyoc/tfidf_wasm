use crate::{normalizer::{lowercase, tokenize}, console_log};

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
extern crate serde_json;
use crate::utils::log;
use std::collections::BTreeMap;
use itertools::Itertools;
use ndarray::prelude::*;
use ndarray::{Array, Axis, concatenate};

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Vocabulary {
    ids: Vec<String>,
    vocabulary: Vec<String>,
    words_count: BTreeMap<String, u32>,
    words_in_doc: BTreeMap<String, u32>,
    min_df: f32,
    max_df: f32,
}

#[wasm_bindgen]
impl Vocabulary {
    pub fn fit(docs: &JsValue, ids: &JsValue) -> Self {

        let ids: Vec<String> = ids.into_serde().unwrap();

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

        let vocabulary: Vec<String> = words_count.keys().cloned().collect();

        Self {
            ids,
            vocabulary,
            words_count,
            words_in_doc,
            min_df: 0.0,
            max_df: 1.0,
        }
    }

    pub fn transform_docs(&self, docs: &JsValue)-> JsValue{
        let doc_list: Vec<String> = docs.into_serde().unwrap();

        let processed_doc: Vec<Vec<String>> = doc_list
            .into_iter()
            .map(tokenize)
            .map(|x| x.into_iter()
                                    .map(|w| lowercase(w.as_str()))
                                    .filter(|w| self.vocabulary.contains(w))
                                    .collect())
            .collect();
        let mut doc_matrix = Array::<usize, _>::zeros((self.vocabulary.len(), processed_doc.len()));
        for (i, doc) in processed_doc.iter().enumerate(){
            //let mut  vector = Array::<usize, _>::zeros(self.vocabulary.len());
            for (j, word) in self.vocabulary.iter().enumerate(){
                doc_matrix[[j,i]] = doc.iter().filter(|&n| n == word).count();
            }
        }
        //console_log!("{:?}", doc_matrix);
        serde_wasm_bindgen::to_value(&doc_matrix).unwrap()
    }

    pub fn transform_doc(&self, docs: &JsValue) -> JsValue{
        let doc: String = docs.into_serde().unwrap();

        let processed_doc: Vec<String> = tokenize(doc).into_iter()
                                                        .map(|w| lowercase(w.as_str()))
                                                        .filter(|w| self.vocabulary.contains(w))
                                                        .collect();

        let mut doc_vector = Array::<usize, _>::zeros(self.vocabulary.len());
        for (j, word) in self.vocabulary.iter().enumerate(){
            doc_vector[[j]] = processed_doc.iter().filter(|&n| n == word).count();
            }
        
        serde_wasm_bindgen::to_value(&doc_vector).unwrap()
    }

    pub fn similarity(){
        
    }


    // Getters

    pub fn vocabulary(&self) -> JsValue {
        let vocab: Vec<String> = self.words_count.keys().cloned().collect();
        serde_wasm_bindgen::to_value(&vocab).unwrap()
    }

    pub fn words_count(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.words_count).unwrap()
    }
    pub fn words_in_doc(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.words_in_doc).unwrap()
    }


}
