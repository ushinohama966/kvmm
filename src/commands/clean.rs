use crate::utils::{get_memo_file_path, init_memo_file, user_confirmation};

fn clean_memo_file() {
    let filepath = get_memo_file_path().unwrap();
    match init_memo_file(&filepath) {
        Ok(_) => println!("clean memo file to {}", &filepath),
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
