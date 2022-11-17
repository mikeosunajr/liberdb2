use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub enum ScanningErrors<M>
where
    M: Debug,
{
    TokenNotFound(M),
    NotADigit(M),
    EOF(M),
}

pub trait Scannable<'a, M>
where
    M: Debug,
{
    fn peek(&mut self) -> Option<char>;
    fn next(&mut self) -> Option<char>;

    fn substr(&mut self, start: &M) -> &'a str;

    fn mark(&mut self) -> M;
    fn restore(&mut self, mark: &M);
}

pub fn one_of<I, O, E, F>(input: &I, parsers: &[F]) -> Result<O, E>
where
    F: Fn(&I) -> Result<O, E>,
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

pub fn some<'a, I, F, O, M, E>(input: &mut I, parser: &F) -> Result<Vec<O>, E>
where
    F: Fn(&I) -> Result<O, E>,
{
    let mut os: Vec<O> = Vec::new();

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

pub fn some_str<'a, I, F, M, E, O>(input: &mut I, parser: &mut F) -> Result<&'a str, E>
where
    F: FnMut(&mut I) -> Result<O, E>,
    I: Scannable<'a, M>,
    M: Debug,
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

pub fn digit<'a, Mark, I>(code: &mut I) -> Result<char, ScanningErrors<Mark>>
where
    I: Scannable<'a, Mark>,
    Mark: Debug,
{
    if let Some(c) = code.peek() {
        if c.is_digit(10) {
            code.next();
            return Ok(c);
        }
        return Err(ScanningErrors::NotADigit((code.mark())));
    }

    Err(ScanningErrors::EOF(code.mark()))
}

pub fn digits<'a, Mark, I>(code: &mut I) -> Result<&'a str, ScanningErrors<Mark>>
where
    I: Scannable<'a, Mark>,
    Mark: Debug,
{
    some_str(code, &mut |i| digit(i))
}

pub fn token<'a, Mark, I>(code: &mut I, token: &str) -> Result<&'a str, ScanningErrors<Mark>>
where
    I: Scannable<'a, Mark>,
    Mark: Debug,
{
    let mut ti = token.chars().peekable();

    let mut parser = move |i: &mut I| {
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

pub fn skip_ws<'a, T, I>(code: &mut I)
where
    I: Scannable<'a, T>,
    T: Debug,
{
    while let Some(c) = code.peek() {
        if !c.is_whitespace() {
            return;
        }
        code.next();
    }
}
