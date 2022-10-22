use std::convert::TryFrom;

/// Vertex
///
/// A basic concept of graph defined as an id.
pub type Vertex = usize;

/// Edge
///
/// A basic concept of graph, works like values (Copy)
pub trait Edge: Copy {
    fn vertices(&self) -> (Vertex, Vertex);
    fn other(&self, v: Vertex) -> Vertex {
        let (v1, v2) = self.vertices();
        if v == v1 {
            v2
        } else {
            v1
        }
    }
}

/// Undirected
///
/// Concept undirected edge
pub trait Undirected: Edge {}

/// Directed
///
/// Concept directed edge has `from` and `to`, so can be `reversed`
pub trait Directed: Edge {
    fn from(&self) -> Vertex {
        self.vertices().0
    }
    fn to(&self) -> Vertex {
        self.vertices().1
    }
    fn reversed(&self) -> Self;
}
// impl<E: Directed> !Undirected for E {}
// impl<E: Undirected> !Directed for E {}

/// Weighted
///
/// Concept weighted has weight on edge
pub trait Weighted: Edge + PartialOrd {
    fn weight(&self) -> f64;
}

/// NonNegative
///
/// Concept non-negative has zero or possitive weight value on edge
pub trait NonNegative: Weighted {}

/// Graph
///
/// A graph consists of vertices and edges.
pub trait Graph: Clone {
    type Edge: Edge;

    fn v_size(&self) -> usize;
    fn e_size(&self) -> usize;
    // fn adj(&self, v: usize) -> impl Iterator<Item = Self::Edge> + '_;
    fn adj(&self, v: usize) -> Box<dyn Iterator<Item = Self::Edge> + '_>;

    // same edge(v,w) repeated for v and w with undirected graphs
    fn edges(&self) -> Box<dyn Iterator<Item = Self::Edge> + '_> {
        Box::new((0..self.v_size()).flat_map(move |v| self.adj(v)))
    }
}

/// Mutable Graph
///
/// A mutuable graph can add edges to it.
pub trait MutableGraph: Graph {
    fn new(v: usize) -> Self;
    fn add_edge(&mut self, edge: Self::Edge);
}

/// Directed acyclic graph
///
/// A DAG is generated from other graphs which passed the cyclic detection,
/// and it can be sorted by topological order.
pub trait Acyclic: Graph + TryFrom<Self::Graph> + Sized {
    type Graph: Graph;

    fn topo_order(&self) -> std::vec::IntoIter<Vertex>;
}
