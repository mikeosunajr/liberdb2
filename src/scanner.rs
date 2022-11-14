#[derive(Debug, PartialEq)]
pub enum ScanningErrors {
    TokenNotFound,
    NotADigit,
    EOF,
}

pub trait Scannable<'a, Mark> {
    fn peek(&mut self) -> Option<char>;
    fn next(&mut self) -> Option<char>;

    fn substr(&mut self, start: &Mark) -> &'a str;

    fn mark(&mut self) -> Option<Mark>;
    fn restore(&mut self, mark: &Mark);
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

pub fn digits<'a, T>(code: &mut impl Scannable<'a, T>) -> Result<&'a str, ScanningErrors> {
    if let Some(mark) = code.mark() {
        let mut found = false;
        while let Ok(_) = digit(code) {
            found = true
        }

        if found {
            Ok(code.substr(&mark))
        } else {
            Err(ScanningErrors::NotADigit)
        }
    } else {
        Err(ScanningErrors::NotADigit)
    }
}

pub fn token<'a, T>(
    code: &mut impl Scannable<'a, T>,
    token: &str,
) -> Result<&'a str, ScanningErrors> {
    if let Some(mark) = code.mark() {
        let mut ti = token.chars();

        while let (Some(c1), Some(c2)) = (ti.next(), code.next()) {
            if c1 != c2 {
                break;
            }
        }

        if ti.next() == None {
            Ok(code.substr(&mark))
        } else {
            code.restore(&mark);
            Err(ScanningErrors::TokenNotFound)
        }
    } else {
        Err(ScanningErrors::EOF)
    }
}

pub fn skip_ws<'a, T>(code: &mut impl Scannable<'a, T>) {
    while let Some(c) = code.peek() {
        if !c.is_whitespace() {
            return;
        }
        code.next();
    }
}
