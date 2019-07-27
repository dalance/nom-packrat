use nom::character::complete::char;
use nom::IResult;
use nom_packrat::{init, packrat_parser, storage};

// Declare storage used by packrat_parser
storage!(String);

// Apply packrat_parser by custom attribute
#[packrat_parser]
pub fn parser(s: &str) -> IResult<&str, String> {
    let (s, x) = char('a')(s)?;
    Ok((s, x.to_string()))
}

fn main() {
    let input = "a";

    // Initialize before parsing
    init!();
    let result = parser(input);

    println!("{:?}", result);
}
