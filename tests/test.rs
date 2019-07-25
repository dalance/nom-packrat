use nom::branch::*;
use nom::character::complete::*;
use nom::IResult;
use nom_packrat::{init, packrat_parser, storage};

storage!(Vec<String>);

#[packrat_parser]
pub fn p1(s: &str) -> IResult<&str, Vec<String>> {
    alt((p2, p3, p4))(s)
}

#[packrat_parser]
pub fn p2(s: &str) -> IResult<&str, Vec<String>> {
    let (s, mut x) = p4(s)?;
    let (s, y) = char('+')(s)?;
    let (s, mut z) = p1(s)?;
    let mut ret = Vec::new();
    ret.append(&mut x);
    ret.append(&mut vec![y.to_string()]);
    ret.append(&mut z);
    Ok((s, ret))
}

#[packrat_parser]
pub fn p3(s: &str) -> IResult<&str, Vec<String>> {
    let (s, mut x) = p4(s)?;
    let (s, y) = char('-')(s)?;
    let (s, mut z) = p1(s)?;
    let mut ret = Vec::new();
    ret.append(&mut x);
    ret.append(&mut vec![y.to_string()]);
    ret.append(&mut z);
    Ok((s, ret))
}

#[packrat_parser]
pub fn p4(s: &str) -> IResult<&str, Vec<String>> {
    alt((p5, p6))(s)
}

#[packrat_parser]
pub fn p5(s: &str) -> IResult<&str, Vec<String>> {
    let (s, x) = char('(')(s)?;
    let (s, mut y) = p1(s)?;
    let (s, z) = char(')')(s)?;
    let mut ret = Vec::new();
    ret.append(&mut vec![x.to_string()]);
    ret.append(&mut y);
    ret.append(&mut vec![z.to_string()]);
    Ok((s, ret))
}

#[packrat_parser]
pub fn p6(s: &str) -> IResult<&str, Vec<String>> {
    let (s, x) = char('a')(s)?;
    Ok((s, vec![x.to_string()]))
}

#[test]
fn it_works() {
    init!();
    let ret = p1("((a))");
    assert_eq!(
        "Ok((\"\", [\"(\", \"(\", \"a\", \")\", \")\"]))",
        format!("{:?}", ret)
    );
}
