//! メモ化付き深さ優先探索（DFS）ライブラリ

/// 探索問題の基本的なルールを定義するトレイト
pub trait Searchable {
    /// グラフや状態空間におけるノード（状態）の型
    type Node: Clone + std::hash::Hash + Eq;
    /// 探索結果として収集される値の型
    type Answer: Clone;

    /// 指定されたノードから遷移可能な次のノードのリストを返します。
    fn successors(&self, node: &Self::Node) -> Vec<Self::Node>;

    /// 指定されたノードがゴール（目的のノード）であるかを判定します。
    fn is_goal(&self, node: &Self::Node) -> bool;

    /// ゴールノードから結果を収集（変換）します。
    fn collect(&self, node: &Self::Node) -> Self::Answer;
}

/// 最適なゴールを探す問題のルールを定義するトレイト
pub trait BestSearchable: Searchable {
    /// 新しい解 `new` が、現在の最適解 `old_best` よりも優れているかを判定します。
    /// 例えば最小値を求めるなら `new < old_best` を返します。
    fn is_better(&self, new: &Self::Answer, old_best: &Self::Answer) -> bool;
}

/// メモ化を利用して深さ優先探索を実行するソルバー
pub struct MemoizedDFS;

impl MemoizedDFS {
    /// 到達可能な全てのゴール、または最初のゴールを探索します。
    ///
    /// # 引数
    /// * `start` - 探索を開始するノード
    /// * `problem` - `Searchable` トレイトを実装した問題定義
    /// * `return_on_first` - `true` の場合、最初のゴールを見つけ次第探索を終了します。
    ///
    /// # 戻り値
    /// 見つかったゴールの値のベクター
    pub fn search<P: Searchable>(
        start: P::Node,
        problem: &P,
        return_on_first: bool,
    ) -> Vec<P::Answer> {
        use rustc_hash::FxHasher;
        use std::collections::HashSet;
        use std::hash::BuildHasherDefault;
        type Hasher = BuildHasherDefault<FxHasher>;

        let mut visited = HashSet::with_hasher(Hasher::default());
        let mut result = vec![];

        fn dfs<P: Searchable>(
            current: P::Node,
            visited: &mut HashSet<P::Node, Hasher>,
            result: &mut Vec<P::Answer>,
            problem: &P,
            return_on_first: bool,
        ) -> bool {
            if !visited.insert(current.clone()) {
                return false;
            }

            if problem.is_goal(&current) {
                result.push(problem.collect(&current));
                if return_on_first {
                    return true;
                }
            }

            for next in problem.successors(&current) {
                if dfs(next, visited, result, problem, return_on_first) {
                    return true;
                }
            }

            false
        }

        dfs(start, &mut visited, &mut result, problem, return_on_first);
        result
    }

