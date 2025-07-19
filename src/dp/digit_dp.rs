const MOD: usize = 1_000_000_007;

/// 桁DP（Digit Dynamic Programming）を実装する構造体
/// 
/// 桁DPは、数値の各桁を順番に決定していく動的プログラミング手法です。
/// 指定された上限値以下の数の中で、特定の条件を満たす数の個数を効率的に計算できます。
pub struct DigitDP;

impl DigitDP {
    /// 桁DPを用いて条件を満たす数の個数を計算します
    /// 
    /// # 引数
    /// 
    /// * `upper` - 上限値を表す文字列（例: "999"）
    /// * `init` - 初期状態
    /// * `trans` - 状態遷移関数。現在の位置、tight制約、現在の状態、桁の上限を受け取り、
    ///            可能な次の桁と次の状態のペアのベクターを返す
    /// * `accept` - 受理条件を判定する関数。最終状態を受け取り、条件を満たすかどうかを返す
    /// 
    /// # 戻り値
    /// 
    /// 条件を満たす数の個数（MODで割った余り）
    /// 
    /// # 例
    /// 
    /// ```
    /// # use rust_macro::dp::DigitDP;
    /// // 99以下の全ての数をカウント
    /// let count = DigitDP::solve(
    ///     "99",
    ///     (),
    ///     |_i, _tight, _state, lim| (0..=lim).map(|d| (d, ())).collect(),
    ///     |_state| true,
    /// );
    /// assert_eq!(count, 100); // 0から99まで100個
    /// 
    /// // 偶数の桁を含む数をカウント
    /// let even_count = DigitDP::solve(
    ///     "99",
    ///     false,
    ///     |_i, _tight, &has_even, lim| (0..=lim).map(|d| (d, has_even || d % 2 == 0)).collect(),
    ///     |&has_even| has_even,
    /// );
    /// ```
    /// 
    /// # アルゴリズムの詳細
    /// 
    /// - **tight制約**: 現在構築中の数が上限値と等しいかどうかを追跡
    /// - **メモ化**: 同じ状態（位置、tight、ユーザー定義状態）の結果をキャッシュ
    /// - **状態遷移**: ユーザー定義の遷移関数により柔軟な条件設定が可能
    /// 
    /// # 計算量
    /// 
    /// - 時間計算量: O(N × S × 10) ここで、Nは桁数、Sは状態数
    /// - 空間計算量: O(N × S)
    pub fn solve<F, T, A>(upper: &str, init: T, mut trans: F, accept: A) -> usize
    where
        T: Clone + Eq + std::hash::Hash,
        F: FnMut(usize, bool, &T, u32) -> Vec<(u32, T)>,
        A: Fn(&T) -> bool,
    {
        use rustc_hash::FxHasher;
        use std::collections::HashMap;
        use std::hash::BuildHasherDefault;
        type Hasher = BuildHasherDefault<FxHasher>;

        let digits: Vec<u32> = upper.chars().map(|c| c.to_digit(10).unwrap()).collect();
        let n = digits.len();
        // HashMapの型を明示的に指定
        let mut memo: HashMap<(usize, bool, T), usize, Hasher> = HashMap::default();

        fn dfs<F, T, A>(
            i: usize,
            tight: bool,
            state: &T,
            digits: &Vec<u32>,
            n: usize,
            memo: &mut HashMap<(usize, bool, T), usize, Hasher>,
            trans: &mut F,
            accept: &A,
        ) -> usize
        where
            T: Clone + Eq + std::hash::Hash,
            F: FnMut(usize, bool, &T, u32) -> Vec<(u32, T)>,
            A: Fn(&T) -> bool,
        {
            if i == n {
                return if accept(state) { 1 } else { 0 };
            }
            if let Some(&res) = memo.get(&(i, tight, state.clone())) {
                return res;
            }

            let lim = if tight { digits[i] } else { 9 };
            let mut res = 0;
            for (d, next_state) in trans(i, tight, state, lim) {
                let next_tight = tight && d == lim;
                res =
                    (res + dfs(
                        i + 1,
                        next_tight,
                        &next_state,
                        digits,
                        n,
                        memo,
                        trans,
                        accept,
                    )) % MOD;
            }

            memo.insert((i, tight, state.clone()), res);
            res
        }

        dfs(0, true, &init, &digits, n, &mut memo, &mut trans, &accept)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_all_numbers() {
        let result = DigitDP::solve(
            "99",
            (),
            |_i, _tight, _state, lim| (0..=lim).map(|d| (d, ())).collect(),
            |_state| true,
        );
        assert_eq!(result, 100);
    }

    #[test]
    fn test_count_even_numbers() {
        let result = DigitDP::solve(
            "10",
            false,
            |_i, _tight, _state, lim| (0..=lim).map(|d| (d, d % 2 == 0)).collect(),
            |&has_even| has_even,
        );
        assert_eq!(result, 6);
    }

    #[test]
    fn test_single_digit() {
        let result = DigitDP::solve(
            "5",
            (),
            |_i, _tight, _state, lim| (0..=lim).map(|d| (d, ())).collect(),
            |_state| true,
        );
        assert_eq!(result, 6);
    }

    #[test]
    fn test_digit_sum_equals_target() {
        let target_sum = 9;
        let result = DigitDP::solve(
            "99",
            0u32,
            |_i, _tight, &sum, lim| (0..=lim).map(|d| (d, sum + d)).collect(),
            |&sum| sum == target_sum,
        );
        assert_eq!(result, 10);
    }

    #[test]
    fn test_no_consecutive_same_digits() {
        let result = DigitDP::solve(
            "99",
            (true, 0u32),
            |_i, _tight, &(is_first, last_digit), lim| {
                if is_first {
                    (1..=lim).map(|d| (d, (false, d))).collect()
                } else {
                    (0..=lim)
                        .filter(|&d| d != last_digit)
                        .map(|d| (d, (false, d)))
                        .collect()
                }
            },
            |_state| true,
        );
        assert_eq!(result, 81);
    }

    #[test]
    fn test_contains_digit_7() {
        let result = DigitDP::solve(
            "20",
            false,
            |_i, _tight, &has_seven, lim| (0..=lim).map(|d| (d, has_seven || d == 7)).collect(),
            |&has_seven| has_seven,
        );
        assert_eq!(result, 2);
    }

    #[test]
    fn test_ascending_digits() {
        let result = DigitDP::solve(
            "999",
            (true, 0u32),
            |_i, _tight, &(is_first, last_digit), lim| {
                if is_first {
                    (1..=lim).map(|d| (d, (false, d))).collect()
                } else {
                    (last_digit..=lim).map(|d| (d, (false, d))).collect()
                }
            },
            |_state| true,
        );
        assert_eq!(result, 165);
    }

    #[test]
    fn test_palindrome_check() {
        let result = DigitDP::solve(
            "99",
            (Vec::<u32>::new(), true),
            |_i, _tight, (digits, _), lim| {
                (0..=lim)
                    .map(|d| {
                        let mut new_digits = digits.clone();
                        new_digits.push(d);
                        (d, (new_digits, false))
                    })
                    .collect()
            },
            |(digits, _)| {
                if digits.is_empty() {
                    return false;
                }
                let s: String = digits
                    .iter()
                    .map(|&d| char::from_digit(d, 10).unwrap())
                    .collect();
                let rev: String = s.chars().rev().collect();
                s == rev
            },
        );
        assert_eq!(result, 10);
    }

    #[test]
    fn test_zero_case() {
        let result = DigitDP::solve(
            "0",
            (),
            |_i, _tight, _state, lim| (0..=lim).map(|d| (d, ())).collect(),
            |_state| true,
        );
        assert_eq!(result, 1);
    }

    #[test]
    fn test_large_number() {
        let result = DigitDP::solve(
            "1000",
            (),
            |_i, _tight, _state, lim| (0..=lim).map(|d| (d, ())).collect(),
            |_state| true,
        );
        assert_eq!(result, 1001);
    }
}
