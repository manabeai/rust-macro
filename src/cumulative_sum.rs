#![allow(
    non_snake_case,
    unused_variables,
    unused_assignments,
    unused_mut,
    unused_imports,
    unused_macros,
    dead_code
)]

use std::ops::{Add, Sub};
use std::{cmp::max, cmp::min, collections::HashMap, collections::HashSet, io::*};

/// 1次元累積和ライブラリ
///
/// 配列の範囲クエリを高速に処理するデータ構造です。
///
/// # 計算量
/// - 構築: O(n)
/// - 範囲クエリ: O(1)
///
/// # 使用例
/// ```
/// # use rust_macro::CumulativeSum;
/// let arr = vec![1, 2, 3, 4, 5];
/// let cum_sum = CumulativeSum::new(&arr);
/// assert_eq!(cum_sum.sum(1, 3), 5); // [1, 3)の和
/// ```
#[derive(Debug, Clone)]
pub struct CumulativeSum<T>
where
    T: Add<Output = T> + Sub<Output = T> + Copy + Default,
{
    data: Vec<T>,
}

impl<T> CumulativeSum<T>
where
    T: Add<Output = T> + Sub<Output = T> + Copy + Default,
{
    /// 配列から累積和を構築
    ///
    /// # 引数
    /// * `arr` - 元の配列
    ///
    /// # 戻り値
    /// 新しいCumulativeSumインスタンス
    pub fn new(arr: &[T]) -> Self {
        let mut data = Vec::with_capacity(arr.len() + 1);
        data.push(T::default());

        for &val in arr {
            let last = *data.last().unwrap();
            data.push(last + val);
        }

        Self { data }
    }

    /// 範囲[l, r)の和を計算
    ///
    /// # 引数
    /// * `l` - 開始位置（含む）
    /// * `r` - 終了位置（含まない）
    ///
    /// # 戻り値
    /// 範囲の和
    pub fn sum(&self, l: usize, r: usize) -> T {
        assert!(l <= r && r < self.data.len());
        self.data[r] - self.data[l]
    }
}

/// 2次元累積和ライブラリ
///
/// 2次元配列の範囲クエリを高速に処理するデータ構造です。
///
/// # 計算量
/// - 構築: O(h×w)
/// - 範囲クエリ: O(1)
///
/// # 使用例
/// ```
/// # use rust_macro::CumulativeSum2D;
/// let arr = vec![vec![1, 2, 3], vec![4, 5, 6]];
/// let cum_sum = CumulativeSum2D::new(&arr);
/// assert_eq!(cum_sum.sum(0, 0, 2, 2), 12); // 左上2x2の和
/// assert_eq!(cum_sum.sum(0, 0, 2, 3), 21); // 全体の和
/// ```
#[derive(Debug, Clone)]
pub struct CumulativeSum2D<T>
where
    T: Add<Output = T> + Sub<Output = T> + Copy + Default,
{
    data: Vec<Vec<T>>,
    h: usize,
    w: usize,
}

impl<T> CumulativeSum2D<T>
where
    T: Add<Output = T> + Sub<Output = T> + Copy + Default,
{
    /// 2次元配列から累積和を構築
    ///
    /// # 引数
    /// * `arr` - 元の2次元配列
    ///
    /// # 戻り値
    /// 新しいCumulativeSum2Dインスタンス
    pub fn new(arr: &[Vec<T>]) -> Self {
        let h = arr.len();
        let w = if h > 0 { arr[0].len() } else { 0 };

        let mut data = vec![vec![T::default(); w + 1]; h + 1];

        for i in 0..h {
            for j in 0..w {
                data[i + 1][j + 1] = data[i][j + 1] + data[i + 1][j] - data[i][j] + arr[i][j];
            }
        }

        Self { data, h, w }
    }

    /// 範囲(x1, y1)から(x2, y2)の和を計算（x2, y2は含まない）
    ///
    /// # 引数
    /// * `x1` - 開始行位置（含む）
    /// * `y1` - 開始列位置（含む）
    /// * `x2` - 終了行位置（含まない）
    /// * `y2` - 終了列位置（含まない）
    ///
    /// # 戻り値
    /// 範囲の和
    pub fn sum(&self, x1: usize, y1: usize, x2: usize, y2: usize) -> T {
        assert!(x1 <= x2 && y1 <= y2 && x2 <= self.h && y2 <= self.w);
        self.data[x2][y2] - self.data[x1][y2] - self.data[x2][y1] + self.data[x1][y1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cumulative_sum() {
        let arr = vec![1, 2, 3, 4, 5];
        let cum_sum = CumulativeSum::new(&arr);

        assert_eq!(cum_sum.sum(0, 0), 0);
        assert_eq!(cum_sum.sum(0, 1), 1);
        assert_eq!(cum_sum.sum(0, 2), 3);
        assert_eq!(cum_sum.sum(0, 3), 6);
        assert_eq!(cum_sum.sum(0, 4), 10);
        assert_eq!(cum_sum.sum(1, 3), 5);
        assert_eq!(cum_sum.sum(1, 4), 9);
    }

    #[test]
    fn test_cumulative_sum_2d() {
        let arr = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let cum_sum = CumulativeSum2D::new(&arr);

        assert_eq!(cum_sum.sum(0, 0, 1, 1), 1);
        assert_eq!(cum_sum.sum(0, 0, 1, 2), 3);
        assert_eq!(cum_sum.sum(0, 0, 2, 2), 12);
        assert_eq!(cum_sum.sum(0, 0, 2, 3), 21);
        assert_eq!(cum_sum.sum(1, 1, 2, 3), 11);
    }
}
