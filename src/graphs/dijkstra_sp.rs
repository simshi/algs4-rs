use super::super::sorting::IndexMinPQ;
use super::edge_weighted_digraph::{DirectedEdge, EdgeWeightedDirectedGraph};

pub struct DijkstraSP {
	edge_to: Vec<Option<DirectedEdge>>,
	dist_to_: Vec<f64>,
}
impl DijkstraSP {
	pub fn new(g: &EdgeWeightedDirectedGraph, s: usize) -> Self {
		let mut sp = DijkstraSP {
			edge_to: vec![None; g.v_size()],
			dist_to_: vec![std::f64::INFINITY; g.v_size()],
		};
		sp.dijkstra(g, s);

		sp
	}

	pub fn has_path_to(&self, v: usize) -> bool {
		self.dist_to_[v] < std::f64::INFINITY
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

	fn dijkstra(&mut self, g: &EdgeWeightedDirectedGraph, s: usize) {
		let mut pq = IndexMinPQ::new(g.v_size());
		pq.upsert(s, 0.0);
		self.dist_to_[s] = 0.0;
		while let Some((v, _)) = pq.pop() {
			for edge in g.adj(v) {
				self.relax(edge, &mut pq);
			}
		}
	}

	fn relax(&mut self, e: &DirectedEdge, pq: &mut IndexMinPQ<f64>) {
		let v = e.from();
		let w = e.to();
		if self.dist_to_[v] + e.weight() < self.dist_to_[w] {
			self.edge_to[w] = Some(e.clone());
			self.dist_to_[w] = self.dist_to_[v] + e.weight();
			pq.upsert(w, self.dist_to_[w]);
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

		let sp = DijkstraSP::new(&g, 0);
		assert_eq!(0, sp.dist_to(0).round() as usize);
	}

	#[test]
	fn one_edge() {
		let mut g = EdgeWeightedDirectedGraph::new(3);
		g.add_edge(&DirectedEdge::new(0, 1, 1.0));

		let sp = DijkstraSP::new(&g, 0);

		assert_eq!(None, sp.path_to(0).next());
		let a = sp.path_to(1).map(|e| e.to()).collect::<Vec<_>>();
		assert_eq!(vec![1], a);
		assert_eq!(None, sp.path_to(2).next());

		assert_eq!(0, sp.dist_to(0).round() as usize);
		assert_eq!(1, sp.dist_to(1).round() as usize);
		assert_eq!(0, sp.dist_to(2).round() as usize);
	}

	#[test]
	fn tiny_ewd() {
		let ewd = vec![
			(4, 5, 0.35),
			(5, 4, 0.35),
			(4, 7, 0.37),
			(5, 7, 0.28),
			(7, 5, 0.28),
			(5, 1, 0.32),
			(0, 4, 0.38),
			(0, 2, 0.26),
			(7, 3, 0.39),
			(1, 3, 0.29),
			(2, 7, 0.34),
			(6, 2, 0.40),
			(3, 6, 0.52),
			(6, 0, 0.58),
			(6, 4, 0.93),
		];
		let mut g = EdgeWeightedDirectedGraph::new(ewd.len());
		for e in ewd {
			g.add_edge(&DirectedEdge::new(e.0, e.1, e.2));
		}

		let sp = DijkstraSP::new(&g, 0);

		assert_eq!(None, sp.path_to(0).next());
		assert_eq!(0, (sp.dist_to(0) * 100.0).round() as usize);

		assert_eq!(
			vec![0, 4, 5],
			sp.path_to(1).map(|e| e.from()).collect::<Vec<_>>()
		);
		assert_eq!(105, (sp.dist_to(1) * 100.0).round() as usize);

		assert_eq!(vec![0], sp.path_to(2).map(|e| e.from()).collect::<Vec<_>>());
		assert_eq!(26, (sp.dist_to(2) * 100.0).round() as usize);

		assert_eq!(
			vec![0, 2, 7],
			sp.path_to(3).map(|e| e.from()).collect::<Vec<_>>()
		);
		assert_eq!(99, (sp.dist_to(3) * 100.0).round() as usize);

		assert_eq!(vec![0], sp.path_to(4).map(|e| e.from()).collect::<Vec<_>>());
		assert_eq!(38, (sp.dist_to(4) * 100.0).round() as usize);

		assert_eq!(
			vec![0, 4],
			sp.path_to(5).map(|e| e.from()).collect::<Vec<_>>()
		);
		assert_eq!(73, (sp.dist_to(5) * 100.0).round() as usize);

		assert_eq!(
			vec![0, 2, 7, 3],
			sp.path_to(6).map(|e| e.from()).collect::<Vec<_>>()
		);
		assert_eq!(151, (sp.dist_to(6) * 100.0).round() as usize);

		assert_eq!(
			vec![0, 2],
			sp.path_to(7).map(|e| e.from()).collect::<Vec<_>>()
		);
		assert_eq!(60, (sp.dist_to(7) * 100.0).round() as usize);
	}
}
