//! A generic all-direction tree DP (re-rooting DP) library.
//!
//! This implementation allows computing DP values for every node in a tree
//! using a user supplied merge function and a function that processes a node
//! given the aggregated results of its children.

#[derive(Clone)]
pub struct AllDirectionTreeDP<T, FMerge, FAdd>
where
    T: Clone,
    FMerge: Fn(T, T) -> T + Copy,
    FAdd: Fn(T) -> T + Copy,
{
    n: usize,
    graph: Vec<Vec<usize>>,
    identity: T,
    merge: FMerge,
    add_root: FAdd,
}

impl<T, FMerge, FAdd> AllDirectionTreeDP<T, FMerge, FAdd>
where
    T: Clone,
    FMerge: Fn(T, T) -> T + Copy,
    FAdd: Fn(T) -> T + Copy,
{
    /// Creates a new instance from the number of nodes and edges.
    pub fn new(
        n: usize,
        edges: &[(usize, usize)],
        identity: T,
        merge: FMerge,
        add_root: FAdd,
    ) -> Self {
        let mut graph = vec![Vec::new(); n];
        for &(u, v) in edges {
            graph[u].push(v);
            graph[v].push(u);
        }
        Self {
            n,
            graph,
            identity,
            merge,
            add_root,
        }
    }

    /// Computes the DP result for each node.
    pub fn solve(&self) -> Vec<T> {
        let mut down = vec![self.identity.clone(); self.n];
        self.dfs1(0, usize::MAX, &mut down);
        let mut ans = vec![self.identity.clone(); self.n];
        self.dfs2(0, usize::MAX, self.identity.clone(), &down, &mut ans);
        ans
    }

    fn dfs1(&self, v: usize, p: usize, down: &mut Vec<T>) -> T {
        let mut acc = self.identity.clone();
        for &to in &self.graph[v] {
            if to == p {
                continue;
            }
            let child = self.dfs1(to, v, down);
            acc = (self.merge)(acc, child);
        }
        let res = (self.add_root)(acc.clone());
        down[v] = res.clone();
        res
    }

    fn dfs2(&self, v: usize, p: usize, from_parent: T, down: &Vec<T>, ans: &mut Vec<T>) {
        let deg = self.graph[v].len();
        let mut prefix = vec![self.identity.clone(); deg + 1];
        let mut suffix = vec![self.identity.clone(); deg + 1];

        for i in 0..deg {
            let to = self.graph[v][i];
            let val = if to == p {
                from_parent.clone()
            } else {
                down[to].clone()
            };
            prefix[i + 1] = (self.merge)(prefix[i].clone(), val.clone());
        }
        for i in (0..deg).rev() {
            let to = self.graph[v][i];
            let val = if to == p {
                from_parent.clone()
            } else {
                down[to].clone()
            };
            suffix[i] = (self.merge)(val.clone(), suffix[i + 1].clone());
        }

        ans[v] = (self.add_root)(prefix[deg].clone());

        for i in 0..deg {
            let to = self.graph[v][i];
            if to == p {
                continue;
            }
            let without = (self.merge)(prefix[i].clone(), suffix[i + 1].clone());
            let next_from_parent = (self.add_root)(without);
            self.dfs2(to, v, next_from_parent, down, ans);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Example DP: compute subtree sizes for all possible roots.
    #[test]
    fn test_subtree_size() {
        let edges = vec![(0, 1), (0, 2), (1, 3), (1, 4)];
        let reroot = AllDirectionTreeDP::new(5, &edges, 0usize, |a, b| a + b, |x| x + 1);
        let result = reroot.solve();
        assert_eq!(result, vec![5, 5, 5, 5, 5]);
    }
}
