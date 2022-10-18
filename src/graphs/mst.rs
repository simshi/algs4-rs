use super::super::sorting::{IndexMinPQ, MinPQ};
use super::base::*;
use super::union_find::UnionFind;

/// Minimum spanning tree
///
/// A spanning tree connected all vertices in a weighted undirected graph with
/// minimum edges.map(_.weight()).sum(), it must has V-1 edges
pub trait MST {
    type Edge: Undirected + Weighted;

    fn kruskal_mst(&self) -> KruskalMST<Self::Edge>;
    fn prim_mst(&self) -> PrimMST<Self::Edge>;
}
// MST implementation for all undirect weighted graphs
impl<G, E> MST for G
where
    E: Undirected + Weighted,
    G: Graph<Edge = E>,
{
    type Edge = G::Edge;

    fn kruskal_mst(&self) -> KruskalMST<E> {
        KruskalMST::new(self)
    }
    fn prim_mst(&self) -> PrimMST<E> {
        PrimMST::new(self)
    }
}

/// Kruskal algorithm for MST
///
/// An edge-based algorithm:
/// build a IndexMinPQ of all edges, for each poped edge, check whether (v,w)
/// was alread connected in mst by `Union Find`, if not, add it into mst, break
/// if V-1 edge is in mst
pub struct KruskalMST<E: Undirected + Weighted> {
    edges_: Vec<E>,
    weight: f64,
}

impl<E> KruskalMST<E>
where
    E: Undirected + Weighted,
{
    fn new<G>(g: &G) -> Self
    where
        G: Graph<Edge = E>,
    {
        let mut t = KruskalMST {
            edges_: Vec::new(),
            weight: 0.0,
        };
        t.kruskal(g);
        t.weight = t.edges().map(|e| e.weight()).sum();

        t
    }

    pub fn weight(&self) -> f64 {
        self.weight
    }
    pub fn edges(&self) -> impl Iterator<Item = &E> {
        self.edges_.iter()
    }

    fn kruskal<G>(&mut self, g: &G)
    where
        G: Graph<Edge = E>,
    {
        let mut pq = MinPQ::new();
        for e in g.edges() {
            pq.push(e);
        }
        let mut uf = UnionFind::new(g.v_size());
        while let Some(e) = pq.pop() {
            let (v, w) = e.vertices();
            if uf.connected(v, w).unwrap() {
                continue;
            }
            uf.union(v, w);
            self.edges_.push(e);
            if self.edges_.len() == g.v_size() - 1 {
                break;
            }
        }
    }
}

/// Eager Prim algorithm for MST
///
/// A vertex-based algorithm:
/// for each edge connected to current mst, inspect the minimum edge by MinPQ,
/// upsert w with minimum weighted edge into mst.
pub struct PrimMST<E: Undirected + Weighted> {
    edge_to: Vec<Option<E>>,
    weight_: f64,
}

