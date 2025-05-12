use std::{env::args, fs::read_to_string};

use parser::parse_str;

mod parser;

fn main() {
    let Some(file_path) = args().nth(1) else {
        println!("ERROR: no argument supplied");
        return;
    };
    let Ok(src) = read_to_string(&file_path) else {
        println!("ERROR: could not read file at path {file_path}");
        return;
    };

    match parse_str(&src) {
        Ok(res) => println!("{res:?}"),
        Err(err) => println!("{err}"),
    }
}
