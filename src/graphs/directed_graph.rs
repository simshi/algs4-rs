use super::base::*;
use super::edge::*;

pub struct DirectedGraph {
	e: usize,
	adj: Vec<Vec<DirectedEdge>>,
}
impl DirectedGraph {
	pub fn add_edge(&mut self, v: usize, w: usize) {
		self.adj[v].push(DirectedEdge::new(v, w));
		self.e += 1;
	}
}
impl Graph for DirectedGraph {
	type Edge = DirectedEdge;

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
impl MutableGraph for DirectedGraph {
	fn new(v: usize) -> Self {
		DirectedGraph {
			e: 0,
			adj: vec![Vec::new(); v],
		}
	}

	fn add_edge(&mut self, edge: &Self::Edge) {
		let v = edge.from();
		self.adj[v].push(*edge);
		self.e += 1;
	}
}
