/// 1次元imos法のライブラリ
///
/// imos法は区間加算を効率的に処理するデータ構造です。
/// 複数の区間に値を加算した後、一度の累積和計算で全体の結果を得ることができます。
///
/// # 計算量
/// - 区間加算: O(1)
/// - 累積和計算: O(n)
///
/// # 使用例
/// ```
/// # use rust_macro::Imos1D;
/// let mut imos = Imos1D::new(5);
/// imos.add(1, 4, 2);  // [1, 4)に2を加算
/// imos.add(2, 5, 3);  // [2, 5)に3を加算
/// let result = imos.build();
/// assert_eq!(result, vec![0, 2, 5, 5, 3]);
/// ```
pub struct Imos1D {
    data: Vec<i64>,
}

impl Imos1D {
    /// 長さnのimos配列を作成
    ///
    /// # 引数
    /// * `n` - 配列の長さ
    ///
    /// # 戻り値
    /// 新しいImos1Dインスタンス
    pub fn new(n: usize) -> Self {
        Imos1D {
            data: vec![0; n + 1],
        }
    }

    /// 区間[l, r)にxを加算
    ///
    /// # 引数
    /// * `l` - 区間の開始位置（含む）
    /// * `r` - 区間の終了位置（含まない）
    /// * `x` - 加算する値
    ///
    /// # 注意
    /// lとrが配列の範囲外の場合は何もしません
    pub fn add(&mut self, l: usize, r: usize, x: i64) {
        if l < self.data.len() {
            self.data[l] += x;
        }
        if r < self.data.len() {
            self.data[r] -= x;
        }
    }

    /// 累積和を計算し、長さnの配列を返す
    ///
    /// この関数を呼び出すと、すべての区間加算が適用された最終的な配列が返されます。
    ///
    /// # 戻り値
    /// 累積和が計算された配列（長さn）
    pub fn build(&mut self) -> Vec<i64> {
        for i in 1..self.data.len() {
            self.data[i] += self.data[i - 1];
        }
        self.data.pop(); // n+1 -> n
        self.data.clone()
    }
}

/// 2次元imos法のライブラリ
///
/// 2次元imos法は2次元平面上の長方形領域への値の加算を効率的に処理するデータ構造です。
/// 複数の長方形領域に値を加算した後、一度の2次元累積和計算で全体の結果を得ることができます。
///
/// # 計算量
/// - 長方形加算: O(1)
/// - 2次元累積和計算: O(h×w)
///
/// # 使用例
/// ```
/// # use rust_macro::Imos2D;
/// let mut imos = Imos2D::new(3, 3);
/// imos.add(0, 0, 2, 2, 1);  // (0,0)から(2,2)の長方形に1を加算
/// imos.add(1, 1, 3, 3, 2);  // (1,1)から(3,3)の長方形に2を加算
/// let result = imos.build();
/// assert_eq!(result, vec![vec![1, 1, 0], vec![1, 3, 2], vec![0, 2, 2]]);
/// ```
pub struct Imos2D {
    data: Vec<Vec<i64>>,
    h: usize,
    w: usize,
}

impl Imos2D {
    /// 高さh, 幅wのimos配列を作成
    ///
    /// # 引数
    /// * `h` - 配列の高さ
    /// * `w` - 配列の幅
    ///
    /// # 戻り値
    /// 新しいImos2Dインスタンス
    pub fn new(h: usize, w: usize) -> Self {
        Imos2D {
            data: vec![vec![0; w + 1]; h + 1],
            h,
            w,
        }
    }

    /// 左上(x1, y1), 右下(x2, y2)の長方形にxを加算 (x2, y2は含まない)
    ///
    /// # 引数
    /// * `x1` - 左上の行座標（含む）
    /// * `y1` - 左上の列座標（含む）
    /// * `x2` - 右下の行座標（含まない）
    /// * `y2` - 右下の列座標（含まない）
    /// * `x` - 加算する値
    ///
    /// # 注意
    /// 座標が配列の範囲外の場合は何もしません
    pub fn add(&mut self, x1: usize, y1: usize, x2: usize, y2: usize, x: i64) {
        if x1 < self.h + 1 && y1 < self.w + 1 {
            self.data[x1][y1] += x;
        }
        if x2 < self.h + 1 && y1 < self.w + 1 {
            self.data[x2][y1] -= x;
        }
        if x1 < self.h + 1 && y2 < self.w + 1 {
            self.data[x1][y2] -= x;
        }
        if x2 < self.h + 1 && y2 < self.w + 1 {
            self.data[x2][y2] += x;
        }
    }

    /// 2次元累積和を計算し、h×wの配列を返す
    ///
    /// この関数を呼び出すと、すべての長方形加算が適用された最終的な2次元配列が返されます。
    ///
    /// # 戻り値
    /// 2次元累積和が計算された配列（h×w）
    pub fn build(&mut self) -> Vec<Vec<i64>> {
        for i in 0..=self.h {
            for j in 1..=self.w {
                self.data[i][j] += self.data[i][j - 1];
            }
        }
        for j in 0..=self.w {
            for i in 1..=self.h {
                self.data[i][j] += self.data[i - 1][j];
            }
        }
        self.data
            .iter()
            .take(self.h)
            .map(|row| row[..self.w].to_vec())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_imos1d_basic() {
        let mut imos = Imos1D::new(5);
        imos.add(1, 4, 2);
        imos.add(2, 5, 3);
        let res = imos.build();
        assert_eq!(res, vec![0, 2, 5, 5, 3]);
    }

    #[test]
    fn test_imos2d_basic() {
        let mut imos = Imos2D::new(3, 3);
        imos.add(0, 0, 2, 2, 1);
        imos.add(1, 1, 3, 3, 2);
        let res = imos.build();
        assert_eq!(res, vec![vec![1, 1, 0], vec![1, 3, 2], vec![0, 2, 2],]);
    }
}
