//!
//! # Circus Tower:
//!
//! A circus is designing a tower routine consisting of people standing atop one another's shoulders.
//! For practical and aesthetic reasons, each person must be both shorter and lighter
//! than the person below him or her.
//! Given the heights and weights of each person in the circus,
//! write a method to compute the largest possible number of people in such a tower.
//!
//! EXAMPLE
//! Input (ht, wt): (65, 100) (70, 150) (56, 90) (75, 190) (60, 95) (68, 110)
//! Output: The longest tower is length 6 and includes from top to bottom:
//! (56, 90) (60, 95) (65, 100) (68, 110) (70, 150) (75, 190)
//!
//! Hints:#638, #657, #666, #682, #699
//!

pub fn circus_tower(_people: &[(usize, usize)]) -> usize {
    todo!() // FIXME!
}

#[ignore]
#[test]
fn test_circus_tower() {
    let test_cases = [
        (vec![], 0),
        (vec![(65, 100), (100, 65)], 1),
        (vec![(65, 100), (65, 100)], 1),
        (vec![(65, 100), (65, 101)], 1),
        (vec![(65, 100), (55, 40), (75, 90), (80, 120)], 3),
        (
            vec![
                (65, 100),
                (70, 150),
                (56, 90),
                (75, 190),
                (60, 95),
                (68, 110),
            ],
            6,
        ),
    ];
    for case in test_cases {
        assert_eq!(circus_tower(&case.0), case.1);
    }
}
