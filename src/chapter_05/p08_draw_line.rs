//!
//! # Draw Line:
//!
//! A monochrome screen is stored as a single array of bytes, allowing eight consecutive pixels to be stored in one byte.
//! The screen has width w, where wis divisible by 8 (that is, no byte will be split across rows).
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

pub fn draw_line(_screen: &mut Screen, _width: usize, _x1: usize, _x2: usize, _y: usize) -> usize {
    todo!()
}

#[ignore]
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
