import * as wasm from "wasm-tfidf";
import data from './datas/base.json'

// wasm.greet();

var json = data["pages"];

console.log(json)

let lowered = wasm.lower("Hello DouDoU");
console.log(lowered);

let tokenized = wasm.make_tokens("Hello this is dog")
console.log(tokenized)

let test = wasm.get_vector();
console.log(test)
console.log(Array.from(test.values()));

let voc = wasm.Vocabulary.new(["Doudou et bibou et bibou", 
                                "All the bibou",
                                "Is this a bibou"]);
console.log(voc.words_count());

console.log(voc.vocabulary());

console.log(voc.words_in_doc());