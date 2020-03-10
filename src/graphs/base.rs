pub type Vertex = usize;

// Edges
pub trait Edge {
	fn vertices(&self) -> (Vertex, Vertex);
	fn other(&self, v: Vertex) -> Vertex {
		let (v1, v2) = self.vertices();
		if v == v1 {
			v2
		} else {
			v1
		}
	}
}

pub trait Undirected: Edge {}

pub trait Directed: Edge {
	fn from(&self) -> Vertex {
		self.vertices().0
	}
	fn to(&self) -> Vertex {
		self.vertices().1
	}
}

pub trait Weighted: Edge {
	fn weight(&self) -> f64;
}

pub trait NonNegative: Weighted {}

// Graphs
pub trait Graph {
	type Edge: Edge;

	fn v_size(&self) -> usize;
	// fn adj<'a>(&'a self, v: usize) -> impl Iterator<Item = Self::Edge> + 'a;
	fn adj<'a>(&'a self, v: usize) -> Box<dyn Iterator<Item = Self::Edge> + 'a>;
}
pub trait Acyclic: Graph {}
