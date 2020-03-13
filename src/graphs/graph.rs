use super::base::*;
use super::edge::*;

pub struct EdgeWeightedUndirectedGraph {
    e: usize,
    adj: Vec<Vec<WeightedUndirectedEdge>>,
}
impl Graph for EdgeWeightedUndirectedGraph {
    type Edge = WeightedUndirectedEdge;

    fn new(v: usize) -> Self {
        EdgeWeightedUndirectedGraph {
            e: 0,
            adj: vec![Vec::new(); v],
        }
    }

    fn add_edge(&mut self, edge: &Self::Edge) {
        let (v, w) = edge.vertices();
        self.adj[v].push(edge.clone());
        self.adj[w].push(edge.clone());
        self.e += 1;
    }

    fn v_size(&self) -> usize {
        self.adj.len()
    }
    fn e_size(&self) -> usize {
        self.e
    }

    fn adj<'a>(&'a self, v: usize) -> Box<dyn Iterator<Item = Self::Edge> + 'a> {
        Box::new(self.adj[v].iter().cloned())
    }
}
