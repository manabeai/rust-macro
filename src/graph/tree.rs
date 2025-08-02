use std::hash::Hash;

use super::{Graph, Node, Tree};

pub trait TreeDP<I, EW, NW> {
    fn dp<V, F1, F2>(&self, start: I, merge: F1, add_node: F2) -> Option<V>
    where
        V: Copy + std::fmt::Debug,
        F1: Fn(V, V) -> V,
        F2: Fn(Option<V>, &Node<NW>, Option<&EW>) -> V;
}

impl<I, EW, NW> TreeDP<I, EW, NW> for Graph<I, EW, NW, Tree>
where
    I: Clone + Eq + Hash + std::fmt::Debug,
    EW: Copy + std::fmt::Debug,
    NW: Copy + std::fmt::Debug,
{
    fn dp<V, F1, F2>(&self, start: I, merge: F1, add_node: F2) -> Option<V>
    where
        V: Copy + std::fmt::Debug,
        F1: Fn(V, V) -> V,
        F2: Fn(Option<V>, &Node<NW>, Option<&EW>) -> V,
    {
        let start_id = self.coord_map.get(&start)?;
        let n = self.nodes.len();
        let mut visited = vec![false; n];

        fn dfs_dp<V, F1, F2, I, EW, NW>(
            graph: &Graph<I, EW, NW, Tree>,
            node: usize,
            parent: Option<usize>,
            parent_edge_weight: Option<&EW>,
            visited: &mut [bool],
            merge: &F1,
            add_node: &F2,
        ) -> V
        where
            V: Copy + std::fmt::Debug,
            F1: Fn(V, V) -> V,
            F2: Fn(Option<V>, &Node<NW>, Option<&EW>) -> V,
            I: Clone + Eq + Hash + std::fmt::Debug,
            EW: Copy + std::fmt::Debug,
            NW: Copy + std::fmt::Debug,
        {
            visited[node] = true;

            let mut child_result: Option<V> = None;

            for &(child, edge_weight) in &graph.adj[node] {
                if Some(child) != parent && !visited[child] {
                    let child_dp = dfs_dp(
                        graph,
                        child,
                        Some(node),
                        edge_weight.as_ref(),
                        visited,
                        merge,
                        add_node,
                    );
                    child_result = Some(match child_result {
                        Some(current) => merge(current, child_dp),
                        None => child_dp,
                    });
                }
            }

            add_node(child_result, &graph.nodes[node], parent_edge_weight)
        }

        Some(dfs_dp(
            self,
            *start_id,
            None,
            None,
            &mut visited,
            &merge,
            &add_node,
        ))
    }
}
