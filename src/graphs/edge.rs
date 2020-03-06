use super::base::*;

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

	fn other(&self, v: Vertex) -> Vertex {
		if v == self.v {
			self.w
		} else {
			self.v
		}
	}
}
impl Directed for DirectedEdge {}
