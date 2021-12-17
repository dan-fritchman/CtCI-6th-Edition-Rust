//!
//! # Eight Queens:
//!
//! Write an algorithm to print all ways of arranging eight queens on an 8x8 chess board
//! so that none of them share the same row, column, or diagonal.
//! In this case, "diagonal" means all diagonals, not just the two that bisect the board.
//!
//! Hints: #308, #350, #371
//!

/// Type alias for the board.
/// Eight rows are represented by storing the column-index at which they hold a queen.
pub type Board = [usize; 8];

/// Primary Implementation
///
/// Recursively check each sub-board for each thus-far valid set of row-selections.
///
pub fn eight_queens() -> Vec<Board> {
    let mut results = Vec::new();
    let mut board = [8; 8]; // Initialize to all invalid values
    helper(&mut board, 0, &mut results);
    results
}
/// Recursive helper
/// Append solutions a row at a time.
fn helper(board: &mut Board, row: usize, results: &mut Vec<Board>) {
    if row >= board.len() {
        // Made it through all eight rows - this is a valid solution.
        return results.push(*board);
    }
    // Check for valid columns in each of the remaining rows
    for col in 0..board.len() {
        if valid(board, row, col) {
            // This column works, at least for now. Set it, and check remaining rows.
            board[row] = col;
            helper(board, row + 1, results);
        }
    }
}
/// Boolean indication of the validity of placing a queen at (row,col),
/// given the current state of `board`.
fn valid(board: &Board, row: usize, col: usize) -> bool {
    // Check whether anything else is in this column
    for item in board.iter().take(row) {
        if *item == col {
            return false;
        }
    }
    // Check diagonals
    for idx in 1..=row {
        let r = row - idx;
        if col >= idx && board[r] == col - idx {
            return false;
        }
        if col + idx < board.len() && board[r] == col + idx {
            return false;
        }
    }
    true // All checked out
}

#[test]
fn test_eight_queens() {
    let mut rv = eight_queens();
    rv.sort();
    let mut correct = correct();
    correct.sort();
    assert_eq!(rv.len(), correct.len());
    assert_eq!(rv, correct);
}

