use std::{iter::Peekable, str::CharIndices};

fn main() {}

#[cfg(test)]
mod tests {
    use crate::Scannable;

    #[test]
    fn it_works() {
        let mut code = super::ParserState::new("1234a");
        let res = super::integer(&mut code);
        assert!(res.is_ok());
        assert_eq!("1234", res.unwrap());
        assert_eq!(Some('a'), code.peek());
        assert_eq!(Some('a'), code.next());
        assert_eq!(None, code.peek());
        assert_eq!(None, code.next());
        code.reset(0);
        assert_eq!(Some('1'), code.peek());
    }
}

trait Parser<'a, Input, Output, Problem>
where
    Input: Scannable<'a>,
{
    fn parse(&mut self, code: &mut Input) -> Result<Output, Problem>;
}

impl<'a, Input, Output, Problem, F> Parser<'a, Input, Output, Problem> for F
where
    Input: Scannable<'a>,
    F: FnMut(&mut Input) -> Result<Output, Problem>,
{
    fn parse(&mut self, state: &mut Input) -> Result<Output, Problem> {
        self(state)
    }
}
#[derive(Debug, PartialEq)]
pub enum ScanningErrors {
    NotADigit,
    EOF,
}

pub trait Scannable<'a> {
    fn peek(&mut self) -> Option<char>;
    fn next(&mut self) -> Option<char>;
    fn slice(&mut self, start: usize) -> &'a str;
    fn offset(&mut self) -> Option<usize>;
    fn reset(&mut self, offset: usize);
}

struct ParserState<'a> {
    code: &'a str,
    char_indices: Peekable<CharIndices<'a>>,
}

impl<'a> Scannable<'a> for ParserState<'a> {
    fn peek(&mut self) -> Option<char> {
        self.char_indices.peek().map(|(_, c)| *c)
    }

    fn next(&mut self) -> Option<char> {
        self.char_indices.next().map(|(_, c)| c)
    }

    fn slice(&mut self, start: usize) -> &'a str {
        if let Some((offset, _)) = self.char_indices.peek() {
            return &self.code[start..*offset];
        }
        &self.code[start..]
    }

    fn offset(&mut self) -> Option<usize> {
        self.char_indices.peek().map(|(o, _)| *o)
    }

    #[allow(unused_must_use)]
    fn reset(&mut self, offset: usize) {
        std::mem::replace(
            &mut self.char_indices,
            self.code[offset..].char_indices().peekable(),
        );
    }
}

impl<'a> ParserState<'a> {
    fn new(code: &'a str) -> Self {
        Self {
            code,
            char_indices: code.char_indices().peekable(),
        }
    }
}

pub fn digit<'a>(code: &mut impl Scannable<'a>) -> Result<char, ScanningErrors> {
    if let Some(c) = code.peek() {
        if c.is_digit(10) {
            code.next();
            return Ok(c);
        }
        return Err(ScanningErrors::NotADigit);
    }

    Err(ScanningErrors::EOF)
}

pub fn integer<'a>(code: &mut impl Scannable<'a>) -> Result<&'a str, ScanningErrors> {
    if let Some(start) = code.offset() {
        while let Ok(_) = digit(code) {}
        return Ok(code.slice(start));
    }

    Err(ScanningErrors::NotADigit)
}
