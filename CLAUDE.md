# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Development Commands

- **Build**: `cargo build`
- **Run tests**: `cargo test`
- **Run specific test**: `cargo test test_name`
- **Check code**: `cargo check`
- **Format code**: `cargo fmt`
- **Lint code**: `cargo clippy`

## Architecture Overview

This is a Rust library focused on competitive programming utilities. The codebase is organized into several modules:

### Core Structure
- `src/lib.rs` - Main library entry point that re-exports public APIs
- `src/macro_utils.rs` - Custom print/println macros with buffered output for performance
- `src/union_find.rs` - Union-Find data structure with path compression and union by size
- `src/cumulative_sum.rs` - Generic cumulative sum implementation for range queries
- `src/binary_search.rs` - Generic binary search implementation for monotonic functions
- `src/utils.rs` - Utility functions like `yesno()` for common competitive programming patterns

### Key Design Patterns
- All data structures use generic types where applicable (e.g., `CumulativeSum<T>`)
- Path compression is implemented in Union-Find for optimal performance
- Macros use buffered I/O (`BufWriter`) for faster output in competitive programming
- Each module includes comprehensive unit tests in `#[cfg(test)]` blocks

### Module Dependencies
- No external dependencies in main code (only std library)
- Modules are self-contained with minimal cross-dependencies
- Public API is exposed through `lib.rs` re-exports

### Testing Strategy
- Unit tests are co-located with implementation code
- Tests cover edge cases and typical usage patterns
- No integration tests directory currently exists