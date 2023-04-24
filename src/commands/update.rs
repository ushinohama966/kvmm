use serde_json::{json, Value};

use crate::utils::{
    get_memo_file_path, read_file, str_to_json, value_to_str_without_quotes, write_file,
};

pub fn update(k: String, v: String) {
    let filepath = get_memo_file_path().unwrap();
    let file_str = read_file(&filepath).unwrap();
    let mut json_value: Value = str_to_json(&file_str).unwrap();

    match json_value.get_mut(&k) {
        Some(value) => {
            if value_to_str_without_quotes(value) == v {
                println!("{} is already set", &v);
                return;
            }
            let old_value = value.clone();
            *value = json!(&v);
            println!(
                "update {} >>> {}",
                json!({ &k: old_value }),
                json!({ &k: &v })
            );
            write_file(&filepath, json_value.to_string().as_bytes()).unwrap();
        }
        None => {
            println!("{} not found", &k);
        }
    }
}