/// List of correct answers, in no particular order.
pub fn correct() -> Vec<Board> {
    vec![
        [0, 4, 7, 5, 2, 6, 1, 3],
        [0, 5, 7, 2, 6, 3, 1, 4],
        [0, 6, 3, 5, 7, 1, 4, 2],
        [0, 6, 4, 7, 1, 3, 5, 2],
        [1, 3, 5, 7, 2, 0, 6, 4],
        [1, 4, 6, 0, 2, 7, 5, 3],
        [1, 4, 6, 3, 0, 7, 5, 2],
        [1, 5, 0, 6, 3, 7, 2, 4],
        [1, 5, 7, 2, 0, 3, 6, 4],
        [1, 6, 2, 5, 7, 4, 0, 3],
        [1, 6, 4, 7, 0, 3, 5, 2],
        [1, 7, 5, 0, 2, 4, 6, 3],
        [2, 0, 6, 4, 7, 1, 3, 5],
        [2, 4, 1, 7, 0, 6, 3, 5],
        [2, 4, 1, 7, 5, 3, 6, 0],
        [2, 4, 6, 0, 3, 1, 7, 5],
        [2, 4, 7, 3, 0, 6, 1, 5],
        [2, 5, 1, 4, 7, 0, 6, 3],
        [2, 5, 1, 6, 0, 3, 7, 4],
        [2, 5, 1, 6, 4, 0, 7, 3],
        [2, 5, 3, 0, 7, 4, 6, 1],
        [2, 5, 3, 1, 7, 4, 6, 0],
        [2, 5, 7, 0, 3, 6, 4, 1],
        [2, 5, 7, 0, 4, 6, 1, 3],
        [2, 5, 7, 1, 3, 0, 6, 4],
        [2, 6, 1, 7, 4, 0, 3, 5],
        [2, 6, 1, 7, 5, 3, 0, 4],
        [2, 7, 3, 6, 0, 5, 1, 4],
        [3, 0, 4, 7, 1, 6, 2, 5],
        [3, 0, 4, 7, 5, 2, 6, 1],
        [3, 1, 4, 7, 5, 0, 2, 6],
        [3, 1, 6, 2, 5, 7, 0, 4],
        [3, 1, 6, 2, 5, 7, 4, 0],
        [3, 1, 6, 4, 0, 7, 5, 2],
        [3, 1, 7, 4, 6, 0, 2, 5],
        [3, 1, 7, 5, 0, 2, 4, 6],
        [3, 5, 0, 4, 1, 7, 2, 6],
        [3, 5, 7, 1, 6, 0, 2, 4],
        [3, 5, 7, 2, 0, 6, 4, 1],
        [3, 6, 0, 7, 4, 1, 5, 2],
        [3, 6, 2, 7, 1, 4, 0, 5],
        [3, 6, 4, 1, 5, 0, 2, 7],
        [3, 6, 4, 2, 0, 5, 7, 1],
        [3, 7, 0, 2, 5, 1, 6, 4],
        [3, 7, 0, 4, 6, 1, 5, 2],
        [3, 7, 4, 2, 0, 6, 1, 5],
        [4, 0, 3, 5, 7, 1, 6, 2],
        [4, 0, 7, 3, 1, 6, 2, 5],
        [4, 0, 7, 5, 2, 6, 1, 3],
        [4, 1, 3, 5, 7, 2, 0, 6],
        [4, 1, 3, 6, 2, 7, 5, 0],
        [4, 1, 5, 0, 6, 3, 7, 2],
        [4, 1, 7, 0, 3, 6, 2, 5],
        [4, 2, 0, 5, 7, 1, 3, 6],
        [4, 2, 0, 6, 1, 7, 5, 3],
        [4, 2, 7, 3, 6, 0, 5, 1],
        [4, 6, 0, 2, 7, 5, 3, 1],
        [4, 6, 0, 3, 1, 7, 5, 2],
        [4, 6, 1, 3, 7, 0, 2, 5],
        [4, 6, 1, 5, 2, 0, 3, 7],
        [4, 6, 1, 5, 2, 0, 7, 3],
        [4, 6, 3, 0, 2, 7, 5, 1],
        [4, 7, 3, 0, 2, 5, 1, 6],
        [4, 7, 3, 0, 6, 1, 5, 2],
        [5, 0, 4, 1, 7, 2, 6, 3],
        [5, 1, 6, 0, 2, 4, 7, 3],
        [5, 1, 6, 0, 3, 7, 4, 2],
        [5, 2, 0, 6, 4, 7, 1, 3],
        [5, 2, 0, 7, 3, 1, 6, 4],
        [5, 2, 0, 7, 4, 1, 3, 6],
        [5, 2, 4, 6, 0, 3, 1, 7],
        [5, 2, 4, 7, 0, 3, 1, 6],
        [5, 2, 6, 1, 3, 7, 0, 4],
        [5, 2, 6, 1, 7, 4, 0, 3],
        [5, 2, 6, 3, 0, 7, 1, 4],
        [5, 3, 0, 4, 7, 1, 6, 2],
        [5, 3, 1, 7, 4, 6, 0, 2],
        [5, 3, 6, 0, 2, 4, 1, 7],
        [5, 3, 6, 0, 7, 1, 4, 2],
        [5, 7, 1, 3, 0, 6, 4, 2],
        [6, 0, 2, 7, 5, 3, 1, 4],
        [6, 1, 3, 0, 7, 4, 2, 5],
        [6, 1, 5, 2, 0, 3, 7, 4],
        [6, 2, 0, 5, 7, 4, 1, 3],
        [6, 2, 7, 1, 4, 0, 5, 3],
        [6, 3, 1, 4, 7, 0, 2, 5],
        [6, 3, 1, 7, 5, 0, 2, 4],
        [6, 4, 2, 0, 5, 7, 1, 3],
        [7, 1, 3, 0, 6, 4, 2, 5],
        [7, 1, 4, 2, 0, 6, 3, 5],
        [7, 2, 0, 5, 1, 4, 6, 3],
        [7, 3, 0, 2, 5, 1, 6, 4],
    ]
}
