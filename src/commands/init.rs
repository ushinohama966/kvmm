use super::utils::{init_memo_file, user_confirmation, MEMO_FILE_PATH};

pub fn init(force: bool) {
    if force {
        init_memo_file(MEMO_FILE_PATH);
        println!("init memo file to {}", MEMO_FILE_PATH);
    } else {
        println!("Are you sure you want to initialize the memo file? (yes/no)");
        if user_confirmation() {
            init_memo_file(MEMO_FILE_PATH);
            println!("init memo file to {}", MEMO_FILE_PATH);
        }
    }
}
