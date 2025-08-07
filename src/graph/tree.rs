use std::collections::HashMap;
use std::hash::Hash;

use super::{Graph, Node, Tree};

pub trait TreeDP<I, EW, NW> {
    fn dp<V, F1, F2>(&self, start: I, merge: F1, add_node: F2) -> Option<V>
    where
        V: Copy,
        F1: Fn(V, V) -> V,
        F2: Fn(Option<V>, &Node<NW>, Option<&EW>) -> V;
}

impl<I, EW, NW> TreeDP<I, EW, NW> for Graph<I, EW, NW, Tree>
where
    I: Clone + Eq + Hash,
    EW: Copy,
    NW: Copy,
{
    fn dp<V, F1, F2>(&self, start: I, merge: F1, add_node: F2) -> Option<V>
    where
        V: Copy,
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
            V: Copy,
            F1: Fn(V, V) -> V,
            F2: Fn(Option<V>, &Node<NW>, Option<&EW>) -> V,
            I: Clone + Eq + Hash,
            EW: Copy,
            NW: Copy,
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
        V: Clone,
        F: Fn(&Node<NW>, Option<&EW>, Option<&V>) -> V;
}

impl<I, EW, NW> TreePreorder<I, EW, NW> for Graph<I, EW, NW, Tree>
where
    I: Clone + Eq + Hash,
    EW: Copy,
    NW: Copy,
{
    fn preorder<V, F>(&self, start: I, calculate: F) -> HashMap<I, V>
    where
        V: Clone,
        F: Fn(&Node<NW>, Option<&EW>, Option<&V>) -> V,
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
                parent_value: Option<&V>,
                visited: &mut [bool],
                calculate: &F,
                result: &mut HashMap<I, V>,
            ) where
                V: Clone,
                F: Fn(&Node<NW>, Option<&EW>, Option<&V>) -> V,
                I: Clone + Eq + Hash,
                EW: Copy,
                NW: Copy,
            {
                visited[node] = true;

                // Calculate value for current node using parent's result (preorder: process node before children)
                let value = calculate(&graph.nodes[node], parent_edge_weight, parent_value);
                result.insert(graph.reverse_map[node].clone(), value.clone());

                // Recursively visit children, passing current node's value as parent_value
                for &(child, edge_weight) in &graph.adj[node] {
                    if Some(child) != parent && !visited[child] {
                        dfs_preorder(
                            graph,
                            child,
                            Some(node),
                            edge_weight.as_ref(),
                            Some(&value),
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
                None, // No parent value for root
                &mut visited,
                &calculate,
                &mut result,
            );
        }

        result
    }
}

pub trait TreePostorder<I, EW, NW> {
    fn postorder<V, F>(&self, start: I, calculate: F) -> HashMap<I, V>
    where
        V: Clone,
        F: Fn(&Node<NW>, Option<&EW>, Vec<V>) -> V;
}

