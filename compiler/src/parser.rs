use std::result;

use winnow::{
    Result,
    ascii::{self, hex_digit1, line_ending, multispace0, space0},
    combinator::{alt, delimited, eof, opt, preceded, repeat, terminated},
    error::ParserError,
    prelude::*,
    token::{take_till, take_while},
};

#[derive(Debug, PartialEq, Eq)]
pub enum Stmt {
    Meta(Meta),
    Category(Category),
    Combination(Combination),
    Element(Element),
    CombinationWithElement(CombinationWithElement),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Meta {
    pub prop: String,
    pub val: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Category {
    pub name: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct CombinationWithElement {
    pub element: Element,
    pub combination: Combination,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Combination {
    pub a: String,
    pub b: String,
    pub result: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Element {
    pub name: String,
    pub category: String,
}

pub fn parse_str(input: &str) -> result::Result<Vec<Stmt>, String> {
    parse_input.parse(input).map_err(|err| format!("{err}"))
}

fn parse_input(input: &mut &str) -> Result<Vec<Stmt>> {
    // parse stmts
    repeat(
        0..,
        delimited(
            multispace0,
            terminated(
                alt((
                    stmt.map(Some),
                    // TODO: make this not dumb
                    comment.map(|_| None), // single line comment
                )),
                (
                    space0,
                    opt(comment), // eol comment
                    alt((line_ending, eof)),
                ),
            ),
            multispace0,
        ),
    )
    // filter None values (comments)
    .map(|stmts: Vec<Option<Stmt>>| stmts.into_iter().flatten().collect())
    .parse_next(input)
}

fn stmt(input: &mut &str) -> Result<Stmt> {
    // the ordering is important or else a combination with element gets parsed by
    // combination() and combination_with_element() never gets called
    alt((
        combination_with_element.map(Stmt::CombinationWithElement),
        combination.map(Stmt::Combination),
        element.map(Stmt::Element),
        category.map(Stmt::Category),
        meta.map(Stmt::Meta),
    ))
    .parse_next(input)
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

fn category(input: &mut &str) -> Result<Category> {
    let name = category_label.parse_next(input)?;

    Ok(Category {
        name: name.to_owned(),
    })
}

fn combination_with_element(input: &mut &str) -> Result<CombinationWithElement> {
    let combination = combination.parse_next(input)?;
    let _ = ascii::space1(input)?;
    let category = category_label.parse_next(input)?;
    let result_name = combination.result.clone();

    Ok(CombinationWithElement {
        element: Element {
            name: result_name,
            category: category.to_owned(),
        },
        combination,
    })
}

fn element(input: &mut &str) -> Result<Element> {
    let name = element_name.parse_next(input)?;
    let _ = ascii::space1(input)?;
    let category = category_label.parse_next(input)?;

    Ok(Element {
        name: name.to_owned(),
        category: category.to_owned(),
    })
}

fn combination(input: &mut &str) -> Result<Combination> {
    alt((combination_res_first, combination_res_last)).parse_next(input)
}

fn combination_res_last(input: &mut &str) -> Result<Combination> {
    let (a, b, result) = (
        ws(element_name),
        preceded(ws('+'), element_name),
        preceded(ws('='), element_name),
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
        ws(element_name),
        preceded(ws('='), element_name),
        preceded(ws('+'), element_name),
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
fn element_name<'s>(input: &mut &'s str) -> Result<&'s str> {
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

fn ws<'a, F, O, E: ParserError<&'a str>>(inner: F) -> impl Parser<&'a str, O, E>
where
    F: Parser<&'a str, O, E>,
{
    delimited(ascii::space0, inner, ascii::space0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_string() {
        let input = r#"


            @title: elem  # coment

                 (Earth)
            ( Fire Man  )

            Mist = Air + Water
            Wind      =      Air     +     Air        
            # epic comment   
            element ( category )
            Air    +    Earth    =    Dust     # moar comment    
              Earth + Fire = Lava (Fire) #even more comments
            Mud = Earth + Water       (  Earth )
            "String With Random Characters   !#$%$^&*" + Bob = "! # $ % $ ^ & *"
            element 2 ( category 2  ) # comments again

        "#;

        let result = match parse_str(input) {
            Ok(res) => res,
            #[allow(clippy::assertions_on_constants)]
            Err(err) => {
                assert!(false, "{err}");
                return;
            }
        };

        let target = vec![
            Stmt::Meta(Meta {
                prop: "title".into(),
                val: "elem".into(),
            }),
            Stmt::Category(Category {
                name: "Earth".into(),
            }),
            Stmt::Category(Category {
                name: "Fire Man".into(),
            }),
            Stmt::Combination(Combination {
                a: "Air".into(),
                b: "Water".into(),
                result: "Mist".into(),
            }),
            Stmt::Combination(Combination {
                a: "Air".into(),
                b: "Air".into(),
                result: "Wind".into(),
            }),
            Stmt::Element(Element {
                name: "element".into(),
                category: "category".into(),
            }),
            Stmt::Combination(Combination {
                a: "Air".into(),
                b: "Earth".into(),
                result: "Dust".into(),
            }),
            Stmt::CombinationWithElement(CombinationWithElement {
                element: Element {
                    name: "Lava".into(),
                    category: "Fire".into(),
                },
                combination: Combination {
                    a: "Earth".into(),
                    b: "Fire".into(),
                    result: "Lava".into(),
                },
            }),
            Stmt::CombinationWithElement(CombinationWithElement {
                element: Element {
                    name: "Mud".into(),
                    category: "Earth".into(),
                },
                combination: Combination {
                    a: "Earth".into(),
                    b: "Water".into(),
                    result: "Mud".into(),
                },
            }),
            Stmt::Combination(Combination {
                a: "String With Random Characters   !#$%$^&*".into(),
                b: "Bob".into(),
                result: "! # $ % $ ^ & *".into(),
            }),
            Stmt::Element(Element {
                name: "element 2".into(),
                category: "category 2".into(),
            }),
        ];

        assert_eq!(result, target);
    }
}
