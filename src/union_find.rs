//! Union-Find data structure implementation

#[derive(Debug)]
pub struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    /// Creates a new Union-Find structure with `n` elements
    pub fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    /// Finds the root of element `x`
    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] == x {
            x
        } else {
            let p = self.find(self.parent[x]);
            self.parent[x] = p;
            p
        }
    }

    /// Unites two sets containing `x` and `y`
    pub fn unite(&mut self, x: usize, y: usize) {
        let x_root = self.find(x);
        let y_root = self.find(y);
        
        if x_root == y_root {
            return;
        }
        
        // Union by size
        if self.size[x_root] < self.size[y_root] {
            self.parent[x_root] = y_root;
            self.size[y_root] += self.size[x_root];
        } else {
            self.parent[y_root] = x_root;
            self.size[x_root] += self.size[y_root];
        }
    }

    /// Checks if `x` and `y` are in the same set
    pub fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    /// Returns the size of the set containing `x`
    pub fn size(&mut self, x: usize) -> usize {
        let root = self.find(x);
        self.size[root]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_union_find() {
        let mut uf = UnionFind::new(5);
        
        // Initial state
        assert!(uf.same(0, 0));
        assert!(!uf.same(0, 1));
        
        // Union operations
        uf.unite(0, 1);
        assert!(uf.same(0, 1));
        
        uf.unite(2, 3);
        assert!(uf.same(2, 3));
        assert!(!uf.same(1, 2));
        
        uf.unite(1, 2);
        assert!(uf.same(0, 3));
        
        // Check size
        assert_eq!(uf.size(0), 4);
    }
}
