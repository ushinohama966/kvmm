use std::{env, io::stdin};

use crate::utils::{init_memo_file, user_confirmation, MEMO_FILE_PATH_ENV_KEY};

fn clean_memo_file() {
    let filepath = env::var(MEMO_FILE_PATH_ENV_KEY).unwrap();
    match init_memo_file(filepath.clone().into()) {
        Ok(_) => println!("clean memo file to {}", filepath.as_str()),
        Err(err) => println!("error: {}", err),
    }
}

pub fn clean(force: bool) {
    if force {
        clean_memo_file();
    } else {
        println!("Are you sure you want to initialize the memo file? (yes/no)");
        let stdin = stdin();
        if user_confirmation(stdin.lock()) {
            clean_memo_file();
        }
    }
}
