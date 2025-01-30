use winnow::{
    ascii::{self, space0},
    combinator::{alt, delimited, opt, preceded, repeat},
    prelude::*,
    token::take_till,
};

#[derive(Debug)]
enum Stmt {
    Meta { prop: String, val: String },
}

fn comment(input: &mut &str) -> PResult<()> {
    ('#', take_till(1.., ['\n', '\r'])).void().parse_next(input)
}

fn take_till_end_or_comment<'s>(input: &mut &'s str) -> PResult<&'s str> {
    let val = take_till(1.., ['#', '\n', '\r']).parse_next(input)?; // dont consume #
    let _ = opt(comment).parse_next(input);
    Ok(val.trim())
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

fn parse_stmt(input: &mut &str) -> PResult<Stmt> {
    prop_val.parse_next(input)
}

fn parse_input(input: &mut &str) -> PResult<Vec<Stmt>> {
    // skip initial whitespace
    let _ = ascii::multispace0.parse_next(input)?;

    // parse stmts
    repeat(
        0..,
        delimited(
            ascii::multispace0,
            alt((
                comment.map(|_| None), // parse and ignore comments
                parse_stmt.map(Some),
            )),
            ascii::multispace0,
        ),
    )
    // filter None values (comments)
    .map(|stmts: Vec<Option<Stmt>>| stmts.into_iter().flatten().collect())
    .parse_next(input)
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
