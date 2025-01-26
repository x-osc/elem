use winnow::{
    ascii::{self, space0, till_line_ending},
    combinator::{opt, preceded},
    prelude::*,
};

fn prop_val<'s>(input: &mut &'s str) -> PResult<(&'s str, &'s str)> {
    let prop = preceded("@", ascii::alphanumeric1).parse_next(input)?;
    let _ = (opt(space0), ":", opt(space0)).parse_next(input)?;
    let val = till_line_ending.parse_next(input)?;

    Ok((prop, val))
}

fn main() {
    let src = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let mut data = src.as_str();

    println!("{:?}", prop_val.parse_next(&mut data).unwrap())
}
