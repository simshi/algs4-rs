use super::base::*;
use super::edge::*;

use std::collections::HashSet;

pub struct DirectedGraph {
	e: usize,
	adj: Vec<HashSet<usize>>,
}
impl DirectedGraph {
	pub fn add_edge(&mut self, v: usize, w: usize) {
		self.adj[v].insert(w);
		self.e += 1;
	}
}
impl Graph for DirectedGraph {
	type Edge = DirectedEdge;

	fn new(v: usize) -> Self {
		DirectedGraph {
			e: 0,
			adj: vec![HashSet::new(); v],
		}
	}

	fn add_edge(&mut self, edge: &Self::Edge) {
		self.add_edge(edge.from(), edge.to());
	}

	fn v_size(&self) -> usize {
		self.adj.len()
	}
	fn e_size(&self) -> usize {
		self.e
	}

	fn adj<'a>(&'a self, v: usize) -> Box<dyn Iterator<Item = Self::Edge> + 'a> {
		Box::new(self.adj[v].iter().map(move |w| Self::Edge::new(v, *w)))
	}
}
