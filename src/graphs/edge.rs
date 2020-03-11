use super::base::*;

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
