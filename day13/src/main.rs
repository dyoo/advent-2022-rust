use std::iter::Peekable;
use std::str::Bytes;

#[derive(Debug, PartialEq)]
enum Data {
    Num(u32),
    List(Vec<Data>),
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

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut parser = Parser::new(Tokenizer::new(&input));
    while let Some(item) = parser.next() {
	println!("{:?}", item);
    }
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
}
