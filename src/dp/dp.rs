use std::collections::HashMap;

/// A generic DP table for memoization
pub struct DpTable<K, V> {
    pub table: HashMap<K, Option<V>>,
}

/// Trait for DP operations
pub trait DP<K, V> {
    fn get(&self, key: &K) -> Option<&V>;
    fn set(&mut self, key: K, value: V);
}

impl<K, V> Default for DpTable<K, V>
where
    K: std::hash::Hash + Eq,
{
    fn default() -> Self {
        Self {
            table: HashMap::new(),
        }
    }
}

impl<K, V> DP<K, V> for DpTable<K, V>
where
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
{
    fn get(&self, key: &K) -> Option<&V> {
        self.table.get(key)?.as_ref()
    }

    fn set(&mut self, key: K, value: V) {
        self.table.insert(key, Some(value));
    }
}
