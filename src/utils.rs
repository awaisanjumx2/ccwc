use std::fs;

pub fn read_file(file: &String) -> Option<String> {
    let res = fs::read_to_string(file);
    match res {
        Ok(file_data) => Some(file_data),
        Err(_) => {
            println!("No such file: {}", file);
            None
        }
    }
}
