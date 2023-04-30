use std::{env, io::stdin};

use crate::utils::{
    init_memo_file, read_file, str_to_json, user_confirmation, value_to_str_without_quotes,
    MEMO_FILE_PATH_ENV_KEY,
};

pub fn get(k: String) {
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
        if let Err(e) = init_memo_file(filepath.into()) {
            println!("{e}");
            return;
        }
    }

    let json_value = json_res.unwrap();

    match json_value.get(&k) {
        Some(value) => {
            println!("{}", value_to_str_without_quotes(value));
        }
        None => {
            println!("{} not found", &k);
        }
    }
}
