use super::super::sorting::IndexMinPQ;
use super::base::*;

pub trait HasDijkstraSP<E>
where
	E: Directed + NonNegative,
{
	fn dijkstra_sp(&self, s: usize) -> SP<E>;
}
// Dijkstra algorithm applied to non-nagative DAG
impl<G, E> HasDijkstraSP<E> for G
where
	E: Directed + NonNegative,
	G: Graph<Edge = E>,
{
	fn dijkstra_sp(&self, s: usize) -> SP<E> {
		let mut p = SP::new(self.v_size());
		dijkstra_sp(&mut p, self, s);
		p
	}
}

pub struct SP<E: Directed> {
	edge_to: Vec<Option<E>>,
	dist_to: Vec<f64>,
}
impl<E> SP<E>
where
	E: Directed + NonNegative,
{
	pub fn new(v: usize) -> Self {
		SP {
			edge_to: vec![None; v],
			dist_to: vec![std::f64::INFINITY; v],
		}
	}

	pub fn has_path_to(&self, v: usize) -> bool {
		self.edge_to[v].is_some()
	}
	pub fn dist_to(&self, v: usize) -> f64 {
		self.dist_to[v]
	}
	pub fn path_to(&self, v: usize) -> impl Iterator<Item = E> {
		let mut q = Vec::new();
		let mut edge = self.edge_to[v];
		while let Some(e) = edge {
			q.push(e);
			edge = self.edge_to[e.from()];
		}

		q.into_iter().rev()
	}
}

fn dijkstra_sp<G, E>(p: &mut SP<E>, g: &G, s: usize)
where
	E: Directed + NonNegative,
	G: Graph<Edge = E>,
{
	let mut pq = IndexMinPQ::new(g.v_size());
	pq.upsert(s, 0.0);
	p.dist_to[s] = 0.0;
	while let Some((v, _)) = pq.pop() {
		for edge in g.adj(v) {
			relax(p, edge, &mut pq);
		}
	}
}

fn relax<E>(p: &mut SP<E>, e: E, pq: &mut IndexMinPQ<f64>)
where
	E: Directed + NonNegative,
{
	let v = e.from();
	let w = e.to();
	if p.dist_to[v] + e.weight() < p.dist_to[w] {
		p.dist_to[w] = p.dist_to[v] + e.weight();
		p.edge_to[w] = Some(e);
		pq.upsert(w, p.dist_to[w]);
	}
}

#[cfg(test)]
mod tests {
	use super::super::EdgeNonNegativeWeightedDirectedGraph as ENNWDG;
	use super::super::NonNegativeWeightedDirectedEdge as NNWDE;
	use super::*;

	#[test]
	fn empty() {
		let g = ENNWDG::new(1);
		let sp = g.dijkstra_sp(0);
		assert_eq!(0, sp.dist_to(0).round() as usize);
	}

	#[test]
	fn one_edge() {
		let mut g = ENNWDG::new(3);
		g.add_edge(&NNWDE::new(0, 1, 1.0).unwrap());

		let sp = g.dijkstra_sp(0);

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
		let mut g = ENNWDG::new(ewd.len());
		for e in ewd {
			g.add_edge(&NNWDE::new(e.0, e.1, e.2).unwrap());
		}

		let sp = g.dijkstra_sp(0);

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
