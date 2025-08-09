use std::hash::Hash;
use rustc_hash::FxHashMap;

/// トポロジカル順序で計算可能なDPの問題定義を表すトレイト
///
/// グリッドDPのように、計算の依存関係（トポロジカル順序）が静的に決まる
/// 動的計画法の問題を抽象化します。
pub trait TopologicalDPRules {
    /// DPの状態（グラフのノード）を表す型。
    type Node: Clone + Eq + Hash;
    /// DPテーブルに格納される値の型。
    type Value: Clone;

    /// 計算対象となる全ノードをトポロジカル順（依存関係が解決される順）で返します。
    /// 例えば、ゴールから逆算するDPでは、ゴールからスタートへの順序になります。
    fn nodes_in_order(&self) -> Vec<Self::Node>;

    /// あるノードの値を計算するために必要となる、次の（依存先の）ノードのリストを返します。
    fn next_nodes(&self, node: &Self::Node) -> Vec<Self::Node>;

    /// 依存先のノードの値を使って、現在のノードの値を計算します。
    ///
    /// # 引数
    /// * `node` - 現在計算対象のノード
    /// * `next_values` - `next_nodes`で返されたノードにそれぞれ対応するDP値のリスト
    fn calculate_value(&self, node: &Self::Node, next_values: &[Self::Value]) -> Self::Value;

    /// DPテーブルに存在しない（=境界外の）ノードにアクセスした場合のデフォルト値を返します。
    /// 最小値を求める場合は非常に大きな値、最大値を求める場合は非常に小さな値などを返します。
    fn boundary_value(&self) -> Self::Value;
}

/// トポロジカル順DPソルバー
///
/// `TopologicalDPRules`で定義された問題を解きます。
pub struct TopologicalDPSolver;

impl TopologicalDPSolver {
    /// # アルゴリズムの詳細
    ///
    /// - `nodes_in_order`で指定された順序でDPテーブルを埋めていきます。
    /// - 各ノードの値は、`next_nodes`で示される依存先ノードの値を参照し、
    ///   `calculate_value`で計算されます。
    /// - 計算済みの値はハッシュマップにメモ化されます。
    ///
    /// # 計算量
    ///
    /// O(N * (D + C))
    /// - N: ノードの総数
    /// - D: 1ノードあたりの依存先ノード数（`next_nodes`の返り値の長さ）
    /// - C: `calculate_value`の計算量
    pub fn solve<P>(problem: &P) -> FxHashMap<P::Node, P::Value>
    where
        P: TopologicalDPRules,
    {
        let mut dp_table = FxHashMap::default();
        let nodes = problem.nodes_in_order();

        for node in nodes {
            let next_nodes = problem.next_nodes(&node);

            let next_values: Vec<P::Value> = next_nodes
                .iter()
                .map(|next_node| {
                    dp_table
                        .get(next_node)
                        .cloned()
                        .unwrap_or_else(|| problem.boundary_value())
                })
                .collect();

            let new_value = problem.calculate_value(&node, &next_values);
            dp_table.insert(node.clone(), new_value);
        }

        dp_table
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::{max, min};

    // --- Test Case 1: E - Hungry Takahashi ---
    struct HungryTakahashi {
        h: usize,
        w: usize,
        a: Vec<Vec<isize>>,
        p: Vec<isize>,
    }

    impl TopologicalDPRules for HungryTakahashi {
        type Node = (usize, usize); // (row, col)
        type Value = isize; // 必要なコインの最小枚数

        fn nodes_in_order(&self) -> Vec<Self::Node> {
            let mut nodes = Vec::with_capacity(self.h * self.w);
            for i in (0..self.h).rev() {
                for j in (0..self.w).rev() {
                    nodes.push((i, j));
                }
            }
            nodes
        }

        fn next_nodes(&self, node: &Self::Node) -> Vec<Self::Node> {
            // dp[i][j]の計算には、dp[i+1][j]とdp[i][j+1]が必要
            vec![(node.0 + 1, node.1), (node.0, node.1 + 1)]
        }

        fn calculate_value(&self, node: &Self::Node, next_values: &[Self::Value]) -> Self::Value {
            let (i, j) = *node;
            // 収支 B_ij = A_ij - P_{i+j}
            // (A, Pは0-indexedなので添字を合わせる)
            let b_ij = self.a[i][j] - self.p[i + j];
            
            // 次のマスで要求される金額の最小値
            let min_next_required = min(next_values[0], next_values[1]);
            
            // 遷移式: dp[i][j] = max(0, min(dp[i+1][j], dp[i][j+1]) - B_ij)
            max(0, min_next_required - b_ij)
        }

        fn boundary_value(&self) -> Self::Value {
            // 最小値(min)を求めるので、境界外は非常に大きな値（事実上の無限大）とする
            1_000_000_000_000_000_000
        }
    }

    #[test]
    fn test_hungry_takahashi_example1() {
        let problem = HungryTakahashi {
            h: 2,
            w: 2,
            a: vec![vec![3, 2], vec![4, 1]],
            p: vec![1, 3, 6], // P_1, P_2, P_3 (0-indexed p[0]..p[2])
        };

        let dp_table = TopologicalDPSolver::solve(&problem);
        let result = dp_table.get(&(0, 0)).unwrap();

        // (1,1)からスタートするために最初に必要な金額は2
        assert_eq!(*result, 2);
    }
    
    // --- Test Case 2: Simple Path Counting ---
    struct PathCounter {
        h: usize,
        w: usize,
    }

    impl TopologicalDPRules for PathCounter {
        type Node = (usize, usize);
        type Value = usize; // 経路数

        fn nodes_in_order(&self) -> Vec<Self::Node> {
             let mut nodes = Vec::with_capacity(self.h * self.w);
            for i in (0..self.h).rev() {
                for j in (0..self.w).rev() {
                    nodes.push((i, j));
                }
            }
            nodes
        }

        fn next_nodes(&self, node: &Self::Node) -> Vec<Self::Node> {
            vec![(node.0 + 1, node.1), (node.0, node.1 + 1)]
        }

        fn calculate_value(&self, node: &Self::Node, next_values: &[Self::Value]) -> Self::Value {
            if node.0 == self.h - 1 && node.1 == self.w - 1 {
                // ゴール地点は1通り
                1
            } else {
                // dp[i][j] = dp[i+1][j] + dp[i][j+1]
                next_values[0] + next_values[1]
            }
        }
        
        fn boundary_value(&self) -> Self::Value {
            // 加算なので、境界外は0
            0
        }
    }

    #[test]
    fn test_path_counting() {
        // 2x2グリッドの左上から右下への経路は2通り
        let problem_2x2 = PathCounter { h: 2, w: 2 };
        let dp_2x2 = TopologicalDPSolver::solve(&problem_2x2);
        assert_eq!(*dp_2x2.get(&(0,0)).unwrap(), 2);

        // 3x3グリッドでは6通り
        let problem_3x3 = PathCounter { h: 3, w: 3 };
        let dp_3x3 = TopologicalDPSolver::solve(&problem_3x3);
        assert_eq!(*dp_3x3.get(&(0,0)).unwrap(), 6);
    }
}