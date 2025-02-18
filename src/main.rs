mod args;
mod commands;
mod utils;

use std::io::{self, Read};

use args::Args;
use clap::Parser;

use utils::{process_args, read_file};

fn main() {
    let args = Args::parse();
    match &args.file {
        Some(file) => {
            if let Some(file_data) = read_file(&file) {
                process_args(&args, &file_data);
                println!(" {}", file);
            }
        }
        None => {
            let mut input = String::new();
            io::stdin().read_to_string(&mut input).unwrap();
            process_args(&args, &input);
            println!("")
        }
    }
}
