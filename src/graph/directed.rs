use std::hash::Hash;

use super::core::Graph;
use super::types::Directed;

// Directed graph specific implementation
impl<I, EW, NW> Graph<I, EW, NW, Directed>
where
    I: Clone + Eq + Hash + std::fmt::Debug,
    EW: Copy + std::fmt::Debug,
    NW: Copy + std::fmt::Debug,
{
    /// Convert directed graph to DSU (Disjoint Set Union / Union-Find)
    ///
    /// Creates a Union-Find data structure representing the strongly connected components
    /// of the directed graph. Two nodes are in the same component if there is a path
    /// from one to the other and vice versa.
    ///
    /// # Algorithm
    ///
    /// Uses Kosaraju's algorithm for finding strongly connected components:
    /// 1. **First DFS**: Compute finish times on the original graph
    /// 2. **Transpose graph**: Reverse all edge directions
    /// 3. **Second DFS**: Process nodes in decreasing finish time order on transposed graph
    /// 4. **Create DSU**: Union nodes that belong to the same SCC
    ///
    /// # Returns
    ///
    /// A `UnionFind` structure where nodes in the same strongly connected component
    /// are unioned together. Node IDs are mapped to internal indices.
    ///
    /// # Examples
    ///
    /// ## Simple Cycle
    ///
    /// ```rust
    /// # use rust_macro::*;
    /// let mut graph = Graph::<usize, (), (), Directed>::new();
    /// // Create cycle: 1 -> 2 -> 3 -> 1
    /// graph.add_edge(1, 2, None);
    /// graph.add_edge(2, 3, None);
    /// graph.add_edge(3, 1, None);
    ///
    /// let mut dsu = graph.to_dsu();
    /// // All nodes should be in the same component
    /// assert!(dsu.same(0, 1)); // Assuming 1 maps to index 0, 2 to index 1
    /// ```
    ///
    /// ## Disconnected Components
    ///
    /// ```rust
    /// # use rust_macro::*;
    /// let mut graph = Graph::<usize, (), (), Directed>::new();
    /// // Create two separate components: 1->2 and 3->4
    /// graph.add_edge(1, 2, None);
    /// graph.add_edge(3, 4, None);
    ///
    /// let dsu = graph.to_dsu();
    /// // Nodes in different components should not be connected
    /// // (exact indices depend on internal mapping)
    /// ```
    ///
    /// # Time Complexity
    ///
    /// - **O(V + E)** where V is vertices and E is edges
    /// - Uses two DFS traversals for Kosaraju's algorithm
    ///
    /// # Space Complexity
    ///
    /// - **O(V)** for the DSU structure and auxiliary data structures
    ///
    /// # Notes
    ///
    /// - Only works on `Directed` graph types (compile-time restriction)
    /// - The returned DSU uses internal node indices (0 to n-1)
    /// - Self-loops and multiple edges are handled correctly
    pub fn to_dsu(&self) -> crate::UnionFind {
        use crate::UnionFind;

        let n = self.nodes.len();
        if n == 0 {
            return UnionFind::new(0);
        }

        // Step 1: First DFS to compute finish times
        let mut visited = vec![false; n];
        let mut finish_order = Vec::new();

        for i in 0..n {
            if !visited[i] {
                self.dfs1(i, &mut visited, &mut finish_order);
            }
        }

        // Step 2: Create transposed graph
        let mut transposed_adj = vec![Vec::new(); n];
        for (from, edges) in self.adj.iter().enumerate() {
            for &(to, _) in edges {
                transposed_adj[to].push(from);
            }
        }

        // Step 3: Second DFS on transposed graph in reverse finish order
        let mut visited2 = vec![false; n];
        let mut dsu = UnionFind::new(n);

        for &node in finish_order.iter().rev() {
            if !visited2[node] {
                let mut component = Vec::new();
                self.dfs2(node, &transposed_adj, &mut visited2, &mut component);

                // Union all nodes in this SCC
                for &other in component.iter().skip(1) {
                    dsu.unite(component[0], other);
                }
            }
        }

        dsu
    }

    // Helper function for first DFS (finish time computation)
    fn dfs1(&self, node: usize, visited: &mut [bool], finish_order: &mut Vec<usize>) {
        visited[node] = true;

        for &(next, _) in &self.adj[node] {
            if !visited[next] {
                self.dfs1(next, visited, finish_order);
            }
        }

        finish_order.push(node);
    }

    // Helper function for second DFS (SCC extraction)
    #[allow(clippy::only_used_in_recursion)]
    fn dfs2(
        &self,
        node: usize,
        transposed_adj: &[Vec<usize>],
        visited: &mut [bool],
        component: &mut Vec<usize>,
    ) {
        visited[node] = true;
        component.push(node);

        for &next in &transposed_adj[node] {
            if !visited[next] {
                self.dfs2(next, transposed_adj, visited, component);
            }
        }
    }
}