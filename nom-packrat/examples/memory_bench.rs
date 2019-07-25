use nom_packrat::{init, storage};
use stats_alloc::{Region, StatsAlloc, INSTRUMENTED_SYSTEM};
use std::alloc::System;

#[global_allocator]
static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

storage!(Vec<String>);

mod packrat_parser {
    use nom::branch::*;
    use nom::character::complete::*;
    use nom::IResult;
    use nom_packrat::packrat_parser;

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

}

mod packrat_parser_opt {
    use nom::branch::*;
    use nom::character::complete::*;
    use nom::IResult;
    use nom_packrat::packrat_parser;

    pub fn p1(s: &str) -> IResult<&str, Vec<String>> {
        alt((p2, p3, p4))(s)
    }

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

    pub fn p6(s: &str) -> IResult<&str, Vec<String>> {
        let (s, x) = char('a')(s)?;
        Ok((s, vec![x.to_string()]))
    }

}

mod non_packrat_parser {
    use nom::branch::*;
    use nom::character::complete::*;
    use nom::IResult;

    pub fn p1(s: &str) -> IResult<&str, Vec<String>> {
        alt((p2, p3, p4))(s)
    }

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

    pub fn p4(s: &str) -> IResult<&str, Vec<String>> {
        alt((p5, p6))(s)
    }

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

    pub fn p6(s: &str) -> IResult<&str, Vec<String>> {
        let (s, x) = char('a')(s)?;
        Ok((s, vec![x.to_string()]))
    }

}

static STR_PAIRS: [&str; 8] = [
    "a",
    "(a)",
    "((a))",
    "(((a)))",
    "((((a))))",
    "(((((a)))))",
    "((((((a))))))",
    "(((((((a)))))))",
];

fn main() {
    for i in 0..8 {
        let reg = Region::new(&GLOBAL);
        let x = non_packrat_parser::p1(STR_PAIRS[i]);
        println!(
            "{}pair: original   : {:<4} bytes",
            i,
            reg.change().bytes_allocated - reg.change().bytes_deallocated
        );
        std::mem::size_of_val(&x);

        init!();
        let reg = Region::new(&GLOBAL);
        let x = packrat_parser::p1(STR_PAIRS[i]);
        println!(
            "{}pair: packrat    : {:<4} bytes",
            i,
            reg.change().bytes_allocated - reg.change().bytes_deallocated
        );
        std::mem::size_of_val(&x);

        init!();
        let reg = Region::new(&GLOBAL);
        let x = packrat_parser_opt::p1(STR_PAIRS[i]);
        println!(
            "{}pair: packrat_opt: {:<4} bytes",
            i,
            reg.change().bytes_allocated - reg.change().bytes_deallocated
        );
        std::mem::size_of_val(&x);
    }
}
