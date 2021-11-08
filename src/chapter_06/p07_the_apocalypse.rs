//!
//! # The Apocalypse:
//!
//! In the new post-apocalyptic world, the world queen is desperately concerned about the birth rate.
//! Therefore, she decrees that all families should ensure that they have one girl or else they face massive fines.
//! If all families abide by this policy-that is, they have continue to have children until they have one girl,
//! at which point they immediately stop-what will the gender ratio of the new generation be?
//! (Assume that the odds of someone having a boy or a girl on any given pregnancy is equal.)
//! Solve this out logically and then write a computer simulation of it.
//!
//! Hints: # 154, #160, #171, #188, #201
//!

#[test]
fn apocalypse() {
    // Each family has a randomly-generated number of boys, and one girl.
    // The number of girls therefore equals the number of families.
    // For each family, pull random booleans until one is false, representing boys.
    let girls = 1_000_000;
    let mut boys = 0;
    for _family in 0..girls {
        while rand::random() {
            boys += 1;
        }
    }

    // Make some coarse checks on the ratio, which should approach 0.5 for large numbers of `girls`.
    let (girls, boys) = (girls as f64, boys as f64);
    let ratio = girls / (girls + boys);

    assert!(ratio < 0.52);
    assert!(ratio > 0.48);
}
