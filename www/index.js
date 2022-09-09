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