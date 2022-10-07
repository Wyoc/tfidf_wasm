// use rake::*;
// use wasm_bindgen::prelude::*;

// use crate::{normalizer::{lowercase, tokenize}, console_log};

// pub fn get_keywords(doc: &str, path_to_stopwords: String){
//     let sw = StopWords::from_file(path_to_stopwords).unwrap();
//     let r = Rake::new(sw);
//     let keywords = r.run(doc);

//     keywords.iter().for_each(
//         |&KeywordScore {
//             ref keyword,
//             ref score,
//         }| console_log!("{}: {}", keyword, score),
//     );
// }