use std::collections::{HashMap, HashSet};
use std::hash::Hash;
// use std::cmp::{ min, max };
use std::cmp::{max, min};
use std::fmt::Debug;

#[derive(Debug, Clone)]
struct Edge<I, EW> {
    to: I,
    weight: Option<EW>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Node<I, NW> {
    id: I,
    weight: Option<NW>,
}
#[derive(Debug, Clone)]
struct Graph<I: std::fmt::Debug, EW: std::fmt::Debug, NW: std::fmt::Debug> {
    n: usize,
    nodes: HashMap<I, Node<I, NW>>,
    adj: HashMap<I, HashMap<I, Edge<I, EW>>>,
}

impl<I: Clone + Eq + Hash + std::fmt::Debug, EW: std::fmt::Debug, NW: std::fmt::Debug>
    Graph<I, EW, NW>
{
    fn new(n: usize) -> Self {
        Graph {
            n,
            nodes: HashMap::new(),
            adj: HashMap::new(),
        }
    }

    fn add_edge(&mut self, from: I, to: I, weight: Option<EW>) {
        self.adj
            .entry(from.clone())
            .or_default()
            .insert(to.clone(), Edge { to, weight });
    }

    fn add_weight_to_node(&mut self, id: I, weight: NW) {
        self.nodes.entry(id.clone()).or_insert(Node {
            id,
            weight: Some(weight),
        });
    }

    fn dfs<V, F1, F2>(&self, start: I, merge: F1, add_node: F2) -> V
    where
        V: Copy + Default + std::fmt::Debug,
        F1: Fn(V, V) -> V,
        F2: Fn(V, &Node<I, NW>, &Edge<I, EW>) -> V,
        I: Clone + Eq + Hash,
        EW: Copy,
        NW: Copy,
    {
        let mut visited = HashSet::new();
        let mut res = V::default();

        fn dfs_inner<I, EW, NW, V, F1, F2>(
            graph: &Graph<I, EW, NW>,
            current: I,
            visited: &mut HashSet<I>,
            res: &V,
            merge: &F1,
            add_node: &F2,
        ) -> V
        where
            V: Copy + Default + std::fmt::Debug,
            F1: Fn(V, V) -> V,
            F2: Fn(V, &Node<I, NW>, &Edge<I, EW>) -> V,
            I: Clone + Eq + Hash + std::fmt::Debug,
            EW: Copy + std::fmt::Debug,
            NW: Copy + std::fmt::Debug,
        {
            visited.insert(current.clone());
            let mut new_res = res.clone();

            // nodeを明示的に持っていないときのtmp用のfallback
            let fallback_node = Node {
                id: current.clone(),
                weight: None,
            };
            let node = graph.nodes.get(&current).unwrap_or(&fallback_node);
            if let Some(neighbors) = graph.adj.get(&current) {
                // println!("Visiting node: {:?}", node);
                // println!("Neighbors: {:?}", neighbors);
                for (next, edge) in neighbors {
                    if visited.contains(next) {
                        continue;
                    }
                    let sub_result = dfs_inner(graph, next.clone(), visited, res, merge, add_node);
                    // println!("Edge from {:?} to {:?} with weight: {:?}", node, next, edge.weight);

                    // println!("Sub-result: {:?}", sub_result);
                    new_res = merge(new_res, add_node(sub_result, node, edge));
                    // println!("New result after merge: {:?}", new_res);
                }
            }
            new_res
        }
        // let mut result = V::default();
        let new_res = dfs_inner(self, start, &mut visited, &res, &merge, &add_node);

        new_res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_path() {
        let mut graph = Graph::<usize, usize, usize>::new(4);
        graph.add_edge(1, 2, Some(5));
        graph.add_edge(2, 1, Some(5));
        graph.add_edge(2, 3, Some(10));
        graph.add_edge(3, 2, Some(10));
        graph.add_edge(1, 4, Some(16));
        graph.add_edge(4, 1, Some(31));

        let merge = |a, b| a + b;
        let add_node =
            |a: usize, b: &Node<usize, usize>, edge: &Edge<usize, usize>| a + edge.weight.unwrap();
        let ans = graph.dfs(1, merge, add_node);
        assert_eq!(ans, 31); // 1 -> 2 -> 3 の経路で最小の重みは 5
    }

    #[test]
    fn test_simple_reachability() {
        let mut graph = Graph::<usize, usize, usize>::new(5);
        graph.add_edge(1, 2, Some(5));
        graph.add_edge(2, 1, Some(5));
        graph.add_edge(2, 3, Some(10));
        graph.add_edge(3, 2, Some(10));
        graph.add_edge(1, 4, Some(16));
        graph.add_edge(4, 1, Some(31));
        // let merge: fn(Option<bool>, Option<bool>) -> Option<bool> = |a, b| {
        //     match (a, b) {
        //         (Some(true), _) | (_, Some(true)) => Some(true),
        //         _ => None,
        //     }
        // };
        // let add_node: fn(Option<bool>, &Edge<usize, usize>) -> Option<bool> = |_, _| Some(true);
        let merge = |a, b| (a || b);

        // 1から2への到達可能性
        let goal = 2;
        let add_node =
            |res, _node: &Node<usize, usize>, edge: &Edge<usize, usize>| res || edge.to == goal;
        assert_eq!(graph.dfs(1, merge, add_node), true);

        let add_node =
            |res, _node: &Node<usize, usize>, edge: &Edge<usize, usize>| res || edge.to == 5;
        // 1から5への到達可能性
        assert_eq!(graph.dfs(1, merge, add_node), false);
    }

    // 接続している頂点の中で重みの最小値と最大値のタプル
    #[test]
    fn test_min_max_weights() {
        let mut graph = Graph::<usize, usize, usize>::new(4);
        graph.add_edge(1, 2, Some(5));
        graph.add_edge(2, 3, Some(10));
        graph.add_edge(1, 3, Some(15));
        graph.add_edge(2, 4, Some(20));

        type V = Option<(usize, usize)>;

        let merge = |a: V, b: V| match (a, b) {
            (Some((amin, amax)), Some((bmin, bmax))) => Some((min(amin, bmin), max(amax, bmax))),
            (Some(pair), None) | (None, Some(pair)) => Some(pair),
            _ => None,
        };
        let add_node = |res, _node: &Node<usize, usize>, edge: &Edge<usize, usize>| {
            let weight = edge.weight.unwrap_or(0);
            match res {
                Some((min_weight, max_weight)) => {
                    Some((min(min_weight, weight), max(max_weight, weight)))
                }
                None => Some((weight, weight)),
            }
        };
        let result = graph.dfs(1, merge, add_node);
        let (min_weight, max_weight) = result.unwrap();
        assert_eq!(min_weight, 5);
        assert_eq!(max_weight, 20);
    }
}
