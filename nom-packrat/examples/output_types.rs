use nom::character::complete::char;
use nom::IResult;
use nom_packrat::{init, packrat_parser, storage};
use std::convert::TryFrom;

// Declare type for packrat_parser storage
#[derive(Clone, Debug)]
pub enum Output {
    A(A),
    AB(AB),
}

// Declare types for parser output
#[derive(Clone, Debug)]
pub struct A(String);
#[derive(Clone, Debug)]
pub struct AB((A, String));

// Declare converters
impl From<A> for Output {
    fn from(x: A) -> Self {
        Output::A(x)
    }
}

impl From<AB> for Output {
    fn from(x: AB) -> Self {
        Output::AB(x)
    }
}

impl TryFrom<Output> for A {
    type Error = ();
    fn try_from(x: Output) -> Result<Self, Self::Error> {
        match x {
            Output::A(x) => Ok(x),
            _ => Err(()),
        }
    }
}

impl TryFrom<Output> for AB {
    type Error = ();
    fn try_from(x: Output) -> Result<Self, Self::Error> {
        match x {
            Output::AB(x) => Ok(x),
            _ => Err(()),
        }
    }
}

// Declare storage used by packrat_parser
storage!(Output);

// Apply packrat_parser by custom attribute
#[packrat_parser]
pub fn parser_a(s: &str) -> IResult<&str, A> {
    let (s, x) = char('a')(s)?;
    Ok((s, A(x.to_string())))
}

// Apply packrat_parser by custom attribute
#[packrat_parser]
pub fn parser_ab(s: &str) -> IResult<&str, AB> {
    let (s, x) = parser_a(s)?;
    let (s, y) = char('b')(s)?;
    Ok((s, AB((x, y.to_string()))))
}

fn main() {
    let input = "ab";

    // Initialize before parsing
    init!();
    let result = parser_ab(input);

    println!("{:?}", result);
}
