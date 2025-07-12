//! Union-Find data structure implementation

use im_rc::Vector;

#[derive(Debug)]
pub struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

/// Persistent Union-Find (Disjoint Set Union) data structure
///
/// A persistent version of the Union-Find data structure that uses immutable data structures
/// (`im_rc::Vector`) to support efficient cloning and versioning. All operations create
/// new persistent states rather than modifying the structure in place.
///
/// # Key Features
///
/// - **Path Compression**: Flattens tree structure for better amortized performance
/// - **Union by Size**: Attaches smaller trees to larger ones to maintain balance
/// - **Persistent**: Supports efficient cloning and maintains immutable history
/// - **Same API**: Identical interface to the standard UnionFind structure
///
/// # Use Cases
///
/// - **Connectivity Queries**: Check if two elements are connected
/// - **Dynamic Connectivity**: Add connections between elements
/// - **Component Sizes**: Query the size of connected components
/// - **Versioning**: Maintain multiple versions of the data structure
/// - **Backtracking Algorithms**: Restore previous states efficiently
///
/// # Time Complexity
///
/// - **Construction**: O(n)
/// - **Find**: Amortized O(α(n)), worst case O(log n)
/// - **Unite**: Amortized O(α(n)), worst case O(log n)
/// - **Same**: Amortized O(α(n)), worst case O(log n)
/// - **Size**: Amortized O(α(n)), worst case O(log n)
/// - **Clone**: O(1) (shallow copy due to immutable structures)
///
/// Where α is the inverse Ackermann function (practically constant for all realistic inputs).
///
/// # Space Complexity
///
/// - **Overall**: O(n) for the data structure
/// - **Per Operation**: O(log n) due to immutable vector updates
/// - **Cloning**: O(1) due to structural sharing
///
/// # Examples
///
/// ## Basic Usage
///
/// ```rust
/// # use rust_macro::PersistentUnionFind;
/// let mut uf = PersistentUnionFind::new(5);
///
/// // Connect elements
/// uf.unite(0, 1);
/// uf.unite(2, 3);
///
/// // Check connectivity
/// assert!(uf.same(0, 1));
/// assert!(!uf.same(0, 2));
///
/// // Query component sizes
/// assert_eq!(uf.size(0), 2); // {0, 1}
/// assert_eq!(uf.size(2), 2); // {2, 3}
/// assert_eq!(uf.size(4), 1); // {4}
/// ```
///
/// ## Persistent Operations
///
/// ```rust
/// # use rust_macro::PersistentUnionFind;
/// let mut uf1 = PersistentUnionFind::new(4);
/// uf1.unite(0, 1);
///
/// // Create a snapshot
/// let mut uf2 = uf1.clone();
///
/// // Modify the original
/// uf1.unite(2, 3);
///
/// // The snapshot remains unchanged
/// assert!(uf1.same(2, 3));  // Modified version
/// assert!(!uf2.same(2, 3)); // Original snapshot
/// ```
#[derive(Debug, Clone)]
pub struct PersistentUnionFind {
    parent: Vector<usize>,
    size: Vector<usize>,
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

impl PersistentUnionFind {
    /// Creates a new persistent Union-Find structure with `n` elements
    ///
    /// Initializes a persistent Union-Find data structure where each element is in its own set.
    /// Uses immutable data structures (`im_rc::Vector`) to support efficient cloning and
    /// persistent operations.
    ///
    /// # Arguments
    ///
    /// * `n` - Number of elements (0 to n-1)
    ///
    /// # Time Complexity
    ///
    /// * **O(n)** - Linear in the number of elements
    ///
    /// # Space Complexity
    ///
    /// * **O(n)** - Linear space for parent and size vectors
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use rust_macro::PersistentUnionFind;
    /// let mut uf = PersistentUnionFind::new(5);
    /// // Creates 5 disjoint sets: {0}, {1}, {2}, {3}, {4}
    /// ```
    pub fn new(n: usize) -> Self {
        PersistentUnionFind {
            parent: (0..n).collect(),
            size: Vector::from(vec![1; n]),
        }
    }

