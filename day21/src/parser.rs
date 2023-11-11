use nom::{bytes::complete::tag, character::complete::digit1, IResult};

pub enum Term {
    Number,
    Variable,
}

fn parse_number(s: &str) -> IResult<&str, &str> {
    digit1(s)
}

fn parse_colon(s: &str) -> IResult<&str, &str> {
    tag(":")(s)
}

#[test]
fn test_parse_number() {
    assert_eq!(parse_number("42:"), Ok((":", "42")));
}

#[test]
fn test_parse_colon() {
    assert_eq!(parse_colon(": 42"), Ok((" 42", ":")));
}
