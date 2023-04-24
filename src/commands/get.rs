use serde_json::Value;

use crate::utils::{get_memo_file_path, read_file, str_to_json, value_to_str_without_quotes};

pub fn get(k: String) {
    let filepath = get_memo_file_path().unwrap();
    let file_str = read_file(&filepath).unwrap();
    let json_value: Value = str_to_json(&file_str).unwrap();

    match json_value.get(&k) {
        Some(value) => {
            println!("{}", value_to_str_without_quotes(value));
        }
        None => {
            println!("{} not found", &k);
        }
    }
}
