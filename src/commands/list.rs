use crate::utils::{read_file, str_to_json, value_to_str_without_quotes, MEMO_FILE_PATH};

pub fn list(line: bool) {
    let file_str = read_file(MEMO_FILE_PATH).unwrap();
    let json_value = str_to_json(&file_str).unwrap();
    if line {
        let json_map = json_value.as_object().unwrap();
        for (k, v) in json_map.iter() {
            println!("{}={}", k, value_to_str_without_quotes(v));
        }
    } else {
        println!("{}", json_value);
    }
}
