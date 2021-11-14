//!
//! # Kth Multiple:
//!
//! Design an algorithm to find the kth number such that the only prime factors are 3, 5, and 7.
//! Note that 3, 5, and 7 do not have to be factors,
//! but it should not have any other prime factors.
//! For example, the first several multiples would be (in order) 1, 3, 5, 7, 9, 15, 21.
//!
//! Hints: #488, #508, #550, #591, #622, #660, #686
//!

use std::collections::VecDeque;

/// Primary Implementation
///
/// Track priority-queues of multiples of 3, 5, and 7,
/// selectively inserting new elements when we pop each.
///
pub fn kth_multiple(k: usize) -> usize {
    // Create the queues
    let mut q3 = VecDeque::new();
    let mut q5 = VecDeque::new();
    let mut q7 = VecDeque::new();

    // And initialize one of them with the initial value, 1
    let mut min = 1;
    q3.push_back(1);

    for _ in 0..=k {
        let q3min = qmin(&q3);
        let q5min = qmin(&q5);
        let q7min = qmin(&q7);
        min = q3min.min(q5min).min(q7min);
        if min == q3min {
            q3.pop_front();
            q3.push_back(3 * min);
            q5.push_back(5 * min);
        } else if min == q5min {
            q5.pop_front();
            q5.push_back(5 * min);
        } else {
            // min == q7min
            q7.pop_front();
        }
        // Always push 7x the min
        q7.push_back(7 * min);
    }
    min
}
/// Get the min value from the front of a queue,
/// or the maximum integer value if empty.
fn qmin(q: &VecDeque<usize>) -> usize {
    match q.front() {
        Some(v) => *v,
        None => usize::MAX,
    }
}
#[test]
fn test_kth_multiple() {
    let test_cases = [
        (0, 1), //
        (1, 3),
        (2, 5),
        (3, 7),
        (4, 9),
        (5, 15),
        (6, 21),
        (7, 25),
        (8, 27),
        (9, 35),
        (100, 35721),
        (1000, 82_046_671_875),
    ];
    for case in test_cases {
        assert_eq!(kth_multiple(case.0), case.1);
    }
}
