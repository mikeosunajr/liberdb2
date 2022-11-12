use std::ops::Range;

#[derive(Debug, PartialEq)]
pub enum ScanningErrors {
    TokenNotFound,
    NotADigit,
    EOF,
}

pub trait Scannable<'a, T> {
    fn peek(&mut self) -> Option<char>;
    fn next(&mut self) -> Option<char>;
    fn offset(&mut self) -> Option<usize>;

    fn substr(&mut self, len: usize) -> Result<&'a str, ScanningErrors>;

    fn mark(&mut self) -> T;
    fn restore(&mut self, mark: T);
}

pub fn digit<'a, T>(code: &mut impl Scannable<'a, T>) -> Result<char, ScanningErrors> {
    if let Some(c) = code.peek() {
        if c.is_digit(10) {
            code.next();
            return Ok(c);
        }
        return Err(ScanningErrors::NotADigit);
    }

    Err(ScanningErrors::EOF)
}

pub fn integer<'a, T>(code: &mut impl Scannable<'a, T>) -> Result<&'a str, ScanningErrors> {
    if let Some(start) = code.offset() {
        while let Ok(_) = digit(code) {}
        return Ok(code.rest(start));
    }

    Err(ScanningErrors::NotADigit)
}

pub fn token<'a, T>(
    code: &mut impl Scannable<'a, T>,
    token: &str,
) -> Result<&'a str, ScanningErrors> {
    if let Some(offset) = code.offset() {
        let end = offset + token.len();
        if let Ok(s) = code.slice(offset..end) {
            if s == token {
                code.reset(end);
                return Ok(s);
            }
        }
    }

    Err(ScanningErrors::TokenNotFound)
}

pub fn skip_ws<'a, T>(code: &mut impl Scannable<'a, T>) {
    while let Some(c) = code.peek() {
        if !c.is_whitespace() {
            return;
        }
        code.next();
    }
}
