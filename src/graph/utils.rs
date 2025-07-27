use std::fmt::Debug;

use super::core::Graph;
use super::types::GraphType;

#[allow(dead_code)]
pub fn gen_grid_graph<V, F, T>(
    input: Vec<Vec<V>>,
    is_connectable: F,
) -> Graph<(usize, usize), usize, V, T>
where
    V: Clone + Debug,
    F: Fn(&V) -> bool,
    T: GraphType,
{
    let h = input.len();
    let w = input[0].len();
    let mut graph = Graph::new();

    for i in 0..h {
        for j in 0..w {
            if is_connectable(&input[i][j]) {
                graph.add_weight_to_node((i, j), input[i][j].clone());

                if i > 0 && is_connectable(&input[i - 1][j]) {
                    graph.add_edge((i, j), (i - 1, j), Some(1));
                }
                if i + 1 < h && is_connectable(&input[i + 1][j]) {
                    graph.add_edge((i, j), (i + 1, j), Some(1));
                }
                if j > 0 && is_connectable(&input[i][j - 1]) {
                    graph.add_edge((i, j), (i, j - 1), Some(1));
                }
                if j + 1 < w && is_connectable(&input[i][j + 1]) {
                    graph.add_edge((i, j), (i, j + 1), Some(1));
                }
            }
        }
    }
    graph
}
