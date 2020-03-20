use super::base::*;
use super::edge::*;

pub struct UndirectedGraph {
	e: usize,
	adj: Vec<Vec<UndirectedEdge>>,
}
impl UndirectedGraph {
	pub fn new(v: usize) -> Self {
		UndirectedGraph {
			e: 0,
			adj: vec![Vec::new(); v],
		}
	}

	pub fn add_edge(&mut self, v: usize, w: usize) {
		self.adj[v].push(UndirectedEdge::new(v, w));
		self.adj[w].push(UndirectedEdge::new(v, w));
		self.e += 1;
	}
}
impl Graph for UndirectedGraph {
	type Edge = UndirectedEdge;

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
