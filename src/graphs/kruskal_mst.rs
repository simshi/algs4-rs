use super::super::sorting::MinPQ;
use super::edge_weighted_graph::{Edge, EdgeWeightedGraph};
use super::union_find::UnionFind;

// minimum spanning tree by Eager Prim algorithm
pub struct KruskalMST {
	edges_: Vec<Edge>,
	weight: f64,
}

impl KruskalMST {
	pub fn new(g: &EdgeWeightedGraph) -> Self {
		let mut t = KruskalMST {
			edges_: Vec::new(),
			weight: 0.0,
		};
		t.kruskal(g);
		t.weight = t.edges().map(|e| e.weight()).sum();

		t
	}

	pub fn weight(&self) -> f64 {
		self.weight
	}
	pub fn edges(&self) -> impl Iterator<Item = &Edge> {
		self.edges_.iter()
	}

	fn kruskal(&mut self, g: &EdgeWeightedGraph) {
		let mut pq = MinPQ::new();
		for e in g.edges() {
			pq.push(e);
		}
		let mut uf = UnionFind::new(g.v_size());
		while let Some(e) = pq.pop() {
			let v = e.either();
			let w = e.other(v);
			if uf.connected(v, w).unwrap() {
				continue;
			}
			uf.union(v, w);
			self.edges_.push(e.clone());
			if self.edges_.len() == g.v_size() - 1 {
				break;
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

		let t = KruskalMST::new(&g);
		assert_eq!(0, (t.weight() * 1000.0) as isize);
		assert_eq!(None, t.edges().next());
	}

	#[test]
	fn one_edge() {
		let mut g = EdgeWeightedGraph::new(2);
		g.add_edge(&Edge::new(0, 1, 0.5));

		let t = KruskalMST::new(&g);
		assert_eq!(5, (t.weight() * 10.0).round() as isize);
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

		let t = KruskalMST::new(&g);

		let mut paths = t
			.edges()
			.map(|e| {
				let v = e.either();
				(v, e.other(v))
			})
			.collect::<Vec<_>>();
		paths.sort();
		assert_eq!(
			vec![(0, 2), (0, 7), (1, 7), (2, 3), (4, 5), (5, 7), (6, 2)],
			paths
		);

		assert_eq!(181, (t.weight() * 100.0).round() as isize);
	}
}
