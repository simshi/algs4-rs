# Graphs

## Design and code structure
  1. `base.rs`: express concepts by traits
      - `Vertex` is expressed as usize
      - `Edge` consists of two vertices
      - represent relations by super traits, e.g. `pub trait Directed:Edge`
      - whether a `Graph` is directed or not is based its associated `Edge` type

  1. `edge.rs` and `graph.rs`: implement vairious edges and graphs
  1. others: apply algorithms on various kinds of graphs based on trait bounds

## Undirected Graph
- no big deal, use HashMap in SymbolGraph

## Directed Graph
- order traversal by `Iterator`

## Minimum Spanning Tree
- `IndexMinPQ::upsert` make code clear
- `Iterator::flatten` in `EdgeWeightedGraph`
- implement Prim MST in eager approach, at most V-1 elements in IndexMinPQ, so E*LogV
- `Iterator::sum` and `Iterator::filter_map` is useful
- Kruskal MST is easier
