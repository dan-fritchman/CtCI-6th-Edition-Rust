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
//! Hints: #638, #657, #666, #682, #699
//!

/// Person, or at least the height and weight thereof
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Person {
    height: usize,
    weight: usize,
}
impl Person {
    /// Create a [Person] from a (height, weight) tuple
    #[cfg(test)]
    fn from(htwt: (usize, usize)) -> Self {
        Self {
            height: htwt.0,
            weight: htwt.1,
        }
    }
    /// Boolean indication of whether `self` can sit below [Person] `other`
    fn can_sit_below(&self, other: &Person) -> bool {
        self.height > other.height && self.weight > other.weight
    }
}

/// Primary Implementation
///
/// First sort `people` by one attribute (height),
/// then walk across the ordered attempting to make expanding-length solution-lists.
/// Returns a newly-allocated Vector of the top-to-bottom people-ordering.
///
pub fn circus_tower(people: &mut [Person]) -> Vec<Person> {
    if people.is_empty() {
        return Vec::new(); // Special case the empty [Person]-vector.
    }
    // First sort `people` by their `height`
    people.sort_unstable_by(|this, other| this.height.cmp(&other.height));

    let mut best_index = 0;
    let mut solutions = Vec::new();
    for (idx, person) in people.iter().enumerate() {
        let longest_at_index = best_at_index(person, &solutions);
        if solutions.is_empty() || longest_at_index.len() > solutions[best_index].len() {
            best_index = idx;
        }
        solutions.push(longest_at_index);
    }
    // Return a two-tuple of the tower-height and its contents
    solutions[best_index].clone()
}

/// Find the best working tower ending with `person`
fn best_at_index(person: &Person, solutions: &[Vec<Person>]) -> Vec<Person> {
    // Find the longest existing `solution` to which `person` can be appended
    let mut best = &Vec::new();
    for solution in solutions.iter() {
        if can_append(solution, person) && solution.len() > best.len() {
            best = solution;
        }
    }
    // And append `person` to it
    let mut best = best.clone();
    best.push(person.clone());
    best
}

/// Boolean indication of whether `person` can be appended to `people`
fn can_append(people: &[Person], person: &Person) -> bool {
    people.is_empty() || person.can_sit_below(people.last().unwrap())
}

#[test]
fn test_circus_tower() {
    let test_cases = [
        (vec![], vec![]),
        (vec![(65, 100), (100, 65)], vec![(65, 100)]),
        (vec![(65, 100), (65, 100)], vec![(65, 100)]),
        (vec![(65, 100), (65, 101)], vec![(65, 100)]),
        (
            vec![(65, 100), (55, 40), (75, 90), (80, 120)],
            vec![(55, 40), (65, 100), (80, 120)],
        ),
        (
            vec![
                (65, 100),
                (70, 150),
                (56, 90),
                (75, 190),
                (60, 95),
                (68, 110),
            ],
            vec![
                (56, 90),
                (60, 95),
                (65, 100),
                (68, 110),
                (70, 150),
                (75, 190),
            ],
        ),
    ];
    for case in test_cases {
        // Make a few pre-conversions from tuples into [Person]s.
        let mut people: Vec<Person> = case.0.iter().map(|htwt| Person::from(*htwt)).collect();
        let correct: Vec<Person> = case.1.iter().map(|htwt| Person::from(*htwt)).collect();
        // And run the test-case
        assert_eq!(circus_tower(&mut people), correct);
    }
}