    /// 最適なゴールを一つ探索します。
    ///
    /// # 引数
    /// * `start` - 探索を開始するノード
    /// * `problem` - `BestSearchable` トレイトを実装した問題定義
    ///
    /// # 戻り値
    /// 見つかった最も良いゴールの値。ゴールが見つからなければ `None`。
    pub fn search_with_best<P: BestSearchable>(start: P::Node, problem: &P) -> Option<P::Answer> {
        use rustc_hash::FxHasher;
        use std::collections::HashSet;
        use std::hash::BuildHasherDefault;
        type Hasher = BuildHasherDefault<FxHasher>;

        let mut visited = HashSet::with_hasher(Hasher::default());
        let mut best: Option<P::Answer> = None;

        fn dfs<P: BestSearchable>(
            current: P::Node,
            visited: &mut HashSet<P::Node, Hasher>,
            best: &mut Option<P::Answer>,
            problem: &P,
        ) {
            if !visited.insert(current.clone()) {
                return;
            }

            if problem.is_goal(&current) {
                let val = problem.collect(&current);
                if best.as_ref().map_or(true, |b| problem.is_better(&val, b)) {
                    *best = Some(val);
                }
            }

            for next in problem.successors(&current) {
                dfs(next, visited, best, problem);
            }
        }

        dfs(start, &mut visited, &mut best, problem);
        best
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- テスト用の問題定義 ---
    struct SimpleGraph;
    impl Searchable for SimpleGraph {
        type Node = i32;
        type Answer = i32;
        fn successors(&self, &node: &Self::Node) -> Vec<Self::Node> {
            if node < 3 {
                vec![node + 1]
            } else {
                vec![]
            }
        }
        fn is_goal(&self, &node: &Self::Node) -> bool {
            node == 3
        }
        fn collect(&self, &node: &Self::Node) -> Self::Answer {
            node
        }
    }

    struct MultiGoalGraph;
    impl Searchable for MultiGoalGraph {
        type Node = i32;
        type Answer = i32;
        fn successors(&self, &node: &Self::Node) -> Vec<Self::Node> {
            if node < 5 {
                vec![node + 1, node + 2]
            } else {
                vec![]
            }
        }
        fn is_goal(&self, &node: &Self::Node) -> bool {
            node >= 3
        }
        fn collect(&self, &node: &Self::Node) -> Self::Answer {
            node
        }
    }

    struct FindMinGoal;
    impl Searchable for FindMinGoal {
        type Node = i32;
        type Answer = i32;
        fn successors(&self, &node: &Self::Node) -> Vec<Self::Node> {
            if node < 5 {
                vec![node + 1, node + 2]
            } else {
                vec![]
            }
        }
        fn is_goal(&self, &node: &Self::Node) -> bool {
            node >= 3
        }
        fn collect(&self, &node: &Self::Node) -> Self::Answer {
            node
        }
    }
    impl BestSearchable for FindMinGoal {
        fn is_better(&self, new: &Self::Answer, old_best: &Self::Answer) -> bool {
            new < old_best // 最小値を求める
        }
    }

    struct FindMaxGoal;
    impl Searchable for FindMaxGoal {
        type Node = i32;
        type Answer = i32;
        fn successors(&self, &node: &Self::Node) -> Vec<Self::Node> {
            if node < 5 {
                vec![node + 1, node + 2]
            } else {
                vec![]
            }
        }
        fn is_goal(&self, &node: &Self::Node) -> bool {
            node >= 3
        }
        fn collect(&self, &node: &Self::Node) -> Self::Answer {
            node
        }
    }
    impl BestSearchable for FindMaxGoal {
        fn is_better(&self, new: &Self::Answer, old_best: &Self::Answer) -> bool {
            new > old_best // 最大値を求める
        }
    }

    struct CyclicGraph;
    impl Searchable for CyclicGraph {
        type Node = i32;
        type Answer = i32;
        fn successors(&self, &node: &Self::Node) -> Vec<Self::Node> {
            match node {
                0 => vec![1],
                1 => vec![2],
                2 => vec![0],
                _ => vec![],
            }
        }
        fn is_goal(&self, &node: &Self::Node) -> bool {
            node == 1
        }
        fn collect(&self, &node: &Self::Node) -> Self::Answer {
            node
        }
    }

    // --- テスト実行 ---
    #[test]
    fn test_search_simple_graph() {
        let result = MemoizedDFS::search(0, &SimpleGraph, false);
        assert_eq!(result, vec![3]);
    }

    #[test]
    fn test_search_multiple_goals() {
        let mut result = MemoizedDFS::search(0, &MultiGoalGraph, false);
        result.sort();
        assert_eq!(result, vec![3, 4, 5, 6]);
    }

    #[test]
    fn test_search_return_on_first() {
        let result = MemoizedDFS::search(0, &MultiGoalGraph, true);
        assert_eq!(result.len(), 1);
        assert!(result[0] >= 3);
    }

    #[test]
    fn test_search_cycle_detection() {
        let result = MemoizedDFS::search(0, &CyclicGraph, false);
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn test_search_with_best_find_minimum() {
        let result = MemoizedDFS::search_with_best(0, &FindMinGoal);
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_search_with_best_find_maximum() {
        let result = MemoizedDFS::search_with_best(0, &FindMaxGoal);
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_complex_graph_structure() {
        #[derive(Clone, Hash, Eq, PartialEq, Debug)]
        struct Node {
            id: i32,
            value: i32,
        }

        struct ComplexProblem;
        impl Searchable for ComplexProblem {
            type Node = Node;
            type Answer = i32;
            fn successors(&self, node: &Self::Node) -> Vec<Self::Node> {
                match node.id {
                    0 => vec![Node { id: 1, value: 20 }, Node { id: 2, value: 15 }],
                    1 => vec![Node { id: 3, value: 30 }],
                    2 => vec![Node { id: 3, value: 25 }],
                    _ => vec![],
                }
            }
            fn is_goal(&self, node: &Self::Node) -> bool {
                node.id == 3
            }
            fn collect(&self, node: &Self::Node) -> Self::Answer {
                node.value
            }
        }
        impl BestSearchable for ComplexProblem {
            fn is_better(&self, new: &Self::Answer, old_best: &Self::Answer) -> bool {
                new > old_best
            }
        }

        let start = Node { id: 0, value: 10 };
        let result = MemoizedDFS::search_with_best(start, &ComplexProblem);
        assert_eq!(result, Some(30));
    }
}
