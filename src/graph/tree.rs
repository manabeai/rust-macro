use im_rc::HashSet as ImHashSet;
use std::hash::Hash;

use super::core::Graph;
use super::types::{Node, Tree};

// Tree-specific implementation for tree DP
impl<I, EW, NW> Graph<I, EW, NW, Tree>
where
    I: Clone + Eq + Hash + std::fmt::Debug,
    EW: Copy + std::fmt::Debug,
    NW: Copy + std::fmt::Debug,
{
    /// Tree DP (Dynamic Programming on Tree)
    ///
    /// Performs dynamic programming calculation on a tree structure using a pure functional approach.
    /// The algorithm traverses the tree from the specified starting node and combines results
    /// from child subtrees using the provided merge function.
    ///
    /// # Algorithm
    ///
    /// The DP computation follows these steps:
    /// 1. **Recursively visit** all children of the current node
    /// 2. **Merge results** from child subtrees using the `merge` function
    /// 3. **Apply node transformation** using the `add_node` function
    /// 4. **Handle identity elements** automatically (users don't need to handle `None` cases)
    ///
    /// # Pure Functional Design
    ///
    /// - Uses immutable visited sets (`im_rc::HashSet`) to maintain purity
    /// - No mutable state during traversal
    /// - Identity element (`None`) operations are handled internally
    ///
    /// # Parameters
    ///
    /// - `start`: The starting node ID for the DP computation
    /// - `merge`: Function to combine two non-identity values (`Fn(V, V) -> V`)
    /// - `add_node`: Function to transform a value at each node (`Fn(Option<V>, &Node<NW>, Option<&EW>) -> V`)
    ///
    /// # Returns
    ///
    /// - `Some(V)`: The computed DP value if the starting node exists
    /// - `None`: If the starting node is not found in the graph
    ///
    /// # Examples
    ///
    /// ## Minimum Path Sum
    ///
    /// ```rust
    /// # use rust_macro::*;
    /// let mut graph = Graph::<usize, usize, (), Tree>::new();
    /// graph.add_edge(1, 2, Some(5));
    /// graph.add_edge(1, 3, Some(3));
    /// graph.add_edge(2, 4, Some(7));
    ///
    /// // Find minimum path sum from root to leaves
    /// let merge = |x: usize, y: usize| x.min(y);
    /// let add_node = |child_min: Option<usize>, _node: &Node<()>, edge_weight: Option<&usize>| {
    ///     let edge_cost = edge_weight.unwrap_or(&0);
    ///     match child_min {
    ///         Some(min_val) => edge_cost + min_val,
    ///         None => *edge_cost, // Leaf node
    ///     }
    /// };
    ///
    /// let min_path = graph.dp(1, merge, add_node);
    /// assert_eq!(min_path, Some(3)); // Path: 1 -> 3(3) -> leaf = 3
    /// ```
    ///
    /// ## Maximum Path Sum
    ///
    /// ```rust
    /// # use rust_macro::*;
    /// let mut graph = Graph::<usize, usize, (), Tree>::new();
    /// graph.add_edge(1, 2, Some(5));
    /// graph.add_edge(1, 3, Some(3));
    /// graph.add_edge(2, 4, Some(7));
    ///
    /// // Find maximum path sum from root to leaves
    /// let merge = |x: usize, y: usize| x.max(y);
    /// let add_node = |child_max: Option<usize>, _node: &Node<()>, edge_weight: Option<&usize>| {
    ///     let edge_cost = edge_weight.unwrap_or(&0);
    ///     match child_max {
    ///         Some(max_val) => edge_cost + max_val,
    ///         None => *edge_cost, // Leaf node
    ///     }
    /// };
    ///
    /// let max_path = graph.dp(1, merge, add_node);
    /// assert_eq!(max_path, Some(12)); // Path: 1 -> 2(5) -> 4(7) = 12
    /// ```
    ///
    /// ## Basic Usage
    ///
    /// ```rust
    /// # use rust_macro::*;
    /// let mut graph = Graph::<usize, usize, (), Tree>::new();
    /// graph.add_edge(1, 2, Some(10));
    ///
    /// // Simple sum calculation
    /// let merge = |x: usize, y: usize| x + y;
    /// let add_node = |child_sum: Option<usize>, _node: &Node<()>, edge_weight: Option<&usize>| {
    ///     let edge_cost = edge_weight.unwrap_or(&0);
    ///     child_sum.unwrap_or(0) + edge_cost
    /// };
    ///
    /// let result = graph.dp(1, merge, add_node);
    /// assert_eq!(result, Some(10)); // Edge weight from 1 to 2
    /// ```
    ///
    /// # Time Complexity
    ///
    /// - **O(N)** where N is the number of nodes in the tree
    /// - Each node is visited exactly once due to the tree structure
    ///
    /// # Space Complexity
    ///
    /// - **O(H)** where H is the height of the tree (recursion stack)
    /// - **O(N)** for the immutable visited set in worst case
    ///
    /// # Notes
    ///
    /// - Only works on `Tree` graph types (compile-time restriction)
    /// - The graph should actually be a tree (connected, acyclic) for correct results
    /// - Uses immutable data structures for pure functional semantics
    /// - Identity element handling is automatic - no need to handle `None` in `merge`
    pub fn dp<V, F1, F2>(&self, start: I, merge: F1, add_node: F2) -> Option<V>
    where
        V: Copy + std::fmt::Debug,
        F1: Fn(V, V) -> V,
        F2: Fn(Option<V>, &Node<NW>, Option<&EW>) -> V,
    {
        let start_id = match self.coord_map.get(&start) {
            Some(&id) => id,
            None => return None,
        };

        let visited = ImHashSet::new();

        fn dp_pure<EW, NW, V, F1, F2>(
            nodes: &[Node<NW>],
            adj: &[Vec<(usize, Option<EW>)>],
            prev_weight: Option<&EW>,
            current: usize,
            visited: ImHashSet<usize>,
            merge: &F1,
            add_node: &F2,
        ) -> Option<V>
        where
            V: Copy + std::fmt::Debug,
            F1: Fn(V, V) -> V,
            F2: Fn(Option<V>, &Node<NW>, Option<&EW>) -> V,
            EW: Copy + std::fmt::Debug,
            NW: Copy + std::fmt::Debug,
        {
            if visited.contains(&current) {
                return None;
            }

            let new_visited = visited.update(current);
            let mut result = None;

            let node = &nodes[current];
            for &(next, edge_weight) in &adj[current] {
                let sub_result = dp_pure(
                    nodes,
                    adj,
                    edge_weight.as_ref(),
                    next,
                    new_visited.clone(),
                    merge,
                    add_node,
                );

                // Handle identity element operations internally
                result = match (result, sub_result) {
                    (Some(x), Some(y)) => Some(merge(x, y)),
                    (Some(x), None) | (None, Some(x)) => Some(x),
                    (None, None) => None,
                };
            }

            if let Some(weight) = prev_weight {
                Some(add_node(result, node, Some(weight)))
            } else {
                result
            }
        }

        dp_pure(
            &self.nodes,
            &self.adj,
            None,
            start_id,
            visited,
            &merge,
            &add_node,
        )
    }
}