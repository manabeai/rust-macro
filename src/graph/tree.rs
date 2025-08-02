use std::collections::HashMap;
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

pub trait TreePreorder<I, EW, NW> {
    fn preorder<V, F>(&self, start: I, calculate: F) -> HashMap<I, V>
    where
        V: Clone + std::fmt::Debug,
        F: Fn(&Node<NW>, Option<&EW>) -> V;
}

impl<I, EW, NW> TreePreorder<I, EW, NW> for Graph<I, EW, NW, Tree>
where
    I: Clone + Eq + Hash + std::fmt::Debug,
    EW: Copy + std::fmt::Debug,
    NW: Copy + std::fmt::Debug,
{
    fn preorder<V, F>(&self, start: I, calculate: F) -> HashMap<I, V>
    where
        V: Clone + std::fmt::Debug,
        F: Fn(&Node<NW>, Option<&EW>) -> V,
    {
        let mut result = HashMap::new();

        if let Some(&start_id) = self.coord_map.get(&start) {
            let n = self.nodes.len();
            let mut visited = vec![false; n];

            fn dfs_preorder<V, F, I, EW, NW>(
                graph: &Graph<I, EW, NW, Tree>,
                node: usize,
                parent: Option<usize>,
                parent_edge_weight: Option<&EW>,
                visited: &mut [bool],
                calculate: &F,
                result: &mut HashMap<I, V>,
            ) where
                V: Clone + std::fmt::Debug,
                F: Fn(&Node<NW>, Option<&EW>) -> V,
                I: Clone + Eq + Hash + std::fmt::Debug,
                EW: Copy + std::fmt::Debug,
                NW: Copy + std::fmt::Debug,
            {
                visited[node] = true;

                // Calculate value for current node (preorder: process node before children)
                let value = calculate(&graph.nodes[node], parent_edge_weight);
                result.insert(graph.reverse_map[node].clone(), value);

                // Recursively visit children
                for &(child, edge_weight) in &graph.adj[node] {
                    if Some(child) != parent && !visited[child] {
                        dfs_preorder(
                            graph,
                            child,
                            Some(node),
                            edge_weight.as_ref(),
                            visited,
                            calculate,
                            result,
                        );
                    }
                }
            }

            dfs_preorder(
                self,
                start_id,
                None,
                None,
                &mut visited,
                &calculate,
                &mut result,
            );
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::super::Tree;
    use super::*;

    #[test]
    fn test_preorder_simple_tree() {
        // Create a simple tree:
        //     1
        //    / \
        //   2   3
        //  /
        // 4
        let mut graph = Graph::<usize, usize, usize, Tree>::new();

        graph.add_edge(1, 2, Some(10));
        graph.add_edge(1, 3, Some(20));
        graph.add_edge(2, 4, Some(30));

        graph.add_weight_to_node(1, 100);
        graph.add_weight_to_node(2, 200);
        graph.add_weight_to_node(3, 300);
        graph.add_weight_to_node(4, 400);

        // Calculate values based on node weight + edge weight (edge weight is 0 for root)
        let calculate = |node: &Node<usize>, edge_weight: Option<&usize>| {
            let node_weight = node.weight.unwrap_or(0);
            let edge_weight = edge_weight.unwrap_or(&0);
            node_weight + edge_weight
        };

        let result = graph.preorder(1, calculate);

        // Expected values:
        // Node 1: 100 + 0 = 100 (root, no parent edge)
        // Node 2: 200 + 10 = 210 (edge weight from 1->2)
        // Node 3: 300 + 20 = 320 (edge weight from 1->3)
        // Node 4: 400 + 30 = 430 (edge weight from 2->4)

        assert_eq!(result.get(&1), Some(&100));
        assert_eq!(result.get(&2), Some(&210));
        assert_eq!(result.get(&3), Some(&320));
        assert_eq!(result.get(&4), Some(&430));
        assert_eq!(result.len(), 4);
    }

    #[test]
    fn test_preorder_linear_tree() {
        // Create a linear tree: 1 -> 2 -> 3
        let mut graph = Graph::<usize, usize, (), Tree>::new();

        graph.add_edge(1, 2, Some(5));
        graph.add_edge(2, 3, Some(7));

        // Calculate depth (distance from root)
        let calculate = |_node: &Node<()>, edge_weight: Option<&usize>| *edge_weight.unwrap_or(&0);

        let result = graph.preorder(1, calculate);

        // Expected values (edge weights):
        // Node 1: 0 (root)
        // Node 2: 5 (edge 1->2)
        // Node 3: 7 (edge 2->3)

        assert_eq!(*result.get(&1).unwrap(), 0);
        assert_eq!(*result.get(&2).unwrap(), 5);
        assert_eq!(*result.get(&3).unwrap(), 7);
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_preorder_single_node() {
        let mut graph = Graph::<usize, (), usize, Tree>::new();
        graph.add_weight_to_node(1, 42);

        let calculate =
            |node: &Node<usize>, _edge_weight: Option<&()>| node.weight.unwrap_or(0) * 2;

        let result = graph.preorder(1, calculate);

        assert_eq!(result.get(&1), Some(&84));
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_preorder_nonexistent_start() {
        let mut graph = Graph::<usize, (), usize, Tree>::new();
        graph.add_weight_to_node(1, 42);

        let calculate = |node: &Node<usize>, _edge_weight: Option<&()>| node.weight.unwrap_or(0);

        // Try to start from a node that doesn't exist
        let result = graph.preorder(999, calculate);

        assert!(result.is_empty());
    }
}
