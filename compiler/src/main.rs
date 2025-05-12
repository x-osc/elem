use std::{env::args, fs::read_to_string};

use parser::parse_str;
use save::stmts_to_json;

mod color;
mod parser;
mod save;

fn main() {
    let Some(file_path) = args().nth(1) else {
        println!("ERROR: no argument supplied");
        return;
    };
    let Ok(src) = read_to_string(&file_path) else {
        println!("ERROR: could not read file at path {file_path}");
        return;
    };

    let stmts = match parse_str(&src) {
        Ok(res) => res,
        Err(err) => {
            println!("{err}");
            return;
        }
    };

    let json_formatted = match stmts_to_json(stmts) {
        Ok(res) => res,
        Err(err) => {
            println!("{err}");
            return;
        }
    };
    
    println!("{json_formatted:#?}")
}
