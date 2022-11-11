use std::sync::Arc;

fn main() {}
#[derive(Debug, PartialEq)]
enum Context {}
#[derive(Debug, PartialEq)]
enum Problem {}

#[derive(PartialEq, Debug)]
enum Expression {
    Empty,
}
#[cfg(test)]
mod tests {

    use crate::run;
    use crate::{Context, Expression, Problem, State};

    #[test]
    fn it_works() {
        assert_eq!(
            Ok((Expression::Empty, State::new("1"))),
            run::<Context, Problem, Expression>(
                |state: &State<Context>| {
                    let s = State::<Context>::new("1");
                    Ok((Expression::Empty, s))
                },
                "1",
            )
        );
    }
}

impl<'a, 'b, F, Context, C2: 'b, Problem, Value> Parser<'a, 'b, Context, Problem, Value> for F
where
    F: Fn(&'a State<'a, Context>) -> PStep<'b, C2, Problem, Value>,
{
    fn parse(&self, state: &'a State<'a, Context>) -> PStep<'b, Context, Problem, Value> {
        self(&state)
    }
}

#[derive(Debug, PartialEq)]
struct Located<Context> {
    row: i64,
    col: i64,
    context: Context,
}

#[derive(Debug, PartialEq)]
struct State<'a, Context: 'a> {
    src: &'a str,
    offset: i64,
    indent: i64,
    context: Vec<Located<Context>>,
    row: i64,
    col: i64,
}
#[derive(Debug, PartialEq)]
struct DeadEnd<Context, Problem> {
    row: i64,
    col: i64,
    problem: Problem,
    context_stack: Vec<Located<Context>>,
}

type PStep<'a, Context, Problem, Value> =
    Result<(Value, State<'a, Context>), Vec<DeadEnd<Context, Problem>>>;

trait Parser<'a, 'b, Context, Problem, Value> {
    fn parse(&self, state: &'a State<'a, Context>) -> PStep<'b, Context, Problem, Value>;
}

impl<'a, Context> State<'a, Context> {
    fn new(src: &'a str) -> Self {
        Self {
            src,
            offset: 0,
            indent: 1,
            row: 1,
            col: 1,
            context: Vec::new(),
        }
    }
}

fn run<'a, 'b, Context, Problem, Value>(
    parser: impl Parser<'a, 'b, Context, Problem, Value>,
    src: &'a str,
) -> PStep<'b, Context, Problem, Value> {
    let s = State::new(src);
    parser.parse(&s)
}
