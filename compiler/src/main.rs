use winnow::{
    Result,
    ascii::{self, hex_digit1, line_ending, multispace0, space0},
    combinator::{alt, delimited, eof, opt, preceded, repeat, terminated},
    error::ParserError,
    prelude::*,
    token::{take_till, take_while},
};

#[derive(Debug)]
enum Stmt {
    Meta(Meta),
    Combination(Combination),
    CombinationWithElement(CombinationWithElement),
}

#[derive(Debug)]
struct Meta {
    prop: String,
    val: String,
}

#[derive(Debug)]
struct CombinationWithElement {
    pub element: Element,
    pub combination: Combination,
}

#[derive(Debug)]
struct Combination {
    pub a: String,
    pub b: String,
    pub result: String,
}

#[derive(Debug)]
struct Element {
    pub name: String,
    pub category: String,
}

fn parse_input(input: &mut &str) -> Result<Vec<Stmt>> {
    // parse stmts
    repeat(
        0..,
        terminated(
            delimited(
                multispace0,
                alt((
                    stmt.map(Some),
                    // TODO: make this not dumb
                    comment.map(|_| None), // single line comment
                )),
                space0,
            ),
            (
                opt(comment), // eol comment
                alt((line_ending, eof)),
            ),
        ),
    )
    // filter None values (comments)
    .map(|stmts: Vec<Option<Stmt>>| stmts.into_iter().flatten().collect())
    .parse_next(input)
}

fn stmt(input: &mut &str) -> Result<Stmt> {
    // the ordering is important or else a combination with element gets parsed
    // by combination() and combination_with_element() never gets called 
    alt((
        combination_with_element.map(Stmt::CombinationWithElement),
        combination.map(Stmt::Combination),
        meta.map(Stmt::Meta),
    ))
    .parse_next(input)
}

fn combination_with_element(input: &mut &str) -> Result<CombinationWithElement> {
    let combination = combination.parse_next(input)?;
    let _ = ascii::space1(input)?;
    let category = category_label.parse_next(input)?;
    let element_name = combination.result.clone();

    Ok(CombinationWithElement {
        element: Element {
            name: element_name,
            category: category.to_owned(),
        },
        combination,
    })
}

fn combination(input: &mut &str) -> Result<Combination> {
    alt((combination_res_first, combination_res_last)).parse_next(input)
}

fn combination_res_last(input: &mut &str) -> Result<Combination> {
    let (a, b, result) = (
        ws(element),
        preceded(ws('+'), element),
        preceded(ws('='), element),
    )
        .parse_next(input)?;

    Ok(Combination {
        a: a.to_string(),
        b: b.to_string(),
        result: result.to_string(),
    })
}

fn combination_res_first(input: &mut &str) -> Result<Combination> {
    let (result, a, b) = (
        ws(element),
        preceded(ws('='), element),
        preceded(ws('+'), element),
    )
        .parse_next(input)?;

    Ok(Combination {
        a: a.to_string(),
        b: b.to_string(),
        result: result.to_string(),
    })
}

/// label for a combination
fn category_label<'s>(input: &mut &'s str) -> Result<&'s str> {
    delimited('(', ws(name_with_spaces), ')').parse_next(input)
}

/// single element name parser
fn element<'s>(input: &mut &'s str) -> Result<&'s str> {
    alt((
        delimited(
            '"',
            take_while(1.., |c: char| {
                c != '"' && c != '\r' && c != '\n' && !c.is_control()
            }),
            '"',
        ),
        name_with_spaces,
    ))
    .parse_next(input)
}

fn name_with_spaces<'s>(input: &mut &'s str) -> Result<&'s str> {
    (
        take_while(1.., is_word_char),
        // have to specify the accumulator even though were gonna take() everything anyway
        repeat::<_, _, String, _, _>(0.., preceded(ascii::space1, take_while(1.., is_word_char))),
    )
        .take()
        .parse_next(input)
}

fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

fn comment(input: &mut &str) -> Result<()> {
    ('#', take_till(1.., ['\n', '\r'])).void().parse_next(input)
}

fn color<'s>(input: &mut &'s str) -> Result<&'s str> {
    let _ = '#'.parse_next(input)?;
    hex_digit1.verify(|s: &str| s.len() == 6).parse_next(input)
}

fn meta(input: &mut &str) -> Result<Meta> {
    let prop = preceded(
        "@",
        take_while(1.., |c: char| c.is_alphanumeric() || c == '_' || c == '.'),
    )
    .parse_next(input)?;
    let _ = (opt(space0), ":", opt(space0)).parse_next(input)?;
    let val = alt((
        delimited(
            '"',
            take_while(1.., |c: char| {
                c != '"' && c != '\r' && c != '\n' && !c.is_control()
            }),
            '"',
        ),
        take_while(1.., |c: char| c.is_alphanumeric() || c == '_' || c == ' ')
            .map(|s: &str| s.trim()),
    ))
    .parse_next(input)?;

    Ok(Meta {
        prop: prop.to_string(),
        val: val.to_string(),
    })
}

fn ws<'a, F, O, E: ParserError<&'a str>>(inner: F) -> impl Parser<&'a str, O, E>
where
    F: Parser<&'a str, O, E>,
{
    delimited(ascii::space0, inner, ascii::space0)
}

fn main() {
    let src = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let mut data = src.as_str();

    match parse_input.parse(&mut data) {
        Ok(res) => println!("{:?}", res),
        Err(err) => println!("{}", err),
    }

    println!("{}", data)
}
