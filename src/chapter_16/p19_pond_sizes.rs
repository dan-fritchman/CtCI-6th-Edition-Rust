//!
//! # Pond Sizes:
//!
//! You have an integer matrix representing a plot of land,
//! where the value at that location represents the height above sea level.
//! A value of zero indicates water. A pond is a region of water connected vertically, horizontally, or diagonally.
//! The size of the pond is the total number of connected water cells.
//! Write a method to compute the sizes of all ponds in the matrix.
//!
//! EXAMPLE
//! Input:
//! 0 2 1 0
//! 0 1 0 1
//! 1 1 0 1
//! 0 1 0 1
//! Output: 2, 4, 1 (in any order)
//!
//! Hints: #674, #687, #706, #723
//!

/// Type alias for a compile-time-sized two-dimensional matrix
pub type Matrix<T, const M: usize, const N: usize> = [[T; M]; N];
/// Type alias for the plot-of-land matrix
pub type PlotOfLand<const M: usize, const N: usize> = Matrix<usize, M, N>;
/// And for the boolean visited-array
pub type Visited<const M: usize, const N: usize> = Matrix<bool, M, N>;

/// And for locations within the land-matrix
pub type Loc = (isize, isize);

/// Primary Implementation
/// Visit each location in the plot of land, using the recursive size-helper
/// to calculate the size of any attached pond.
pub fn pond_sizes<const M: usize, const N: usize>(land: &PlotOfLand<M, N>) -> Vec<usize> {
    let mut ponds = Vec::new();
    let mut visited = [[false; M]; N];
    for m in 0..M {
        for n in 0..N {
            let size = pond_size(land, &mut visited, (m as isize, n as isize));
            if size > 0 {
                ponds.push(size);
            }
        }
    }
    ponds
}
/// Recursive "pond-sizer"
pub fn pond_size<const M: usize, const N: usize>(
    land: &PlotOfLand<M, N>,
    visited: &mut Visited<M, N>,
    loc: Loc,
) -> usize {
    // Check for out-of-bounds and already-visited
    if loc.0 < 0
        || loc.1 < 0
        || loc.0 >= M as isize
        || loc.1 >= N as isize
        || visited[loc.0 as usize][loc.1 as usize]
    {
        return 0;
    }
    // Whether pond or not, mark this location as visited
    visited[loc.0 as usize][loc.1 as usize] = true;
    // Check for land
    if land[loc.0 as usize][loc.1 as usize] > 0 {
        return 0;
    }
    // We're in a pond. Start searching around its neighbors
    1 + pond_size(land, visited, (loc.0 - 1, loc.1 - 1))
        + pond_size(land, visited, (loc.0 - 1, loc.1))
        + pond_size(land, visited, (loc.0 - 1, loc.1 + 1))
        + pond_size(land, visited, (loc.0, loc.1 - 1))
        + pond_size(land, visited, (loc.0, loc.1 + 1))
        + pond_size(land, visited, (loc.0 + 1, loc.1 + 1))
        + pond_size(land, visited, (loc.0 + 1, loc.1))
        + pond_size(land, visited, (loc.0 + 1, loc.1 - 1))
}

#[test]
fn test_pond_sizes() {
    let land = [
        [0, 2, 1, 0], //
        [0, 1, 0, 1], //
        [1, 1, 0, 1], //
        [0, 1, 0, 1], //
    ];
    let ponds = pond_sizes(&land);
    assert_eq!(ponds, vec![2, 4, 1]);
}
