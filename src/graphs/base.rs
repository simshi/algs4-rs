pub type Vertex = usize;

pub trait Edge {
	fn vertices(&self) -> (Vertex, Vertex);
	fn other(&self, v: Vertex) -> Vertex;
}
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

pub trait Graph {
	type Edge: Edge;

	fn v_size(&self) -> usize;
	// fn adj<'a>(&'a self, v: usize) -> impl Iterator<Item = Self::Edge> + 'a;
	fn adj<'a>(&'a self, v: usize) -> Box<dyn Iterator<Item = Self::Edge> + 'a>;
}
pub trait Acyclic: Graph {}

// }
// // combinations
// trait DG<E>: Graph
// where
// 	E: Directed,
// {
// 	type Adjacency = E;

// 	type Iter: Iterator<Item = &E>;
// 	fn pre_order(&self) -> Self::Iter;
// }

// trait DAG<E>: Acyclic
// where
// 	E: Directed,
// {
// 	type Adjacency = E;
// 	type Iter: Iterator<Item = &E>;

// 	fn topo_order(&self) -> Self::Iter;
// }
