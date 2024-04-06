use std::vec;
use mr_model::KeyValue;

#[no_mangle]
pub fn map(filename: String, content: String) -> Vec<KeyValue> {
    return vec![KeyValue { key: "kv key".to_string(), value: "kv value".to_string() }];
}

#[no_mangle]
pub fn reduce(key: String, values: Vec<String>) -> String {
    return String::from("reduce invoked");
}
