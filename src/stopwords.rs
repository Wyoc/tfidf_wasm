use wasm_bindgen::prelude::*;
extern crate serde_json;

macro_rules! Vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

#[wasm_bindgen]
pub fn get_stopwords_en() -> js_sys::Array{
    let stopwords = Vec_of_strings!["she","were","there","hasn","isn","our","yourself","be","only","been","theirs","me","of","ll","mightn't","here","doesn't","hadn't","and","d","isn't","shan","y","hadn","couldn","few","ve","didn't","he","re","should've","once","the","some","if","you'll","himself","on","they","other","hers","by","that'll","don","a","no","off","same","ma","t","its","it","after","own","weren't","during","about","out","her","so","too","itself","won","with","under","shan't","your","does","is","when","who","now","up","needn't","before","at","didn","doesn","where","this","between","shouldn't","very","needn","haven","below","having","she's","you'd","you","such","each","it's","those","can","have","aren't","themselves","haven't","aren","myself","their","what","did","in","more","most","wouldn't","further","than","yours","from","but","both","doing","couldn't","you've","had","which","we","my","them","just","won't","herself","being","mightn","mustn","whom","shouldn","not","don't","how","for","while","any","down","s","weren","do","ours","into","these","that","again","over","am","until","yourselves","through","against","o","has","ain","wasn","or","hasn't","as","above","you're","him","an","his","i","why","are","was","should","m","mustn't","wasn't","ourselves","because","nor","will","to","all","then","wouldn"];
    stopwords.into_iter().map(JsValue::from).collect()
}


#[wasm_bindgen]
pub fn get_stopwords_fr() -> js_sys::Array{
    let stopwords = Vec_of_strings!["d","aurez","avaient","soient","tes","une","leur","fussiez","eu","les","étais","étées","des","aurons","sont","soyons","aviez","mes","serait","sera","il","aurais","mon","serions","auront","eût","fusses","du","même","sois","ils","aient","nos","fût","auras","c","aura","que","ayante","eurent","ne","eut","étantes","ait","te","ta","avez","étaient","ayons","on","moi","fus","fusse","aux","étant","furent","sa","serons","t","au","ayantes","ses","ce","elle","fut","ayez","me","étante","étiez","y","vos","serais","était","fussent","fûtes","ayant","à","avait","avec","êtes","seraient","es","eux","auraient","votre","pour","ton","aurions","étions","suis","été","je","eusse","as","et","étés","serai","auriez","sommes","est","ces","de","s","qu","ou","soit","avions","aurait","sur","nous","ma","eussiez","se","lui","fussions","son","étants","eûtes","aie","la","le","notre","eussions","un","eue","par","en","seriez","tu","mais","seras","eûmes","eussent","seront","ont","j","qui","l","ayants","ai","eues","dans","toi","vous","soyez","aies","eusses","n","eus","fûmes","pas","m","avons","aurai","avais","étée","serez"];
    stopwords.into_iter().map(JsValue::from).collect()
}