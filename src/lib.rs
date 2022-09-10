//! Main Elemental processing library.

use std::{
    fmt::{
        Display,
        Result,
        Formatter,
    },
    collections::HashMap,
};

use colored::*;

mod error;
mod tokenizer;
mod parser;
mod interpreter;
mod standard;

pub use tokenizer::{
    Token,
    TokenClass,
    Tokenizer,
};

pub use parser::Parser;


/// Allows parselet files to easily access necessary
/// abstractions without long `use` statements.
/// 
/// For internal use only.
pub mod parselet_utils {
    pub use crate::parser::{
        Parser,
        InfixParselet,
        PrefixParselet,
    };
    pub use crate::tokenizer::{
        Token,
        Tokenizer,
        TokenClass,
    };
    pub use crate::Expression;
}


/// Defines the expression types that are available in Elemental.
#[derive(Clone, Debug)]
pub enum Expression {
    Assignment {
        identifier: String,
        value: Box<Expression>,
    },
    Identifier (String),
    Int (i64),
    Float (f64),
    BinOp {
        left: Box<Expression>,
        op: String,
        right: Box<Expression>,
    },
    Nil,
}

/// Implementing `std::fmt::Display` allows us to print expressions
/// using the default formatter.
impl Display for Expression {
    /// Display each `Expression`.
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Expression::Assignment {
                identifier: _,
                value: v,
            } => {
                write!(f, "{}", v)
            },
            Expression::Identifier (s) => {
                write!(f, "{}", s)
            },
            Expression::Int (i) => {
                write!(f, "{}", i)
            },
            Expression::Float (float) => {
                write!(f, "{:.8}", float)
            },
            Expression::BinOp {
                left: l,
                op: o,
                right: r,
            } => {
                write!(f, "{} {} {}", l, o, r)
            },
            Expression::Nil => {
                write!(f, "")
            },
        }
    }
}

impl Expression {
    /// Simplify this expression, given a reference to a list of variables.
    pub fn simplify(&self, variables: &mut HashMap<String, Expression>) -> Self {
        match self {
            Expression::Identifier (s) => {
                match variables.get(s) {
                    Some(e) => (*e).to_owned(),
                    None => {
                        print!("{}: variable {} has not been declared", "error".red().bold(), s);
                        return Expression::Nil;
                    },
                }
            },
            Expression::Assignment {
                identifier: ref i,
                value: ref v,
            } => {
                variables.insert(i.to_owned(), *v.to_owned());
                return (**v).to_owned();
            }
            Expression::BinOp {
                left: l,
                op: o,
                right: r,
            } => {
                // Simplify the left-hand and right-hand sides
                let left = l.simplify(variables);
                let right = r.simplify(variables);

                if let Expression::Int (l) = left {
                    if let Expression::Int (r) = right {
                        Expression::Int (binop_int(l, r, &o))
                    } else if let Expression::Float (r) = right {
                        let left_float = l as f64;
                        Expression::Float (binop_float(left_float, r, &o))
                    } else {
                        todo!()
                    }
                } else if let Expression::Float (l) = left {
                    if let Expression::Int (r) = right {
                        let right_float = r as f64;
                        Expression::Float (binop_float(l, right_float, &o))
                    } else if let Expression::Float (r) = right {
                        Expression::Float (binop_float(l, r, &o))
                    } else {
                        todo!()
                    }
                } else {
                    todo!()
                }
            },
            Expression::Int (_) => self.to_owned(),
            Expression::Float (_) => self.to_owned(),
            _ => {
                todo!()
            },
        }
    }
}


/// Executes the given binary operation on two integers.
pub fn binop_int(x: i64, y: i64, binop: &str) -> i64 {
    match binop {
        "+" => x + y,
        "-" => x - y,
        "*" => x * y,
        "/" => x / y,
        _ => todo!(),
    }
}


/// Executes the given binary operation on two floats.
pub fn binop_float(x: f64, y: f64, binop: &str) -> f64 {
    match binop {
        "+" => x + y,
        "-" => x - y,
        "*" => x * y,
        "/" => x / y,
        _ => todo!(),
    }
}


/// Interprets a `String` of code into an `Expression`.
pub fn interpret(variables: &mut HashMap<String, Expression>, code: String) -> Expression {
    // Create a token stream from the code input.
    let mut tokenizer = Tokenizer::from(code);

    // Create a parser and parse from the tokenizer.
    let parser = Parser::new();
    let expression = parser.parse(&mut tokenizer, 0);

    expression.simplify(variables)
}


#[test]
fn interpret_00() {
    let code = "3.1415".to_string();
    println!("{}", interpret(code));
}

#[test]
fn interpret_01() {
    let code = "x = 4".to_string();
    println!("{}", interpret(code));
}