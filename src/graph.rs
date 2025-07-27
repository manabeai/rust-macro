use rustc_hash::FxHasher;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::{BuildHasherDefault, Hash};
use std::marker::PhantomData;

pub trait GraphType {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Undirected {}
impl GraphType for Undirected {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Directed {}
impl GraphType for Directed {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tree {}
impl GraphType for Tree {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dag {}
impl GraphType for Dag {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Node<NW> {
    pub weight: Option<NW>,
}

#[derive(Debug, Clone)]
pub struct Graph<I: Debug, EW: Debug, NW: Debug, T: GraphType> {
    pub coord_map: HashMap<I, usize, BuildHasherDefault<FxHasher>>,
    pub reverse_map: Vec<I>,
    pub nodes: Vec<Node<NW>>,
    pub adj: Vec<Vec<(usize, Option<EW>)>>,
    _phantom: PhantomData<T>,
}

impl<I: Clone + Eq + Hash + Debug, EW: Debug, NW: Debug, T: GraphType> Graph<I, EW, NW, T> {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Graph {
            coord_map: HashMap::<I, usize, BuildHasherDefault<FxHasher>>::default(),
            reverse_map: Vec::new(),
            nodes: Vec::new(),
            adj: Vec::new(),
            _phantom: PhantomData,
        }
    }

    fn get_or_create_id(&mut self, key: I) -> usize {
        if let Some(&id) = self.coord_map.get(&key) {
            id
        } else {
            let id = self.reverse_map.len();
            self.coord_map.insert(key.clone(), id);
            self.reverse_map.push(key);
            self.nodes.push(Node { weight: None });
            self.adj.push(Vec::new());
            id
        }
    }

    pub fn add_edge(&mut self, from: I, to: I, weight: Option<EW>) {
        let from_id = self.get_or_create_id(from);
        let to_id = self.get_or_create_id(to);
        self.adj[from_id].push((to_id, weight));
    }

    pub fn add_weight_to_node(&mut self, id: I, weight: NW) {
        let node_id = self.get_or_create_id(id);
        self.nodes[node_id].weight = Some(weight);
    }
}


// Tree-specific implementation for tree DP
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
        let start_id = match self.coord_map.get(&start) {
            Some(&id) => id,
            None => return None,
        };


