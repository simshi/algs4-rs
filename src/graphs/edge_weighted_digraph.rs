use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DirectedEdge {
	v: usize,
	w: usize,
	weight: f64,
}
impl DirectedEdge {
	pub fn new(v: usize, w: usize, weight: f64) -> Self {
		DirectedEdge { v, w, weight }
	}

	pub fn from(&self) -> usize {
		self.v
	}
	pub fn to(&self) -> usize {
		self.w
	}
	pub fn weight(&self) -> f64 {
		self.weight
	}
}
impl PartialOrd for DirectedEdge {
	fn partial_cmp(&self, to: &DirectedEdge) -> Option<Ordering> {
		self.weight.partial_cmp(&to.weight)
	}
}

pub struct EdgeWeightedDigraph {
	v: usize,
	e: usize,
	adj: Vec<Vec<DirectedEdge>>,
}
impl EdgeWeightedDigraph {
	pub fn new(v: usize) -> Self {
		EdgeWeightedDigraph {
			v,
			e: 0,
			adj: vec![Vec::new(); v],
		}
	}

	pub fn edges(&self) -> impl Iterator<Item = &DirectedEdge> {
		self.adj.iter().flatten()
	}

	pub fn add_edge(&mut self, e: &DirectedEdge) {
		let v = e.from();
		self.adj[v].push(e.clone());
		self.e += 1;
	}

	pub fn v_size(&self) -> usize {
		self.v
	}
	pub fn adj(&self, v: usize) -> impl Iterator<Item = &DirectedEdge> {
		self.adj[v].iter()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn empty() {
		let g = EdgeWeightedDigraph::new(1);
		assert_eq!(1, g.v_size());
	}

	#[test]
	fn add_get() {
		let mut g = EdgeWeightedDigraph::new(5);
		assert_eq!(5, g.v_size());
		g.add_edge(&DirectedEdge::new(0, 1, 0.5));

		let mut it = g.adj(0);
		let e = it.next().unwrap();
		assert_eq!(1, e.to());
		assert_eq!(None, it.next());

		let mut it = g.adj(1);
		assert_eq!(None, it.next());
	}

	#[test]
	fn multiple_edges() {
		let mut g = EdgeWeightedDigraph::new(8);
		assert_eq!(8, g.v_size());
		g.add_edge(&DirectedEdge::new(0, 1, 0.5));
		g.add_edge(&DirectedEdge::new(0, 2, 0.26));
		g.add_edge(&DirectedEdge::new(0, 4, 0.38));
		g.add_edge(&DirectedEdge::new(2, 5, 0.5));
		g.add_edge(&DirectedEdge::new(2, 6, 0.6));
		g.add_edge(&DirectedEdge::new(1, 3, 0.5));
		g.add_edge(&DirectedEdge::new(3, 6, 0.52));

		g.add_edge(&DirectedEdge::new(4, 5, 0.5));

		let a = g.adj(0).map(|e| e.to()).collect::<Vec<_>>();
		assert_eq!(vec![1, 2, 4], a);

		let a = g.adj(1).map(|e| e.to()).collect::<Vec<_>>();
		assert_eq!(vec![3], a);

		let a = g.adj(2).map(|e| e.to()).collect::<Vec<_>>();
		assert_eq!(vec![5, 6], a);

		let a = g.adj(3).map(|e| e.to()).collect::<Vec<_>>();
		assert_eq!(vec![6], a);

		let a = g.adj(4).map(|e| e.to()).collect::<Vec<_>>();
		assert_eq!(vec![5], a);

		let a = g.adj(5).next();
		assert_eq!(None, a);
	}
}
