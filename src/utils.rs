use crate::args::Args;
use crate::commands::{bytes_count, characters_count, lines_count, words_count};
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

pub fn process_args(args: &Args, data: &String) {
    if args.lines || args.no_flags_passed() {
        lines_count(&data);
    }
    if args.words || args.no_flags_passed() {
        words_count(&data);
    }
    if args.bytes || args.no_flags_passed() {
        bytes_count(&data);
    }
    if args.characters {
        characters_count(&data);
    }
}
