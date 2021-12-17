//!
//! # Towers of Hanoi:
//!
//! In the classic problem of the Towers of Hanoi, you have 3 towers and N disks of different sizes which can slide onto any tower.
//! The puzzle starts with disks sorted in ascending order of size from top to bottom
//! (I.e., each disk sits on top of an even larger one).
//! You have the following constraints:
//! (1) Only one disk can be moved at a time.
//! (2) A disk is slid off the top of one tower onto another tower.
//! (3) A disk cannot be placed on top of a smaller disk.
//! Write a program to move the disks from the first tower to the last using stacks.
//!
//! Hints: # 144, #224, #250, #272, #318
//!

/// Tower of Disks
/// Wraps a vector of [usize]-valued "disk sizes".
#[derive(Debug, Default)]
pub struct Tower {
    disks: Vec<usize>,
}
impl Tower {
    /// Pop from the underlying disks
    fn pop(&mut self) -> Option<usize> {
        self.disks.pop()
    }
    /// Push onto the disk-tower.
    /// Panics if new entry `val` is not less than the current top-of-tower.
    fn push(&mut self, val: usize) {
        match self.disks.last() {
            Some(l) => {
                if val >= *l {
                    panic!("Disk Error!");
                }
                self.disks.push(val);
            }
            None => self.disks.push(val),
        }
    }
    /// Start a [Tower] in the initial game-state, adding `n` disks with weights 1 through `n`.
    fn start(n: usize) -> Self {
        let mut this = Self::default();
        for i in (1..=n).rev() {
            this.push(i)
        }
        this
    }
}

/// Hanoi Game State
/// The three towers: source, destination, and other.
#[derive(Debug, Default)]
pub struct Hanoi {
    src: Tower,
    dest: Tower,
    other: Tower,
}
impl Hanoi {
    /// Create a new [Hanoi] game-state, with `n` disks initialized to the `src` [Tower].
    pub fn new(n: usize) -> Self {
        Self {
            src: Tower::start(n),
            dest: Tower::default(),
            other: Tower::default(),
        }
    }
}

/// Primary Implementation
///
/// Solves the towers of hanoi game of size `n`.
/// Create a [Hanoi] game-state, initialized with all disks on its source-[Tower].
/// Kicks off a recursive call-stack solving sub-problems to move `n-1` disks onto the game's `other` [Tower],
/// so that the largest disk can be moved to `dest`.
/// Finally (and again recursively) move those `n-1` disks from `other` to `dest`.
///
pub fn towers_of_hanoi(n: usize) -> Hanoi {
    let mut hanoi = Hanoi::new(n);
    helper(&mut hanoi.src, &mut hanoi.dest, &mut hanoi.other, n);
    hanoi
}
// Recursive helper (which does all the real work)
fn helper(src: &mut Tower, dest: &mut Tower, other: &mut Tower, n: usize) {
    if n == 0 {
        panic!("Invalid Input"); // Fail if we land here to move "zero" disks
    } else if n == 1 {
        return dest.push(src.pop().unwrap()); // Base case: move from `src` to `dest`
    }
    // Recursive case.
    // First get `n-1` disks onto `other`. Note `dest` and `other` are swapped for ths call.
    helper(src, other, dest, n - 1);
    // Now move the largest disk onto `dest`.
    dest.push(src.pop().unwrap());
    // And transfer the stack on `other` to `dest`, using `src` as the spare/other Tower.
    helper(other, dest, src, n - 1);
}

#[test]
fn test_towers_of_hanoi() {
    for k in 1..20 {
        let mut hanoi = towers_of_hanoi(k);
        assert_eq!(hanoi.src.disks.len(), 0);
        assert_eq!(hanoi.other.disks.len(), 0);
        assert_eq!(hanoi.dest.disks.len(), k);
        // Check that the destination data is in order
        for j in 1..=k {
            assert_eq!(hanoi.dest.pop().unwrap(), j);
        }
    }
}
