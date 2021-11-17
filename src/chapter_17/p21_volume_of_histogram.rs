//!
//! # Volume of Histogram:
//!
//! Imagine a histogram (bar graph).
//! Design an algorithm to compute the volume of water it could hold if someone poured water across the top.
//! You can assume that each histogram bar has width 1.
//!
//! EXAMPLE (Black bars (in the missing figure) are the histogram. Gray is water.)
//! Input : { 0, 0, 4, 0, 0, 6, 0, 0, 3, 0, 5, 0, 1, 0, 0, 0 }
//! Output: 26
//!
//! Hints: #629, #640, #651, #658, #662, #676, #693, #734, #742
//!

pub fn volume_of_histogram(_hist: &[usize]) -> usize {
    todo!()
}

#[ignore]
#[test]
fn test_volume_of_histogram() {
    let hist = vec![0, 0, 4, 0, 0, 6, 0, 0, 3, 0, 5, 0, 1, 0, 0, 0];
    assert_eq!(volume_of_histogram(&hist), 26);
}
