use super::utils::{read_file, str_to_json, write_file, MEMO_FILE_PATH};
use serde_json::{json, Value};

pub fn delete(k: String) {
    let file_str = read_file(MEMO_FILE_PATH).unwrap();
    let mut json_value: Value = str_to_json(&file_str).unwrap();

    match json_value.as_object_mut().unwrap().remove(&k) {
        Some(value) => {
            println!("delete >>> {}", json!({ &k: value }));
            write_file(MEMO_FILE_PATH, json_value.to_string().as_bytes())
                .expect("write file error");
        }
        None => {
            println!("{} not found", &k);
        }
    }
}
