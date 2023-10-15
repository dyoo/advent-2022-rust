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

fn main() {}

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
}
