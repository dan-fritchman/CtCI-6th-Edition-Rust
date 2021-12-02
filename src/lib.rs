//!
//! # Cracking the Coding Interview Crate
//!
//! Each chapter's solutions are contained in an internal module named `chapter_{num}`.
//! Problems are generally organized as sub-modules named `p{number}_{name}`.
//!

pub mod chapter_01;
pub mod chapter_02;
pub mod chapter_03;
pub mod chapter_04;
pub mod chapter_05;
pub mod chapter_06;
pub mod chapter_07;
pub mod chapter_08;
pub mod chapter_10;
pub mod chapter_16;
pub mod chapter_17;

pub mod binary_search_tree;
pub mod binary_tree;
pub mod dijkstra;
pub mod heap;
pub mod ptr;
pub mod ptr_list;
pub mod sort;

/// Library-Level Do-Nothing Test
#[test]
fn alive() {}
