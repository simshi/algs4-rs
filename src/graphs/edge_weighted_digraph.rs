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
	// valid if and only if it's acyclic
	pub fn topo_order(&self) -> TopoOrderIter {
		TopoOrderIter::new(&self)
	}
}

pub struct TopoOrderIter {
	order: Vec<usize>,
}
impl TopoOrderIter {
	pub fn new(g: &EdgeWeightedDigraph) -> Self {
		let mut order = Vec::new();
		post_order(&mut order, g);

		TopoOrderIter { order }
	}
}
impl Iterator for TopoOrderIter {
	type Item = usize;

	fn next(&mut self) -> Option<Self::Item> {
		self.order.pop()
	}
}

// post_order helper functions
fn post_order(order: &mut Vec<usize>, g: &EdgeWeightedDigraph) {
	let mut marked = vec![false; g.v_size()];
	for v in 0..g.v_size() {
		post_order_dfs(order, g, &mut marked, v);
	}
}
fn post_order_dfs(
	order: &mut Vec<usize>,
	g: &EdgeWeightedDigraph,
	marked: &mut Vec<bool>,
	v: usize,
) {
	if marked[v] {
		return;
	}

	marked[v] = true;
	for &e in g.adj(v) {
		post_order_dfs(order, g, marked, e.to());
	}
	order.push(v);
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

	#[test]
	fn topo_order() {
		let ewdag = vec![(0, 1), (1, 2), (1, 3), (3, 4)];
		let mut g = EdgeWeightedDigraph::new(5);
		for e in ewdag {
			g.add_edge(&DirectedEdge::new(e.0, e.1, 0.1));
		}

		let r = g.topo_order().collect::<Vec<_>>();
		assert_eq!(5, r.len());
		if r[2] == 2 {
			assert_eq!(vec![0, 1, 2, 3, 4], r);
		} else {
			assert_eq!(vec![0, 1, 3, 4, 2], r);
		}
	}
}
