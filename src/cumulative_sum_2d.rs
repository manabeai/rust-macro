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

#[derive(Debug, Clone)]
pub struct CumulativeSum2D<T>
where
    T: Add<Output = T> + Sub<Output = T> + Copy + Default,
{
    data: Vec<Vec<T>>, // (n+1) x (m+1) prefix sums
}

impl<T> CumulativeSum2D<T>
where
    T: Add<Output = T> + Sub<Output = T> + Copy + Default,
{
    /// Creates a new 2D cumulative sum from a matrix slice.
    pub fn new(matrix: &[Vec<T>]) -> Self {
        let n = matrix.len();
        let m = if n > 0 { matrix[0].len() } else { 0 };
        let mut data = vec![vec![T::default(); m + 1]; n + 1];
        for i in 0..n {
            for j in 0..m {
                data[i + 1][j + 1] = data[i + 1][j] + data[i][j + 1] - data[i][j] + matrix[i][j];
            }
        }
        Self { data }
    }

    /// Returns the sum on the rectangle [x1, x2) x [y1, y2).
    pub fn sum(&self, x1: usize, y1: usize, x2: usize, y2: usize) -> T {
        assert!(x1 <= x2 && y1 <= y2);
        assert!(x2 < self.data.len() && y2 < self.data[0].len());
        self.data[x2][y2] - self.data[x1][y2] - self.data[x2][y1] + self.data[x1][y1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cumulative_sum_2d() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let cum = CumulativeSum2D::new(&matrix);

        assert_eq!(cum.sum(0, 0, 1, 1), 1); // single element
        assert_eq!(cum.sum(0, 0, 2, 2), 12); // top-left 2x2 block
        assert_eq!(cum.sum(1, 1, 3, 3), 28); // bottom-right 2x2 block
        assert_eq!(cum.sum(0, 0, 3, 3), 45); // whole matrix
    }
}
