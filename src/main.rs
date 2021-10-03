#![allow(dead_code)]
mod parser;

use std::fmt;
use std::collections::HashSet;

#[derive(Clone, Debug, Copy, Hash, Eq)]
struct Variable(char, usize);

impl PartialEq for Variable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl Variable {
    fn next(self) -> Self {
        Variable(self.0, self.1+1)
    }
}

impl From<char> for Variable {
    fn from(c: char) -> Variable {
        Variable(c, 0)
    }
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:'<1$}", self.0, self.1+1)
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Expression {
    Identifier(Variable),
    Application(Box<Expression>, Box<Expression>),
    Abstraction(Variable, Box<Expression>),
}


impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Identifier(x) => write!(f, "{}", x),
            Application(x1, x2) => write!(f, "({}{})", x1, x2),
            Abstraction(var, body) => write!(f, "(λ{}.{})", var, body),
        }
    }
}

// enum Associativity {
//     Left,
//     Right
// }

use Expression::*;

trait ExtendExt<T> {
    fn extend(&mut self, other: T);
}

impl ExtendExt<Expression> for Option<Expression> {
    fn extend(&mut self, other: Expression) {
        *self = Some(match self.take() {
            Some(Application(x1, x2)) => Application(Box::new(Application(x1, x2)), Box::new(other)),
            Some(x) => Application(Box::new(x), Box::new(other)),
            None => other,
        })
    }
}

/*
(
    ((λ x. (λ y. x)) (λ a. a))
    ((λx. (x x)) (λx. (x x)))
)
*/

fn main() {
    let expr = "((λ a. (λ b. (a (a (a b))))) (λ c. (λ d. (c (c d)))))";

    // let mut tree = parser::parse(&mut String::from(expr)).expect("Empty base expression");
    let tree = parser::parse(&mut String::from(expr));

    println!("{} -> {}", tree, normalize(tree.clone()));
    // loop {
    //     x += 1;
    //     let norm = normalize(tree.clone());
    //     println!("{} -> {}", tree, norm);
    //     tree = norm;

    //     if x > 20 {
    //         return;
    //     }
    // }
}

// fn normalize(mut expr: Expression) -> Expression {
//     let mut count = 0;

//     loop {
//         match expr {
//             e @ Application(_, _) => {
//                 expr = e;
//                 count += 1;

//                 if count >= u16::MAX {
//                     panic!("Stack Overflow: possible infinite recursion!")
//                 }
//             }
//             e @ _ => return e,
//         }
//     }
// }

/*
((λx.(λy.x))(λa.a))
(λy.(λa.a))
*/

fn normalize(mut expr: Expression) -> Expression {
    let mut count = 0;

    loop {
        match expr {
            Application(e1, e2) => {
                if let Abstraction(y, n) = normalize(*e1.clone()) {
                    expr = substitute(*n, y, &*e2)
                } else {
                    // panic!("Tried to apply non lambda!")
                    // expr = Application(e1, e2)
                    return Application(e1, Box::new(normalize(*e2)));
                }
            }
            Abstraction(x, e) => return Abstraction(x, Box::new(normalize(*e))),
            x @ _ => return x,
        }

        count += 1;
        if count >= u16::MAX {
            panic!("Stack Overflow: possible infinite recursion!")
        }
    }
    // println!("KICKING: '{}' -> '{}'", expr, out);
}

fn alpha(expr: Expression, x: Variable, z: Variable) -> Expression {
    match expr {
        Identifier(y) if y == x => Identifier(z),
        Identifier(y) => Identifier(y),
        Application(e1, e2) =>
            Application(Box::new(alpha(*e1, x, z)), Box::new(alpha(*e2, x, z))),
        Abstraction(y, e) if y == x => Abstraction(y, e),
        Abstraction(y, e) => Abstraction(y, Box::new(alpha(*e, x, z))),
    }
}

fn free(expr: Expression) -> HashSet<Variable> {
    let mut map = HashSet::new();

    match expr {
        Identifier(x) => { map.insert(x); },
        Application(e1, e2) => map = free(*e1).union(&free(*e2)).cloned().collect(),
        Abstraction(x, e) => {
            map = free(*e);
            map.remove(&x);
        },
    }

    map
}

fn substitute(expr: Expression, y: Variable, n: &Expression) -> Expression {
    // println!("{}", expr);

    match expr {
        Identifier(x) if x == y => n.clone(),
        x @ Identifier(_) => x,
        Application(x1, x2) =>
            Application(Box::new(substitute(*x1, y, n)), Box::new(substitute(*x2, y, n))),
        Abstraction(ref x, _) if *x == y => expr.clone(),
        Abstraction(x, e) => {
            // println!("{:?} free in '{}' contains '{:?}' ? {:?}", free(n.clone()), n, x,
            //          free(n.clone()).contains(&x));

            if free(n.clone()).contains(&x) {
                let nxt = x.next();
                let e_prime = alpha(*e, x, nxt);
                Abstraction(nxt, Box::new(substitute(e_prime, y, n)))
            } else {
                Abstraction(x, Box::new(substitute(*e, y, n)))
            }
        }
    }

    // println!("BINDING: '{}' in '{}' with '{}' -> '{}'", y, expr, n, out);
}
