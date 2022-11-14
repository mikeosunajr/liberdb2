use std::{iter::Peekable, str::CharIndices};

use crate::scanner::{digits, skip_ws, Scannable};

pub struct ParserState<'a> {
    code: &'a str,
    char_indices: Peekable<CharIndices<'a>>,
    offset: usize,
    row: u16,
    col: u16,
}

pub struct Mark {
    offset: usize,
    row: u16,
    col: u16,
}

impl<'a> Scannable<'a, Mark> for ParserState<'a> {
    fn peek(&mut self) -> Option<char> {
        self.char_indices.peek().map(|(_, c)| *c)
    }

    fn next(&mut self) -> Option<char> {
        let c = self.char_indices.next().map(|(_, c)| c);

        if let Some(c) = c {
            self.offset += c.len_utf8();
            if c == '\n' {
                self.col = 1;
                self.row += 1;
            }
        }

        c
    }

    fn substr(&mut self, mark: &Mark) -> &'a str {
        if let Some((cur, _)) = self.char_indices.peek() {
            &self.code[mark.offset..*cur]
        } else {
            &self.code[mark.offset..]
        }
    }

    fn mark(&mut self) -> Option<Mark> {
        self.char_indices.peek().map(|(_, _)| Mark {
            offset: self.offset,
            row: self.row,
            col: self.col,
        })
    }

    fn restore(&mut self, mark: &Mark) {
        self.char_indices = self.code[mark.offset..].char_indices().peekable();
        self.col = mark.col;
        self.row = mark.row;
    }
}

impl<'a> ParserState<'a> {
    pub fn new(code: &'a str) -> Self {
        Self {
            code,
            char_indices: code.char_indices().peekable(),
            row: 1,
            col: 1,
            offset: 0,
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

    let i = match digits(code) {
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
    let mark = code.mark().unwrap();
    let res = expression(&mut code);
    assert!(res.is_ok());
    assert_eq!(Expression::Integer(1), res.unwrap());

    assert_eq!(" 1 ", code.substr(&mark));

    code.restore(&mark);
}
