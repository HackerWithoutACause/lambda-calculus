use crate::{Expression, Variable, ExtendExt};
use crate::Expression::*;

pub(crate) fn parse(line: &mut String) -> Expression {
    let mut expr = None;

    while !line.is_empty() {
        match line.remove(0) {
            '(' => expr.extend(parse(line)),
            ')' => break,
            'λ' | '\\' => {
                expr.extend(parse_lambda(line));
                break;
            },
            c if c.is_alphabetic() => expr.extend(Identifier(Variable::from(c))),
            c if c.is_whitespace() => (),
            c => panic!("Unknown character: {}", c)
        }
    }

    expr.expect("Parsing Error: Found empty expression!")
}

fn parse_lambda(line: &mut String) -> Expression {
    *line = String::from(line.trim_start());
    let x = line.remove(0);
    assert!(x.is_alphabetic(), "Parsing Error: Lambda arguments must be alphabetic!");

    {
        *line = String::from(line.trim_start());
        let c = line.remove(0);
        assert!(c == '.', "Parsing Error: Expected '.' found '{}'!", c);
    }

    Abstraction(Variable::from(x), Box::new(parse(line)))
}
