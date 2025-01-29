use winnow::{
    ascii::{self, line_ending, space0},
    combinator::{alt, eof, opt, preceded, repeat, repeat_till, terminated},
    prelude::*,
    token::{any, take, take_till},
};

#[derive(Debug)]
enum Stmt {
    Meta { prop: String, val: String },
}

fn comment(input: &mut &str) -> PResult<()> {
    ('#', take_till(1.., ['\n', '\r'])).void().parse_next(input)
}

fn take_till_end_or_comment<'s>(input: &mut &'s str) -> PResult<&'s str> {
    let val = take_till(1.., ['#', '\n', '\r']).parse_next(input)?;
    let _ = opt(comment).parse_next(input);
    Ok(val.trim())
}

fn parse_input(input: &mut &str) -> PResult<Vec<Stmt>> {
    let _ = ascii::multispace0.parse_next(input)?;
    repeat(0.., terminated(prop_val, opt(ascii::multispace0))).parse_next(input)
}

fn prop_val(input: &mut &str) -> PResult<Stmt> {
    let prop = preceded("@", ascii::alphanumeric1).parse_next(input)?;
    let _ = (opt(space0), ":", opt(space0)).parse_next(input)?;
    let val = take_till_end_or_comment.parse_next(input)?;

    Ok(Stmt::Meta {
        prop: prop.to_string(),
        val: val.to_string(),
    })
}

fn main() {
    let src = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let mut data = src.as_str();

    match parse_input.parse(&mut data) {
        Ok(res) => println!("{:?}", res),
        Err(err) => println!("{}", err),
    }

    println!("{:?}", data)
}
