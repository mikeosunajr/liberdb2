use std::{iter::Peekable, ops::Range, str::CharIndices};

use crate::scanner::{integer, skip_ws, Scannable, ScanningErrors};

pub struct ParserState<'a> {
    code: &'a str,
    char_indices: Peekable<CharIndices<'a>>,
}

pub struct Mark {
    offset: Option<usize>,
}

impl<'a> Scannable<'a, Mark> for ParserState<'a> {
    fn peek(&mut self) -> Option<char> {
        self.char_indices.peek().map(|(_, c)| *c)
    }

    fn next(&mut self) -> Option<char> {
        self.char_indices.next().map(|(_, c)| c)
    }

    fn offset(&mut self) -> Option<usize> {
        self.char_indices.peek().map(|(o, _)| *o)
    }

    fn slice(&mut self, range: Range<usize>) -> Result<&'a str, ScanningErrors> {
        if range.end > self.code.len() - 1 {
            return Err(ScanningErrors::EOF);
        }
        return Ok(&self.code[range]);
    }

    fn mark(&mut self) -> Mark {
        Mark {
            offset: self.offset(),
        }
    }

    fn restore(&mut self, mark: Mark) {
        std::mem::replace(
            &mut self.char_indices,
            self.code[mark.offset.unwrap_or(0)..]
                .char_indices()
                .peekable(),
        );
    }
}

impl<'a> ParserState<'a> {
    pub fn new(code: &'a str) -> Self {
        Self {
            code,
            char_indices: code.char_indices().peekable(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Plus,
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Integer(i64),
    BinaryOperation(Box<Expression>, Operator, Box<Expression>),
}

#[derive(Debug, PartialEq)]
pub enum ParsingError {
    Invalid,
}

fn operator<'a, T>(code: &mut impl Scannable<'a, T>) -> Result<Operator, ParsingError> {
    if let Some(op) = code.peek() {
        if op == '+' {
            code.next();
            return Ok(Operator::Plus);
        }
    }

    Err(ParsingError::Invalid)
}

pub fn expression<'a, T>(code: &mut impl Scannable<'a, T>) -> Result<Expression, ParsingError> {
    skip_ws(code);

    let i = match integer(code) {
        Ok(i) => i
            .parse::<i64>()
            .map(|i| Expression::Integer(i))
            .map_err(|_| ParsingError::Invalid),
        Err(_) => Err(ParsingError::Invalid),
    };

    skip_ws(code);

    i
}

#[test]
fn it_works() {
    let mut code = ParserState::new(" 1 + 2 ");
    let mark = code.mark();
    let res = expression(&mut code);
    assert!(res.is_ok());
    assert_eq!(Expression::Integer(1), res.unwrap());

    code.restore(mark);

    assert_eq!(Ok(" 1 + 2 "), code.slice(0usize..));
}
