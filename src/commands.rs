pub fn bytes_count(file_data: &String) {
    let count = file_data.bytes().count();
    print!("  {count}")
}

pub fn lines_count(file_data: &String) {
    let count = file_data.lines().count();
    print!("    {count}")
}

pub fn words_count(file_data: &String) {
    let count = file_data.split_ascii_whitespace().count();
    print!("   {count}")
}

pub fn characters_count(file_data: &String) {
    let count = file_data.chars().count();
    print!("  {count}")
}