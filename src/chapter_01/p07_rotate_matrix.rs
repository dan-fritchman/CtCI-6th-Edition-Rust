//!
//! # Rotate Matrix:
//!
//! Given an image represented by an NxN matrix, where each pixel in the image is 4 bytes,
//! write a method to rotate the image by 90 degrees.
//! Can you do this in place?
//!
//! Hints: #51, #100
//!

/// Primary Implementation
///
/// Rotate 90 degrees clockwise, inline, running a "layer" at a time from the matrix exterior to center.
/// Note the ultra-cool const-generic usage.
///
pub fn rotate_matrix<const N: usize>(mat: &mut [[isize; N]; N]) {
    for layer in 0..N / 2 {
        let last = N - 1 - layer;
        for idx in layer..last {
            // The reverse-index, used for rows & cols moving "backward"
            let rev = N - 1 - idx;
            // Use one temporary to store the initial top-left
            let temp = mat[layer][idx];
            // Move left to top
            mat[layer][idx] = mat[rev][layer];
            // Move bottom to left
            mat[rev][layer] = mat[last][rev];
            // Move right to bottom
            mat[last][rev] = mat[idx][last];
            // Move top to right, via the temporary
            mat[idx][last] = temp;
        }
    }
}

#[test]
fn test_rotate_matrix() {
    let mut test_case = (
        [[1, 2, 3], [4, 5, 6], [7, 8, 9]],
        [[7, 4, 1], [8, 5, 2], [9, 6, 3]],
    );
    rotate_matrix(&mut test_case.0);
    assert_eq!(test_case.0, test_case.1);

    let mut test_case = (
        [
            [1, 2, 3, 4, 5],
            [6, 7, 8, 9, 10],
            [11, 12, 13, 14, 15],
            [16, 17, 18, 19, 20],
            [21, 22, 23, 24, 25],
        ],
        [
            [21, 16, 11, 6, 1],
            [22, 17, 12, 7, 2],
            [23, 18, 13, 8, 3],
            [24, 19, 14, 9, 4],
            [25, 20, 15, 10, 5],
        ],
    );
    rotate_matrix(&mut test_case.0);
    assert_eq!(test_case.0, test_case.1);
}
