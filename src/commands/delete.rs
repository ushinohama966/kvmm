use std::{env, io::stdin};

use crate::utils::{
    init_memo_file, read_file, str_to_json, user_confirmation, write_file, MEMO_FILE_PATH_ENV_KEY,
};
use serde_json::json;

pub fn delete(k: String) {
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

    match json_value.as_object_mut().unwrap().remove(&k) {
        Some(value) => {
            println!("delete >>> {}", json!({ &k: value }));
            write_file(filepath.into(), json_value.to_string().as_bytes())
                .expect("write file error");
        }
        None => {
            println!("{} not found", &k);
        }
    }
}
