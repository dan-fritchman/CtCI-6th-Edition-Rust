//!
//! # Calculator:
//!
//! Given an arithmetic equation consisting of positive integers, +, -, * and / (no parentheses), compute the result.
//!
//! EXAMPLE
//!
//! Input: 2*3+5/6*3+15
//! Output: 23.5
//!
//! Hints: #527, #624, #665, #698
//!

use std::str::Chars;

// Every parser's favorite example.
//
// While the CtCI solutions use a more direct, targeted approach,
// this solution does what a general-purpose parser and evaluator would:
// Read the source into a (binary) expression-tree,
// then in a separate step evaluate the tree to a numeric result.

/// Enumerated, Supported Binary Operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}
/// Arithmetic Expression Enumeration
#[derive(Debug, Clone)]
pub enum Expr {
    Num(f64),
    BinOp {
        left: Box<Expr>,
        op: BinOp,
        right: Box<Expr>,
    },
}

/// Primary Implementation
///
/// Parse the source string into an [Expr] tree,
/// and then evaluate the tree to a numeric result.
///
pub fn calculator(src: &str) -> Result<f64, Error> {
    let expr = Parser::parse(src)?;
    let rv = eval(&expr);
    Ok(rv)
}
/// Expression Parser
///
/// Creates an [Expr] tree from a source string.
/// Traditional lexing and parsing are generally intertwined here;
/// this object does both, without an explicit Token stream or type.
///
struct Parser<'src> {
    chars: Chars<'src>,
    next: Option<char>,
}
impl<'src> Parser<'src> {
    /// Primary Entry Point. Parse source-string `src`.
    pub fn parse(src: &str) -> Result<Expr, Error> {
        let mut parser = Parser::new(src);
        parser.parse_expr()
    }
    /// Create a new parser over `src`.
    fn new(src: &'src str) -> Self {
        let mut chars = src.chars();
        let next = chars.next();
        Self { chars, next }
    }
    /// Parse an [Expr], as a cascade of additions and subtractions
    fn parse_expr(&mut self) -> Result<Expr, Error> {
        // Parse an initial term
        let mut expr = self.parse_term()?;
        // And expand across any additions and subtractions
        while let Some('+') | Some('-') = self.peek() {
            let op = self.parse_op()?;
            let right = self.parse_term()?;
            expr = Expr::BinOp {
                op,
                left: Box::new(expr),
                right: Box::new(right),
            }
        }
        Ok(expr)
    }
    /// Parse a cascade of multiplies and divides
    fn parse_term(&mut self) -> Result<Expr, Error> {
        // Parse an initial number
        let mut expr = self.parse_num()?;
        // And expand across any additions and subtractions
        while let Some('*') | Some('/') = self.peek() {
            let op = self.parse_op()?;
            let right = self.parse_num()?;
            expr = Expr::BinOp {
                op,
                left: Box::new(expr),
                right: Box::new(right),
            }
        }
        Ok(expr)
    }
    /// Parse a numeric value
    fn parse_num(&mut self) -> Result<Expr, Error> {
        let mut chars = Vec::new();
        while let Some(c) = self.peek() {
            if c.is_digit(10) {
                chars.push(*c);
                self.advance();
            } else {
                break;
            }
        }
        let s: String = chars.into_iter().collect();
        let f = s.parse::<f64>().unwrap();
        Ok(Expr::Num(f))
    }
    /// Parse an operator to a [BinOp]
    fn parse_op(&mut self) -> Result<BinOp, Error> {
        let op = match self.peek() {
            Some('+') => Ok(BinOp::Add),
            Some('-') => Ok(BinOp::Sub),
            Some('*') => Ok(BinOp::Mul),
            Some('/') => Ok(BinOp::Div),
            _ => Err(Error),
        }?;
        // Advance, only in the non-error cases
        self.advance();
        // And return the operator
        Ok(op)
    }
    /// Peek at (a reference to) the next character
    fn peek(&self) -> &Option<char> {
        &self.next
    }
    /// Advance a character. Returns a reference to the new `next`.
    fn advance(&mut self) -> &Option<char> {
        self.next = self.chars.next();
        self.peek()
    }
}
/// Evaluate potentially-nested expression `expr`
fn eval(expr: &Expr) -> f64 {
    match expr {
        Expr::Num(num) => *num, // Numbers are "already evaluated"
        Expr::BinOp {
            op,
            ref left,
            ref right,
        } => {
            // Recursively evaluate the left and right sides
            let left = eval(left);
            let right = eval(right);
            // And apply the operator
            match op {
                BinOp::Add => left + right,
                BinOp::Sub => left - right,
                BinOp::Mul => left * right,
                BinOp::Div => left / right,
            }
        }
    }
}
#[test]
fn test_calculator() {
    let s = "1";
    assert_eq!(calculator(&s).unwrap(), 1.0);

    let s = "1+2";
    assert_eq!(calculator(&s).unwrap(), 3.0);

    let s = "1+2*3";
    assert_eq!(calculator(&s).unwrap(), 7.0);

    let s = "1*2+3";
    assert_eq!(calculator(&s).unwrap(), 5.0);

    let s = "2*3+5/6*3+15";
    assert_eq!(calculator(&s).unwrap(), 23.5);

    let s = "2-6-7*8/2+5";
    assert_eq!(calculator(&s).unwrap(), -27.);
}

/// Local Error Type
#[derive(Debug)]
pub struct Error;
