//!
//! # Zero Matrix:
//!
//! Write an algorithm such that if an element in an MxN matrix is 0, its entire row and column are set to O.
//!
//! Hints: # 17, #74, #102
//!

use std::collections::HashSet;

/// Primary Implementation
///
/// Take two passes through the matrix.
/// In the first collect rows and columns to be zero'ed.
/// In the second set them to zero.
///
pub fn zero_matrix<const N: usize>(mat: &mut [[isize; N]; N]) {
    // Rows and column indices to zero will each be stored in a HashSet, preventing duplicates.
    let mut rows_to_zero = HashSet::new();
    let mut cols_to_zero = HashSet::new();

    // Pass one: find the zeroes
    for (rownum, row) in mat.iter().enumerate().take(N) {
        for (colnum, entry) in row.iter().enumerate().take(N) {
            if *entry == 0 {
                rows_to_zero.insert(rownum);
                cols_to_zero.insert(colnum);
            }
        }
    }
    // Pass two: set new zeroes
    for row in rows_to_zero {
        for col in 0..N {
            mat[row][col] = 0;
        }
    }
    for col in cols_to_zero {
        for row in mat.iter_mut().take(N) {
            row[col] = 0;
        }
    }
}

#[test]
fn test_zero_matrix() {
    let mut test_case = (
        [
            [1, 2, 3, 4, 0],
            [6, 0, 8, 9, 10],
            [11, 12, 13, 14, 15],
            [16, 0, 18, 19, 20],
            [21, 22, 23, 24, 25],
        ],
        [
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [11, 0, 13, 14, 0],
            [0, 0, 0, 0, 0],
            [21, 0, 23, 24, 0],
        ],
    );
    zero_matrix(&mut test_case.0);
    assert_eq!(test_case.0, test_case.1);
}
