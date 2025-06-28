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

    fn dfs<V, F1, F2, F3>(
        &self,
        start: I,
        goal_check: F3,
        merge: F1,
        add_node: F2,
    ) -> Option<V>
    where
        V: Copy,
        F1: Fn(Option<V>, Option<V>) -> Option<V>,
        F2: Fn(Option<V>, &Edge<I, W>) -> Option<V>,
        F3: Fn(&I) -> bool,
        I: Clone + Eq + Hash,
        W: Copy,
    {
        let mut visited = HashSet::new();
        let mut res: Option<V> = None;

        fn dfs_inner<W, I, V, F1, F2, F3>(
            graph: &Graph<W, I>,
            current: I,
            goal_check: &F3,
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
            F3: Fn(&I) -> bool,
            I: Clone + Eq + Hash,
            W: Copy,
        {
            if goal_check(&current) {
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
                        goal_check,
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
            &goal_check,
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
        let ans = graph.dfs(1, |x| *x == 3, merge, add_node);
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

        assert_eq!(graph.dfs(1, |x| *x == 4, merge, add_node), Some(true));
        assert_eq!(graph.dfs(1, |x| *x == 5, merge, add_node), Some(true));
        assert_eq!(graph.dfs(2, |x| *x == 4, merge, add_node), Some(true));
        assert_eq!(graph.dfs(2, |x| *x == 1, merge, add_node), None);
        assert_eq!(graph.dfs(4, |x| *x == 1, merge, add_node), None);
    }

    #[test]
    fn test_disconnected_nodes() {
        let mut graph = Graph::<(), usize>::new(6);
        // 接続された部分: 1-2-3
        graph.add_edge(1, 2, None);
        graph.add_edge(2, 3, None);
        // 接続された部分: 4-5
        graph.add_edge(4, 5, None);
        // 6は完全に孤立
        
        let merge: fn(Option<bool>, Option<bool>) -> Option<bool> = |a, b| match (a, b) {
            (Some(true), _) | (_, Some(true)) => Some(true),
            _ => None,
        };
        let add_node: fn(Option<bool>, &Edge<usize, ()>) -> Option<bool> = |_, _| Some(true);
        
        // 1から3への到達可能性
        assert_eq!(graph.dfs(1, |x| *x == 3, merge, add_node), Some(true));
        
        // 1から4への到達不可能性（異なる連結成分）
        assert_eq!(graph.dfs(1, |x| *x == 4, merge, add_node), None);
        
        // 1から6への到達不可能性（6は孤立）
        assert_eq!(graph.dfs(1, |x| *x == 6, merge, add_node), None);
        
        // 4から5への到達可能性
        assert_eq!(graph.dfs(4, |x| *x == 5, merge, add_node), Some(true));
        
        // 4から1への到達不可能性（異なる連結成分）
        assert_eq!(graph.dfs(4, |x| *x == 1, merge, add_node), None);
        
        // 6からどこへも到達不可能（孤立ノード）
        assert_eq!(graph.dfs(6, |x| *x == 1, merge, add_node), None);
        assert_eq!(graph.dfs(6, |x| *x == 4, merge, add_node), None);
    }

    #[test]
    fn test_sum_all_connected_edges() {
        let mut graph = Graph::<usize, usize>::new(5);
        graph.add_edge(1, 2, Some(10));
        graph.add_edge(2, 3, Some(20));
        graph.add_edge(1, 3, Some(15));
        graph.add_edge(2, 4, Some(25));
        // 5は孤立

        let merge: fn(Option<usize>, Option<usize>) -> Option<usize> = |a, b| {
            match (a, b) {
                (Some(x), Some(y)) => Some(x + y),
                (Some(x), None) => Some(x),
                (None, Some(y)) => Some(y),
                (None, None) => None,
            }
        };

        let add_node: fn(Option<usize>, &Edge<usize, usize>) -> Option<usize> = |acc, edge| {
            match (acc, edge.weight) {
                (Some(sum), Some(weight)) => Some(sum + weight),
                (None, Some(weight)) => Some(weight),
                _ => acc,
            }
        };

        // 現在の実装では |_| false は結果がNoneになる
        let result = graph.dfs(1, |_| false, merge, add_node);
        assert_eq!(result, None);

        // 特定のゴールへの経路の重みを計算
        let result_to_4 = graph.dfs(1, |x| *x == 4, merge, add_node);
        assert_eq!(result_to_4, Some(35)); // 1->2->4: 10 + 25 = 35

        // 5からは辞がないのでNone
        let result_isolated = graph.dfs(5, |_| false, merge, add_node);
        assert_eq!(result_isolated, None);
    }

    #[test]
    fn test_node_and_edge_weights() {
        let mut graph = Graph::<usize, usize>::new(4);
        // 辺の重み
        graph.add_edge(1, 2, Some(10));
        graph.add_edge(2, 3, Some(20));
        graph.add_edge(1, 3, Some(15));
        // 頂点の重み
        graph.add_weight_to_node(1, 5);
        graph.add_weight_to_node(2, 8);
        graph.add_weight_to_node(3, 12);

        let merge: fn(Option<usize>, Option<usize>) -> Option<usize> = |a, b| {
            match (a, b) {
                (Some(x), Some(y)) => Some(x + y),
                (Some(x), None) => Some(x),
                (None, Some(y)) => Some(y),
                (None, None) => None,
            }
        };

        // グラフへの参照をクロージャで使用するため、別の関数として定義
        let get_node_weight = |node_id: &usize| -> usize {
            graph.nodes.get(node_id)
                .and_then(|node| node.weight)
                .unwrap_or(0)
        };

        let add_node_with_vertex = |acc: Option<usize>, edge: &Edge<usize, usize>| -> Option<usize> {
            // 辺の重みと到達ノードの重みを両方加算
            let edge_weight = edge.weight.unwrap_or(0);
            let node_weight = get_node_weight(&edge.to);
            match acc {
                Some(sum) => Some(sum + edge_weight + node_weight),
                None => Some(edge_weight + node_weight),
            }
        };

        // 辺の重みのみを考慮
        let add_edge_only: fn(Option<usize>, &Edge<usize, usize>) -> Option<usize> = |acc, edge| {
            match (acc, edge.weight) {
                (Some(sum), Some(weight)) => Some(sum + weight),
                (None, Some(weight)) => Some(weight),
                _ => acc,
            }
        };

        // 辺の重みのみ: 1->3への全経路の合計
        // 経路1: 1->3 (15), 経路2: 1->2->3 (10+20=30) → 合計45
        let result_edge_only = graph.dfs(1, |x| *x == 3, merge, add_edge_only);
        assert_eq!(result_edge_only, Some(45));

        // 辺と頂点の重み両方を考慮した場合
        // 経路1: 1->3 (辺15 + ノード3の12 = 27)
        // 経路2: 1->2->3 (辺10 + ノード2の8 + 辺20 + ノード3の12 = 50)
        // 合計: 27 + 50 = 77
        let result_with_nodes = graph.dfs(1, |x| *x == 3, merge, add_node_with_vertex);
        assert_eq!(result_with_nodes, Some(77));
    }
}
