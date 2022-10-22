use super::base::*;

/// Reversed
///
/// generates a reversed graph from a directed graph
pub trait Reversed: Graph {
    // TODO: associated type bounds are unstable
    // pub trait Reversed: Graph<Edge: Directed> {
    fn reversed(&self) -> Self;
}
// implement Reversed for all mutuable directed graphs
impl<G, E> Reversed for G
where
    E: Directed,
    G: MutableGraph<Edge = E>,
{
    fn reversed(&self) -> Self {
        let mut gr = Self::new(self.v_size());
        for i in 0..self.v_size() {
            for e in self.adj(i) {
                gr.add_edge(e.reversed());
            }
        }
        gr
    }
}

#[cfg(test)]
mod tests {
    use super::super::DirectedGraph;
    use super::*;

    #[test]
    fn empty() {
        let g = DirectedGraph::new(1);
        let g = g.reversed();
        assert_eq!(1, g.v_size());
        assert_eq!(None, g.adj(0).next());
    }

    #[test]
    fn reversed() {
        let mut g = DirectedGraph::new(5);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(1, 3);
        g.add_edge(2, 3);
        g.add_edge(3, 4);

        let g = g.reversed();

        assert_eq!(None, g.adj(0).next());

        let mut a = g.adj(1).map(|e| e.to()).collect::<Vec<_>>();
        a.sort_unstable();
        assert_eq!(vec![0], a);

        let mut a = g.adj(2).map(|e| e.to()).collect::<Vec<_>>();
        a.sort_unstable();
        assert_eq!(vec![1], a);

        let mut a = g.adj(3).map(|e| e.to()).collect::<Vec<_>>();
        a.sort_unstable();
        assert_eq!(vec![1, 2], a);

        let mut a = g.adj(4).map(|e| e.to()).collect::<Vec<_>>();
        a.sort_unstable();
        assert_eq!(vec![3], a);
    }
}
