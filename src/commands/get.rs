use serde_json::Value;

use super::utils::{read_file, str_to_json, value_to_str_without_quotes, MEMO_FILE_PATH};

pub fn get(k: String) {
    let file_str = read_file(MEMO_FILE_PATH).unwrap();
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
