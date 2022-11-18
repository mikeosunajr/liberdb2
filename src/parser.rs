use std::{collections::HashMap, iter::Peekable, str::CharIndices};

use crate::scanner::{digits, skip_ws, ErrorWithMark, Scannable};

struct Precedence {
    operator: Operator,
    lpb: u8,
    rpb: u8,
}

pub struct ParserState<'a> {
    code: &'a str,
    char_indices: Peekable<CharIndices<'a>>,
    offset: usize,
    row: u16,
    col: u16,
    operations: HashMap<&'a str, Precedence>,
}

#[derive(Debug, PartialEq)]
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

    fn mark(&mut self) -> Mark {
        return Mark {
            offset: self.offset,
            row: self.row,
            col: self.col,
        };
    }

    fn restore(&mut self, mark: &Mark) {
        self.char_indices = self.code[mark.offset..].char_indices().peekable();
        self.col = mark.col;
        self.row = mark.row;
    }
}

impl<'a> ParserState<'a> {
    pub fn infixl(&mut self, token: &'a str, operator: Operator, bp: u8) {
        self.operations.insert(
            token,
            Precedence {
                operator,
                lpb: bp,
                rpb: bp + 1,
            },
        );
    }

    pub fn new(code: &'a str) -> Self {
        let mut a = Self {
            code,
            char_indices: code.char_indices().peekable(),
            row: 1,
            col: 1,
            offset: 0,
            operations: HashMap::new(),
        };

        a.infixl("+", Operator::Plus, 1);
        a.infixl("*", Operator::Multiply, 3);

        a
    }
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Plus,
    Multiply,
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Integer(i64),
    BinaryOperation(Box<Expression>, Operator, Box<Expression>),
}

#[derive(Debug, PartialEq)]
pub enum ParsingError {
    Invalid(Mark),
}

fn infix<'a>(code: &mut ParserState) -> Result<Operator, ParsingError> {
    let res = match code.peek() {
        Some('+') => Ok(Operator::Plus),
        Some('*') => Ok(Operator::Multiply),
        _ => Err(ParsingError::Invalid(code.mark())),
    };

    code.next();

    res
}

fn integer(code: &mut ParserState) -> Result<Expression, ParsingError> {
    match digits(code) {
        Ok(i) => i
            .parse::<i64>()
            .map(|i| Expression::Integer(i))
            .map_err(|_| ParsingError::Invalid(code.mark())),
        Err(e) => Err(ParsingError::Invalid(e.mark())),
    }
}

pub fn expression_bp(code: &mut ParserState, min_bp: u64) -> Result<Expression, ParsingError> {
    skip_ws(code);

    // Get the prefix term
    let mut lhs = integer(code)?;

    loop {
        skip_ws(code);

        if let Ok(op) = infix(code) {
            let bp = op.infix_binding_power()?;
            if bp < min_bp {
                break;
            }

            let rhs = expression_bp(code, bp)?;

            lhs = Expression::BinaryOperation(Box::new(lhs), op, Box::new(rhs));

            continue;
        }

        break;
    }

    skip_ws(code);

    Ok(lhs)
}

#[test]
fn it_works() {
    let mut code = ParserState::new(" 1 + 2 * 3     ");
    let res = expression_bp(&mut code, 0);
    assert!(res.is_ok());
    assert_eq!(
        Expression::BinaryOperation(
            Box::new(Expression::Integer(1)),
            Operator::Plus,
            Box::new(Expression::BinaryOperation(
                Box::new(Expression::Integer(2)),
                Operator::Multiply,
                Box::new(Expression::Integer(3))
            ))
        ),
        res.unwrap()
    );
}
