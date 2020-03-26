use super::base::*;
use super::dfs_order::*;
use super::DirectedGraph;

pub trait HasSCC {
    fn scc(&self) -> SCC;
}
// only applied to directed graphs
impl<G> HasSCC for G
where
    G::Edge: Directed,
    G: Graph,
{
    fn scc(&self) -> SCC {
        SCC::new(self)
    }
}

// KosarajuShrir strongly-connected components
pub struct SCC {
    ids: Vec<usize>,
    sizes: Vec<usize>,
}
impl SCC {
    pub fn count(&self) -> usize {
        self.sizes.len()
    }
    pub fn size(&self, v: usize) -> usize {
        self.sizes[self.ids[v]]
    }

    pub fn id(&self, v: usize) -> usize {
        self.ids[v]
    }
    pub fn connected(&self, v: usize, w: usize) -> bool {
        self.ids[v] == self.ids[w]
    }
}

// private methods
impl SCC {
    fn new<G, E: Directed>(g: &G) -> Self
    where
        G: Graph<Edge = E>,
    {
        let mut c = SCC {
            ids: vec![0; g.v_size()],
            sizes: Vec::new(),
        };
        c.init(g);
        c
    }

    fn init<G, E: Directed>(&mut self, g: &G)
    where
        G: Graph<Edge = E>,
    {
        let mut dg = DirectedGraph::new(g.v_size());
        for v in 0..g.v_size() {
            for e in g.adj(v) {
                dg.add_edge(e.to(), e.from());
            }
        }

        let mut marked = vec![false; g.v_size()];
        for v in dg.reversed_post_order() {
            if !marked[v] {
                self.sizes.push(0);
                self.dfs(g, v, &mut marked);
            }
        }
    }

    fn dfs<G, E: Directed>(&mut self, g: &G, v: usize, marked: &mut Vec<bool>)
    where
        G: Graph<Edge = E>,
    {
        marked[v] = true;
        let id = self.count() - 1;
        self.ids[v] = id;
        self.sizes[id] += 1;
        for w in g.adj(v).map(|e| e.to()) {
            if !marked[w] {
                self.dfs(g, w, marked);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::DirectedGraph;
    use super::*;

    #[test]
    fn empty() {
        let g = DirectedGraph::new(3);
        let c = g.scc();
        assert_eq!(3, c.count());
        assert_eq!(2, c.id(0));
        assert_eq!(1, c.id(1));
        assert_eq!(0, c.id(2));

        assert_eq!(1, c.size(0));
        assert_eq!(1, c.size(1));
        assert_eq!(1, c.size(2));
    }

    #[test]
    fn connected() {
        let mut g = DirectedGraph::new(8);
        // 0, (1,2,3) -> 4 -> (5,6) -> 7
        g.add_edge(1, 2);
        g.add_edge(2, 3);
        g.add_edge(3, 1);

        g.add_edge(1, 4);
        g.add_edge(4, 5);
        g.add_edge(5, 6);
        g.add_edge(6, 5);

        g.add_edge(5, 7);

        let c = g.scc();
        assert_eq!(5, c.count());

        assert_eq!(0, c.id(7));
        assert_eq!(1, c.size(7));

        assert_eq!(1, c.id(6));
        assert_eq!(1, c.id(5));
        assert_eq!(2, c.size(5));
        assert!(c.connected(5, 6));
        assert!(c.connected(6, 5));

        assert_eq!(2, c.id(4));
        assert_eq!(1, c.size(4));

        assert_eq!(3, c.id(3));
        assert_eq!(3, c.id(2));
        assert_eq!(3, c.id(1));
        assert_eq!(3, c.size(2));
        assert!(c.connected(1, 3));
        assert!(c.connected(1, 2));
        assert!(c.connected(3, 2));

        assert_eq!(4, c.id(0));
        assert_eq!(1, c.size(0));

        assert!(!c.connected(1, 0));
        assert!(!c.connected(1, 4));
        assert!(!c.connected(4, 5));
        assert!(!c.connected(5, 7));
    }
}
