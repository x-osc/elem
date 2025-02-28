use winnow::{
    ascii::{self, hex_digit1, line_ending, space0},
    combinator::{alt, delimited, opt, preceded, repeat, repeat_till},
    error::ParserError,
    prelude::*,
    token::{take_till, take_until, take_while},
};

#[derive(Debug)]
enum Stmt {
    Meta { prop: String, val: String },
}

#[derive(Debug)]
struct Combination {
    pub a: String,
    pub b: String,
    pub result: String,
}

// fn parse_input(input: &mut &str) -> PResult<Vec<Stmt>> {
//     // skip initial whitespace
//     let _ = ascii::multispace0.parse_next(input)?;

//     // parse stmts
//     repeat(
//         0..,
//         delimited(
//             ascii::multispace0,
//             alt((
//                 comment.map(|_| None), // parse and ignore comments
//                 parse_stmt.map(Some),
//             )),
//             ascii::multispace0,
//         ),
//     )
//     // filter None values (comments)
//     .map(|stmts: Vec<Option<Stmt>>| stmts.into_iter().flatten().collect())
//     .parse_next(input)
// }

fn combination<'s>(input: &mut &'s str) -> PResult<Combination> {
    let (result, a, b) = (
        ws(element),
        preceded(ws('='), ws(element)),
        preceded(ws('+'), ws(element)),
    )
        .parse_next(input)?;

    let _ = opt(line_ending).parse_next(input)?;

    Ok(Combination {
        a: a.to_string(),
        b: b.to_string(),
        result: result.to_string(),
    })
}

fn element<'s>(input: &mut &'s str) -> PResult<&'s str> {
    alt((
        delimited('"', take_while(1.., |c| c != '"'), '"'),
        take_while(1.., |c: char| c.is_alphanumeric() || c == '_'),
    ))
    .parse_next(input)
}

fn comment(input: &mut &str) -> PResult<()> {
    ("//", take_till(1.., ['\n', '\r']))
        .void()
        .parse_next(input)
}

// fn take_till_end_or_comment<'s>(input: &mut &'s str) -> PResult<&'s str> {
//     let val = repeat_till(1.., alt(comment, line_ending)).parse_next(input)?;
//     let _ = opt(comment).parse_next(input);
//     Ok(val.trim())
// }

fn color<'s>(input: &mut &'s str) -> PResult<&'s str> {
    let _ = '#'.parse_next(input)?;
    hex_digit1.verify(|s: &str| s.len() == 6).parse_next(input)
}

// fn prop_val(input: &mut &str) -> PResult<Stmt> {
//     let prop = preceded("@", ascii::alphanumeric1).parse_next(input)?;
//     let _ = (opt(space0), ":", opt(space0)).parse_next(input)?;
//     let val = take_till_end_or_comment.parse_next(input)?;

//     Ok(Stmt::Meta {
//         prop: prop.to_string(),
//         val: val.to_string(),
//     })
// }

// fn parse_stmt(input: &mut &str) -> PResult<Stmt> {
//     prop_val.parse_next(input)
// }

fn ws<'a, F, O, E: ParserError<&'a str>>(inner: F) -> impl Parser<&'a str, O, E>
where
    F: Parser<&'a str, O, E>,
{
    delimited(ascii::space0, inner, ascii::space0)
}

fn main() {
    let src = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let mut data = src.as_str();

    match combination.parse(&mut data) {
        Ok(res) => println!("{:?}", res),
        Err(err) => println!("{}", err),
    }

    println!("{}", data)
}