    /// Finds the root of element `x`
    ///
    /// Finds the root (representative) of the set containing element `x` with path compression.
    /// Path compression flattens the tree structure to improve future query performance.
    /// Uses immutable data structures, so path compression creates a new persistent state.
    ///
    /// # Arguments
    ///
    /// * `x` - Element to find the root of
    ///
    /// # Returns
    ///
    /// The root element of the set containing `x`
    ///
    /// # Time Complexity
    ///
    /// * **Amortized O(α(n))** - Where α is the inverse Ackermann function (practically constant)
    /// * **Worst case O(log n)** - For a single operation without prior compression
    ///
    /// # Space Complexity
    ///
    /// * **O(log n)** - Due to immutable vector updates during path compression
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use rust_macro::PersistentUnionFind;
    /// let mut uf = PersistentUnionFind::new(5);
    /// uf.unite(1, 2);
    /// uf.unite(2, 3);
    ///
    /// let root1 = uf.find(1);
    /// let root3 = uf.find(3);
    /// assert_eq!(root1, root3); // Same root means same set
    /// ```
    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] == x {
            x
        } else {
            let p = self.clone().find_immut(self.parent[x]);
            self.parent = self.parent.update(x, p);
            p
        }
    }

    /// Helper function for immutable find operations
    fn find_immut(&self, x: usize) -> usize {
        if self.parent[x] == x {
            x
        } else {
            self.find_immut(self.parent[x])
        }
    }

    /// Unites two sets containing `x` and `y`
    ///
    /// Merges the sets containing elements `x` and `y` into a single set.
    /// Uses union by size heuristic to keep trees balanced, attaching the smaller
    /// tree to the root of the larger tree. Creates a new persistent state.
    ///
    /// # Arguments
    ///
    /// * `x` - Element from the first set
    /// * `y` - Element from the second set
    ///
    /// # Time Complexity
    ///
    /// * **Amortized O(α(n))** - Where α is the inverse Ackermann function
    /// * **Worst case O(log n)** - Due to find operations
    ///
    /// # Space Complexity
    ///
    /// * **O(log n)** - Due to immutable vector updates
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use rust_macro::PersistentUnionFind;
    /// let mut uf = PersistentUnionFind::new(5);
    ///
    /// // Initially: {0}, {1}, {2}, {3}, {4}
    /// uf.unite(0, 1); // Now: {0,1}, {2}, {3}, {4}
    /// uf.unite(2, 3); // Now: {0,1}, {2,3}, {4}
    /// uf.unite(1, 2); // Now: {0,1,2,3}, {4}
    ///
    /// assert!(uf.same(0, 3)); // All in same set
    /// assert!(!uf.same(0, 4)); // 4 is separate
    /// ```
    pub fn unite(&mut self, x: usize, y: usize) {
        let x_root = self.find(x);
        let y_root = self.find(y);

        if x_root == y_root {
            return;
        }

        // Union by size
        if self.size[x_root] < self.size[y_root] {
            self.parent = self.parent.update(x_root, y_root);
            self.size = self
                .size
                .update(y_root, self.size[y_root] + self.size[x_root]);
        } else {
            self.parent = self.parent.update(y_root, x_root);
            self.size = self
                .size
                .update(x_root, self.size[x_root] + self.size[y_root]);
        }
    }

    /// Checks if `x` and `y` are in the same set
    ///
    /// Determines whether two elements belong to the same connected component.
    /// This operation may trigger path compression through the find operations.
    ///
    /// # Arguments
    ///
    /// * `x` - First element to check
    /// * `y` - Second element to check
    ///
    /// # Returns
    ///
    /// `true` if both elements are in the same set, `false` otherwise
    ///
    /// # Time Complexity
    ///
    /// * **Amortized O(α(n))** - Where α is the inverse Ackermann function
    /// * **Worst case O(log n)** - Due to two find operations
    ///
    /// # Space Complexity
    ///
    /// * **O(log n)** - Due to immutable vector updates during path compression
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use rust_macro::PersistentUnionFind;
    /// let mut uf = PersistentUnionFind::new(4);
    ///
    /// assert!(!uf.same(0, 1)); // Initially separate
    /// uf.unite(0, 1);
    /// assert!(uf.same(0, 1)); // Now connected
    /// assert!(!uf.same(0, 2)); // Still separate from 2
    /// ```
    pub fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    /// Returns the size of the set containing `x`
    ///
    /// Gets the number of elements in the connected component containing element `x`.
    /// This operation may trigger path compression through the find operation.
    ///
    /// # Arguments
    ///
    /// * `x` - Element whose set size to query
    ///
    /// # Returns
    ///
    /// The number of elements in the set containing `x`
    ///
    /// # Time Complexity
    ///
    /// * **Amortized O(α(n))** - Where α is the inverse Ackermann function
    /// * **Worst case O(log n)** - Due to find operation
    ///
    /// # Space Complexity
    ///
    /// * **O(log n)** - Due to immutable vector updates during path compression
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use rust_macro::PersistentUnionFind;
    /// let mut uf = PersistentUnionFind::new(5);
    ///
    /// assert_eq!(uf.size(0), 1); // Initially size 1
    /// uf.unite(0, 1);
    /// assert_eq!(uf.size(0), 2); // Now size 2
    /// uf.unite(0, 2);
    /// assert_eq!(uf.size(1), 3); // All connected elements have size 3
    /// ```
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

    #[test]
    fn test_persistent_union_find() {
        let mut uf = PersistentUnionFind::new(5);

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

    #[test]
    fn test_persistent_clone() {
        let mut uf1 = PersistentUnionFind::new(5);
        uf1.unite(0, 1);

        // Clone at this state
        let mut uf2 = uf1.clone();

        // Modify original
        uf1.unite(2, 3);

        // Check that clone is unaffected
        assert!(uf1.same(2, 3));
        assert!(!uf2.same(2, 3));
        assert!(uf2.same(0, 1));
    }
}
