use serde_json::ser::to_string;
use serde_json::{json, Result, Value};
use std::fs::File;
use std::io::{self, Read, Write};

// TODO: memoファイルのパスの指定の仕方を考える
pub const MEMO_FILE_PATH: &str = "$HOME/project/kvmemo/memo.json";

pub fn value_to_str_without_quotes(v: &Value) -> String {
    let mut options = to_string(v).unwrap();
    options.pop();
    options.remove(0);
    options
}

pub fn init_memo_file(file_path: &str) -> io::Result<()> {
    let mut file = File::create(file_path).unwrap();
    file.write_all("{}".as_bytes()).unwrap();
    file.flush().unwrap();
    Ok(())
}

pub fn read_file(file_path: &str) -> std::io::Result<String> {
    match File::open(file_path) {
        Ok(mut f) => {
            let mut contents = String::new();
            f.read_to_string(&mut contents).unwrap();
            f.flush().unwrap();
            Ok(contents)
        }
        Err(e) => {
            println!("{}", e);
            println!("init memo file({})", file_path);
            init_memo_file(file_path)?;
            Ok("{}".to_string())
        }
    }
}

pub fn write_file(file_path: &str, buf: &[u8]) -> std::io::Result<()> {
    let mut file = File::create(file_path).unwrap();
    file.write_all(buf).unwrap();
    file.flush().unwrap();
    Ok(())
}

pub fn user_confirmation() -> bool {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let input = input.trim().to_lowercase();

    input == "yes" || input == "y"
}

pub fn str_to_json(s: &str) -> std::io::Result<Value> {
    let json_value: Result<Value> = serde_json::from_str(s);
    match json_value {
        Ok(v) => Ok(v),
        Err(e) => {
            println!("The json format of the memo file is broken");
            println!("Do you init memo file? (yes/no)");
            if !user_confirmation() {
                println!("{}", e);
                println!("Please fix the memo file yourself of initialize it");
                panic!("force quit")
            }
            init_memo_file(MEMO_FILE_PATH)?;
            Ok(json!({}))
        }
    }
}

#[cfg(test)]
mod tests {
    mod value_to_str_without_quotes {
        use serde_json::json;

        use crate::commands::utils::value_to_str_without_quotes;

        #[test]
        fn success() {
            let v = json!("test");
            assert_eq!(value_to_str_without_quotes(&v), "test");
        }
    }
    mod init_memo_file {
        #[test]
        fn success() {}
    }
}
