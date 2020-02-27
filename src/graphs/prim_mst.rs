use super::super::sorting::IndexMinPQ;
use super::edge_weighted_graph::{Edge, EdgeWeightedGraph};

// minimum spanning tree by Eager Prim algorithm
pub struct PrimMST {
	edge_to: Vec<Option<Edge>>,
	weight: f64,
}

impl PrimMST {
	pub fn new(g: &EdgeWeightedGraph) -> Self {
		let mut t = PrimMST {
			edge_to: vec![None; g.v_size()],
			weight: 0.0,
		};
		// eager approach, at most V-1 vertices in PQ
		let mut pq = IndexMinPQ::new(g.v_size());
		let mut marked = vec![false; g.v_size()];
		// handle forest
		for v in 0..g.v_size() {
			t.prim(v, g, &mut pq, &mut marked);
		}
		for e in t.edge_to.iter() {
			t.weight += e.map_or(0.0, |e| e.weight());
		}

		t
	}

	pub fn weight(&self) -> f64 {
		self.weight
	}
	pub fn edges(&self) -> impl Iterator<Item = Edge> + '_ {
		self.edge_to.iter().filter_map(|e| *e)
	}

	fn prim(
		&mut self,
		v: usize,
		g: &EdgeWeightedGraph,
		pq: &mut IndexMinPQ<f64>,
		marked: &mut Vec<bool>,
	) {
		if marked[v] {
			return;
		}
		pq.upsert(v, 0.0);
		while let Some((v, _)) = pq.pop() {
			self.scan(v, g, pq, marked);
		}
	}

	fn scan(
		&mut self,
		v: usize,
		g: &EdgeWeightedGraph,
		pq: &mut IndexMinPQ<f64>,
		marked: &mut Vec<bool>,
	) {
		marked[v] = true;
		for e in g.adj(v) {
			let w = e.other(v);
			if marked[w] {
				continue;
			}

			// update w to the cheaper edge
			let cheaper = match pq.get(w) {
				Some(weight) => e.weight() < *weight,
				_ => true,
			};
			if cheaper {
				self.edge_to[w] = Some(e.clone());
				pq.upsert(w, e.weight());
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn empty() {
		let g = EdgeWeightedGraph::new(0);
		assert_eq!(0, g.v_size());

		let t = PrimMST::new(&g);
		assert_eq!(0, (t.weight() * 1000.0) as isize);
		assert_eq!(None, t.edges().next());
	}

	#[test]
	fn one_edge() {
		let mut g = EdgeWeightedGraph::new(2);
		g.add_edge(&Edge::new(0, 1, 0.5));

		let t = PrimMST::new(&g);
		assert_eq!(5, (t.weight() * 10.0) as isize);
		let edges = t.edges().collect::<Vec<_>>();
		assert_eq!(1, edges.len());

		assert_eq!(1, edges[0].other(0));
		assert_eq!(0, edges[0].other(1));
	}

	#[test]
	fn multiple_edges() {
		// as the example in the book
		let mut g = EdgeWeightedGraph::new(8);
		g.add_edge(&Edge::new(0, 2, 0.26));
		g.add_edge(&Edge::new(0, 4, 0.38));
		g.add_edge(&Edge::new(0, 7, 0.16));
		g.add_edge(&Edge::new(1, 2, 0.36));
		g.add_edge(&Edge::new(1, 3, 0.29));
		g.add_edge(&Edge::new(1, 5, 0.32));
		g.add_edge(&Edge::new(1, 7, 0.19));
		g.add_edge(&Edge::new(2, 3, 0.17));
		g.add_edge(&Edge::new(2, 7, 0.34));
		g.add_edge(&Edge::new(3, 6, 0.52));
		g.add_edge(&Edge::new(4, 5, 0.35));
		g.add_edge(&Edge::new(4, 7, 0.37));
		g.add_edge(&Edge::new(5, 7, 0.28));
		g.add_edge(&Edge::new(6, 0, 0.58));
		g.add_edge(&Edge::new(6, 2, 0.40));
		g.add_edge(&Edge::new(6, 4, 0.93));

		let t = PrimMST::new(&g);

		assert_eq!(180, (t.weight() * 100.0) as isize);
		let s = t.edges().map(|e| e.weight()).sum::<f64>();
		assert_eq!(180, (s * 100.0) as isize);

		let paths = t
			.edges()
			.enumerate()
			.map(|(i, e)| e.other(i + 1))
			.collect::<Vec<_>>();
		// i.e. 1->7, 2->0, 3->2, ..., 7->0
		assert_eq!(vec![7, 0, 2, 5, 7, 2, 0], paths);
	}
}
