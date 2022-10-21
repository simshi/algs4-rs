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

    - Reversing a directed graph by making a new graph with all `edge.reversed()`
    - TODO: impl<G, MG> From<G> for MG or impl<G> Reversed<G> for DirectedGraph?

  - DFS Order: implemented as `Iterator` by a stack
  - **Union Find**
    - to solve dynamic connectivity
    - connect small tree to larger tree to get lower level tree
    - do path compression in `union`
  - **Minimum Spanning Tree**
    - a spanning tree with minimum sum(weight of edges), in a weighted undirected graph
    - implemented for all undirected weighted graphs
    - Prim MST in eager approach(`E+VlogV`): for each v added into the growing tree; for e in v.adj(); upsert e; done; done; max V-1 in IndexMinPQ, so `V*LogV`; and each edge is visited to check `marked[w]`, so `E+VlogV`.
    - Kruskal(`ElogE`): for edge in MinPQ(edges).pop(); if u,v not connected add to tree, if tree.edges.len()==V-1 break; done.
    - `IndexMinPQ::upsert` make code clear
  - **Connected Components**
    - a connected component of an undirected graph is a connected subgraph that is not part of any larger connected subgraph.
    - algorithm: use DFS to traversal from a vertex, assign a subgraph id to them; To handle a forest, loop all unmarked vertices as a subgraph root.
  - **Strongly connected components**
    - A graph is said to be strongly connected if every vertex is reachable from every other vertex. The strongly connected components of an arbitrary directed graph form a partition into subgraphs that are themselves strongly connected.
    - Kosaraju-Shrir's algorithm: 生成G的反图Gr，求解Gr的反向后序遍历（强连通分量是一个环，理论上没有拓扑序列，但是**将G的每个强连通分量看着一个点**时，即可拓扑排序Gr），最后根据此序列DFS原图G，每次DFS即可得一个强连通分量.
     ```
     例：
     原图G：|A|->B->|C|, A、C是强连通分量，B是单顶点，
     反向图Gr：|A|<-B<-|C|，
     对Gr（近似）拓扑排序后，起点必然是|C|的一个顶点v，
     从v开始DFS原图G可得第一个分量C，然后序列到B，第二个分量只含B，最后得到A，求解完成.
     ```
    - an imporovement: elimated `count` for component's id, calculated by the length of `sizes:Vec<usize>`
  - Acyclic shortest path
    - `Acyclic` graph has `fn topo_order()` which clearly expressed the algorithm's requirement
  - Bellman Ford
    - `Result<WeightedPath<E>, Cycle>` express the result of the algorithm, either a shortest path found, or an cycle detected
    - `Iterator::flatten` in `Graph`

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
