//!
//! # Paint Fill:
//!
//! Implement the "paint fill" function that one might see on many image editing programs.
//! That is, given a screen (represented by a two-dimensional array of colors), a point, and a new color,
//! fill in the surrounding area until the color changes from the original color.
//!
//! Hints: #364, #382
//!

use std::collections::HashSet;

/// Primary Implementation
///
/// Creates a helper struct [PaintFiller] which does all the recursive heavy lifting.
/// Visit pixels starting from `loc`, and if appropriate, change its color, and visit each of its neighbors.
///
pub fn paint_fill(screen: &mut Screen, loc: &Loc, newcolor: isize) {
    PaintFiller::fill(screen, loc, newcolor)
}

/// Paint-Filling Helper Struct
struct PaintFiller<'s> {
    screen: &'s mut Screen,
    start: Loc,
    orig: Color,        // Original color
    new: Color,         // New color
    done: HashSet<Loc>, // Seen-set
}
impl<'s> PaintFiller<'s> {
    /// Primary entrypoint. Create a [PaintFiller] instance and kick off its recursive `fill_helper` calls.
    fn fill(screen: &'s mut Screen, loc: &Loc, new: Color) {
        let mut this = Self::new(screen, loc, new);
        let start = this.start;
        this.fill_helper(&start)
    }
    /// Create a new [PainFiller]
    fn new(screen: &'s mut Screen, loc: &Loc, new: Color) -> Self {
        let orig = screen[loc.0 as usize][loc.1 as usize];
        let done = HashSet::new();
        let start = *loc;
        Self {
            screen,
            start,
            orig,
            new,
            done,
        }
    }
    /// Primary recursive method. If appropriate, update the pixel at `loc`, and visit all neighbors.
    fn fill_helper(&mut self, loc: &Loc) {
        if self.done.contains(loc) || !self.inbounds(loc) {
            return; // Already done, or out-of-bounds
        }
        self.done.insert(*loc);
        if self.screen[loc.0 as usize][loc.1 as usize] == self.orig {
            // Update the color
            self.screen[loc.0 as usize][loc.1 as usize] = self.new;
            // And visit all the neighbors
            self.fill_helper(&(loc.0 - 1, loc.1 - 1));
            self.fill_helper(&(loc.0 - 1, loc.1));
            self.fill_helper(&(loc.0 - 1, loc.1 + 1));
            self.fill_helper(&(loc.0, loc.1 - 1));
            self.fill_helper(&(loc.0, loc.1));
            self.fill_helper(&(loc.0, loc.1 + 1));
            self.fill_helper(&(loc.0 + 1, loc.1 - 1));
            self.fill_helper(&(loc.0 + 1, loc.1));
            self.fill_helper(&(loc.0 + 1, loc.1 + 1));
        }
    }
    /// Boolean indication of whether location `loc` lies within the screen
    fn inbounds(&self, loc: &Loc) -> bool {
        !(loc.0 < 0
            || loc.1 < 0
            || loc.0 >= self.screen.len() as isize
            || loc.1 >= self.screen[0].len() as isize)
    }
}

// Type aliases
pub type Color = isize;
pub type Screen = Vec<Vec<Color>>;
pub type Loc = (isize, isize);

#[test]
fn test_paint_fill() {
    let test_cases = [(
        vec![vec![1, 2, 5], vec![2, 2, 4], vec![2, 8, 6]],
        (1, 1),
        3,
        vec![vec![1, 3, 5], vec![3, 3, 4], vec![3, 8, 6]],
    )];

    for mut case in test_cases {
        paint_fill(&mut case.0, &case.1, case.2);
        assert_eq!(case.0, case.3);
    }
}
