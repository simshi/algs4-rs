use super::base::*;

/// Reversed
///
/// A directed edge can be reversed
pub trait Reversed<E: Directed> {
    fn reversed(&self) -> Self;
}

// Any directed graph can be reversed by reversing all its edges.
impl<G, E: Directed> Reversed<E> for G
where
    G: MutableGraph<Edge = E>,
{
    fn reversed(&self) -> Self {
        let mut r = Self::new(self.v_size());
        for v in 0..self.v_size() {
            for e in self.adj(v) {
                r.add_edge(&e.reversed());
            }
        }
        r
    }
}

#[cfg(test)]
mod tests {
    use super::super::DirectedGraph;
    use super::*;

    #[test]
    fn empty() {
        let g = DirectedGraph::new(1);
        let r = g.reversed();
        assert_eq!(1, r.v_size());
        assert_eq!(None, r.adj(0).next());
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
