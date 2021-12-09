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

/// Primary Implementation
///
/// This solution's primary insight is that each bin has a "water level" dictated by the minimum of two values:
/// (a) the tallest bin to its right, and (b) the tallest bin to its left.
/// The water-volume contribution of each bin is then the difference between this water-level,
/// and the height of the bin.
///
/// This takes two passes over the histogram-array, the first going right-to-left and finding the "max value to the right"s.
/// The second pass does the remaining work, finding the "max value to the left"s, water-levels, and volume contributions.
///
pub fn volume_of_histogram(hist: &[usize]) -> usize {
    if hist.is_empty() {
        return 0; // Special-case the empty histogram, allowing a few `unwrap` calls below.
    }
    // First walk right-to-left over the histogram, collecting a vector of "max value to the right" for each bin.
    let mut right_maxes = vec![0; hist.len()];
    {
        let mut max = *hist.last().unwrap();
        for idx in (0..hist.len()).rev() {
            if hist[idx] > max {
                max = hist[idx];
            }
            right_maxes[idx] = max;
        }
    }

    // Now walk left-to-right, calculating the "max value to the left", water-level, and water-volume contributions.
    let mut total = 0;
    let mut left_max = *hist.first().unwrap();
    for (idx, bar_height) in hist.iter().enumerate() {
        // First update the left-max value
        if *bar_height > left_max {
            left_max = *bar_height;
        }
        // Take the min of the left-max and right-max
        let water_level = left_max.min(right_maxes[idx]);
        // Subtract the bar-height to get this bin's water-volume
        let water_volume = water_level - bar_height;
        // And add that water to the total
        total += water_volume;
    }
    total
}

#[test]
fn test_volume_of_histogram() {
    let test_cases = [
        (vec![], 0),
        (vec![1, 1], 0),
        (vec![1, 0, 1], 1),
        (vec![1, 0, 0, 0, 0, 0, 0, 1], 6),
        (vec![5, 0, 0, 0, 0, 0, 0, 7], 30),
        (vec![5, 0, 0, 3, 0, 0, 0, 7], 27),
        (vec![0, 0, 4, 0, 0, 6, 0, 0, 3, 0, 5, 0, 1, 0, 0, 0], 26),
    ];
    for case in test_cases {
        assert_eq!(volume_of_histogram(&case.0), case.1);
    }
}
