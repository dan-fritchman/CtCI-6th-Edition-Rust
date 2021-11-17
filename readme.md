# Rust Solutions to _Cracking the Coding Interview, 6th Edition_

These are **Rust** solutions for the book [Cracking the Coding Interview, 6th Edition](https://www.careercup.com/book) by _Gayle Laakmann McDowell_.

This repository's structure, naming conventions, and most unit-test data are adapted from @careercup's Python solutions available at
https://github.com/careercup/CtCI-6th-Edition-Python. All solution-code is written from scratch.

## How to use?

- The book-worth of problems is configured as a cargo library-package
- Each chapter is a module within said package, generally named `chapter_<chapter number>`
- Each problem is a module within said chapter-module, generally named `p<number>_<name>`
- Each solution has one or more unit tests
- To run the tests: `cargo test`

## Goals

These solutions are intended to be a combo demonstration of McDowell's problems and the Rust language. Primarily they intend to:

- Above all, solve each of the problems
- Demonstrating idiomatic Rust style and design patterns
  - Prominent example: many of `CtCI`'s problems feature linked lists, trees, and graphs, the data structures \
    which most commonly vex new adopters of Rust's ownership system. Solutions use a combination of popular patterns \
    including index-references and `Rc<RefCell>`
- Demonstrate other best language-practices, such as thorough documentation of each public entity, \
  unit testing, linting via `clippy`, and other code-quality tools.

## Contributions

Contributions welcome! Please submit separate pull requests for each solution you work on.
