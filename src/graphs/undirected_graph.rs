use super::base::*;
use super::edge::*;

use std::collections::HashSet;

pub struct UndirectedGraph {
	e: usize,
	adj: Vec<HashSet<usize>>,
}
impl UndirectedGraph {
	pub fn add_edge(&mut self, v: usize, w: usize) {
		self.adj[v].insert(w);
		self.adj[w].insert(v);
		self.e += 1;
	}
}
impl Graph for UndirectedGraph {
	type Edge = UndirectedEdge;

	fn new(v: usize) -> Self {
		UndirectedGraph {
			e: 0,
			adj: vec![HashSet::new(); v],
		}
	}

	fn add_edge(&mut self, edge: &Self::Edge) {
		let (v, w) = edge.vertices();
		self.add_edge(v, w);
	}

	fn v_size(&self) -> usize {
		self.adj.len()
	}

	fn adj<'a>(&'a self, v: usize) -> Box<dyn Iterator<Item = Self::Edge> + 'a> {
		Box::new(self.adj[v].iter().map(move |w| Self::Edge::new(v, *w)))
	}
}
