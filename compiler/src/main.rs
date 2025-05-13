use std::{
    env::args,
    fs::{self},
};

use data::stmts_to_data;
use parser::parse_str;
use save::{JsonFiles, data_to_files};

mod color;
mod data;
mod parser;
mod save;

fn main() {
    let Some(file_path) = args().nth(1) else {
        println!("ERROR: no argument supplied");
        return;
    };
    let Ok(src) = fs::read_to_string(&file_path) else {
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

    let JsonFiles {
        elements,
        categories,
        combinations,
    } = match data_to_files(&data) {
        Ok(files) => files,
        Err(err) => {
            println!("{err}");
            return;
        }
    };

    fs::write("elements.json", &elements).unwrap();
    fs::write("categories.json", &categories).unwrap();
    fs::write("combinations.json", &combinations).unwrap();

    println!("wrote {} categories, {} elements, and {} combinations", data.categories.len(), data.elements.len(), data.combinations.len());
}
