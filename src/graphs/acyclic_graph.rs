use super::base::*;
use super::cycle::*;
use super::dfs_order::*;
use super::edge::*;
use super::EdgeWeightedDirectedGraph;

pub struct EdgeWeightedDAG<G>
where
	G: Graph<Edge = WeightedDirectedEdge>,
{
	g: G,
}
impl<G> Graph for EdgeWeightedDAG<G>
where
	G: Graph<Edge = WeightedDirectedEdge>,
{
	type Edge = WeightedDirectedEdge;

	fn v_size(&self) -> usize {
		self.g.v_size()
	}
	fn e_size(&self) -> usize {
		self.g.e_size()
	}

	fn adj(&self, v: usize) -> Box<dyn Iterator<Item = Self::Edge> + '_> {
		self.g.adj(v)
	}
}
impl Acyclic for EdgeWeightedDAG<EdgeWeightedDirectedGraph> {
	type Graph = EdgeWeightedDirectedGraph;

	fn new(g: Self::Graph) -> Option<Self> {
		if CycleDetection::detect_directed(&g).is_some() {
			return None;
		}

		Some(Self { g })
	}
	fn topo_order(&self) -> std::vec::IntoIter<Vertex> {
		self.g.reversed_post_order()
	}
}
