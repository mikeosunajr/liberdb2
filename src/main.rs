use std::{fmt, iter::Peekable, str::CharIndices};

fn main() {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let res = super::Parser::new("1 + 2 * 3").parse();
        assert!(res.is_ok());
        assert_eq!("(+ 1 (* 2 3))", res.unwrap().to_string());
    }
}
#[derive(Debug, PartialEq)]
enum Operator {
    Multiply,
    Sum,
}
#[derive(Debug)]
enum ParseError {
    InvalidInteger,
}

type ParseResult = Result<Expression, ParseError>;

#[derive(Debug, PartialEq)]
enum Expression {
    Integer(u64),
    BinaryOperation(Box<Expression>, Operator, Box<Expression>),
}

fn infix_binding_power(op: &Operator) -> (u8, u8) {
    match op {
        Operator::Sum => (1, 2),
        Operator::Multiply => (3, 4),
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Integer(i) => write!(f, "{}", i),
            Expression::BinaryOperation(lhs, op, rhs) => write!(f, "({} {} {})", op, lhs, rhs),
        }
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operator::Sum => write!(f, "+"),
            Operator::Multiply => write!(f, "*"),
        }
    }
}

struct Parser<'a> {
    input: &'a str,
    chars: Peekable<CharIndices<'a>>,
}

trait Scanner {
    fn offset(&self) -> usize;
    fn peek(&self) -> Option<char>;
    fn next(&self) -> Option<char>;
    fn slice(&self, start: usize) -> &str;
}

impl<'a> Parser<'a> {
    pub fn new(str: &'a str) -> Self {
        Self {
            input: str,
            chars: str.char_indices().peekable(),
        }
    }

    pub fn parse(&mut self) -> ParseResult {
        self.expression(0)
    }

    pub fn expression(&mut self, min_bp: u8) -> ParseResult {
        self.ws();
        let mut lhs = self.number()?;
        self.ws();
        loop {
            if self.eof() {
                break;
            }
            let op = self.op()?;
            let (l_bp, r_bp) = infix_binding_power(&op);
            if l_bp < min_bp {
                break;
            }
            let rhs = self.expression(r_bp)?;

            lhs = Expression::BinaryOperation(Box::new(lhs), op, Box::new(rhs));
        }

        Ok(lhs)
    }

    fn op(&mut self) -> Result<Operator, ParseError> {
        match self.chars.next() {
            Some((_, b)) => match b {
                '+' => Ok(Operator::Sum),
                '*' => Ok(Operator::Multiply),
                _ => Err(ParseError::InvalidInteger),
            },
            None => Err(ParseError::InvalidInteger),
        }
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

    fn cc(&mut self) -> Option<char> {
        self.chars.peek().map(|(_, c)| *c)
    }

    fn eof(&mut self) -> bool {
        self.chars.peek().is_none()
    }

    fn number(&mut self) -> ParseResult {
        match self.chars.peek() {
            Some((_start, _ch)) => {
                let start = *_start;
                let mut ch = *_ch;
                while ch.is_digit(10) {
                    self.chars.next();

                    if let Some(_ch) = self.cc() {
                        ch = _ch;
                    } else {
                        break;
                    }
                }

                let number_string = self.get_slice(start);
                if number_string.is_empty() {
                    return Err(ParseError::InvalidInteger);
                }

                number_string
                    .parse::<u64>()
                    .map(|num| Expression::Integer(num))
                    .map_err(|_| ParseError::InvalidInteger)
            }
            None => Err(ParseError::InvalidInteger),
        }
    }
}
