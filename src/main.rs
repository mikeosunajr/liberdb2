use std::{iter::Peekable, str::CharIndices};

fn main() {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let res = super::Parser::new("00000112   dsfasdf").parse();
        assert!(res.is_ok());
        assert_eq!(super::Expression::Integer(112), res.unwrap());
    }
}

// enum Operator {
//     Multiply,
//     Sum,
// }
#[derive(Debug)]
enum ParseError {
    InvalidInteger,
}

type ParseResult = Result<Expression, ParseError>;

#[derive(Debug, PartialEq)]
enum Expression {
    Integer(u64),
    //BinaryOperation(Box<Expression>, Operator, Box<Expression>),
}

struct Parser<'a> {
    input: &'a str,
    chars: Peekable<CharIndices<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(str: &'a str) -> Self {
        Self {
            input: str,
            chars: str.char_indices().peekable(),
        }
    }

    pub fn parse(&mut self) -> ParseResult {
        self.ws();
        let num = self.number()?;
        self.ws();

        return Ok(num);
    }

    fn get_slice(&mut self, start: usize) -> &str {
        match self.chars.peek() {
            Some((end, _)) => &self.input[start..*end],
            None => &self.input[start..],
        }
    }

    fn ws(&mut self) {
        while let Some((_, _ch)) = self.chars.peek() {
            if !_ch.is_whitespace() {
                break;
            }
            self.chars.next();
        }
    }

    fn number(&mut self) -> ParseResult {
        if let Some((_start, _ch)) = self.chars.peek() {
            let start = *_start;
            let mut ch = *_ch;
            while ch.is_digit(10) {
                self.chars.next();

                if let Some((_, _ch)) = self.chars.peek() {
                    ch = *_ch;
                } else {
                    break;
                }
            }

            let number_string = self.get_slice(start);

            if let Ok(num) = number_string.parse::<u64>() {
                return Ok(Expression::Integer(num));
            } else {
                return Err(ParseError::InvalidInteger);
            }
        } else {
            return Err(ParseError::InvalidInteger);
        }
    }
}
