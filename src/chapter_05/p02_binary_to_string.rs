//!
//! # Binary to String:
//!
//! Given a real number between 0 and 1 (e.g., 0.72) that is passed in as a double,
//! print the binary representation.
//! If the number cannot be represented accurately in binary with at most 32 characters,
//! print "ERROR:'
//!
//! Hints: #143, #167, #173, #269, #297
//!

/// Primary Implementation
/// Loop over bits, subtracting residuals whenever `f` is greater than `2**-n`.
pub fn bin_decimal_to_str(mut f: f64) -> Result<String, &'static str> {
    if f <= 0.0 || f >= 1.0 {
        return Err("Invalid Inputs");
    }
    // Loop over powers of 2**-n, removing residual until `f` is zero or less.
    let mut rv = String::from("b0.");
    let mut comp = 0.5;
    while f > 0.0 {
        if rv.len() > 32 + 3 {
            // Perhaps more descriptively "too long", but this is the problem statement
            // "+3" is for the header characters "b0.".
            return Err("ERROR");
        }
        if f >= comp {
            rv.push('1');
            f -= comp;
        } else {
            rv.push('0');
        }
        comp /= 2.0;
    }
    Ok(rv)
}
/// (Very) short-hand for [String::from]
pub fn s(str_: &'static str) -> String {
    String::from(str_)
}
#[test]
fn test_bin_decimal_to_str() {
    let test_cases = [
        (0.5, Ok(s("b0.1"))),
        (0.25, Ok(s("b0.01"))),
        (0.125, Ok(s("b0.001"))),
        (0.875, Ok(s("b0.111"))),
        (0.625, Ok(s("b0.101"))),
        // Invalid input cases
        (0.0, Err("Invalid Inputs")),
        (1.0, Err("Invalid Inputs")),
        (2.0, Err("Invalid Inputs")),
        // Result-too-long cases
        (0.1, Err("ERROR")),
        (0.101, Err("ERROR")),
        (0.2, Err("ERROR")),
        (0.785, Err("ERROR")),
    ];
    for case in test_cases {
        assert_eq!(bin_decimal_to_str(case.0), case.1);
    }
}
