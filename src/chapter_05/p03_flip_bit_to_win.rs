//!
//! # Flip Bit to Win:
//!
//! You have an integer and you can flip exactly one bit from a 0 to a 1.
//! Write code to find the length of the longest sequence of 1s you could create.
//!
//! EXAMPLE
//! Input: 1775 (or: 11011101111)
//! Output: 8
//!
//! Hints: #159, #226, #314, #352
//!

/// Primary Implementation
///
/// Collect streaks of bits, and find the largest one or pair separated by a single zero.
///
pub fn flip_bit_to_win(mut num: u32) -> usize {
    // First create a vectore of "streaks" of alternating bits
    // `streaks` counts up from LSBs, always beginning with a number of *zeroes* (which may be zero)
    let mut streaks = Vec::new();
    let mut state = 0;
    let mut consec = 0;
    for _ in 0..32 {
        if num & 0b1 == state {
            consec += 1;
        } else {
            streaks.push(consec);
            consec = 1;
            // Flip `state` between 0 and 1
            state = state ^ 0b1;
        }
        num >>= 1;
    }
    streaks.push(consec);

    // Handle our two special cases: all zeroes, and all ones
    // Note our many "+1" add-ons are for sake of flipping an adjacent bit,
    // including potentially the bit above the 1-valued MSB.
    if streaks.len() == 0 {
        panic!("Internal Error");
    } else if streaks.len() == 1 {
        return 1; // All zeroes
    } else if streaks.len() == 2 {
        return streaks[1] + 1; // All ones
    }

    // More than one streak of ones. Walk through those streaks.
    let mut longest = streaks[1] + 1;
    for k in (3..streaks.len()).step_by(2) {
        let candidate = if streaks[k - 1] == 1 {
            streaks[k] + streaks[k - 2] + 1
        } else {
            streaks[k] + 1
        };
        if candidate > longest {
            longest = candidate;
        }
    }
    longest
}
#[test]
fn test_flip_bit_to_win() {
    let test_cases = [
        (0b0, 1),
        (0b111, 4),
        (0b10011100111, 4),
        (0b11011101111, 8),
        (!0, 33), // All ones
    ];
    for case in test_cases {
        assert_eq!(flip_bit_to_win(case.0), case.1);
    }
}
