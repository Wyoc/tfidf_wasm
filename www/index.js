import * as wasm from "wasm-tfidf";
import json_pages from './datas/base.json'
import json_sw from './datas/stop_words_english.json'

// wasm.greet();

// Build pages array
var data = json_pages["pages"];
var rows = [];
var idx = []
for (var i = 1; i <= 3; i++) {
    rows.push(data[i]["extract"]);
    idx.push(data[i]["pageid"]);
}
// console.log(rows)
console.log(idx)
 
// build stopwords array
var sw = [];
for (var w in json_sw) {
    sw.push(json_sw[w]);
}
console.log(sw)
// let voc = wasm.Vocabulary.new_from_docs(rows, idx);

let voc = wasm.Vocabulary.fit(["Doudou et bibou et bibou", 
                                "All the bibou",
                                "Is this a bibou"],
                                idx);

console.log(voc.words_count());

console.log(voc.vocabulary());

console.log(voc.words_in_doc());

let test_corpus = voc.transform_docs(["Doudou et bibou et bibou", 
                                    "All the bibou",
                                    "On attends pas partrick ?"]);
// console.log(tfidf.vocabulary.vocabulary())

console.log(test_corpus)
// wasm.matmul();

let test_doc = voc.transform_docs(["C'est nous les bibou"]);
// console.log(tfidf.vocabulary.vocabulary())

console.log(test_doc)
// wasm.matmul();

let keywords = wasm.get_keywords(rows[0], sw, 1, 1);
console.log(keywords);