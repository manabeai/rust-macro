use bitvec::prelude::*;
use std::collections::BTreeMap;

/// 値の座圧（座標圧縮）を行う構造体
#[derive(Debug, Clone)]
pub struct Compress<T> {
    /// 値から圧縮後のインデックスへのマッピング
    mapping: BTreeMap<T, usize>,
    /// 圧縮後のインデックスから元の値への逆変換テーブル
    rev: Vec<T>,
}

impl<T: Ord + Clone> Compress<T> {
    /// 値のリストから座圧構造体を生成する
    ///
    /// # 引数
    /// * `values` - 圧縮したい値のベクタ
    ///
    /// # 例
    /// ```
    /// use rust_macro::utils::Compress;
    /// let c = Compress::new(vec![100, 200, 100, 300]);
    /// assert_eq!(c.get(&100), 0);
    /// assert_eq!(c.get(&200), 1);
    /// assert_eq!(c.get(&300), 2);
    /// ```
    pub fn new(mut values: Vec<T>) -> Self {
        values.sort();
        values.dedup();
        let mapping = values
            .iter()
            .enumerate()
            .map(|(i, v)| (v.clone(), i))
            .collect();
        Compress {
            mapping,
            rev: values,
        }
    }

    /// 値xの圧縮後のインデックスを取得する
    ///
    /// # パニック
    /// xが存在しない場合panicします
    pub fn get(&self, x: &T) -> usize {
        self.mapping[x]
    }

    /// 圧縮後の値の種類数を返す
    pub fn size(&self) -> usize {
        self.rev.len()
    }

    /// 圧縮後のインデックスiに対応する元の値を返す
    pub fn rev(&self, i: usize) -> &T {
        &self.rev[i]
    }
}

/// 真偽値に応じて"Yes"/"No"を出力するユーティリティ関数
pub fn yesno(b: bool) {
    if b {
        println!("Yes");
    } else {
        println!("No");
    }
}

pub fn fmt_bitvec(bits: &BitVec<usize, Msb0>) -> String {
    bits.iter().map(|b| if *b { '1' } else { '0' }).collect()
}

pub fn fmt_u2bit(bits: usize) -> String {
    let mut s = String::new();
    for i in (0..30).rev() {
        s.push(if (bits >> i) & 1 == 1 { '1' } else { '0' });
    }
    s
}

/// イテレータを受け取って回文であるか判定する
pub fn is_palindrome<I, T>(iter: I) -> bool
where
    I: IntoIterator<Item = T>,
    T: PartialEq,
{
    let items: Vec<T> = iter.into_iter().collect();
    items.iter().eq(items.iter().rev())
}

/// 10進数をb進数に変換して返す
pub fn to_base(mut n: usize, base: usize) -> Vec<usize> {
    if n == 0 {
        return vec![0];
    }
    let mut digits = Vec::new();
    while n > 0 {
        digits.push(n % base);
        n /= base;
    }
    digits.reverse();
    digits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome(vec![1, 2, 3, 2, 1]));
        assert!(is_palindrome(vec![1, 2, 2, 1]));
        assert!(is_palindrome(vec![5]));
        assert!(is_palindrome(Vec::<i32>::new()));
        assert!(!is_palindrome(vec![1, 2, 3, 4]));
        assert!(is_palindrome("racecar".chars()));
        assert!(!is_palindrome("hello".chars()));
    }

    #[test]
    fn test_to_base() {
        assert_eq!(to_base(0, 2), vec![0]);
        assert_eq!(to_base(10, 2), vec![1, 0, 1, 0]);
        assert_eq!(to_base(255, 16), vec![15, 15]);
        assert_eq!(to_base(8, 3), vec![2, 2]);
        assert_eq!(to_base(27, 3), vec![1, 0, 0, 0]);
        assert_eq!(to_base(123, 10), vec![1, 2, 3]);
    }
}
