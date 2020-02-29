# Graphs

## Undirected Graph
- no big deal, use HashMap in SymbolGraph

## Directed Graph
- extract BaseGraph trait
- order traversal by `Iterator`

## Minimum Spanning Tree
- `IndexMinPQ::upsert` make code clear
- `Iterator::flatten` in `EdgeWeightedGraph`
- implement Prim MST in eager approach, at most V-1 elements in IndexMinPQ, so E*LogV
- `Iterator::sum` and `Iterator::filter_map` is useful
- Kruskal MST is easier
