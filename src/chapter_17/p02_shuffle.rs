//!
//! # Shuffle:
//!
//! Write a method to shuffle a deck of cards.
//! It must be a perfect shuffle - in other words, each of the 52! permutations of the deck has to be equally likely.
//! Assume that you are given a random number generator which is perfect.
//!
//! Hints: #483, #579, #634
//!

// From CtCI:
// This is a very well known interview question, and a well known algorithm.
// If you aren't one of the lucky few to already know this algorithm, read on.

/// Primary Implementation
///
/// For each "card" (number) in the "deck" (array),
/// swap it with another card at a randomly chosen index
/// between zero and its index.
pub fn shuffle(deck: &mut [usize]) {
    use rand::Rng; // Trait, must be in-scope for `gen_range`

    for i in 0..deck.len() {
        // Select a random index from 0 to i
        let rn = rand::thread_rng().gen_range(0..i + 1);
        // And swap it with `deck[i]`
        let temp = deck[i];
        deck[i] = deck[rn];
        deck[rn] = temp;
    }
}

#[test]
fn test_shuffle() {
    let mut deck: Vec<usize> = (0..52).collect();
    shuffle(&mut deck);
    // Since this produces a random result, we don't have a great affirmative test for it.
    // But, we can check we didn't screw it up in a few ways.
    assert_eq!(deck.len(), 52);
    assert_eq!(deck.iter().sum::<usize>(), (0..52).sum());
}
