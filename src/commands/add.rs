use serde_json::{json, Value};

use super::utils::{read_file, str_to_json, user_confirmation, write_file, MEMO_FILE_PATH};

pub fn add(k: String, v: String) {
    let file_str = read_file(MEMO_FILE_PATH).unwrap();
    let mut json_value: Value = str_to_json(&file_str).unwrap();

    match json_value.get_mut(&k) {
        Some(value) => {
            println!("{} is already set", v);
            if value.to_string() == json!(v).to_string() {
                return;
            }
            println!("Do you overwrite {} to {}? (yes/no)", value.to_string(), v);
            if !user_confirmation() {
                return;
            }
            let old_value = value.clone();
            *value = json!(&v);
            println!(
                "update {} >>> {}",
                json!({ &k: old_value }),
                json!({ &k: &v })
            );
            write_file(MEMO_FILE_PATH, json_value.to_string().as_bytes()).unwrap();
        }
        None => {
            json_value[&k] = Value::String(v.clone());
            println!("add >>> {}", json!({ &k: &v }));
            write_file(MEMO_FILE_PATH, json_value.to_string().as_bytes()).unwrap();
        }
    }
}

#[cfg(test)]
mod tests {}
