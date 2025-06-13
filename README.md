# Rust Macro Library for Competitive Programming

A collection of useful macros and data structures for competitive programming in Rust.

## Features

- **Macros**
  - `print!(...)`: Prints values to stdout with space separation
  - `println!(...)`: Prints values to stdout with space separation and adds newline

- **Data Structures**
  - `UnionFind`: Efficient Union-Find/Disjoint Set Union data structure
  - `CumulativeSum`: 1D prefix sums for range queries
  - `CumulativeSum2D`: 2D prefix sums for rectangular queries

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
rust-macro = { git = "https://github.com/yourusername/rust-macro" }
```

### Macros

```rust
use rust_macro::*;

fn main() {
    print!("Hello", "world!");  // Prints: Hello world!
    println!(1, 2, 3);          // Prints: 1 2 3\n
}
```

### Union-Find

```rust
use rust_macro::UnionFind;

fn main() {
    let mut uf = UnionFind::new(5);
    
    uf.unite(0, 1);
    uf.unite(2, 3);
    
    println!("Same set:", uf.same(0, 1));  // true
    println!("Set size:", uf.size(0));     // 2
}
```

### 2D Cumulative Sum

```rust
use rust_macro::CumulativeSum2D;

fn main() {
    let matrix = vec![
        vec![1, 2],
        vec![3, 4],
    ];
    let cs = CumulativeSum2D::new(&matrix);
    println!("{}", cs.sum(0, 0, 2, 2)); // 10
}
```

## Running Tests

```bash
cargo test
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a pull request

## License

MIT
