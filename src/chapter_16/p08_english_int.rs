//!
//! # English Int:
//!
//! Given any integer, print an English phrase that describes the integer
//! (e.g., "One Thousand, Two Hundred Thirty Four").
//!
//! Hints: #502, #588, #688
//!

pub fn english_int(i: u64) -> String {
    if i == 0 {
        // Treat the special case of zero,
        // which must be held out of the recursive calls below.
        return String::from("Zero");
    }
    helper(i)
}
fn helper(mut i: u64) -> String {
    // List of suffices to print individually
    let suffixes = [
        (12, "Trillion"),
        (9, "Billion"),
        (6, "Million"),
        (3, "Thousand"),
        (2, "Hundred"),
    ];
    // Iterate over these suffixes, recursively generating
    // sub-strings such as "One Hundred Ten (Thousand)".
    // Note this runs *big to small* to enable the sub-problem breakdowns.
    let mut rv = String::new();
    for suffix in suffixes {
        let power = 10_u64.pow(suffix.0);
        if i >= power {
            push_word(&mut rv, &helper(i / power));
            push_word(&mut rv, suffix.1);
            i -= (i / power) * power;
        }
    }
    // Now handle everything below 100, which is more niche. Especially those teens.
    assert!(i < 100);
    match i / 10 {
        1 => {
            // Teens are special, handle them special
            let teen = match i {
                10 => "Ten",
                11 => "Eleven",
                12 => "Twelve",
                13 => "Thirteen",
                14 => "Fourteen",
                15 => "Fifteen",
                16 => "Sixteen",
                17 => "Seventeen",
                18 => "Eighteen",
                19 => "Nineteen",
                _ => panic!("Internal Error"),
            };
            push_word(&mut rv, teen);
        }
        0 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 => {
            let tens = match i / 10 {
                0 => "",
                2 => "Twenty",
                3 => "Thirty",
                4 => "Fourty",
                5 => "Fifty",
                6 => "Sixty",
                7 => "Seventy",
                8 => "Eighty",
                9 => "Ninety",
                _ => panic!("Internal Error"),
            };
            push_word(&mut rv, tens);
            let ones = match i % 10 {
                0 => "",
                1 => "One",
                2 => "Two",
                3 => "Three",
                4 => "Four",
                5 => "Five",
                6 => "Six",
                7 => "Seven",
                8 => "Eight",
                9 => "Nine",
                _ => panic!("Internal Error"),
            };
            push_word(&mut rv, ones);
        }
        _ => panic!("Internal Error"),
    };
    rv
}
/// Helper function to add a word to `s`, plus a preceding space if `s` has prior content.
fn push_word(s: &mut String, word: &str) {
    if !s.is_empty() && !word.is_empty() {
        s.push(' ');
    }
    s.push_str(word)
}
#[test]
fn test_english_int() {
    let test_cases = [
        (0, "Zero"),
        (1, "One"),
        (10, "Ten"),
        (13, "Thirteen"),
        (19, "Nineteen"),
        (20, "Twenty"),
        (23, "Twenty Three"),
        (50, "Fifty"),
        (73, "Seventy Three"),
        (93, "Ninety Three"),
        (100, "One Hundred"),
        (101, "One Hundred One"),
        (110, "One Hundred Ten"),
        (119, "One Hundred Nineteen"),
        (195, "One Hundred Ninety Five"),
        (300, "Three Hundred"),
        (504, "Five Hundred Four"),
        (950, "Nine Hundred Fifty"),
        (974, "Nine Hundred Seventy Four"),
        (999, "Nine Hundred Ninety Nine"),
        (1000, "One Thousand"),
        (10_000, "Ten Thousand"),
        (909_000, "Nine Hundred Nine Thousand"),
        (1_000_000, "One Million"),
        (9_000_009, "Nine Million Nine"),
        (
            19_323_984,
            "Nineteen Million Three Hundred Twenty Three Thousand Nine Hundred Eighty Four",
        ),
        (
            908_900_034,
            "Nine Hundred Eight Million Nine Hundred Thousand Thirty Four",
        ),
        (
            100_000_000_781,
            "One Hundred Billion Seven Hundred Eighty One",
        ),
    ];
    for case in test_cases {
        assert_eq!(english_int(case.0), case.1);
    }
}
