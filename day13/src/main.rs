use std::cmp::Ordering;
use std::iter::Peekable;
use std::str::Bytes;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Data {
    Num(u32),
    List(Vec<Data>),
}

impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Data {
    fn cmp(&self, rhs: &Data) -> Ordering {
        match (self, rhs) {
            (Data::Num(lhs), Data::Num(rhs)) => lhs.cmp(rhs),
            (Data::Num(lhs), rhs @ Data::List(_)) => {
                Data::cmp(&Data::List(vec![Data::Num(*lhs)]), rhs)
            }
            (lhs @ Data::List(_), Data::Num(rhs)) => {
                Data::cmp(lhs, &Data::List(vec![Data::Num(*rhs)]))
            }
            (Data::List(lhs_items), Data::List(rhs_items)) => {
                let mut lhs_iter = lhs_items.iter();
                let mut rhs_iter = rhs_items.iter();

                loop {
                    match (lhs_iter.next(), rhs_iter.next()) {
                        (None, None) => {
                            return Ordering::Equal;
                        }
                        (None, Some(_)) => {
                            return Ordering::Less;
                        }
                        (Some(_), None) => {
                            return Ordering::Greater;
                        }
                        (Some(l), Some(r)) => match Data::cmp(l, r) {
                            Ordering::Less => {
                                return Ordering::Less;
                            }
                            Ordering::Greater => {
                                return Ordering::Greater;
                            }
                            Ordering::Equal => {}
                        },
                    }
                }
            }
        }
    }
}

#[derive(Debug, PartialEq)]
enum Token {
    Num(u32),
    Lbracket,
    Rbracket,
    Comma,
}

struct Tokenizer<'a> {
    peekable: Peekable<Bytes<'a>>,
}

impl<'a> Tokenizer<'a> {
    fn new(s: &'a str) -> Self {
        Tokenizer {
            peekable: s.bytes().peekable(),
        }
    }

    fn tokenize_number(&mut self) -> u32 {
        let mut n: u32 = 0;
        while let Some(digit @ b'0'..=b'9') = self.peekable.peek() {
            n = n * 10 + (digit - b'0') as u32;
            self.peekable.next();
        }
        n
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.peekable.peek() {
                Some(b'0'..=b'9') => {
                    return Some(Token::Num(self.tokenize_number()));
                }
                Some(b'[') => {
                    self.peekable.next();
                    return Some(Token::Lbracket);
                }
                Some(b']') => {
                    self.peekable.next();
                    return Some(Token::Rbracket);
                }
                Some(b',') => {
                    self.peekable.next();
                    return Some(Token::Comma);
                }
                None => {
                    return None;
                }

                _ => {
                    // Skip unknown characters.
                    self.peekable.next();
                }
            }
        }
    }
}

struct Parser<I>
where
    I: Iterator<Item = Token>,
{
    peekable: Peekable<I>,
}

impl<I> Parser<I>
where
    I: Iterator<Item = Token>,
{
    fn new(iter: I) -> Self {
        Self {
            peekable: iter.peekable(),
        }
    }
}

impl<I> Iterator for Parser<I>
where
    I: Iterator<Item = Token>,
{
    type Item = Data;

    fn next(&mut self) -> Option<Self::Item> {
        match self.peekable.peek() {
            Some(Token::Num(n)) => {
                let result = Some(Data::Num(*n));
                self.peekable.next();
                result
            }

            Some(Token::Lbracket) => {
                self.peekable.next();

                let mut data_items = Vec::new();
                loop {
                    // Recursive call: pick up items
                    if let Some(data_item) = self.next() {
                        data_items.push(data_item);
                    }
                    // Not great error handling up ahead.  In reality, we should
                    // take a look at nom.

                    // Consume separating commas
                    if let Some(Token::Comma) = self.peekable.peek() {
                        self.peekable.next();
                    }
                    // If the next item is a ']', finish reading items.
                    if let Some(Token::Rbracket) = self.peekable.peek() {
                        self.peekable.next();
                        break;
                    }
                }

                Some(Data::List(data_items))
            }
            _ => None,
        }
    }
}

fn part1(input: &str) -> i32 {
    let mut parser = Parser::new(Tokenizer::new(input));
    let mut index = 1;
    let mut sum = 0;
    while let (Some(l), Some(r)) = (parser.next(), parser.next()) {
        if Data::cmp(&l, &r).is_lt() {
            sum += index;
        }
        index += 1;
    }
    sum
}

