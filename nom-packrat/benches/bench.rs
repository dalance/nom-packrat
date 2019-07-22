#![feature(test)]

extern crate test;

use nom_packrat::{init, storage};
use test::Bencher;

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

fn init() {
    crate::PACKRAT_STORAGE.with(|storage| storage.borrow_mut().clear())
}

static STR_0PAIR: &str = "a";
static STR_1PAIR: &str = "(a)";
static STR_2PAIR: &str = "((a))";
static STR_3PAIR: &str = "(((a)))";
static STR_4PAIR: &str = "((((a))))";
static STR_5PAIR: &str = "(((((a)))))";
static STR_6PAIR: &str = "((((((a))))))";
static STR_7PAIR: &str = "(((((((a)))))))";

#[bench]
fn bench_0pair_packrat_parser(b: &mut Bencher) {
    b.iter(|| {
        init();
        packrat_parser::p1(STR_0PAIR)
    })
}

#[bench]
fn bench_0pair_packrat_parser_opt(b: &mut Bencher) {
    b.iter(|| {
        init();
        packrat_parser_opt::p1(STR_0PAIR)
    })
}

#[bench]
fn bench_0pair_non_packrat_parser(b: &mut Bencher) {
    b.iter(|| non_packrat_parser::p1(STR_0PAIR));
}

#[bench]
fn bench_1pair_packrat_parser(b: &mut Bencher) {
    b.iter(|| {
        init();
        packrat_parser::p1(STR_1PAIR)
    })
}

#[bench]
fn bench_1pair_packrat_parser_opt(b: &mut Bencher) {
    b.iter(|| {
        init();
        packrat_parser_opt::p1(STR_1PAIR)
    })
}

#[bench]
fn bench_1pair_non_packrat_parser(b: &mut Bencher) {
    b.iter(|| non_packrat_parser::p1(STR_1PAIR));
}

#[bench]
fn bench_2pair_packrat_parser(b: &mut Bencher) {
    b.iter(|| {
        init();
        packrat_parser::p1(STR_2PAIR)
    })
}

#[bench]
fn bench_2pair_packrat_parser_opt(b: &mut Bencher) {
    b.iter(|| {
        init();
        packrat_parser_opt::p1(STR_2PAIR)
    })
}

#[bench]
fn bench_2pair_non_packrat_parser(b: &mut Bencher) {
    b.iter(|| non_packrat_parser::p1(STR_2PAIR));
}

#[bench]
fn bench_3pair_packrat_parser(b: &mut Bencher) {
    b.iter(|| {
        init();
        packrat_parser::p1(STR_3PAIR)
    })
}

#[bench]
fn bench_3pair_packrat_parser_opt(b: &mut Bencher) {
    b.iter(|| {
        init();
        packrat_parser_opt::p1(STR_3PAIR)
    })
}

#[bench]
fn bench_3pair_non_packrat_parser(b: &mut Bencher) {
    b.iter(|| non_packrat_parser::p1(STR_3PAIR));
}

#[bench]
fn bench_4pair_packrat_parser(b: &mut Bencher) {
    b.iter(|| {
        init();
        packrat_parser::p1(STR_4PAIR)
    })
}

#[bench]
fn bench_4pair_packrat_parser_opt(b: &mut Bencher) {
    b.iter(|| {
        init();
        packrat_parser_opt::p1(STR_4PAIR)
    })
}

#[bench]
fn bench_4pair_non_packrat_parser(b: &mut Bencher) {
    b.iter(|| non_packrat_parser::p1(STR_4PAIR));
}

#[bench]
fn bench_5pair_packrat_parser(b: &mut Bencher) {
    b.iter(|| {
        init();
        packrat_parser::p1(STR_5PAIR)
    })
}

#[bench]
fn bench_5pair_packrat_parser_opt(b: &mut Bencher) {
    b.iter(|| {
        init();
        packrat_parser_opt::p1(STR_5PAIR)
    })
}

#[bench]
fn bench_5pair_non_packrat_parser(b: &mut Bencher) {
    b.iter(|| non_packrat_parser::p1(STR_5PAIR));
}

#[bench]
fn bench_6pair_packrat_parser(b: &mut Bencher) {
    b.iter(|| {
        init();
        packrat_parser::p1(STR_6PAIR)
    })
}

#[bench]
fn bench_6pair_packrat_parser_opt(b: &mut Bencher) {
    b.iter(|| {
        init();
        packrat_parser_opt::p1(STR_6PAIR)
    })
}

#[bench]
fn bench_6pair_non_packrat_parser(b: &mut Bencher) {
    b.iter(|| non_packrat_parser::p1(STR_6PAIR));
}

#[bench]
fn bench_7pair_packrat_parser(b: &mut Bencher) {
    b.iter(|| {
        init();
        packrat_parser::p1(STR_7PAIR)
    })
}

#[bench]
fn bench_7pair_packrat_parser_opt(b: &mut Bencher) {
    b.iter(|| {
        init();
        packrat_parser_opt::p1(STR_7PAIR)
    })
}

#[bench]
fn bench_7pair_non_packrat_parser(b: &mut Bencher) {
    b.iter(|| non_packrat_parser::p1(STR_7PAIR));
}
