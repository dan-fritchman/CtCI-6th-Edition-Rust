# Rust Solutions to _Cracking the Coding Interview, 6th Edition_

These are [**Rust**](https://www.rust-lang.org/) solutions for the book [Cracking the Coding Interview, 6th Edition](https://www.careercup.com/book) by _Gayle Laakmann McDowell_.

This repository's structure, naming conventions, and most unit-test data are adapted from [@careercup](https://github.com/careercup)'s [Python solutions](https://github.com/careercup/CtCI-6th-Edition-Python). All solution-code is written from scratch.

Credit is also due to Bryan Cantrill's 2018 QCon Presentation, [Is It Time to Rewrite the Operating System in Rust?](https://www.youtube.com/watch?v=HgtRAbE1nBM), for injecting the idea to use McDowell's problems in the first place.

## How to use?

- The book-worth of problems is configured as a [cargo library-package](https://crates.io/)
- Each chapter is a module within said package, generally named `chapter_<number>`
- Each problem is a module within said chapter-module, generally named `p<number>_<name>`
- Each solution generally has a single unit test
- To run the tests: `cargo test`

## Goals

These solutions are intended to be a combo demonstration of McDowell's problems and the Rust language. They primarily intend to:

- Above all, solve each of the problems
- Demonstrating idiomatic Rust style and design patterns
  - Prominent example: many of `CtCI`'s problems feature linked lists, trees, and graphs, the data structures which most commonly vex new adopters of Rust's ownership system. Solutions use a combination of popular patterns including index-references and `Rc<RefCell>`
- Demonstrate other best language-practices, such as thorough documentation of each public entity, unit testing, linting via `clippy`, and other code-quality tools.

Less emphasis is placed on providing alternate solutions, or profiling comparative results between solutions.

## Contributions

Contributions welcome! Please submit separate pull requests for each solution you work on.  
Areas for contribution particularly include:

- Adding thus-far uncovered problems. 
- Adding test cases.
- More Rust-idiomaticity, wherever you can find it.
- Adding coverage or other code-quality tools, mostly as demos of how to use them.
