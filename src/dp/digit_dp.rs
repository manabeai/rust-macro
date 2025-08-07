const MOD: usize = 1_000_000_007;

/// 桁DPの問題定義を表すトレイト
///
/// このトレイトを実装することで、特定の条件を満たす数の個数を数える問題を定義できます。
pub trait DigitDPRules {
    /// 桁DPの状態を表す型。ハッシュ可能で比較可能である必要があります。
    type State: Clone + Eq + std::hash::Hash;

    /// 初期状態を返します。
    fn init(&self) -> Self::State;

    /// 状態遷移関数。
    ///
    /// # 引数
    /// * `i` - 現在の桁位置 (0-indexed)
    /// * `tight` - tight制約が有効かどうか
    /// * `state` - 現在の状態
    /// * `lim` - 現在の桁に入れられる数字の上限 (0-9)
    ///
    /// # 戻り値
    /// (次の桁の数字, 次の状態) のペアのベクター
    fn transition(
        &self,
        i: usize,
        tight: bool,
        state: &Self::State,
        lim: u32,
    ) -> Vec<(u32, Self::State)>;

    /// 最終状態が受理可能かどうかを判定します。
    fn is_accept(&self, state: &Self::State) -> bool;
}

/// 桁DP（Digit Dynamic Programming）を実装する構造体
///
/// 桁DPは、数値の各桁を順番に決定していく動的プログラミング手法です。
/// 指定された上限値以下の数の中で、特定の条件を満たす数の個数を効率的に計算できます。
pub struct DigitDP;

