use super::utils::{init_memo_file, user_confirmation, MEMO_FILE_PATH};

fn clean_memo_file() {
    match init_memo_file(MEMO_FILE_PATH) {
        Ok(_) => println!("clean memo file to {}", MEMO_FILE_PATH),
        Err(err) => println!("error: {}", err),
    }
}

pub fn clean(force: bool) {
    if force {
        clean_memo_file();
    } else {
        println!("Are you sure you want to initialize the memo file? (yes/no)");
        if user_confirmation() {
            clean_memo_file();
        }
    }
}
