use std::fmt::Debug;

pub trait ErrorWithMark<Mark> {
    fn mark(self) -> Mark;
}

#[derive(Debug, PartialEq)]
pub enum ScanningErrors<Mark>
where
    Mark: Debug,
{
    TokenNotFound(Mark),
    NotADigit(Mark),
    EOF(Mark),
}

impl<Mark> ErrorWithMark<Mark> for ScanningErrors<Mark>
where
    Mark: Debug,
{
    fn mark(self) -> Mark {
        match self {
            ScanningErrors::EOF(m) => m,
            ScanningErrors::NotADigit(m) => m,
            ScanningErrors::TokenNotFound(m) => m,
        }
    }
}

pub trait Scannable<'a, Mark>
where
    Mark: Debug,
{
    fn peek(&mut self) -> Option<char>;
    fn next(&mut self) -> Option<char>;

    fn substr(&mut self, start: &Mark) -> &'a str;

    fn mark(&mut self) -> Mark;
    fn restore(&mut self, mark: &Mark);
}

pub fn one_of<Input, Output, Error, Parser, Mark>(
    input: &Input,
    parsers: &[Parser],
) -> Result<Output, Error>
where
    Parser: Fn(&Input) -> Result<Output, Error>,
    Error: ErrorWithMark<Mark>,
{
    for i in 0..parsers.len() {
        match parsers[i](input) {
            Ok(o) => return Ok(o),
            Err(e) => {
                if i == parsers.len() - 1 {
                    return Err(e);
                }
            }
        }
    }

    core::panic!()
}

pub fn some<'a, Input, Parser, Ooutput, Mark, Error>(
    input: &mut Input,
    parser: &Parser,
) -> Result<Vec<Ooutput>, Error>
where
    Parser: Fn(&Input) -> Result<Ooutput, Error>,
    Error: ErrorWithMark<Mark>,
{
    let mut os: Vec<Ooutput> = Vec::new();

    loop {
        match parser(input) {
            Ok(o) => os.push(o),
            Err(e) => {
                if os.len() == 0 {
                    return Err(e);
                }
                break;
            }
        }
    }

    Ok(os)
}

pub fn some_str<'a, Input, Parser, Mark, Error, Output>(
    input: &mut Input,
    parser: &mut Parser,
) -> Result<&'a str, Error>
where
    Parser: FnMut(&mut Input) -> Result<Output, Error>,
    Input: Scannable<'a, Mark>,
    Mark: Debug,
    Error: ErrorWithMark<Mark>,
{
    let mark = input.mark();
    let mut found = false;

    loop {
        match parser(input) {
            Ok(_) => found = true,
            Err(e) => {
                if !found {
                    return Err(e);
                }
                break;
            }
        }
    }

    return Ok(input.substr(&mark));
}

pub fn digit<'a, Mark, Input>(code: &mut Input) -> Result<char, ScanningErrors<Mark>>
where
    Input: Scannable<'a, Mark>,
    Mark: Debug,
{
    if let Some(c) = code.peek() {
        if c.is_digit(10) {
            code.next();
            return Ok(c);
        }
        return Err(ScanningErrors::NotADigit(code.mark()));
    }

    Err(ScanningErrors::EOF(code.mark()))
}

pub fn digits<'a, Mark, Input>(code: &mut Input) -> Result<&'a str, ScanningErrors<Mark>>
where
    Input: Scannable<'a, Mark>,
    Mark: Debug,
{
    some_str(code, &mut |i| digit(i))
}

pub fn token<'a, Mark, Input>(
    code: &mut Input,
    token: &str,
) -> Result<&'a str, ScanningErrors<Mark>>
where
    Input: Scannable<'a, Mark>,
    Mark: Debug,
{
    let mut ti = token.chars().peekable();

    let mut parser = move |i: &mut Input| {
        if let (Some(c1), Some(c2)) = (ti.next(), i.peek()) {
            if c1 != c2 {
                return Err(ScanningErrors::TokenNotFound(i.mark()));
            }
            i.next();
            return Ok(c1);
        }
        return Err(ScanningErrors::TokenNotFound(i.mark()));
    };

    some_str(code, &mut |i| parser(i))
}

pub fn skip_ws<'a, Mark, Input>(code: &mut Input)
where
    Input: Scannable<'a, Mark>,
    Mark: Debug,
{
    while let Some(c) = code.peek() {
        if !c.is_whitespace() {
            return;
        }
        code.next();
    }
}
