use winnow::{
    ascii::{self, line_ending, space0},
    combinator::{alt, eof, opt, preceded, repeat, repeat_till, terminated},
    prelude::*,
    token::{any, take, take_till},
};

pub fn comment(input: &mut &str) -> PResult<()> {
    ('#', take_till(1.., ['\n', '\r'])).void().parse_next(input)
}

fn take_till_end_or_comment<'s>(input: &mut &'s str) -> PResult<&'s str> {
    let val = take_till(1.., ['#', '\n', '\r']).parse_next(input)?;
    let _ = opt(comment).parse_next(input);
    Ok(val.trim())
}

fn elem_parser<'s>(input: &mut &'s str) -> PResult<Vec<(&'s str, &'s str)>> {
    let _ = ascii::multispace0.parse_next(input)?;
    repeat(0.., terminated(prop_val, opt(ascii::multispace0))).parse_next(input)
}

fn prop_val<'s>(input: &mut &'s str) -> PResult<(&'s str, &'s str)> {
    let prop = preceded("@", ascii::alphanumeric1).parse_next(input)?;
    let _ = (opt(space0), ":", opt(space0)).parse_next(input)?;
    let val = take_till_end_or_comment.parse_next(input)?;

    Ok((prop, val))
}

fn main() {
    let src = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let mut data = src.as_str();

    match elem_parser.parse(&mut data) {
        Ok(res) => println!("{:?}", res),
        Err(err) => println!("{}", err),
    }

    println!("{:?}", data)
}