        fn dp_pure<EW, NW, V, F1, F2>(
            nodes: &[Node<NW>],
            adj: &[Vec<(usize, Option<EW>)>],
            prev_weight: Option<&EW>,
            current: usize,
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

            let mut result = None;

            let node = &nodes[current];
            for &(next, edge_weight) in &adj[current] {
                let sub_result = dp_pure(
                    nodes,
                    adj,
                    edge_weight.as_ref(),
                    next,
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
            &merge,
            &add_node,
        )
    }
}

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

#[allow(dead_code)]
fn gen_grid_graph<V, F, T>(
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

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    // #[test]
    // fn test_simple_path() {
    //     let mut graph = Graph::<usize, usize, usize, Tree>::new();
    //     graph.add_edge(1, 2, Some(5));
    //     graph.add_edge(2, 1, Some(5));
    //     graph.add_edge(2, 3, Some(10));
    //     graph.add_edge(3, 2, Some(10));
    //     graph.add_edge(1, 4, Some(16));
    //     graph.add_edge(4, 1, Some(16));
    //     graph.add_edge(5, 6, Some(34));
    //     graph.add_edge(6, 5, Some(34));

    //     let merge = |x: usize, y: usize| x + y;
    //     let add_node = |a: Option<usize>, _: &Node<usize>, edge_weight: Option<&usize>| {
    //         let weight = edge_weight.unwrap_or(&0);
    //         match a {
    //             Some(x) => x + weight,
    //             None => *weight,
    //         }
    //     };
    //     let ans = graph.dp(1, merge, add_node);
    //     assert_eq!(ans, Some(31));
    //     assert_eq!(
    //         graph.dp(6, merge, add_node),
    //         Some(34),
    //         "The total weight from node 6 should be 34"
    //     );
    // }

    #[test]
    // fn test_simple_reachability() {
    //     let mut graph = Graph::<usize, usize, usize, Tree>::new();
    //     graph.add_edge(1, 2, Some(5));
    //     graph.add_edge(2, 1, Some(5));
    //     graph.add_edge(2, 3, Some(10));
    //     graph.add_edge(3, 2, Some(10));
    //     graph.add_edge(1, 4, Some(16));
    //     graph.add_edge(4, 1, Some(31));

    //     let merge = |x: bool, y: bool| x || y;

    //     let _goal = 2;
    //     let add_node = |res: Option<bool>, _node: &Node<usize>, _edge_weight: Option<&usize>| {
    //         res.unwrap_or(false) // Note: We can't check edge.to anymore, need different approach
    //     };
    //     // This test needs to be redesigned since we don't store 'to' anymore
    //     // For now, just test that DFS completes without error
    //     let _result = graph.dp(1, merge, add_node);
    // }

    #[test]
    fn test_min_max_weights() {
        use std::cmp::{max, min};
        let mut graph = Graph::<usize, usize, usize, Tree>::new();
        graph.add_edge(1, 2, Some(5));
        graph.add_edge(2, 3, Some(10));
        graph.add_edge(1, 3, Some(15));
        graph.add_edge(2, 4, Some(20));

        type V = (usize, usize);

        let merge = |(amin, amax): V, (bmin, bmax): V| (min(amin, bmin), max(amax, bmax));
        let add_node = |res: Option<V>, _node: &Node<usize>, edge_weight: Option<&usize>| {
            let weight = edge_weight.unwrap_or(&0);
            match res {
                Some((min_weight, max_weight)) => {
                    (min(min_weight, *weight), max(max_weight, *weight))
                }
                None => (*weight, *weight),
            }
        };
        let result = graph.dp(1, merge, add_node);
        let (min_weight, max_weight) = result.unwrap();
        assert_eq!(min_weight, 5);
        assert_eq!(max_weight, 20);
    }

    #[test]
    fn test_grid_graph_connected() {
        let g = vec![vec![1, 0, 0], vec![1, 1, 0], vec![0, 1, 1]];

        let graph = gen_grid_graph::<_, _, Undirected>(g, |&x| x == 1);

        // Grid graph connectivity test - verify correct number of nodes created
        // Grid: [1,0,0]
        //       [1,1,0]
        //       [0,1,1]
        // Should create nodes for positions: (0,0), (1,0), (1,1), (2,1), (2,2) = 5 nodes
        assert_eq!(graph.nodes.len(), 5);
    }

    #[test]
    fn test_directed_to_dsu_simple_cycle() {
        let mut graph = Graph::<usize, (), (), Directed>::new();
        // Create cycle: 1 -> 2 -> 3 -> 1
        graph.add_edge(1, 2, None);
        graph.add_edge(2, 3, None);
        graph.add_edge(3, 1, None);

        let mut dsu = graph.to_dsu();

        // All nodes in the cycle should be in the same strongly connected component
        let node1_idx = graph.coord_map[&1];
        let node2_idx = graph.coord_map[&2];
        let node3_idx = graph.coord_map[&3];

        assert!(dsu.same(node1_idx, node2_idx));
        assert!(dsu.same(node2_idx, node3_idx));
        assert!(dsu.same(node1_idx, node3_idx));
    }

    #[test]
    fn test_directed_to_dsu_separate_components() {
        let mut graph = Graph::<usize, (), (), Directed>::new();
        // Create two separate strongly connected components
        // Component 1: 1 -> 2 -> 1
        graph.add_edge(1, 2, None);
        graph.add_edge(2, 1, None);
        // Component 2: 3 -> 4 -> 3
        graph.add_edge(3, 4, None);
        graph.add_edge(4, 3, None);

        let mut dsu = graph.to_dsu();

        let node1_idx = graph.coord_map[&1];
        let node2_idx = graph.coord_map[&2];
        let node3_idx = graph.coord_map[&3];
        let node4_idx = graph.coord_map[&4];

        // Nodes within the same SCC should be connected
        assert!(dsu.same(node1_idx, node2_idx));
        assert!(dsu.same(node3_idx, node4_idx));

        // Nodes from different SCCs should not be connected
        assert!(!dsu.same(node1_idx, node3_idx));
        assert!(!dsu.same(node1_idx, node4_idx));
        assert!(!dsu.same(node2_idx, node3_idx));
        assert!(!dsu.same(node2_idx, node4_idx));
    }

    #[test]
    fn test_directed_to_dsu_linear_graph() {
        let mut graph = Graph::<usize, (), (), Directed>::new();
        // Create linear directed graph: 1 -> 2 -> 3 -> 4
        graph.add_edge(1, 2, None);
        graph.add_edge(2, 3, None);
        graph.add_edge(3, 4, None);

        let mut dsu = graph.to_dsu();

        // In a linear directed graph, each node is its own SCC
        let node1_idx = graph.coord_map[&1];
        let node2_idx = graph.coord_map[&2];
        let node3_idx = graph.coord_map[&3];
        let node4_idx = graph.coord_map[&4];

        assert!(!dsu.same(node1_idx, node2_idx));
        assert!(!dsu.same(node2_idx, node3_idx));
        assert!(!dsu.same(node3_idx, node4_idx));
        assert!(!dsu.same(node1_idx, node3_idx));
        assert!(!dsu.same(node1_idx, node4_idx));
        assert!(!dsu.same(node2_idx, node4_idx));
    }

    #[test]
    fn test_tree_dp_min_path_sum() {
        // Tree structure:
        //     1
        //    / \
        //   2   3
        //  /   / \
        // 4   5   6
        let mut graph = Graph::<usize, usize, (), Tree>::new();

        // Add edges with weights (parent -> child direction only)
        graph.add_edge(1, 2, Some(5));
        graph.add_edge(1, 3, Some(3));
        graph.add_edge(2, 4, Some(7));
        graph.add_edge(3, 5, Some(2));
        graph.add_edge(3, 6, Some(8));

        // DP for minimum path sum from root to leaves
        let merge = |x: usize, y: usize| x.min(y);

        let add_node = |child_min: Option<usize>, _node: &Node<()>, edge_weight: Option<&usize>| {
            let edge_cost = edge_weight.unwrap_or(&0);
            match child_min {
                Some(min_val) => edge_cost + min_val,
                None => *edge_cost, // Leaf node
            }
        };

        let result = graph.dp(1, merge, add_node);
        // Min path: 1 -> 3(3) -> 5(2) = 5
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_tree_dp_max_path_sum() {
        // Same tree structure as min test
        let mut graph = Graph::<usize, usize, (), Tree>::new();

        // Add edges with weights (parent -> child direction only)
        graph.add_edge(1, 2, Some(5));
        graph.add_edge(1, 3, Some(3));
        graph.add_edge(2, 4, Some(7));
        graph.add_edge(3, 5, Some(2));
        graph.add_edge(3, 6, Some(8));

        // DP for maximum path sum from root to leaves
        let merge = |x: usize, y: usize| x.max(y);

        let add_node = |child_max: Option<usize>, _node: &Node<()>, edge_weight: Option<&usize>| {
            let edge_cost = edge_weight.unwrap_or(&0);
            match child_max {
                Some(max_val) => edge_cost + max_val,
                None => *edge_cost, // Leaf node
            }
        };

        let result = graph.dp(1, merge, add_node);
        // Max path: 1 -> 2(5) -> 4(7) = 12
        assert_eq!(result, Some(12));
    }
}
