use std::{env, io::stdin};

use serde_json::{json, to_value, Value};

use crate::utils::{
    init_memo_file, read_file, str_to_json, user_confirmation, write_file, MEMO_FILE_PATH_ENV_KEY,
};

pub fn add(k: String, v: String) {
    let filepath = env::var(MEMO_FILE_PATH_ENV_KEY).unwrap();
    let file_str = read_file(filepath.clone().into()).unwrap();
    let json_res = str_to_json(&file_str);

    if let Err(e) = &json_res {
        println!("The json format of the memo file is broken");
        println!("Do you init memo file? (yes/no)");
        let stdin = stdin();
        if !user_confirmation(stdin.lock()) {
            println!("{}", e);
            println!("Please fix the memo file yourself of initialize it");
            panic!("force quit")
        }
        if let Err(e) = init_memo_file(filepath.clone().into()) {
            println!("{e}");
            return;
        }
    }

    let mut json_value = json_res.unwrap();

    match json_value.get_mut(&k) {
        Some(value) => {
            println!("{} is already set", v);
            if *value == to_value(&v).unwrap() {
                return;
            }
            println!("Do you overwrite {} to {}? (yes/no)", value, v);
            let stdin = stdin();
            if !user_confirmation(stdin.lock()) {
                return;
            }
            let old_value = value.clone();
            *value = json!(&v);
            println!(
                "update {} >>> {}",
                json!({ &k: old_value }),
                json!({ &k: &v })
            );
            write_file(filepath.into(), json_value.to_string().as_bytes()).unwrap();
        }
        None => {
            json_value[&k] = Value::String(v.clone());
            println!("add >>> {}", json!({ &k: &v }));
            write_file(filepath.into(), json_value.to_string().as_bytes()).unwrap();
        }
    }
}