impl<E> PrimMST<E>
where
    E: Undirected + Weighted,
{
    pub fn new<G>(g: &G) -> Self
    where
        G: Graph<Edge = E>,
    {
        let mut t = PrimMST {
            edge_to: vec![None; g.v_size()],
            weight_: 0.0,
        };
        // eager approach, at most V-1 vertices in PQ
        let mut pq = IndexMinPQ::new(g.v_size());
        let mut marked = vec![false; g.v_size()];
        // handle forest
        for v in 0..g.v_size() {
            t.prim(v, g, &mut pq, &mut marked);
        }
        t.weight_ = t.edges().map(|e| e.weight()).sum();

        t
    }

    pub fn weight(&self) -> f64 {
        self.weight_
    }
    pub fn edges(&self) -> impl Iterator<Item = E> + '_ {
        self.edge_to.iter().filter_map(|e| *e)
    }

    fn prim<G>(&mut self, v: usize, g: &G, pq: &mut IndexMinPQ<f64>, marked: &mut [bool])
    where
        G: Graph<Edge = E>,
    {
        if marked[v] {
            return;
        }
        pq.upsert(v, 0.0);
        while let Some((v, _)) = pq.pop() {
            self.scan(v, g, pq, marked);
        }
    }

    fn scan<G>(&mut self, v: usize, g: &G, pq: &mut IndexMinPQ<f64>, marked: &mut [bool])
    where
        G: Graph<Edge = E>,
    {
        marked[v] = true;
        for e in g.adj(v) {
            let w = e.other(v);
            if marked[w] {
                continue;
            }

            // update w to the cheaper edge
            let cheaper = pq.get(w).map_or(true, |&w| e.weight() < w);
            if cheaper {
                pq.upsert(w, e.weight());
                self.edge_to[w] = Some(e);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::EdgeWeightedUndirectedGraph as EWG;
    use super::super::WeightedUndirectedEdge as WE;
    use super::*;

    #[test]
    fn empty() {
        let g = EWG::new(0);
        assert_eq!(0, g.v_size());

        let t = g.kruskal_mst();
        assert_eq!(0, (t.weight() * 1000.0) as isize);
        assert_eq!(None, t.edges().next());

        let t = g.prim_mst();
        assert_eq!(0, (t.weight() * 1000.0) as isize);
        assert_eq!(None, t.edges().next());
    }

    #[test]
    fn one_edge() {
        let mut g = EWG::new(2);
        g.add_edge(&WE::new(0, 1, 0.5));

        let t = g.kruskal_mst();
        assert_eq!(5, (t.weight() * 10.0).round() as isize);
        let edges = t.edges().collect::<Vec<_>>();
        assert_eq!(1, edges.len());

        assert_eq!(1, edges[0].other(0));
        assert_eq!(0, edges[0].other(1));

        let t = g.prim_mst();
        assert_eq!(5, (t.weight() * 10.0).round() as isize);
        let edges = t.edges().collect::<Vec<_>>();
        assert_eq!(1, edges.len());

        assert_eq!(1, edges[0].other(0));
        assert_eq!(0, edges[0].other(1));
    }

    #[test]
    fn multiple_edges() {
        // as the example in the book
        let mut g = EWG::new(8);
        g.add_edge(&WE::new(0, 2, 0.26));
        g.add_edge(&WE::new(0, 4, 0.38));
        g.add_edge(&WE::new(0, 7, 0.16));
        g.add_edge(&WE::new(1, 2, 0.36));
        g.add_edge(&WE::new(1, 3, 0.29));
        g.add_edge(&WE::new(1, 5, 0.32));
        g.add_edge(&WE::new(1, 7, 0.19));
        g.add_edge(&WE::new(2, 3, 0.17));
        g.add_edge(&WE::new(2, 7, 0.34));
        g.add_edge(&WE::new(3, 6, 0.52));
        g.add_edge(&WE::new(4, 5, 0.35));
        g.add_edge(&WE::new(4, 7, 0.37));
        g.add_edge(&WE::new(5, 7, 0.28));
        g.add_edge(&WE::new(6, 0, 0.58));
        g.add_edge(&WE::new(6, 2, 0.40));
        g.add_edge(&WE::new(6, 4, 0.93));

        let t = g.kruskal_mst();
        let mut paths = t.edges().map(|e| e.vertices()).collect::<Vec<_>>();
        paths.sort();
        assert_eq!(
            vec![(0, 2), (0, 7), (1, 7), (2, 3), (4, 5), (5, 7), (6, 2)],
            paths
        );
        assert_eq!(181, (t.weight() * 100.0).round() as isize);

        let t = g.prim_mst();
        let mut paths = t.edges().map(|e| e.vertices()).collect::<Vec<_>>();
        paths.sort();
        assert_eq!(
            vec![(0, 2), (0, 7), (1, 7), (2, 3), (4, 5), (5, 7), (6, 2)],
            paths
        );
        assert_eq!(181, (t.weight() * 100.0).round() as isize);
    }
}
