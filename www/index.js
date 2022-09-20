import * as wasm from "wasm-tfidf";
import json from './datas/base.json'

// wasm.greet();

var data = json["pages"];
var rows = [];
for (var i = 1; i <= 3; i++) {
    rows.push(data[i]["extract"]);
}
console.log(rows)

let voc = wasm.Vocabulary.new(rows);

// let tfidf = wasm.Tfidf.new(["Doudou et bibou et bibou", 
//                                 "All the bibou",
//                                 "Is this a bibou"]);

console.log(voc.words_count());

console.log(voc.vocabulary());

console.log(voc.words_in_doc());

// console.log(tfidf.vocabulary.vocabulary())