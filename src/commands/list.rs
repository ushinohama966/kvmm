use crate::utils::{get_memo_file_path, read_file, str_to_json, value_to_str_without_quotes};

pub fn list(line: bool) {
    let filepath = get_memo_file_path().unwrap();
    let file_str = read_file(&filepath).unwrap();
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
