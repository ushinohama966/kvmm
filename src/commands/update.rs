use std::{env, io::stdin};

use serde_json::json;

use crate::utils::{
    init_memo_file, read_file, str_to_json, user_confirmation, value_to_str_without_quotes,
    write_file, MEMO_FILE_PATH_ENV_KEY,
};

pub fn update(k: String, v: String) {
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
            write_file(filepath.into(), json_value.to_string().as_bytes()).unwrap();
        }
        None => {
            println!("{} not found", &k);
        }
    }
}
