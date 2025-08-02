pub mod directed;
pub mod tree;

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

    // fn key2id(&self, key: &I) -> Option<usize> {
    //     self.coord_map.get(key).cloned()
    // }

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

    pub fn get_node_weight(&self, id: &I) -> Option<&NW> {
        self.coord_map
            .get(id)
            .and_then(|&node_id| self.nodes[node_id].weight.as_ref())
    }

    pub fn get_node(&self, id: &I) -> Option<&Node<NW>> {
        self.coord_map.get(id).map(|&node_id| &self.nodes[node_id])
    }

    // pub fn get_adjacent_nodes(&self, id: &I) -> Option<&[(usize, Option<EW>)]> {
    //     self.coord_map.get(id).map(|&node_id| &self.adj[node_id])
    // }
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

pub use tree::TreeDP;

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_graph_creation() {
        let mut graph = Graph::<usize, usize, usize, Undirected>::new();
        graph.add_edge(1, 2, Some(5));
        graph.add_edge(2, 3, Some(10));
        graph.add_weight_to_node(1, 5);
        graph.add_weight_to_node(2, 10);

        assert_eq!(graph.get_node_weight(&1), Some(&5));
    }

    #[test]
    fn test_min_max_weights() {
        use std::cmp::{max, min};
        let mut graph = Graph::<usize, usize, usize, Tree>::new();
        graph.add_edge(1, 2, Some(5));
        graph.add_edge(2, 3, Some(10));
        graph.add_edge(2, 4, Some(20));

        type V = (usize, usize);

        let merge = |(amin, amax): V, (bmin, bmax): V| (min(amin, bmin), max(amax, bmax));
        let add_node = |res: Option<V>, _node: &Node<usize>, edge_weight: Option<&usize>| {
            let weight = edge_weight.unwrap_or(&5);
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
