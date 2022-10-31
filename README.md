# Learning Basic Algorithms in Rust
- Idiomatic Rust (`cargo clippy`)
- [Algorithms, 4th Edition by Robert Sedgewick](https://algs4.cs.princeton.edu/) && other algorithms

## From Algorithms
  - [x] [Sorting](./src/sorting/README.md)
  - [x] [Searching](./src/searching/README.md)
  - [x] [Graph](./src/graphs/README.md)
  - [x] [String](./src/strings/README.md)

## Other
  - [ ] [Dynamic Planning](./src/dp/README.md)

## Test && Benchmark
 - `cargo run --release --bin union_find < data/largeUF.txt`
 - `cargo run --release --bin symbol_graph data/routes.txt`
 - `cargo test sorting::`
 - `cargo bench`
