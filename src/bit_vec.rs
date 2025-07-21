use std::fmt;
use std::ops::{Add, BitAnd, BitOr, BitXor};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BitVec {
    data: usize,
    n: usize,
}

impl BitVec {
    /// 長さnの0初期化ビット列
    pub fn new(n: usize) -> Self {
        Self { data: 0, n }
    }

    /// usize値から nビットにマスクして作成
    pub fn from_usize(data: usize, n: usize) -> Self {
        Self {
            data: data & ((1 << n) - 1),
            n,
        }
    }

    /// usize値として取得
    pub fn to_usize(self) -> usize {
        self.data
    }

    /// ビット長
    pub fn len(self) -> usize {
        self.n
    }

    /// ビット長が0かどうか
    pub fn is_empty(self) -> bool {
        self.n == 0
    }

    /// 上位ビットから見て i 番目のビットを取得（i=0が最上位）
    pub fn get(self, i: usize) -> bool {
        assert!(i < self.n);
        (self.data >> (self.n - 1 - i)) & 1 == 1
    }

    /// i番目に1が立っている単位ビット列を作る（i=0が最上位）
    pub fn unit(i: usize, n: usize) -> Self {
        assert!(i < n);
        Self::from_usize(1 << (n - 1 - i), n)
    }

    /// ビット列を縦に表示
    pub fn dump(self) {
        for i in 0..self.n {
            println!("bit {:2}: {}", i, if self.get(i) { 1 } else { 0 });
        }
    }

    /// ビット列のイテレータ（上位bitから順に）
    pub fn iter(self) -> BitVecIter {
        BitVecIter {
            data: self.data,
            n: self.n,
            pos: 0,
        }
    }

    /// i 番目（上位0始まり）のビットを 0 または 1 に設定した新しい BitVec を返す
    pub fn set(self, i: usize, value: u8) -> Self {
        assert!(i < self.n);
        assert!(value == 0 || value == 1);
        let shift = self.n - 1 - i;
        let mask = 1 << shift;
        let new_data = match value {
            0 => self.data & !mask,
            1 => self.data | mask,
            _ => unreachable!(),
        };
        Self::from_usize(new_data, self.n)
    }

    pub fn set_mut(&mut self, i: usize, value: u8) {
        assert!(i < self.n);
        assert!(value == 0 || value == 1);
        let shift = self.n - 1 - i;
        let mask = 1 << shift;
        match value {
            0 => self.data &= !mask,
            1 => self.data |= mask,
            _ => unreachable!(),
        }
    }

    pub fn mask(n: usize) -> Self {
        assert!(n <= usize::BITS as usize);
        Self {
            data: (1 << n) - 1,
            n,
        }
    }
}

/// 表示（例: "0101"）
impl fmt::Display for BitVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.n {
            write!(f, "{}", if self.get(i) { 1 } else { 0 })?;
        }
        Ok(())
    }
}

// === ビット演算 ===

impl Add for BitVec {
    type Output = BitVec;
    fn add(self, rhs: BitVec) -> BitVec {
        assert_eq!(self.n, rhs.n);
        BitVec::from_usize(self.data.wrapping_add(rhs.data), self.n)
    }
}

impl BitXor for BitVec {
    type Output = BitVec;
    fn bitxor(self, rhs: BitVec) -> BitVec {
        assert_eq!(self.n, rhs.n);
        BitVec::from_usize(self.data ^ rhs.data, self.n)
    }
}

impl BitAnd for BitVec {
    type Output = BitVec;
    fn bitand(self, rhs: BitVec) -> BitVec {
        assert_eq!(self.n, rhs.n);
        BitVec::from_usize(self.data & rhs.data, self.n)
    }
}

impl BitOr for BitVec {
    type Output = BitVec;
    fn bitor(self, rhs: BitVec) -> BitVec {
        assert_eq!(self.n, rhs.n);
        BitVec::from_usize(self.data | rhs.data, self.n)
    }
}

// === イテレータ ===

pub struct BitVecIter {
    data: usize,
    n: usize,
    pos: usize,
}

impl Iterator for BitVecIter {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.n {
            None
        } else {
            let shift = self.n - 1 - self.pos;
            let bit = (self.data >> shift) & 1 == 1;
            self.pos += 1;
            Some(bit)
        }
    }
}

impl IntoIterator for BitVec {
    type Item = bool;
    type IntoIter = BitVecIter;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

// === 全探索列挙 BitVecRange ===

pub struct BitVecRange {
    n: usize,
    curr: usize,
    end: usize,
}

impl BitVecRange {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            curr: 0,
            end: 1 << n,
        }
    }
}

