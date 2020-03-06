use super::base::*;
use super::dfs_order::{PreOrder, PreOrderIter};
use super::edge::*;

use std::collections::HashSet;

pub struct DiGraph {
	e: usize,
	adj: Vec<HashSet<usize>>,
}
impl DiGraph {
	pub fn new(v: usize) -> Self {
		DiGraph {
			e: 0,
			adj: vec![HashSet::new(); v],
		}
	}

	pub fn add_edge(&mut self, v: usize, w: usize) {
		self.adj[v].insert(w);
		self.e += 1;
	}
}
impl Graph for DiGraph {
	type Edge = DirectedEdge;

	fn v_size(&self) -> usize {
		self.adj.len()
	}

	fn adj<'a>(&'a self, v: usize) -> Box<dyn Iterator<Item = Self::Edge> + 'a> {
		Box::new(self.adj[v].iter().map(move |w| Self::Edge::new(v, *w)))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn empty() {
		let g = DiGraph::new(1);
		assert_eq!(vec![0], g.pre_order().collect::<Vec<_>>());
	}

	#[test]
	fn pre_order_simple() {
		let mut g = DiGraph::new(6);
		g.add_edge(0, 4);
		g.add_edge(4, 5);
		g.add_edge(4, 3);
		g.add_edge(3, 1);

		let r = g.pre_order().collect::<Vec<_>>();
		assert_eq!(6, r.len());
		if r[2] == 5 {
			assert_eq!(vec![0, 4, 5, 3, 1, 2], r);
		} else {
			assert_eq!(vec![0, 4, 3, 1, 5, 2], r);
		}
	}
}