fn part2(input: &str) -> Option<usize> {
    let mut items: Vec<Data> = Parser::new(Tokenizer::new(input)).collect();
    let divider1 = parse("[[2]]");
    let divider2 = parse("[[6]]");
    items.push(divider1.clone());
    items.push(divider2.clone());

    items.sort();

    let index1 = items.binary_search(&divider1);
    let index2 = items.binary_search(&divider2);
    Some(index1.map(|x| x + 1).ok()? * index2.map(|x| x + 1).ok()?)
}

fn parse(s: &str) -> Data {
    Parser::new(Tokenizer::new(s)).next().expect("a data")
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("part 1: {:?}", part1(&input));
    println!("part 2: {:?}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_number() {
        let input = "42";
        let mut tokenizer = Tokenizer::new(input);
        assert_eq!(tokenizer.next(), Some(Token::Num(42)));
        assert_eq!(tokenizer.next(), None);
    }

    #[test]
    fn test_tokenize_lbracket() {
        let input = "[";
        let mut tokenizer = Tokenizer::new(input);
        assert_eq!(tokenizer.next(), Some(Token::Lbracket));
        assert_eq!(tokenizer.next(), None);
    }

    #[test]
    fn test_tokenize_rbracket() {
        let input = "]";
        let mut tokenizer = Tokenizer::new(input);
        assert_eq!(tokenizer.next(), Some(Token::Rbracket));
        assert_eq!(tokenizer.next(), None);
    }

    #[test]
    fn test_tokenize_comma() {
        let input = ",";
        let mut tokenizer = Tokenizer::new(input);
        assert_eq!(tokenizer.next(), Some(Token::Comma));
        assert_eq!(tokenizer.next(), None);
    }

    #[test]
    fn test_tokenize_list() {
        let input = "[10,22,[301]]";
        let tokenizer = Tokenizer::new(input);
        assert_eq!(
            tokenizer.collect::<Vec<_>>(),
            vec![
                Token::Lbracket,
                Token::Num(10),
                Token::Comma,
                Token::Num(22),
                Token::Comma,
                Token::Lbracket,
                Token::Num(301),
                Token::Rbracket,
                Token::Rbracket,
            ]
        );
    }

    #[test]
    fn test_parse_number() {
        let input = "42";
        let tokenizer = Tokenizer::new(input);
        let mut parser = Parser::new(tokenizer);
        assert_eq!(parser.next(), Some(Data::Num(42)));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn test_parse_empty_list() {
        let input = "[]";
        let tokenizer = Tokenizer::new(input);
        let mut parser = Parser::new(tokenizer);
        assert_eq!(parser.next(), Some(Data::List(vec![])));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn test_parse_list() {
        let input = "[1, 2]";
        let tokenizer = Tokenizer::new(input);
        let mut parser = Parser::new(tokenizer);
        assert_eq!(
            parser.next(),
            Some(Data::List(vec![Data::Num(1), Data::Num(2)]))
        );
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn test_parse_nested_list() {
        let input = "[1, [2], 3]";
        let tokenizer = Tokenizer::new(input);
        let mut parser = Parser::new(tokenizer);
        assert_eq!(
            parser.next(),
            Some(Data::List(vec![
                Data::Num(1),
                Data::List(vec![Data::Num(2)]),
                Data::Num(3)
            ]))
        );
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn test_cmp_data() {
        assert_eq!(
            Data::cmp(&parse("[1,1,3,1,1]"), &parse("[1,1,5,1,1]")),
            Ordering::Less
        );

        assert_eq!(
            Data::cmp(&parse("[[1],[2,3,4]]"), &parse("[[1],4]")),
            Ordering::Less
        );

        assert_eq!(
            Data::cmp(&parse("[9]"), &parse("[[8,7,6]]")),
            Ordering::Greater
        );

        assert_eq!(
            Data::cmp(&parse("[[4,4],4,4]"), &parse("[[4,4],4,4,4]")),
            Ordering::Less
        );

        assert_eq!(
            Data::cmp(&parse("[7,7,7,7]"), &parse("[7,7,7]")),
            Ordering::Greater
        );

        assert_eq!(Data::cmp(&parse("[]"), &parse("[3]")), Ordering::Less);

        assert_eq!(
            Data::cmp(&parse("[[[]]]"), &parse("[[]]")),
            Ordering::Greater
        );

        assert_eq!(
            Data::cmp(
                &parse("[1,[2,[3,[4,[5,6,7]]]],8,9]"),
                &parse("[1,[2,[3,[4,[5,6,0]]]],8,9]")
            ),
            Ordering::Greater
        );

        assert_eq!(Data::cmp(&parse("[[2]]"), &parse("[[2]]")), Ordering::Equal);
        assert_eq!(Data::cmp(&parse("[[6]]"), &parse("[[6]]")), Ordering::Equal);
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(
                "
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"
            ),
            Some(140)
        );
    }
}
