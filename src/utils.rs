use serde_json::ser::to_string;
use serde_json::Value;
use std::fs::File;
use std::io::{self, BufRead, Read, Write};
use std::path::PathBuf;

pub const MEMO_FILE_PATH_ENV_KEY: &str = "MEMO_FILE_PATH";

pub fn value_to_str_without_quotes(v: &Value) -> String {
    let mut options = to_string(v).unwrap();
    options.pop();
    options.remove(0);
    options
}

pub fn init_memo_file(file_path: PathBuf) -> io::Result<()> {
    let mut file = File::create::<PathBuf>(file_path)?;
    file.write_all("{}".as_bytes())?;
    file.flush()?;
    Ok(())
}

pub fn read_file(file_path: PathBuf) -> std::io::Result<String> {
    match File::open::<PathBuf>(file_path.clone()) {
        Ok(mut f) => {
            let mut contents = String::new();
            f.read_to_string(&mut contents)?;
            f.flush()?;
            Ok(contents)
        }
        Err(e) => {
            println!("{}", e);
            println!("init memo file({})", file_path.to_str().unwrap());
            init_memo_file(file_path)?;
            Ok("{}".to_string())
        }
    }
}

pub fn write_file(file_path: PathBuf, buf: &[u8]) -> std::io::Result<()> {
    let mut file = File::create(file_path)?;
    file.write_all(buf)?;
    file.flush()?;
    Ok(())
}

pub fn user_confirmation<R: BufRead>(mut r: R) -> bool {
    let mut input = String::new();
    r.read_line(&mut input).expect("Failed to read input");
    let input = input.trim().to_lowercase();

    input == "yes" || input == "y"
}

pub fn str_to_json(s: &str) -> serde_json::Result<Value> {
    let json_value: serde_json::Result<Value> = serde_json::from_str(s);
    json_value
}

#[cfg(test)]
mod tests {
    mod value_to_str_without_quotes {
        use serde_json::json;

        use crate::utils::value_to_str_without_quotes;

        #[test]
        fn should_success() {
            let v = json!("test");
            assert_eq!(value_to_str_without_quotes(&v), "test");
        }
    }
    mod init_memo_file {

        use std::path::PathBuf;

        use crate::utils::init_memo_file;

        #[test]
        fn should_success() {
            let file_path = "./test.json";
            let result = init_memo_file(file_path.into());
            assert_eq!(result.unwrap(), ());
            assert_eq!(
                std::fs::remove_file::<PathBuf>(file_path.into()).unwrap(),
                ()
            )
        }
        #[test]
        fn should_error_when_invvalid_path() {
            let file_path = "./test/test.json";
            let result = init_memo_file(file_path.into());
            assert!(result.is_err());
        }
    }
    mod read_file {
        use std::io::Write;
        use std::path::PathBuf;

        use crate::utils::read_file;

        #[test]
        fn should_success() {
            let file_path = "./test.txt";
            let test_file = std::fs::File::create::<PathBuf>(file_path.into());
            assert!(test_file.is_ok());
            let test_value = "test";
            assert_eq!(
                test_file.unwrap().write_all(test_value.as_bytes()).unwrap(),
                ()
            );
            let result = read_file(file_path.into());
            assert_eq!(result.unwrap(), test_value);
            assert_eq!(
                std::fs::remove_file::<PathBuf>(file_path.into()).unwrap(),
                ()
            )
        }
        #[test]
        fn should_error_when() {
            let file_path = "./test/test.txt";
            let result = read_file(file_path.into());
            assert!(result.is_err());
        }
    }
    mod write_file {
        use std::{io::Read, path::PathBuf};

        use crate::utils::write_file;

        #[test]
        fn should_success() {
            let file_path = "./test.json";
            assert!(std::fs::File::create::<PathBuf>(file_path.into()).is_ok());
            let test_value = "test";
            let result = write_file(file_path.into(), test_value.as_bytes());
            assert_eq!(result.unwrap(), ());
            let mut act_file_str = String::new();
            assert!(std::fs::File::open::<PathBuf>(file_path.into())
                .unwrap()
                .read_to_string(&mut act_file_str)
                .is_ok());
            assert_eq!(act_file_str, test_value);
            assert_eq!(
                std::fs::remove_file::<PathBuf>(file_path.into()).unwrap(),
                ()
            )
        }
        #[test]
        fn should_error_when_invalid_file_path() {
            let file_path = "./test/test.json";
            let test_value = "test";
            let result = write_file(file_path.into(), test_value.as_bytes());
            assert!(result.is_err());
        }
    }
    mod user_confirmation {
        use crate::utils::user_confirmation;

        #[test]
        fn should_success_when_input_lowercase_yes() {
            let test_input_value = "yes";
            let user_input = user_confirmation(test_input_value.as_bytes());
            assert!(user_input);
        }
        #[test]
        fn should_success_when_lowercase_y() {
            let test_input_value = "y";
            let user_input = user_confirmation(test_input_value.as_bytes());
            assert!(user_input);
        }
        #[test]
        fn should_success_when_uppercase_yes() {
            let test_input_values = ["Yes", "yEs", "yeS", "YEs", "yES", "YeS"];
            for input in test_input_values {
                let user_input = user_confirmation(input.as_bytes());
                assert!(user_input);
            }
        }
        #[test]
        fn should_success_when_uppercase_y() {
            let test_input_value = "Y";
            let user_input = user_confirmation(test_input_value.as_bytes());
            assert!(user_input);
        }
        #[test]
        fn should_error_when_other_than_yes() {
            let test_input_value = "no";
            let user_input = user_confirmation(test_input_value.as_bytes());
            assert!(!user_input);
        }
    }
    mod str_to_json {
        use serde_json::Value;

        use crate::utils::str_to_json;

        #[test]
        fn should_success() {
            let test_value = "{\"value\": 20}";
            let exp_value: serde_json::Result<Value> = serde_json::from_str(test_value);
            assert!(exp_value.is_ok());
            let act_value = str_to_json(test_value);
            assert!(act_value.is_ok());
            assert_eq!(exp_value.unwrap(), act_value.unwrap());
        }
        #[test]
        fn should_error_when_invalid_json_format() {
            let test_value = "test";
            let act_value = str_to_json(test_value);
            assert!(act_value.is_err());
        }
    }
}
