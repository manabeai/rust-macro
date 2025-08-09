pub mod bucked_dp;
pub mod digit_dp;
pub mod memorized_dfs;
pub mod topological_dp;

pub use bucked_dp::{DPBucketed, Engine};
pub use digit_dp::DigitDP;
pub use memorized_dfs::MemoizedDFS;
// pub use topological_dp::TopologicalMap;

#[derive(Clone)]
pub struct DpValue<V> {
    pub value: V,
}