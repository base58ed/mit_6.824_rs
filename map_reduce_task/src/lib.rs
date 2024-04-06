use std::vec;

#[no_mangle]
pub fn map(filename: String, content: String) -> Vec<String> {
    return vec![String::from("hello world of map")];
}

#[no_mangle]
pub fn reduce(key: String, values: Vec<String>) -> String {
    return String::from("reduce invoked")
}
