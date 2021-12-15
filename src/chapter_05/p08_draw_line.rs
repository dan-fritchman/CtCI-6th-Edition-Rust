//!
//! # Draw Line:
//!
//! A monochrome screen is stored as a single array of bytes, allowing eight consecutive pixels to be stored in one byte.
//! The screen has width w, where w is divisible by 8 (that is, no byte will be split across rows).
//! The height of the screen, of course, can be derived from the length of the array and the width.
//! Implement a function that draws a horizontal line from (x1, y) to (x2, y).
//!
//! The method signature should look something like:
//! drawLine(byte[] screen, int width, int x1, int x2, int y)
//!
//! Hints: #366, #381, #384, #391
//!

/// Type Alias for the Screen
type Screen = Vec<u8>;

/// Primary Implementation
///
/// First set each all-one byte, then handle the edge-byte(s).
/// Note the [Screen] order is least-significant *byte* and most-significant *bit* first.
///
pub fn draw_line(screen: &mut Screen, width: usize, x1: usize, x2: usize, y: usize) {
    let height = 8 * screen.len() / width;
    if width < 1 || x1 >= width || x2 >= width || y >= height {
        panic!("Invalid arguments");
    }
    // Rearrange x-values into min and max, so that either order works.
    let xmin = x1.min(x2);
    let xmax = x1.max(x2);

    // Filter down to the effected row of `screen`
    let the_row = &mut screen[(width * y / 8)..(width * (y + 1) / 8)];

    // Set all full-bytes to ones (0xFF)
    let first_full_byte = (xmin / 8) + if xmin % 8 != 0 { 1 } else { 0 };
    let last_full_byte = xmax / 8;
    for i in first_full_byte..last_full_byte {
        the_row[i] = 0xFF;
    }

    // Now handle the "edge bytes".
    let start_mask = 0xFF >> (xmin % 8);
    // Rust's built-in methods really don't want wrapping/ overflowing shifts.
    // So, so trickeration for when `xmax % 8 == 7`.
    let mut end_mask = 0xFF;
    if xmax % 8 != 7 {
        end_mask = !(0xFF >> ((xmax % 8) + 1))
    }
    // These fall in one of two cases, depending whether `xmin` and `xmax` are part of the same byte.
    if xmin / 8 == xmax / 8 {
        the_row[xmin / 8] |= start_mask & end_mask;
    } else {
        the_row[xmin / 8] |= start_mask;
        the_row[xmax / 8] |= end_mask;
    }
}

#[test]
fn test_draw_line() {
    let mut screen = vec![0; 2];
    draw_line(&mut screen, 16, 0, 15, 0);
    assert_eq!(screen, vec![0b11111111, 0b11111111]);

    let mut screen = vec![0; 1];
    draw_line(&mut screen, 8, 1, 5, 0);
    assert_eq!(screen, vec![0b01111100]);

    let mut screen = vec![0; 3];
    draw_line(&mut screen, 8, 1, 5, 1);
    assert_eq!(screen, vec![0b00000000, 0b01111100, 0b000000000]);

    let mut screen = vec![0; 3];
    draw_line(&mut screen, 24, 6, 17, 0);
    assert_eq!(screen, vec![0b00000011, 0b11111111, 0b11000000]);
}
