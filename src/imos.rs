/// 1次元imos法のライブラリ
pub struct Imos1D {
    data: Vec<i64>,
}

impl Imos1D {
    /// 長さnのimos配列を作成
    pub fn new(n: usize) -> Self {
        Imos1D { data: vec![0; n + 1] }
    }

    /// 区間[l, r)にxを加算
    pub fn add(&mut self, l: usize, r: usize, x: i64) {
        if l < self.data.len() {
            self.data[l] += x;
        }
        if r < self.data.len() {
            self.data[r] -= x;
        }
    }

    /// 累積和を計算し、長さnの配列を返す
    pub fn build(&mut self) -> Vec<i64> {
        for i in 1..self.data.len() {
            self.data[i] += self.data[i - 1];
        }
        self.data.pop(); // n+1 -> n
        self.data.clone()
    }
}

/// 2次元imos法のライブラリ
pub struct Imos2D {
    data: Vec<Vec<i64>>,
    h: usize,
    w: usize,
}

impl Imos2D {
    /// 高さh, 幅wのimos配列を作成
    pub fn new(h: usize, w: usize) -> Self {
        Imos2D { data: vec![vec![0; w + 1]; h + 1], h, w }
    }

    /// 左上(x1, y1), 右下(x2, y2)の長方形にxを加算 (x2, y2は含まない)
    pub fn add(&mut self, x1: usize, y1: usize, x2: usize, y2: usize, x: i64) {
        if x1 < self.h + 1 && y1 < self.w + 1 { self.data[x1][y1] += x; }
        if x2 < self.h + 1 && y1 < self.w + 1 { self.data[x2][y1] -= x; }
        if x1 < self.h + 1 && y2 < self.w + 1 { self.data[x1][y2] -= x; }
        if x2 < self.h + 1 && y2 < self.w + 1 { self.data[x2][y2] += x; }
    }

    /// 累積和を計算し、h×wの配列を返す
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
        self.data.iter().take(self.h).map(|row| row[..self.w].to_vec()).collect()
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
        assert_eq!(res, vec![
            vec![1, 1, 0],
            vec![1, 3, 2],
            vec![0, 2, 2],
        ]);
    }
}

