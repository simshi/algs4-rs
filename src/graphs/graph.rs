use super::base::*;
use super::edge::*;

pub struct EdgeWeightedUndirectedGraph {
    e: usize,
    adj: Vec<Vec<WeightedUndirectedEdge>>,
}
impl EdgeWeightedUndirectedGraph {
    pub fn new(v: usize) -> Self {
        EdgeWeightedUndirectedGraph {
            e: 0,
            adj: vec![Vec::new(); v],
        }
    }

    pub fn add_edge(&mut self, edge: &WeightedUndirectedEdge) {
        let (v, w) = edge.vertices();
        self.adj[v].push(*edge);
        self.adj[w].push(*edge);
        self.e += 1;
    }
}
impl Graph for EdgeWeightedUndirectedGraph {
    type Edge = WeightedUndirectedEdge;
    // type Iter<'a> = std::slice::Iter<'a, Self::Edge>; // GAT is not ready

    fn v_size(&self) -> usize {
        self.adj.len()
    }
    fn e_size(&self) -> usize {
        self.e
    }

    fn adj(&self, v: usize) -> Box<dyn Iterator<Item = Self::Edge> + '_> {
        Box::new(self.adj[v].iter().cloned())
    }
}

pub struct EdgeWeightedDirectedGraph {
    e: usize,
    adj: Vec<Vec<WeightedDirectedEdge>>,
}
impl Graph for EdgeWeightedDirectedGraph {
    type Edge = WeightedDirectedEdge;

    fn v_size(&self) -> usize {
        self.adj.len()
    }
    fn e_size(&self) -> usize {
        self.e
    }

    fn adj(&self, v: usize) -> Box<dyn Iterator<Item = Self::Edge> + '_> {
        Box::new(self.adj[v].iter().cloned())
    }
}
impl MutableGraph for EdgeWeightedDirectedGraph {
    fn new(v: usize) -> Self {
        EdgeWeightedDirectedGraph {
            e: 0,
            adj: vec![Vec::new(); v],
        }
    }

    fn add_edge(&mut self, edge: &Self::Edge) {
        self.adj[edge.from()].push(*edge);
        self.e += 1;
    }
}

pub struct EdgeNonNegativeWeightedDirectedGraph {
    e: usize,
    adj: Vec<Vec<NonNegativeWeightedDirectedEdge>>,
}
impl Graph for EdgeNonNegativeWeightedDirectedGraph {
    type Edge = NonNegativeWeightedDirectedEdge;

    fn v_size(&self) -> usize {
        self.adj.len()
    }
    fn e_size(&self) -> usize {
        self.e
    }

    fn adj(&self, v: usize) -> Box<dyn Iterator<Item = Self::Edge> + '_> {
        Box::new(self.adj[v].iter().cloned())
    }
}
impl MutableGraph for EdgeNonNegativeWeightedDirectedGraph {
    fn new(v: usize) -> Self {
        EdgeNonNegativeWeightedDirectedGraph {
            e: 0,
            adj: vec![Vec::new(); v],
        }
    }

    fn add_edge(&mut self, edge: &Self::Edge) {
        self.adj[edge.from()].push(*edge);
        self.e += 1;
    }
}
