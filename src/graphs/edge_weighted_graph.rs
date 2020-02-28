use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Edge {
	v: usize,
	w: usize,
	weight: f64,
}
impl Edge {
	pub fn new(v: usize, w: usize, weight: f64) -> Self {
		Edge { v, w, weight }
	}

	pub fn either(&self) -> usize {
		self.v
	}
	pub fn other(&self, v: usize) -> usize {
		if v == self.v {
			self.w
		} else {
			self.v
		}
	}
	pub fn weight(&self) -> f64 {
		self.weight
	}
}
impl PartialOrd for Edge {
	fn partial_cmp(&self, other: &Edge) -> Option<Ordering> {
		self.weight.partial_cmp(&other.weight)
	}
}

pub struct EdgeWeightedGraph {
	v: usize,
	e: usize,
	adj: Vec<Vec<Edge>>,
}
impl EdgeWeightedGraph {
	pub fn new(v: usize) -> Self {
		EdgeWeightedGraph {
			v,
			e: 0,
			adj: vec![Vec::new(); v],
		}
	}

	pub fn edges(&self) -> impl Iterator<Item = &Edge> {
		self.adj.iter().flatten()
	}

	pub fn add_edge(&mut self, e: &Edge) {
		let v = e.either();
		let w = e.other(v);
		self.adj[v].push(e.clone());
		self.adj[w].push(e.clone());
		self.e += 1;
	}

	pub fn v_size(&self) -> usize {
		self.v
	}
	pub fn adj(&self, v: usize) -> impl Iterator<Item = &Edge> {
		self.adj[v].iter()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn empty() {
		let g = EdgeWeightedGraph::new(1);
		assert_eq!(1, g.v_size());
	}

	#[test]
	fn add_get() {
		let mut g = EdgeWeightedGraph::new(5);
		assert_eq!(5, g.v_size());
		g.add_edge(&Edge::new(0, 1, 0.5));

		let mut it = g.adj(0);
		let e = it.next().unwrap();
		assert_eq!(0, e.other(1));
		assert_eq!(1, e.other(0));
		assert_eq!(None, it.next());

		let mut it = g.adj(1);
		let e = it.next().unwrap();
		assert_eq!(0, e.other(1));
		assert_eq!(1, e.other(0));
		assert_eq!(None, it.next());
	}

	#[test]
	fn multiple_edges() {
		let mut g = EdgeWeightedGraph::new(8);
		assert_eq!(8, g.v_size());
		g.add_edge(&Edge::new(0, 1, 0.5));
		g.add_edge(&Edge::new(2, 1, 0.6));
		g.add_edge(&Edge::new(1, 3, 0.5));

		g.add_edge(&Edge::new(4, 5, 0.5));

		let a = g.adj(0).map(|e| e.other(0)).collect::<Vec<_>>();
		assert_eq!(vec![1], a);

		let a = g.adj(1).map(|e| e.other(1)).collect::<Vec<_>>();
		assert_eq!(vec![0, 2, 3], a);

		let a = g.adj(2).map(|e| e.other(2)).collect::<Vec<_>>();
		assert_eq!(vec![1], a);

		let a = g.adj(3).map(|e| e.other(3)).collect::<Vec<_>>();
		assert_eq!(vec![1], a);

		let a = g.adj(4).map(|e| e.other(4)).collect::<Vec<_>>();
		assert_eq!(vec![5], a);

		let a = g.adj(5).map(|e| e.other(5)).collect::<Vec<_>>();
		assert_eq!(vec![4], a);
	}
}
