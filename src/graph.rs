use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::cmp::min;

#[derive(Debug, Clone)]
struct Edge<I, W> {
    to: I,
    weight: Option<W>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Node<I, W> {
    id: I,
    weight: Option<W>,
}
#[derive(Debug, Clone)]
struct Graph<W, I> {
    n: usize,
    nodes: HashMap<I, Node<I, W>>,
    adj: HashMap<I, HashMap<I, Edge<I, W>>>,
}

impl<W, I: Clone + Eq + Hash> Graph<W, I> {
    fn new(n: usize) -> Self {
        Graph {
            n,
            nodes: HashMap::new(),
            adj: HashMap::new(),
        }
    }

    fn add_edge(&mut self, from: I, to: I, weight: Option<W>) {
        self.adj.entry(from.clone()).or_default().insert(
            to.clone(),
            Edge { to, weight },
        );
    }

    fn add_weight_to_node(&mut self, id: I, weight: W) {
        self.nodes.entry(id.clone()).or_insert(Node {
            id,
            weight: Some(weight),
        });
    }

    fn dfs<V, F1, F2>(
        &self,
        start: I,
        goal: I,
        merge: F1,
        add_node: F2,
    ) -> Option<V>
    where
        V: Copy,
        F1: Fn(Option<V>, Option<V>) -> Option<V>,
        F2: Fn(Option<V>, &Edge<I, W>) -> Option<V>,
        I: Clone + Eq + Hash,
        W: Copy,
    {
        let mut visited = HashSet::new();
        let mut res: Option<V> = None;

        fn dfs_inner<W, I, V, F1, F2>(
            graph: &Graph<W, I>,
            current: I,
            goal: &I,
            visited: &mut HashSet<I>,
            acc: Option<V>,
            res: &mut Option<V>,
            merge: &F1,
            add_node: &F2,
        )
        where
            V: Copy,
            F1: Fn(Option<V>, Option<V>) -> Option<V>,
            F2: Fn(Option<V>, &Edge<I, W>) -> Option<V>,
            I: Clone + Eq + Hash,
            W: Copy,
        {
            if current == *goal {
                if let Some(r) = res {
                    *res = merge(Some(*r), acc);
                } else {
                    *res = acc;
                }
                return;
            }

            visited.insert(current.clone());

            if let Some(neighbors) = graph.adj.get(&current) {
                for (next, edge) in neighbors {
                    if visited.contains(next) {
                        continue;
                    }
                    let new_acc = add_node(acc, edge);
                    dfs_inner(
                        graph,
                        next.clone(),
                        goal,
                        visited,
                        new_acc,
                        res,
                        merge,
                        add_node,
                    );
                }
            }

            visited.remove(&current);
        }

        dfs_inner(
            self,
            start,
            &goal,
            &mut visited,
            None,
            &mut res,
            &merge,
            &add_node,
        );

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_path() {
        let mut graph = Graph::<usize, usize>::new(3);
        graph.add_edge(1, 2, Some(5));
        graph.add_edge(2, 3, Some(2));
        graph.add_edge(1, 3, Some(10));

        let merge: fn(Option<usize>, Option<usize>) -> Option<usize> = |a, b| match (a, b) {
            (Some(x), Some(y)) => Some(min(x, y)),
            (Some(x), None) => Some(x),
            (None, Some(y)) => Some(y),
            (None, None) => None,
        };
        let add_node: fn(Option<usize>, &Edge<usize, usize>) -> Option<usize> = |a, b| {
            match (a, b.weight) {
                (Some(acc), Some(w)) => Some(acc + w),
                (None, Some(w)) => Some(w),
                _ => None,
            }
        };
        let ans = graph.dfs(1, 3, merge, add_node);
        assert_eq!(ans, Some(7));
    }

    #[test]
    fn test_simple_reachability() {
        let mut graph = Graph::<(), usize>::new(5);
        graph.add_edge(1, 2, None);
        graph.add_edge(2, 3, None);
        graph.add_edge(3, 4, None);
        graph.add_edge(1, 5, None);

        let merge: fn(Option<bool>, Option<bool>) -> Option<bool> = |a, b| match (a, b) {
            (Some(true), _) | (_, Some(true)) => Some(true),
            _ => None,
        };
        let add_node: fn(Option<bool>, &Edge<usize, ()>) -> Option<bool> = |_, _| Some(true);

        assert_eq!(graph.dfs(1, 4, merge, add_node), Some(true));
        assert_eq!(graph.dfs(1, 5, merge, add_node), Some(true));
        assert_eq!(graph.dfs(2, 4, merge, add_node), Some(true));
        assert_eq!(graph.dfs(2, 1, merge, add_node), None);
        assert_eq!(graph.dfs(4, 1, merge, add_node), None);
    }

    // #[test]
    // fn test_direct_path_better() {
    //     let mut graph = Graph::<usize, usize>::new(3);
    //     graph.add_edge(1, 2, Some(5));
    //     graph.add_edge(2, 3, Some(10));
    //     graph.add_edge(1, 3, Some(4));

    //     assert_eq!(graph.dfs(1, 3, usize::MAX, |a, b| min(a, b), |a, b| a + b.weight.unwrap()), 4);
    // }

    // #[test]
    // fn test_no_path() {
    //     let mut graph = Graph::<usize, usize>::new(3);
    //     graph.add_edge(1, 2, Some(5));
    //     graph.add_edge(2, 1, Some(5));
    //     // 3には繋がっていない

    //     let abs = graph.dfs(1, 3, usize::MAX,
    //         |a: Option<usize>, b| match (a, b) {
    //             (Some(x), Some(y)) => Some(min(x, y)),
    //             (Some(x), None) => Some(x),
    //             (None, Some(y)) => Some(y),
    //             (None, None) => None,
    //         },
    //         |a: V, b| match (a, b.weight) {
    //             (Some(acc), Some(w)) => Some(acc + w),
    //             (None, Some(w)) => Some(w),
    //             _ => None,
    //         });
    //     assert_eq!(ans, None);
    // }

    // #[test]
    // fn test_cycle_graph() {
    //     let mut graph = Graph::<usize, usize>::new(4);
    //     graph.add_edge(1, 2, Some(1));
    //     graph.add_edge(2, 3, Some(1));
    //     graph.add_edge(3, 4, Some(1));
    //     graph.add_edge(4, 1, Some(1));
    //     graph.add_edge(1, 3, Some(5));

    //     assert_eq!(graph.dfs(1, 3, usize::MAX, |a, b| min(a, b), |a, b| a + b.weight.unwrap()), 2);
    // }
}
