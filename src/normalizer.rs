use unidecode::unidecode;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn lowercase(text: &str) -> String {
    text.to_lowercase()
}

pub fn tokenize(text: &str) -> Vec<String> {
    let tokens =text
                                .split_whitespace()
                                .map(String::from)
                                .collect();
    tokens
}

#[wasm_bindgen]
pub fn to_unicode(text: &str) -> String {
    unidecode(text)
}



#[test]
fn to_unicode_test(){
    assert_eq!(to_unicode("étude"), "etude");
    assert_eq!(to_unicode("Zèbre"), "Zebre");
    assert_eq!(to_unicode("Pêche"), "Peche");
    assert_eq!(to_unicode("à"), "a");
    assert_eq!(to_unicode("où"), "ou");
}

#[test]
fn lowercase_test(){
    let original = "Hello MeetSYS";
    let lowercased = "hello meetsys";
    assert_ne!(original, lowercased);
    let lowercased_original = lowercase(original);
    assert_eq!(lowercased_original, lowercased);
}

#[test]
fn tokenize_test(){
    let original = "Hello MeetSYS";
    let tokenized = tokenize(original);
    assert_eq!(tokenized, vec!["Hello", "MeetSYS"]);

    let original2 = "Hello  MeetSYS";
    let tokenized2 = tokenize(original2);
    assert_eq!(tokenized2, vec!["Hello", "MeetSYS"]);
}

