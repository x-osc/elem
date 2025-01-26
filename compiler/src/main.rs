use winnow::{
    ascii::{self, space0},
    combinator::{opt, preceded, repeat, terminated},
    prelude::*,
};

fn elem_parser<'s>(input: &mut &'s str) -> PResult<Vec<(&'s str, &'s str)>> {
    let _ = ascii::multispace0.parse_next(input)?;
    repeat(0.., terminated(prop_val, opt(ascii::multispace0))).parse_next(input)
}

fn prop_val<'s>(input: &mut &'s str) -> PResult<(&'s str, &'s str)> {
    let prop = preceded("@", ascii::alphanumeric1).parse_next(input)?;
    let _ = (opt(space0), ":", opt(space0)).parse_next(input)?;
    let val = ascii::till_line_ending.parse_next(input)?;

    Ok((prop, val))
}

fn main() {
    let src = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let mut data = src.as_str();

    println!("{:?}", elem_parser.parse(data).unwrap())
}
