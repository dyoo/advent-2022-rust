use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, digit1},
    combinator::{map, map_res},
    sequence::Tuple,
    IResult,
};

#[derive(Debug, PartialEq, Eq)]
pub enum Expr<'a> {
    Num(i32),
    Var(&'a str),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

fn parse_num(s: &str) -> IResult<&str, Expr> {
    let mut parser = map_res(digit1, |s: &str| s.parse::<i32>().map(Expr::Num));
    parser(s)
}

fn parse_colon(s: &str) -> IResult<&str, &str> {
    tag(":")(s)
}

fn parse_var(s: &str) -> IResult<&str, Expr> {
    map(alpha1, Expr::Var)(s)
}

fn parse_op(s: &str) -> IResult<&str, Op> {
    let parser = alt((char('+'), char('-'), char('*'), char('/')));
    let mut parser = map(parser, |op: char| match op {
        '+' => Op::Add,
        '-' => Op::Sub,
        '*' => Op::Mul,
        '/' => Op::Div,
        _ => panic!(),
    });

    parser(s)
}

#[test]
fn test_parse_number() {
    assert_eq!(parse_num("42:"), Ok((":", Expr::Num(42))));
}
#[test]
fn test_parse_var() {
    assert_eq!(parse_var("abcd:"), Ok((":", Expr::Var("abcd"))));
}

#[test]
fn test_parse_colon() {
    assert_eq!(parse_colon(": 42"), Ok((" 42", ":")));
}

#[test]
fn test_parse_op() {
    assert_eq!(parse_op("+ abcd"), Ok((" abcd", Op::Add)));
}
