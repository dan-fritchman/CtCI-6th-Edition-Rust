//!
//! # Boolean Evaluation:
//!
//! Given a boolean expression consisting of the symbols 0 (false), 1 (true), & (AND), | (OR), and ^ (XOR),
//! and a desired boolean result value `result`,
//! implement a function to count the number of ways of parenthesizing the expression such that it evaluates to result.
//!
//! EXAMPLE
//!
//! countEval("1^0|0|1", false) -> 2
//! countEval("0&0&0&1^1", true) -> 10
//!
//! Hints: #148, #168, #197, #305, #327
//!

use std::collections::HashMap;

/// Primary Implementation
///
/// Split at each operator, recursively counting the number of paren-sets in each of its operands
/// correspond to the desired result.
///
pub fn boolean_evaluation(expr: &str, result: bool) -> usize {
    // Chop the string-based `expr` into a character array, largely so we can index single [char]s.
    let chars = expr.chars().collect::<Vec<char>>();

    // Create our cache
    let mut cache = HashMap::new();

    // And kick off the recursive call-tree
    helper(&chars, result, &mut cache)
}

/// Primary recursive workhorse. Split at each operator and total paren-counts for each side.
fn helper<'s>(expr: &'s [char], result: bool, cache: &mut Cache<'s>) -> usize {
    if cache.contains_key(&(expr, result)) {
        return *cache.get(&(expr, result)).unwrap();
    }
    if expr.len() % 2 == 0 {
        panic!("Invalid expression");
    }
    if expr.len() == 1 {
        // Base case: a single character
        let rv = unary(&expr[0], result);
        cache.insert((expr, result), rv);
        return rv;
    }

    // Iterate over operators, splitting each way and totalling up the number of paren-groups that work
    let mut total = 0;
    let mut idx = 1;
    while idx < expr.len() {
        let (left, op, right) = split(expr, idx);
        total += ternary(left, op, right, result, cache);
        idx += 2;
    }
    cache.insert((expr, result), total);
    total
}

/// Get the total for a single-character unary expression. Always either zero or one.
fn unary(c: &char, result: bool) -> usize {
    let parsed = match c {
        '1' => true,
        '0' => false,
        _ => panic!("Invalid unary expression"),
    };
    if parsed == result {
        1
    } else {
        0
    }
}

/// Get the total for the ternary expression `left op right`
fn ternary<'s>(
    left: &'s [char],
    op: &'s char,
    right: &'s [char],
    result: bool,
    cache: &mut Cache<'s>,
) -> usize {
    // This is really the core logic of our method.
    // For each logical operator, total up the number of parenthesizations
    // via combinations of (left, right) and (true, false) opportunities.
    match Op::parse(op) {
        Ok(Op::And) => match result {
            true => helper(left, true, cache) * helper(right, true, cache),
            false => {
                let [lt, rt, lf, rf] = combos(left, right, cache);
                lf * rf + lt * rf + lf * rt
            }
        },
        Ok(Op::Or) => match result {
            true => {
                let [lt, rt, lf, rf] = combos(left, right, cache);
                lt * rt + lt * rf + lf * rt
            }
            false => helper(left, false, cache) * helper(right, false, cache),
        },
        Ok(Op::Xor) => {
            let [lt, rt, lf, rf] = combos(left, right, cache);
            match result {
                true => lt * rf + lf * rt,
                false => lt * rt + lf * rf,
            }
        }
        Err(_) => panic!(
            "Invalid ternary expression ({:?}, {:?}, {:?})",
            left, op, right
        ),
    }
}

/// Get the four combinations [left_true, right_true, left_false, right_false], in that order.
fn combos<'s>(left: &'s [char], right: &'s [char], cache: &mut Cache<'s>) -> [usize; 4] {
    [
        helper(left, true, cache),
        helper(right, true, cache),
        helper(left, false, cache),
        helper(right, false, cache),
    ]
}

/// Split `chars` in three. Generally for left, operator, and right parts.
fn split(chars: &[char], idx: usize) -> (&[char], &char, &[char]) {
    (&chars[0..idx], &chars[idx], &chars[idx + 1..chars.len()])
}

/// Enumerated Operators
pub enum Op {
    And,
    Or,
    Xor,
}
impl Op {
    /// Parse from a [char]
    fn parse(c: &char) -> Result<Op, ()> {
        match c {
            '&' => Ok(Op::And),
            '|' => Ok(Op::Or),
            '^' => Ok(Op::Xor),
            _ => Err(()),
        }
    }
}

/// Type alias for the cache.
/// Rust offers new surprises nearly every day, including today's: that `&[char]` is a hashable type.
type Cache<'s> = HashMap<(&'s [char], bool), usize>;

#[test]
fn test_boolean_evaluation() {
    let test_cases = [
        ("0|0|1", true, 2),
        ("1^0|0|1", false, 2),
        ("0&0&0&1^1|0", true, 10),
    ];
    for case in test_cases {
        assert_eq!(boolean_evaluation(case.0, case.1), case.2);
    }
}
