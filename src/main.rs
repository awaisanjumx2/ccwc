mod args;
mod commands;
mod utils;

use args::Args;
use clap::Parser;
use commands::{bytes_count, characters_count, lines_count, words_count};
use utils::read_file;

fn main() {
    let args = Args::parse();
    if let Some(file_data) = read_file(&args.file) {
        if args.lines || args.no_flags_passed() {
            lines_count(&file_data);
        }
        if args.words || args.no_flags_passed() {
            words_count(&file_data);
        }
        if args.bytes || args.no_flags_passed() {
            bytes_count(&file_data);
        }
        if args.characters {
            characters_count(&file_data);
        }
        println!(" {}", args.file);
    }
}
