use super::base::*;

pub struct WeightedPath<E: Directed + Weighted> {
	pub(super) edge_to: Vec<Option<E>>,
	pub(super) dist_to: Vec<f64>,
}
impl<E> WeightedPath<E>
where
	E: Directed + Weighted,
{
	pub fn new(v: usize, init: f64) -> Self {
		Self {
			edge_to: vec![None; v],
			dist_to: vec![init; v],
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
