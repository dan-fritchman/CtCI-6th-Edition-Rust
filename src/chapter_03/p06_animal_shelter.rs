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

use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq)]
pub struct Dog<'name>(&'name str);

#[derive(Debug, PartialEq, Eq)]
pub struct Cat<'name>(&'name str);

#[derive(Debug, PartialEq, Eq)]
pub enum Animal<'name> {
    Dog(Dog<'name>),
    Cat(Cat<'name>),
}
#[derive(Debug, Default)]
pub struct AnimalShelter<'n> {
    dogs: VecDeque<(Dog<'n>, usize)>,
    cats: VecDeque<(Cat<'n>, usize)>,
    timestamp: usize,
}
impl<'n> AnimalShelter<'n> {
    pub fn new() -> Self {
        Self {
            dogs: VecDeque::new(),
            cats: VecDeque::new(),
            timestamp: 0,
        }
    }
    pub fn enqueue(&mut self, a: Animal<'n>) {
        match a {
            Animal::Dog(d) => self.dogs.push_back((d, self.timestamp)),
            Animal::Cat(c) => self.cats.push_back((c, self.timestamp)),
        }
        self.timestamp += 1;
    }
    pub fn dequeue_any(&mut self) -> Option<Animal<'n>> {
        let dog_entry = match self.dogs.get(0) {
            Some(d) => d,
            None => return self.dequeue_cat().map(Animal::Cat),
        };
        let cat_entry = match self.cats.get(0) {
            Some(c) => c,
            None => return self.dequeue_dog().map(Animal::Dog),
        };
        // We have at least one dog and cat, compare their timestamps
        if dog_entry.1 < cat_entry.1 {
            self.dequeue_dog().map(Animal::Dog)
        } else {
            self.dequeue_cat().map(Animal::Cat)
        }
    }
    pub fn dequeue_dog(&mut self) -> Option<Dog<'n>> {
        self.dogs.pop_front().map(|e| e.0)
    }
    pub fn dequeue_cat(&mut self) -> Option<Cat<'n>> {
        self.cats.pop_front().map(|e| e.0)
    }
    pub fn size(&self) -> usize {
        self.dogs.len() + self.cats.len()
    }
}

#[test]
fn test_animal_shelter() {
    let mut animal_shelter = AnimalShelter::new();
    animal_shelter.enqueue(Animal::Cat(Cat("Fluffy")));
    animal_shelter.enqueue(Animal::Dog(Dog("Sparky")));
    animal_shelter.enqueue(Animal::Cat(Cat("Sneezy")));
    assert_eq!(animal_shelter.size(), 3);
}