impl DigitDP {
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
    pub fn solve<P: DigitDPRules>(upper: &str, problem: &P) -> usize {
        use rustc_hash::FxHasher;
        use std::collections::HashMap;
        use std::hash::BuildHasherDefault;
        type Hasher = BuildHasherDefault<FxHasher>;

        let digits: Vec<u32> = upper.chars().map(|c| c.to_digit(10).unwrap()).collect();
        let n = digits.len();
        let mut memo: HashMap<(usize, bool, P::State), usize, Hasher> = HashMap::default();

        fn dfs<P: DigitDPRules>(
            i: usize,
            tight: bool,
            state: &P::State,
            digits: &Vec<u32>,
            n: usize,
            memo: &mut HashMap<(usize, bool, P::State), usize, Hasher>,
            problem: &P,
        ) -> usize {
            if i == n {
                return if problem.is_accept(state) { 1 } else { 0 };
            }
            if let Some(&res) = memo.get(&(i, tight, state.clone())) {
                return res;
            }

            let lim = if tight { digits[i] } else { 9 };
            let mut res = 0;
            for (d, next_state) in problem.transition(i, tight, state, lim) {
                let next_tight = tight && d == lim;
                res = (res + dfs(i + 1, next_tight, &next_state, digits, n, memo, problem)) % MOD;
            }

            memo.insert((i, tight, state.clone()), res);
            res
        }

        let init_state = problem.init();
        dfs(0, true, &init_state, &digits, n, &mut memo, problem)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_all_numbers() {
        struct Problem;
        impl DigitDPRules for Problem {
            type State = ();
            fn init(&self) -> Self::State {
                ()
            }
            fn transition(
                &self,
                _i: usize,
                _tight: bool,
                _state: &Self::State,
                lim: u32,
            ) -> Vec<(u32, Self::State)> {
                (0..=lim).map(|d| (d, ())).collect()
            }
            fn is_accept(&self, _state: &Self::State) -> bool {
                true
            }
        }
        assert_eq!(DigitDP::solve("99", &Problem), 100);
    }

    #[test]
    fn test_count_even_numbers() {
        // 0から10までの数の中で、その数自体が偶数であるものの個数を数える
        // （最後の桁が偶数かどうかで判定）
        struct Problem;
        impl DigitDPRules for Problem {
            type State = bool; // is_even
            fn init(&self) -> Self::State {
                false
            }
            fn transition(
                &self,
                _i: usize,
                _tight: bool,
                _state: &Self::State,
                lim: u32,
            ) -> Vec<(u32, Self::State)> {
                (0..=lim).map(|d| (d, d % 2 == 0)).collect()
            }
            fn is_accept(&self, &is_even: &Self::State) -> bool {
                is_even
            }
        }
        assert_eq!(DigitDP::solve("10", &Problem), 6);
    }

    #[test]
    fn test_single_digit() {
        struct Problem;
        impl DigitDPRules for Problem {
            type State = ();
            fn init(&self) -> Self::State {
                ()
            }
            fn transition(
                &self,
                _i: usize,
                _tight: bool,
                _state: &Self::State,
                lim: u32,
            ) -> Vec<(u32, Self::State)> {
                (0..=lim).map(|d| (d, ())).collect()
            }
            fn is_accept(&self, _state: &Self::State) -> bool {
                true
            }
        }
        assert_eq!(DigitDP::solve("5", &Problem), 6);
    }

    #[test]
    fn test_digit_sum_equals_target() {
        struct Problem {
            target_sum: u32,
        }
        impl DigitDPRules for Problem {
            type State = u32; // sum
            fn init(&self) -> Self::State {
                0
            }
            fn transition(
                &self,
                _i: usize,
                _tight: bool,
                &sum: &Self::State,
                lim: u32,
            ) -> Vec<(u32, Self::State)> {
                (0..=lim).map(|d| (d, sum + d)).collect()
            }
            fn is_accept(&self, &sum: &Self::State) -> bool {
                sum == self.target_sum
            }
        }
        assert_eq!(DigitDP::solve("99", &Problem { target_sum: 9 }), 10);
    }

    #[test]
    fn test_no_consecutive_same_digits() {
        // 1から99までで隣り合う桁が同じでない数の個数
        struct Problem;
        impl DigitDPRules for Problem {
            type State = (bool, u32); // (is_first, last_digit)
            fn init(&self) -> Self::State {
                (true, 0)
            }
            fn transition(
                &self,
                _i: usize,
                _tight: bool,
                &(is_first, last_digit): &Self::State,
                lim: u32,
            ) -> Vec<(u32, Self::State)> {
                if is_first {
                    (1..=lim).map(|d| (d, (false, d))).collect()
                } else {
                    (0..=lim)
                        .filter(|&d| d != last_digit)
                        .map(|d| (d, (false, d)))
                        .collect()
                }
            }
            fn is_accept(&self, _state: &Self::State) -> bool {
                true
            }
        }
        assert_eq!(DigitDP::solve("99", &Problem), 81);
    }

    #[test]
    fn test_contains_digit_7() {
        struct Problem;
        impl DigitDPRules for Problem {
            type State = bool; // has_seven
            fn init(&self) -> Self::State {
                false
            }
            fn transition(
                &self,
                _i: usize,
                _tight: bool,
                &has_seven: &Self::State,
                lim: u32,
            ) -> Vec<(u32, Self::State)> {
                (0..=lim).map(|d| (d, has_seven || d == 7)).collect()
            }
            fn is_accept(&self, &has_seven: &Self::State) -> bool {
                has_seven
            }
        }
        assert_eq!(DigitDP::solve("20", &Problem), 2); // 7, 17
    }

    #[test]
    fn test_ascending_digits() {
        struct Problem;
        impl DigitDPRules for Problem {
            type State = (bool, u32); // (is_first, last_digit)
            fn init(&self) -> Self::State {
                (true, 0)
            }
            fn transition(
                &self,
                _i: usize,
                _tight: bool,
                &(is_first, last_digit): &Self::State,
                lim: u32,
            ) -> Vec<(u32, Self::State)> {
                if is_first {
                    (1..=lim).map(|d| (d, (false, d))).collect()
                } else {
                    (last_digit..=lim).map(|d| (d, (false, d))).collect()
                }
            }
            fn is_accept(&self, _state: &Self::State) -> bool {
                true
            }
        }
        assert_eq!(DigitDP::solve("999", &Problem), 165);
    }

    #[test]
    fn test_palindrome_check() {
        struct Problem;
        impl DigitDPRules for Problem {
            type State = Vec<u32>;
            fn init(&self) -> Self::State {
                Vec::new()
            }
            fn transition(
                &self,
                _i: usize,
                _tight: bool,
                digits: &Self::State,
                lim: u32,
            ) -> Vec<(u32, Self::State)> {
                (0..=lim)
                    .map(|d| {
                        let mut new_digits = digits.clone();
                        new_digits.push(d);
                        (d, new_digits)
                    })
                    .collect()
            }
            fn is_accept(&self, digits: &Self::State) -> bool {
                if digits.is_empty() {
                    return false;
                }
                let s: String = digits
                    .iter()
                    .map(|&d| char::from_digit(d, 10).unwrap())
                    .collect();
                let rev: String = s.chars().rev().collect();
                s == rev
            }
        }
        assert_eq!(DigitDP::solve("99", &Problem), 10);
    }

    #[test]
    fn test_zero_case() {
        struct Problem;
        impl DigitDPRules for Problem {
            type State = ();
            fn init(&self) -> Self::State {
                ()
            }
            fn transition(
                &self,
                _i: usize,
                _tight: bool,
                _state: &Self::State,
                lim: u32,
            ) -> Vec<(u32, Self::State)> {
                (0..=lim).map(|d| (d, ())).collect()
            }
            fn is_accept(&self, _state: &Self::State) -> bool {
                true
            }
        }
        assert_eq!(DigitDP::solve("0", &Problem), 1);
    }

    #[test]
    fn test_large_number() {
        struct Problem;
        impl DigitDPRules for Problem {
            type State = ();
            fn init(&self) -> Self::State {
                ()
            }
            fn transition(
                &self,
                _i: usize,
                _tight: bool,
                _state: &Self::State,
                lim: u32,
            ) -> Vec<(u32, Self::State)> {
                (0..=lim).map(|d| (d, ())).collect()
            }
            fn is_accept(&self, _state: &Self::State) -> bool {
                true
            }
        }
        assert_eq!(DigitDP::solve("1000", &Problem), 1001);
    }
}
