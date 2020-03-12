use super::edge_weighted_digraph::{DirectedEdge, EdgeWeightedDirectedGraph};

pub struct AcyclicLP {
	edge_to: Vec<Option<DirectedEdge>>,
	dist_to_: Vec<f64>,
}
impl AcyclicLP {
	pub fn new(g: &EdgeWeightedDirectedGraph, s: usize) -> Self {
		let mut sp = AcyclicLP {
			edge_to: vec![None; g.v_size()],
			dist_to_: vec![std::f64::NEG_INFINITY; g.v_size()],
		};
		sp.acyclic(g, s);

		sp
	}

	pub fn has_path_to(&self, v: usize) -> bool {
		self.dist_to_[v] > std::f64::NEG_INFINITY
	}
	pub fn dist_to(&self, v: usize) -> f64 {
		self.dist_to_[v]
	}
	pub fn path_to(&self, v: usize) -> impl Iterator<Item = DirectedEdge> {
		let mut q = Vec::new();
		let mut edge = self.edge_to[v];
		while let Some(e) = edge {
			q.push(e.clone());
			edge = self.edge_to[e.from()];
		}

		q.into_iter().rev()
	}

	fn acyclic(&mut self, g: &EdgeWeightedDirectedGraph, s: usize) {
		self.dist_to_[s] = 0.0;
		for v in g.topo_order() {
			for edge in g.adj(v) {
				self.relax(edge);
			}
		}
	}

	fn relax(&mut self, e: &DirectedEdge) {
		let v = e.from();
		let w = e.to();
		if self.dist_to_[v] + e.weight() > self.dist_to_[w] {
			self.edge_to[w] = Some(e.clone());
			self.dist_to_[w] = self.dist_to_[v] + e.weight();
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn empty() {
		let g = EdgeWeightedDirectedGraph::new(1);
		assert_eq!(1, g.v_size());

		let sp = AcyclicLP::new(&g, 0);
		assert_eq!(0, sp.dist_to(0).round() as usize);
	}

	#[test]
	fn one_edge() {
		let mut g = EdgeWeightedDirectedGraph::new(3);
		g.add_edge(&DirectedEdge::new(0, 1, 1.0));

		let sp = AcyclicLP::new(&g, 0);

		assert_eq!(None, sp.path_to(0).next());
		let a = sp.path_to(1).map(|e| e.to()).collect::<Vec<_>>();
		assert_eq!(vec![1], a);
		assert_eq!(None, sp.path_to(2).next());

		assert_eq!(0, sp.dist_to(0).round() as usize);
		assert_eq!(1, sp.dist_to(1).round() as usize);
		assert!(!sp.has_path_to(2));
	}

	#[test]
	fn tiny_ewdag() {
		let ewdag = vec![
			(5, 4, 0.35),
			(4, 7, 0.37),
			(5, 7, 0.28),
			(5, 1, 0.32),
			(4, 0, 0.38),
			(0, 2, 0.26),
			(3, 7, 0.39),
			(1, 3, 0.29),
			(7, 2, 0.34),
			(6, 2, 0.40),
			(3, 6, 0.52),
			(6, 0, 0.58),
			(6, 4, 0.93),
		];
		let mut g = EdgeWeightedDirectedGraph::new(ewdag.len());
		for e in ewdag {
			g.add_edge(&DirectedEdge::new(e.0, e.1, e.2));
		}

		let sp = AcyclicLP::new(&g, 5);

		assert_eq!(
			vec![5, 1, 3, 6, 4],
			sp.path_to(0).map(|e| e.from()).collect::<Vec<_>>()
		);
		assert_eq!(244, (sp.dist_to(0) * 100.0).round() as usize);

		assert_eq!(vec![5], sp.path_to(1).map(|e| e.from()).collect::<Vec<_>>());
		assert_eq!(32, (sp.dist_to(1) * 100.0).round() as usize);

		assert_eq!(
			vec![5, 1, 3, 6, 4, 7],
			sp.path_to(2).map(|e| e.from()).collect::<Vec<_>>()
		);
		assert_eq!(277, (sp.dist_to(2) * 100.0).round() as usize);

		assert_eq!(
			vec![5, 1],
			sp.path_to(3).map(|e| e.from()).collect::<Vec<_>>()
		);
		assert_eq!(61, (sp.dist_to(3) * 100.0).round() as usize);

		assert_eq!(
			vec![5, 1, 3, 6],
			sp.path_to(4).map(|e| e.from()).collect::<Vec<_>>()
		);
		assert_eq!(206, (sp.dist_to(4) * 100.0).round() as usize);

		assert_eq!(None, sp.path_to(5).next());
		assert_eq!(0, (sp.dist_to(5) * 100.0).round() as usize);

		assert_eq!(
			vec![5, 1, 3],
			sp.path_to(6).map(|e| e.from()).collect::<Vec<_>>()
		);
		assert_eq!(113, (sp.dist_to(6) * 100.0).round() as usize);

		assert_eq!(
			vec![5, 1, 3, 6, 4],
			sp.path_to(7).map(|e| e.from()).collect::<Vec<_>>()
		);
		assert_eq!(243, (sp.dist_to(7) * 100.0).round() as usize);
	}
}
