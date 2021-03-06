pub type Vertex = usize;

// Edges
// should work like values (Copy)
pub trait Edge: Copy {
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
	fn reversed(&self) -> Self;
}
// impl<E: Directed> !Undirected for E {}
// impl<E: Undirected> !Directed for E {}

pub trait Weighted: Edge + PartialOrd {
	fn weight(&self) -> f64;
}

pub trait NonNegative: Weighted {}

// Graphs
pub trait Graph {
	type Edge: Edge;

	fn v_size(&self) -> usize;
	fn e_size(&self) -> usize;
	// fn adj(&self, v: usize) -> impl Iterator<Item = Self::Edge> + '_;
	fn adj(&self, v: usize) -> Box<dyn Iterator<Item = Self::Edge> + '_>;

	// same edge(v,w) repeated for v and w with undirected graphs
	fn edges(&self) -> Box<dyn Iterator<Item = Self::Edge> + '_> {
		Box::new((0..self.v_size()).map(move |v| self.adj(v)).flatten())
	}
}
pub trait MutableGraph: Graph {
	fn new(v: usize) -> Self;
	fn add_edge(&mut self, edge: &Self::Edge);
}
pub trait Acyclic: Graph + Sized {
	type Graph: Graph;

	fn new(g: Self::Graph) -> Option<Self>;
	fn topo_order(&self) -> std::vec::IntoIter<Vertex>;
}
