//!
//! # Robot in a Grid:
//!
//! Imagine a robot sitting on the upper left corner of grid with r rows and c columns.
//! The robot can only move in two directions, right and down,
//! but certain cells are "off limits" such that the robot cannot step on them.
//! Design an algorithm to find a path for the robot from the top left to the bottom right.
//!
//! Hints: #331, #360, #388
//!

use std::collections::HashSet;

// Type Aliases
pub type Grid = Vec<Vec<bool>>;
pub type Point = (isize, isize);
pub type Path = Vec<Point>;
pub type Cache = HashSet<Point>;

/// Primary Implementation
///
/// Recursively work back from the end of the grid,
/// checking for prior-accessible squares that have valid paths.
///
pub fn robot_grid(grid: &Grid) -> Option<Path> {
    // Check for validitiy (or at least emptiness) of the grid
    if grid.is_empty() || grid[0].is_empty() {
        return None;
    }

    // Initialize our path and cache
    let mut path = Path::new();
    let mut failed = Cache::new();
    let pos = ((grid.len() - 1) as isize, (grid[0].len() - 1) as isize);

    // And run our recursive helper
    if helper(grid, &mut path, &mut failed, pos) {
        Some(path) // Return the path collected along the way
    } else {
        None // No path found
    }
}

/// Recursive helper function, including the running path and failed-cache
fn helper(grid: &Grid, path: &mut Path, failed: &mut Cache, pos: Point) -> bool {
    if pos.0 < 0 || pos.1 < 0 || failed.contains(&pos) || !grid[pos.0 as usize][pos.1 as usize] {
        return false;
    }
    if pos == (0, 0)
        || helper(grid, path, failed, (pos.0 - 1, pos.1))
        || helper(grid, path, failed, (pos.0, pos.1 - 1))
    {
        // Found a path to our predecessor - so there's also a path to us.
        path.push(pos);
        return true;
    }
    // No path found. Add to our failed set.
    failed.insert(pos);
    false
}
#[test]
fn test_robot_grid() {
    let test_cases = [
        (vec![vec![true]], Some(vec![(0, 0)])),
        (vec![vec![false]], None),
        (
            vec![vec![true, true], vec![true, true]],
            Some(vec![(0, 0), (0, 1), (1, 1)]),
        ),
        (
            vec![vec![true, false], vec![true, true]],
            Some(vec![(0, 0), (1, 0), (1, 1)]),
        ),
        (vec![vec![false, true], vec![true, true]], None),
        (vec![vec![true, true], vec![true, false]], None),
        (vec![vec![false, false], vec![false, false]], None),
        (
            vec![
                vec![true, true, true],
                vec![true, true, true],
                vec![true, true, true],
            ],
            Some(vec![(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)]),
        ),
        (
            vec![
                vec![false, false, false],
                vec![false, false, false],
                vec![false, false, false],
            ],
            None,
        ),
        (
            vec![
                vec![true, false, true],
                vec![true, true, true],
                vec![true, true, true],
            ],
            Some(vec![(0, 0), (1, 0), (1, 1), (1, 2), (2, 2)]),
        ),
    ];
    for case in test_cases {
        assert_eq!(robot_grid(&case.0), case.1);
    }
}
