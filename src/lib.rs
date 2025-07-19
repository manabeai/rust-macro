//! A collection of useful utilities for competitive programming in Rust

pub mod cumulative_sum;
pub mod dp;
pub mod graph;
pub mod imos;
pub mod macro_utils;
pub mod union_find;
pub mod utils;

pub use cumulative_sum::{CumulativeSum, CumulativeSum2D};
pub use dp::DigitDP;
pub use graph::{Directed, Graph, Node, Tree, Undirected};
pub use imos::{Imos1D, Imos2D};
pub use union_find::{PersistentUnionFind, UnionFind};
pub use utils::{fmt_bitvec, fmt_u2bit, is_palindrome, to_base, yesno, Compress};
