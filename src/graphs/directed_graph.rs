use super::base::*;
use super::edge::*;

use std::collections::HashSet;

pub struct DiGraph {
	e: usize,
	adj: Vec<HashSet<usize>>,
}
impl DiGraph {
	pub fn add_edge(&mut self, v: usize, w: usize) {
		self.adj[v].insert(w);
		self.e += 1;
	}
}
impl Graph for DiGraph {
	type Edge = DirectedEdge;

	fn new(v: usize) -> Self {
		DiGraph {
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

	fn adj<'a>(&'a self, v: usize) -> Box<dyn Iterator<Item = Self::Edge> + 'a> {
		Box::new(self.adj[v].iter().map(move |w| Self::Edge::new(v, *w)))
	}
}