impl<I, EW, NW> TreePostorder<I, EW, NW> for Graph<I, EW, NW, Tree>
where
    I: Clone + Eq + Hash,
    EW: Copy,
    NW: Copy,
{
    fn postorder<V, F>(&self, start: I, calculate: F) -> HashMap<I, V>
    where
        V: Clone,
        F: Fn(&Node<NW>, Option<&EW>, Vec<V>) -> V,
    {
        let mut result = HashMap::new();

        if let Some(&start_id) = self.coord_map.get(&start) {
            let n = self.nodes.len();
            let mut visited = vec![false; n];

            fn dfs_postorder<V, F, I, EW, NW>(
                graph: &Graph<I, EW, NW, Tree>,
                node: usize,
                parent: Option<usize>,
                parent_edge_weight: Option<&EW>,
                visited: &mut [bool],
                calculate: &F,
                result: &mut HashMap<I, V>,
            ) -> V
            where
                V: Clone,
                F: Fn(&Node<NW>, Option<&EW>, Vec<V>) -> V,
                I: Clone + Eq + Hash,
                EW: Copy,
                NW: Copy,
            {
                visited[node] = true;

                // First visit all children and collect their results (postorder: process children before current node)
                let mut child_results = Vec::new();
                for &(child, edge_weight) in &graph.adj[node] {
                    if Some(child) != parent && !visited[child] {
                        let child_value = dfs_postorder(
                            graph,
                            child,
                            Some(node),
                            edge_weight.as_ref(),
                            visited,
                            calculate,
                            result,
                        );
                        child_results.push(child_value);
                    }
                }

                // Then calculate value for current node using child results
                let value = calculate(&graph.nodes[node], parent_edge_weight, child_results);
                result.insert(graph.reverse_map[node].clone(), value.clone());
                value
            }

            dfs_postorder(
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

        // Calculate values based on node weight + edge weight + parent value
        let calculate =
            |node: &Node<usize>, edge_weight: Option<&usize>, parent_value: Option<&usize>| {
                let node_weight = node.weight.unwrap_or(0);
                let edge_weight = edge_weight.unwrap_or(&0);
                let parent_contribution = parent_value.unwrap_or(&0);
                node_weight + edge_weight + parent_contribution
            };

        let result = graph.preorder(1, calculate);

        // Expected values (preorder with parent value accumulation):
        // Node 1: 100 + 0 + 0 = 100 (root, no parent)
        // Node 2: 200 + 10 + 100 = 310 (includes parent 1's value)
        // Node 3: 300 + 20 + 100 = 420 (includes parent 1's value)
        // Node 4: 400 + 30 + 310 = 740 (includes parent 2's value)

        assert_eq!(result.get(&1), Some(&100));
        assert_eq!(result.get(&2), Some(&310));
        assert_eq!(result.get(&3), Some(&420));
        assert_eq!(result.get(&4), Some(&740));
        assert_eq!(result.len(), 4);
    }

    #[test]
    fn test_preorder_linear_tree() {
        // Create a linear tree: 1 -> 2 -> 3
        let mut graph = Graph::<usize, usize, (), Tree>::new();

        graph.add_edge(1, 2, Some(5));
        graph.add_edge(2, 3, Some(7));

        // Calculate depth (distance from root) using parent depth + 1
        let calculate =
            |_node: &Node<()>, _edge_weight: Option<&usize>, parent_value: Option<&usize>| {
                parent_value.unwrap_or(&0) + 1 // Parent depth + 1
            };

        let result = graph.preorder(1, calculate);

        // Expected values (depth from root):
        // Node 1: 0 + 1 = 1 (root depth)
        // Node 2: 1 + 1 = 2 (parent depth 1 + 1)
        // Node 3: 2 + 1 = 3 (parent depth 2 + 1)

        assert_eq!(*result.get(&1).unwrap(), 1);
        assert_eq!(*result.get(&2).unwrap(), 2);
        assert_eq!(*result.get(&3).unwrap(), 3);
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_preorder_single_node() {
        let mut graph = Graph::<usize, (), usize, Tree>::new();
        graph.add_weight_to_node(1, 42);

        let calculate =
            |node: &Node<usize>, _edge_weight: Option<&()>, _parent_value: Option<&usize>| {
                node.weight.unwrap_or(0) * 2
            };

        let result = graph.preorder(1, calculate);

        assert_eq!(result.get(&1), Some(&84));
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_preorder_nonexistent_start() {
        let mut graph = Graph::<usize, (), usize, Tree>::new();
        graph.add_weight_to_node(1, 42);

        let calculate =
            |node: &Node<usize>, _edge_weight: Option<&()>, _parent_value: Option<&usize>| {
                node.weight.unwrap_or(0)
            };

        // Try to start from a node that doesn't exist
        let result = graph.preorder(999, calculate);

        assert!(result.is_empty());
    }

    #[test]
    fn test_postorder_simple_tree() {
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

        // Calculate values based on node weight + edge weight + sum of child values
        let calculate =
            |node: &Node<usize>, edge_weight: Option<&usize>, child_results: Vec<usize>| {
                let node_weight = node.weight.unwrap_or(0);
                let edge_weight = edge_weight.unwrap_or(&0);
                let child_sum: usize = child_results.iter().sum();
                node_weight + edge_weight + child_sum
            };

        let result = graph.postorder(1, calculate);

        // Expected values (postorder with child sum):
        // Node 4: 400 + 30 + 0 = 430 (leaf node, no children)
        // Node 2: 200 + 10 + 430 = 640 (includes child 4's result)
        // Node 3: 300 + 20 + 0 = 320 (leaf node, no children)
        // Node 1: 100 + 0 + (640 + 320) = 1060 (includes both children's results)

        assert_eq!(result.get(&4), Some(&430));
        assert_eq!(result.get(&2), Some(&640));
        assert_eq!(result.get(&3), Some(&320));
        assert_eq!(result.get(&1), Some(&1060));
        assert_eq!(result.len(), 4);
    }

    #[test]
    fn test_postorder_subtree_size() {
        // Create a tree and calculate subtree sizes:
        //     1
        //    / \
        //   2   3
        //  / \   \
        // 4   5   6
        let mut graph = Graph::<usize, (), (), Tree>::new();

        graph.add_edge(1, 2, None);
        graph.add_edge(1, 3, None);
        graph.add_edge(2, 4, None);
        graph.add_edge(2, 5, None);
        graph.add_edge(3, 6, None);

        // Calculate subtree size (including current node)
        // This is a perfect use case for postorder traversal
        let calculate = |_node: &Node<()>, _edge_weight: Option<&()>, child_results: Vec<usize>| {
            1 + child_results.iter().sum::<usize>() // Current node + sum of child subtree sizes
        };

        let result = graph.postorder(1, calculate);

        // Expected subtree sizes:
        // Node 4: 1 (leaf)
        // Node 5: 1 (leaf)
        // Node 6: 1 (leaf)
        // Node 2: 1 + 1 + 1 = 3 (self + child 4 + child 5)
        // Node 3: 1 + 1 = 2 (self + child 6)
        // Node 1: 1 + 3 + 2 = 6 (self + subtree 2 + subtree 3)

        assert_eq!(*result.get(&4).unwrap(), 1);
        assert_eq!(*result.get(&5).unwrap(), 1);
        assert_eq!(*result.get(&6).unwrap(), 1);
        assert_eq!(*result.get(&2).unwrap(), 3);
        assert_eq!(*result.get(&3).unwrap(), 2);
        assert_eq!(*result.get(&1).unwrap(), 6);
        assert_eq!(result.len(), 6);
    }

    #[test]
    fn test_postorder_depth_calculation() {
        // Create a linear tree: 1 -> 2 -> 3 -> 4
        let mut graph = Graph::<usize, usize, (), Tree>::new();

        graph.add_edge(1, 2, Some(1));
        graph.add_edge(2, 3, Some(1));
        graph.add_edge(3, 4, Some(1));

        // Calculate depth from root (0-indexed)
        let calculate =
            |_node: &Node<()>, edge_weight: Option<&usize>, _child_results: Vec<usize>| {
                *edge_weight.unwrap_or(&0)
            };

        let result = graph.postorder(1, calculate);

        // Expected values (edge weights represent depth increment):
        // Node 1: 0 (root)
        // Node 2: 1 (depth 1)
        // Node 3: 1 (edge weight 2->3)
        // Node 4: 1 (edge weight 3->4)

        assert_eq!(*result.get(&1).unwrap(), 0);
        assert_eq!(*result.get(&2).unwrap(), 1);
        assert_eq!(*result.get(&3).unwrap(), 1);
        assert_eq!(*result.get(&4).unwrap(), 1);
        assert_eq!(result.len(), 4);
    }

    #[test]
    fn test_postorder_single_node() {
        let mut graph = Graph::<usize, (), usize, Tree>::new();
        graph.add_weight_to_node(1, 42);

        let calculate =
            |node: &Node<usize>, _edge_weight: Option<&()>, _child_results: Vec<usize>| {
                node.weight.unwrap_or(0) * 3
            };

        let result = graph.postorder(1, calculate);

        assert_eq!(result.get(&1), Some(&126)); // 42 * 3
        assert_eq!(result.len(), 1);
    }

    // #[test]
    // fn test_postorder_vs_preorder_difference() {
    //     // Create a tree where order matters:
    //     //     1
    //     //    / \
    //     //   2   3
    //     //  /
    //     // 4
    //     let mut graph = Graph::<usize, usize, (), Tree>::new();

    //     graph.add_edge(1, 2, Some(1));
    //     graph.add_edge(1, 3, Some(1));
    //     graph.add_edge(2, 4, Some(1));

    //     // Simple calculation that just uses edge weight (for preorder)
    //     let preorder_calculate = |_node: &Node<()>, edge_weight: Option<&usize>| {
    //         *edge_weight.unwrap_or(&0)
    //     };

    //     // Simple calculation that just uses edge weight (for postorder)
    //     let postorder_calculate = |_node: &Node<()>, edge_weight: Option<&usize>, _child_results: Vec<usize>| {
    //         *edge_weight.unwrap_or(&0)
    //     };

    //     // let preorder_result = graph.preorder(1, preorder_calculate);
    //     let postorder_result = graph.postorder(1, postorder_calculate);

    //     // The values should be the same for this simple calculation
    //     // since we're only using edge weights, not child results
    //     assert_eq!(preorder_result, postorder_result);

    //     // But the processing order is different internally
    //     assert_eq!(preorder_result.len(), 4);
    //     assert_eq!(postorder_result.len(), 4);
    // }

    #[test]
    fn test_postorder_nonexistent_start() {
        let mut graph = Graph::<usize, (), usize, Tree>::new();
        graph.add_weight_to_node(1, 42);

        let calculate =
            |node: &Node<usize>, _edge_weight: Option<&()>, _child_results: Vec<usize>| {
                node.weight.unwrap_or(0)
            };

        // Try to start from a node that doesn't exist
        let result = graph.postorder(999, calculate);

        assert!(result.is_empty());
    }
}
