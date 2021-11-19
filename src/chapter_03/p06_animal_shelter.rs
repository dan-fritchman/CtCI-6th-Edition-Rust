//!
//! # Animal Shelter:
//!
//! An animal shelter, which holds only dogs and cats, operates on a strictly "first in, first out" basis.
//! People must adopt either the "oldest" (based on arrival time) of all animals at the shelter,
//! or they can select whether they would prefer a dog or a cat (and will receive the oldest animal of that type).
//! They cannot select which specific animal they would like.
//! Create the data structures to maintain this system and implement operations such as
//! enqueue, dequeueAny, dequeueDog, and dequeueCat.
//! You may use the built-in LinkedList data structure.
//!
//! Hints: #22, #56, #63
//!

pub enum Animal<'name> {
    Dog(&'name str),
    Cat(&'name str),
}

pub struct AnimalShelter;
impl AnimalShelter {
    pub fn new() -> Self {
        todo!()
    }
    pub fn enqueue(&mut self, _a: Animal) {
        todo!()
    }
    pub fn size(&self) -> usize {
        todo!()
    }
}
pub fn animal_shelter() {
    todo!()
}

#[ignore] // FIXME!
#[test]
fn test_animal_shelter() {
    let mut animal_shelter = AnimalShelter::new();
    animal_shelter.enqueue(Animal::Cat("Fluffy"));
    animal_shelter.enqueue(Animal::Dog("Sparky"));
    animal_shelter.enqueue(Animal::Cat("Sneezy"));
    assert_eq!(animal_shelter.size(), 3);
}
