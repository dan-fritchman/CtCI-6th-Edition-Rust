//!
//! # The Masseuse:
//!
//! A popular masseuse receives a sequence of back-to-back appointment requests
//! and is debating which ones to accept.
//! She needs a 15-minute break between appointments and therefore she cannot accept any adjacent requests.
//! Given a sequence of back-to-back appointment requests
//! (all multiples of 15 minutes, none overlap, and none can be moved),
//! find the optimal (highest total booked minutes) set the masseuse can honor.
//! Return the number of minutes.
//!
//! EXAMPLE
//! Input {30, 15, 60, 75, 45, 15, 15, 45}
//! Output 180 minutes ({30, 60, 45, 45}).
//!
//! Hints: #495, #504, #516, #526, #542, #554, #562, #568, #578, #587, #607
//!

pub fn the_masseuse(appts: &[usize]) -> usize {
    let mut prev = 0;
    let mut sec = 0;
    for appt in appts.iter().rev() {
        let best_with = appt + sec;
        let best_without = prev;
        sec = prev;
        prev = best_with.max(best_without);
    }
    prev
}
#[test]
fn test_the_masseuse() {
    let test_cases = [
        (vec![30, 15, 60, 75, 45, 15, 15, 45], 180),
        (vec![30, 15, 60, 15, 45, 15, 45], 180),
        (vec![30, 15, 15, 60], 90),
    ];
    for case in test_cases {
        assert_eq!(the_masseuse(&case.0), case.1);
    }
}
