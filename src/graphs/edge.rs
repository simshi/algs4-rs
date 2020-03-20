use super::base::*;

use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq)]
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

#[derive(Debug, Clone, Copy, PartialEq)]
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

#[derive(Debug, Clone, Copy, PartialEq)]
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
impl PartialOrd for WeightedUndirectedEdge {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		self.weight().partial_cmp(&other.weight())
	}
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WeightedDirectedEdge {
	v: usize,
	w: usize,
	weight: f64,
}
impl WeightedDirectedEdge {
	pub fn new(v: usize, w: usize, weight: f64) -> Self {
		WeightedDirectedEdge { v, w, weight }
	}
}
impl Edge for WeightedDirectedEdge {
	fn vertices(&self) -> (Vertex, Vertex) {
		(self.v, self.w)
	}
}
impl Directed for WeightedDirectedEdge {
	fn reversed(&self) -> Self {
		Self::new(self.w, self.v, self.weight)
	}
}
impl Weighted for WeightedDirectedEdge {
	fn weight(&self) -> f64 {
		self.weight
	}
}
impl PartialOrd for WeightedDirectedEdge {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		self.weight().partial_cmp(&other.weight())
	}
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NonNegativeWeightedDirectedEdge {
	v: usize,
	w: usize,
	weight: f64,
}
impl NonNegativeWeightedDirectedEdge {
	pub fn new(v: usize, w: usize, weight: f64) -> Option<Self> {
		if weight < 0.0 {
			None
		} else {
			Some(NonNegativeWeightedDirectedEdge { v, w, weight })
		}
	}
}
impl Edge for NonNegativeWeightedDirectedEdge {
	fn vertices(&self) -> (Vertex, Vertex) {
		(self.v, self.w)
	}
}
impl Directed for NonNegativeWeightedDirectedEdge {
	fn reversed(&self) -> Self {
		NonNegativeWeightedDirectedEdge {
			v: self.w,
			w: self.v,
			weight: self.weight,
		}
	}
}
impl Weighted for NonNegativeWeightedDirectedEdge {
	fn weight(&self) -> f64 {
		self.weight
	}
}
impl NonNegative for NonNegativeWeightedDirectedEdge {}
impl PartialOrd for NonNegativeWeightedDirectedEdge {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		self.weight().partial_cmp(&other.weight())
	}
}
