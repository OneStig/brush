mod tokens;
mod lexer;
mod ast;

use std::fs::File;
use std::io::Read;
use std::env;
use crate::tokens::{code_to_token};

fn open_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut rdin = String::new();
    file.read_to_string(&mut rdin)?;
    Ok(rdin)
}

fn main() {
    let mut args = env::args();

    if args.len() > 1 {
        let filename: String = args.nth(1).unwrap();

        match open_file(filename.as_str()) {
            Ok(raw) => {
                println!("reading content from {}:\n{}", filename, raw);
                code_to_token(raw);
            }

            Err(e) => {
                println!("failed to run {}: {}", filename, e);
            }
        }
    }
}