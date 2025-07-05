//! A collection of useful utilities for competitive programming in Rust

pub mod cumulative_sum;
pub mod graph;
pub mod imos;
pub mod macro_utils;
pub mod union_find;
pub mod utils;

pub use cumulative_sum::{CumulativeSum, CumulativeSum2D};
pub use graph::{Edge, Graph, Node};
pub use imos::{Imos1D, Imos2D};
pub use union_find::UnionFind;
pub use utils::{yesno, Compress};
