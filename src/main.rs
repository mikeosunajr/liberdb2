mod parser;
mod scanner;

fn main() {}

#[cfg(test)]
mod tests {
    use crate::scanner::{token, Scannable};

    #[test]
    fn it_works() {
        let mut code = crate::parser::ParserState::new("let a = 1");
        let res = token(&mut code, "let");
        assert!(res.is_ok());
        assert_eq!("let", res.unwrap());
        assert_eq!(Some(' '), code.peek());
        assert_eq!(Some(' '), code.next());
        assert_eq!(Some('a'), code.next());
    }
}