impl Iterator for BitVecRange {
    type Item = BitVec;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr < self.end {
            let bv = BitVec::from_usize(self.curr, self.n);
            self.curr += 1;
            Some(bv)
        } else {
            None
        }
    }
}

impl From<BitVec> for usize {
    fn from(bv: BitVec) -> Self {
        bv.to_usize()
    }
}

// === Trait による .all() API ===

pub trait BitVecAll {
    fn all(n: usize) -> BitVecRange;
}

impl BitVecAll for BitVec {
    fn all(n: usize) -> BitVecRange {
        BitVecRange::new(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let bv = BitVec::new(8);
        assert_eq!(bv.len(), 8);
        assert_eq!(bv.to_usize(), 0);
    }

    #[test]
    fn test_from_usize() {
        let bv = BitVec::from_usize(0b1010, 4);
        assert_eq!(bv.len(), 4);
        assert_eq!(bv.to_usize(), 0b1010);
    }

    #[test]
    fn test_from_usize_with_mask() {
        let bv = BitVec::from_usize(0b11110000, 4);
        assert_eq!(bv.to_usize(), 0b0000);

        let bv = BitVec::from_usize(0b11111111, 4);
        assert_eq!(bv.to_usize(), 0b1111);
    }

    #[test]
    fn test_get() {
        let bv = BitVec::from_usize(0b1010, 4);
        assert_eq!(bv.get(0), true); // MSB
        assert_eq!(bv.get(1), false);
        assert_eq!(bv.get(2), true);
        assert_eq!(bv.get(3), false); // LSB
    }

    #[test]
    #[should_panic]
    fn test_get_out_of_bounds() {
        let bv = BitVec::new(4);
        bv.get(4);
    }

    #[test]
    fn test_display() {
        let bv = BitVec::from_usize(0b1010, 4);
        assert_eq!(format!("{}", bv), "1010");

        let bv = BitVec::from_usize(0b0001, 4);
        assert_eq!(format!("{}", bv), "0001");
    }

    #[test]
    fn test_add() {
        let bv1 = BitVec::from_usize(0b0101, 4);
        let bv2 = BitVec::from_usize(0b0011, 4);
        let result = bv1 + bv2;
        assert_eq!(result.to_usize(), 0b1000);
    }

    #[test]
    fn test_add_with_overflow() {
        let bv1 = BitVec::from_usize(0b1111, 4);
        let bv2 = BitVec::from_usize(0b0001, 4);
        let result = bv1 + bv2;
        assert_eq!(result.to_usize(), 0b0000);
    }

    #[test]
    #[should_panic]
    fn test_add_different_lengths() {
        let bv1 = BitVec::new(4);
        let bv2 = BitVec::new(8);
        let _ = bv1 + bv2;
    }

    #[test]
    fn test_bitxor() {
        let bv1 = BitVec::from_usize(0b1010, 4);
        let bv2 = BitVec::from_usize(0b1100, 4);
        let result = bv1 ^ bv2;
        assert_eq!(result.to_usize(), 0b0110);
    }

    #[test]
    fn test_bitand() {
        let bv1 = BitVec::from_usize(0b1010, 4);
        let bv2 = BitVec::from_usize(0b1100, 4);
        let result = bv1 & bv2;
        assert_eq!(result.to_usize(), 0b1000);
    }

    #[test]
    fn test_bitor() {
        let bv1 = BitVec::from_usize(0b1010, 4);
        let bv2 = BitVec::from_usize(0b0101, 4);
        let result = bv1 | bv2;
        assert_eq!(result.to_usize(), 0b1111);
    }

    #[test]
    fn test_clone_and_equality() {
        let bv1 = BitVec::from_usize(0b1010, 4);
        let bv2 = bv1.clone();
        assert_eq!(bv1, bv2);
    }

    #[test]
    fn test_bitvec_range() {
        let mut range = BitVecRange::new(2);
        let mut results = Vec::new();

        while let Some(bv) = range.next() {
            results.push(bv.to_usize());
        }

        assert_eq!(results, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_bitvec_range_collect() {
        let range = BitVecRange::new(3);
        let bitvecs: Vec<BitVec> = range.collect();

        assert_eq!(bitvecs.len(), 8);
        for (i, bv) in bitvecs.iter().enumerate() {
            assert_eq!(bv.to_usize(), i);
            assert_eq!(bv.len(), 3);
        }
    }

    #[test]
    fn test_edge_cases() {
        let bv = BitVec::new(0);
        assert_eq!(bv.len(), 0);

        let bv = BitVec::from_usize(0, 1);
        assert_eq!(bv.get(0), false);

        let bv = BitVec::from_usize(1, 1);
        assert_eq!(bv.get(0), true);
    }
}
