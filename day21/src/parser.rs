use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, digit1, multispace0},
    combinator::{map, map_res},
    sequence::{delimited, separated_pair, tuple},
    IResult,
};

#[derive(Debug, PartialEq, Eq)]
pub enum Expr<'a> {
    Num(i64),
    BinOp { op: Op, lhs: &'a str, rhs: &'a str },
}

#[derive(Debug, PartialEq, Eq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Job<'a>(pub &'a str, pub Expr<'a>);

pub fn parse_job(s: &str) -> IResult<&str, Job> {
    let parser = separated_pair(alpha1, tag(":"), parse_expr);
    let mut parser = map(parser, |(name, expr)| Job(name, expr));
    parser(s)
}

fn parse_expr(s: &str) -> IResult<&str, Expr> {
    let mut parser = alt((delimited(multispace0, parse_num, multispace0), parse_binop));
    parser(s)
}

fn parse_num(s: &str) -> IResult<&str, Expr> {
    let parser = map_res(digit1, |s: &str| s.parse::<i64>());
    let mut parser = map(parser, Expr::Num);
    parser(s)
}

fn parse_binop(s: &str) -> IResult<&str, Expr> {
    let parser = tuple((
        delimited(multispace0, alpha1, multispace0),
        delimited(multispace0, parse_op, multispace0),
        delimited(multispace0, alpha1, multispace0),
    ));
    let mut parser = map(parser, |(lhs, op, rhs): (&str, Op, &str)| Expr::BinOp {
        op,
        lhs,
        rhs,
    });
    parser(s)
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
fn test_parse_binop() {
    assert_eq!(
        parse_binop("abcd+cdef"),
        Ok((
            "",
            Expr::BinOp {
                op: Op::Add,
                lhs: "abcd",
                rhs: "cdef"
            }
        ))
    );
}

#[test]
fn test_parse_binop_spaces() {
    assert_eq!(
        parse_binop("abcd + cdef"),
        Ok((
            "",
            Expr::BinOp {
                op: Op::Add,
                lhs: "abcd",
                rhs: "cdef"
            }
        ))
    );
}

#[test]
fn test_parse_number() {
    assert_eq!(parse_num("42:"), Ok((":", Expr::Num(42))));
}

#[test]
fn test_parse_op() {
    assert_eq!(parse_op("+ abcd"), Ok((" abcd", Op::Add)));
}

#[test]
fn test_parse_expr() {
    assert_eq!(parse_expr("1234"), Ok(("", Expr::Num(1234))));
    assert_eq!(
        parse_expr("abcd/xyz"),
        Ok((
            "",
            Expr::BinOp {
                op: Op::Div,
                lhs: "abcd",
                rhs: "xyz"
            }
        ))
    );
}

#[test]
fn test_parse_job_num() {
    assert_eq!(parse_job("dbpl: 5"), Ok(("", Job("dbpl", Expr::Num(5)))));
}

#[test]
fn test_parse_job_add() {
    assert_eq!(
        parse_job("root: pppw + sjmn"),
        Ok((
            "",
            Job(
                "root",
                Expr::BinOp {
                    op: Op::Add,
                    lhs: "pppw",
                    rhs: "sjmn"
                }
            )
        ))
    );
}
