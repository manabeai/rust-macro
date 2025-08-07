use rustc_hash::FxHasher;
use std::collections::HashMap;
use std::hash::{BuildHasherDefault, Hash};

use super::{Graph, Tree};

pub trait TreeDp<I, EW, NW>
where
    I: Clone + Eq + Hash,
{
    /// 各ノードで計算・保持されるDP値の型。
    /// 例えば、部分木のノード数を数えるなら `usize` になります。
    type DpValue: Clone;

    /// ノードが単独で持つDPの初期値を計算します。
    ///
    /// これは通常、葉ノードのDP値や、マージ処理の起点となる値です。
    /// 例えば、「部分木のサイズ」を求める問題なら、各ノードは自身（サイズ1）から始まるので
    /// `1` を返すことになるでしょう。
    ///
    /// # 引数
    /// * `node_id` - 対象ノードの内部ID (`usize`)
    /// * `graph` - グラフ全体への参照。ノードの重み `NW` などを参照できます。
    fn initial_value(&self) -> Self::DpValue;

    /// 親ノードのDP値と、その子ノードのDP値をマージ（統合）します。
    ///
    /// この操作は、親ノードが持つすべての子ノードに対して順番に適用され、
    /// 最終的な親ノードのDP値が決定されます。
    ///
    /// # 引数
    /// * `parent_dp_value` - 親ノードの現在のDP値。
    /// * `child_dp_value` - マージ対象の子ノードのDP値。
    /// * `parent_id` - 親ノードの内部ID。
    /// * `child_id` - 子ノードの内部ID。
    /// * `edge_weight` - 親子間のエッジの重み (`EW`) への参照。
    /// * `graph` - グラフ全体への参照。
    ///
    /// # 戻り値
    /// マージ後の親ノードの新しいDP値。
    fn merge(
        &self,
        parent_dp_value: Self::DpValue,
        child_dp_value: Self::DpValue,
        edge_weight: Option<&EW>,
    ) -> Self::DpValue;
}

/// 木DPソルバーの振る舞いを定義するトレイト
pub trait Solver<I, EW, NW, P>
where
    I: Clone + Eq + Hash,
    P: TreeDp<I, EW, NW>,
{
    /// 木DPを実行し、結果をキーと値のペアを持つHashMapとして返します。
    fn solve(
        &self,
        graph: &Graph<I, EW, NW, Tree>,
        root_key: &I,
        problem: &P,
    ) -> Result<HashMap<I, P::DpValue, BuildHasherDefault<FxHasher>>, String>;
}

/// `TreeDp` トレイトに基づいて木DPを実行するソルバー
pub struct TreeDpSolver;

impl<I, EW, NW, P> Solver<I, EW, NW, P> for TreeDpSolver
where
    I: Clone + Eq + Hash,
    P: TreeDp<I, EW, NW>,
{
    fn solve(
        &self,
        graph: &Graph<I, EW, NW, Tree>,
        root_key: &I,
        problem: &P,
    ) -> Result<HashMap<I, P::DpValue, BuildHasherDefault<FxHasher>>, String> {
        let root_id = match graph.key2id(root_key) {
            Some(id) => id,
            None => return Err("Root key not found in graph.".to_string()),
        };

        let mut dp_table: Vec<Option<P::DpValue>> = vec![None; graph.nodes.len()];
        Self::dfs(root_id, usize::MAX, graph, problem, &mut dp_table);

        // 結果を Vec から HashMap に変換
        let result_map = dp_table
            .into_iter()
            .enumerate()
            .filter_map(|(id, dp_value_opt)| {
                // 計算済みのノード（Noneでない）のみを対象にする
                dp_value_opt.map(|dp_value| {
                    // 内部IDから元のキーを復元
                    let key = graph.reverse_map[id].clone();
                    (key, dp_value)
                })
            })
            .collect::<HashMap<_, _, _>>();

        Ok(result_map)
    }
}

// `impl TreeDpSolver` ブロック内にヘルパー関数を移動
impl TreeDpSolver {
    /// DFSを用いて再帰的にDP値を計算するヘルパー関数
    fn dfs<I, EW, NW, P>(
        u: usize,
        p: usize,
        graph: &Graph<I, EW, NW, Tree>,
        problem: &P,
        dp_table: &mut Vec<Option<P::DpValue>>,
    ) where
        I: Clone + Eq + Hash,
        P: TreeDp<I, EW, NW>,
    {
        let mut u_dp_value = problem.initial_value();
        for (v_id, edge_weight) in &graph.adj[u] {
            let v = *v_id;
            if v == p {
                continue;
            }
            if dp_table[v].is_none() {
                Self::dfs(v, u, graph, problem, dp_table);
            }
            let v_dp_value = dp_table[v].as_ref().unwrap().clone();
            u_dp_value = problem.merge(u_dp_value, v_dp_value, edge_weight.as_ref());
        }
        dp_table[u] = Some(u_dp_value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // (SubtreeSizeProblem の実装は変更なし)
    struct SubtreeSizeProblem;
    impl<I, EW, NW> TreeDp<I, EW, NW> for SubtreeSizeProblem
    where
        I: Clone + Eq + Hash,
    {
        type DpValue = usize;

        fn initial_value(&self) -> Self::DpValue {
            1
        }

        fn merge(
            &self,
            parent_dp_value: Self::DpValue,
            child_dp_value: Self::DpValue,
            _edge_weight: Option<&EW>,
        ) -> Self::DpValue {
            parent_dp_value + child_dp_value
        }
    }

    #[test]
    fn test_subtree_size() {
        // 1. グラフのセットアップ (変更なし)
        let mut graph = Graph::<&str, (), (), Tree>::new();
        graph.add_edge("0", "1", None);
        graph.add_edge("0", "2", None);
        graph.add_edge("2", "3", None);
        graph.add_edge("2", "4", None);
        graph.add_edge("1", "0", None);
        graph.add_edge("2", "0", None);
        graph.add_edge("3", "2", None);
        graph.add_edge("4", "2", None);

        // 2. ソルバーと問題定義のインスタンスを作成
        let solver = TreeDpSolver;
        let problem = SubtreeSizeProblem;

        // 3. トレイト経由でソルバーを実行
        let dp_values = solver.solve(&graph, &"0", &problem).unwrap();

        // 4. HashMapとして得られた結果を検証
        assert_eq!(dp_values.len(), 5); // 全ノードの結果が含まれているか
        assert_eq!(dp_values[&"0"], 5); // 根"0"の結果
        assert_eq!(dp_values[&"1"], 1); // 葉"1"の結果
        assert_eq!(dp_values[&"2"], 3); // 中間ノード"2"の結果
        assert_eq!(dp_values[&"3"], 1); // 葉"3"の結果
        assert_eq!(dp_values[&"4"], 1); // 葉"4"の結果
    }
}
