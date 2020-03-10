use super::base::*;

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
impl Directed for DirectedEdge {}
