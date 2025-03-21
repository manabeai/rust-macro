#![allow(
    non_snake_case,
    unused_variables,
    unused_assignments,
    unused_mut,
    unused_imports,
    unused_macros,
    dead_code
)]

use std::{cmp::max, cmp::min, collections::HashMap, collections::HashSet, io::*};
use std::ops::{Add, Sub};

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
    fn new(arr: &[T]) -> Self {
        let mut data = Vec::with_capacity(arr.len() + 1);
        data.push(T::default());

        for &val in arr {
            let last = *data.last().unwrap();
            data.push(last + val);
        }

        Self { data }
    }

    fn sum(&self, l: usize, r: usize) -> T {
        assert!(l <= r && r < self.data.len());
        self.data[r] - self.data[l]
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
}
