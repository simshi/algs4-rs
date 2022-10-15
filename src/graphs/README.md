# Graphs

## Concepts (as domain specific language, `base.rs`)
  - `Vertex` is expressed as usize
  - `Edge` consists of two vertices
  - represent relations by super traits, e.g. `pub trait Directed: Edge`
  - `Graph` has `Edge` as an associated type

## Concrete Types
  - `edge.rs`: edges, like `WeightedDirectedEdge`, `UndirectedEdge`;
  - `graph.rs`: graphs, like `EdgeWeightedDirectedGraph`
  - users can implement their own concrete types (which `impl` `Graph` and `Edge`)

## Algorithms
  - apply algorithms on various kinds of graphs based on trait bounds, e.g.
      ``` rust
      // DFS order applied only to directed graphs
      impl<'a, G, E: Directed> DFSOrder<G, E> for G {
          fn pre_order(&self) -> PreOrderIter<'_, G>;
          fn post_order(&self) -> PostOrderIter<'_, G, E>;
          fn reversed_post_order(&self) -> std::vec::IntoIter<Vertex>;
      }
      ```
  - bounds composition table, customer graph/edge types automatically implemented those algorithms:
      | *Algorithm/Bounds*            | **Graph Type** | **Edge Type**        |
      | ----------------------------- | -------------- | -------------------- |
      | Reversed                      | Mutable        | Directed             |
      | DFS Order                     | -              | Directed             |
      | Minimum Spanning Tree         | -              | Undirected           |
      | Connected Components          | -              | Undirected           |
      | Strongly Connected Components | -              | Directed             |
      | Cycle                         | -              | -                    |
      | Dijkstra Shortest Path        | -              | Directed+NonNegative |
      | Acyclic Shortest/Longest Path | Acyclic        | Directed+Weighted    |
      | Bellman Ford Shortest Path    | -              | Directed+Weighted    |

### Description
  - Reversed (not `fn reverse(&mut self)`)

    Reversing a directed graph by making a new graph with all `edge.reversed()`

  - Minimum Spanning Tree
    - `IndexMinPQ::upsert` make code clear
    - `Iterator::flatten` in `Graph`
    - implement Prim MST in eager approach, at most V-1 elements in IndexMinPQ, so E*LogV
    - `Iterator::sum` and `Iterator::filter_map` is useful
    - Kruskal MST is easier
  - DFS Order: implemented as `Iterator` by a stack
  - (Strongly) connected components
    - an imporovement: elimated `count` for component's id, calculated by the length of `sizes:Vec<usize>`
  - Acyclic shortest path
    - `Acyclic` graph has `fn topo_order()` which clearly expressed the algorithm's requirement
  - Bellman Ford
    - `Result<WeightedPath<E>, Cycle>` express the result of the algorithm, either a shortest path found, or an cycle detected

## Known Issues
  - impl can't disjoint based on associated type, e.g.
    ```rust
    impl<G, E:Directed> CycleDetection for G where G:Graph<Edge=E>
    //impl<G, E:Undirected> CycleDetection for G where G:Graph<Edge=E>
    ```
  - and below code wouldn't compile:
    ```rust
    impl<E: Directed> !Undirected for E {}
    impl<E: Undirected> !Directed for E {}
    ```
  - return type of `adj` is `Box`ed, whichi is not ideal, have tried `AdjacencyIter` as associated type, hit various kinds of issues:
    ```rust
    pub trait Graph {
      // type AdjacencyIter: Iterator<Item = Self::Edge>;
	    fn adj(&self, v: usize) -> Box<dyn Iterator<Item = Self::Edge> + '_>;
        // ...
    }
    ```

## Some Thoughts On Design
  - comparing to OOP (e.g. Java) design, `trait` provides more flexiblities, can do more compositions;
  - by `trait` seems we can achieve a perfect **Mixin** mechanism without messing up with inheritance, mixin is a poison in OOP but feels very nature implemented with `trait`
