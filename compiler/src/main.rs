use std::{env::args, fs::read_to_string};

use parser::parse_str;
use data::stmts_to_data;
use save::{data_to_files, JsonFiles};

mod color;
mod parser;
mod data;
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

    let data = match stmts_to_data(stmts) {
        Ok(res) => res,
        Err(err) => {
            println!("{err}");
            return;
        }
    };

    let JsonFiles { elements, categories, combinations } = match data_to_files(data) {
        Ok(files) => files,
        Err(err) => {
            println!("{err}");
            return;
        }
    };

    println!("{elements}\n{categories}\n{combinations}")
}
