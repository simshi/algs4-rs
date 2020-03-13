use super::base::*;

// use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub struct UndirectedEdge {
	v: usize,
	w: usize,
}
impl UndirectedEdge {
	pub fn new(v: usize, w: usize) -> Self {
		UndirectedEdge { v, w }
	}
}
impl Edge for UndirectedEdge {
	fn vertices(&self) -> (Vertex, Vertex) {
		(self.v, self.w)
	}
}
impl Undirected for UndirectedEdge {}

#[derive(Debug, PartialEq)]
pub struct DirectedEdge {
	v: usize,
	w: usize,
}
impl DirectedEdge {
	pub fn new(v: usize, w: usize) -> Self {
		DirectedEdge { v, w }
	}
}
impl Edge for DirectedEdge {
	fn vertices(&self) -> (Vertex, Vertex) {
		(self.v, self.w)
	}
}
impl Directed for DirectedEdge {
	fn reversed(&self) -> Self {
		Self::new(self.to(), self.from())
	}
}

// impl<E: Weighted> PartialOrd for E {
// 	fn partial_cmp(&self, other: &E) -> Option<Ordering> {
// 		self.weight().partial_cmp(&other.weight())
// 	}
// }

#[derive(Debug, Clone, PartialEq)]
pub struct WeightedUndirectedEdge {
	v: usize,
	w: usize,
	weight: f64,
}
impl WeightedUndirectedEdge {
	pub fn new(v: usize, w: usize, weight: f64) -> Self {
		WeightedUndirectedEdge { v, w, weight }
	}
}
impl Edge for WeightedUndirectedEdge {
	fn vertices(&self) -> (Vertex, Vertex) {
		(self.v, self.w)
	}
}
impl Undirected for WeightedUndirectedEdge {}
impl Weighted for WeightedUndirectedEdge {
	fn weight(&self) -> f64 {
		self.weight
	}
}
