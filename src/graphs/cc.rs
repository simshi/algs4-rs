use super::base::*;

pub trait HasCC {
    fn cc(&self) -> CC;
}
// CC only applied to undirected graphs
impl<'a, G, E: Undirected> HasCC for G
where
    G: Graph<Edge = E>,
{
    fn cc(&self) -> CC {
        CC::new(self)
    }
}
// connected component
pub struct CC {
    ids: Vec<usize>,
    sizes: Vec<usize>,
}
impl CC {
    fn new<G, E: Undirected>(g: &G) -> Self
    where
        G: Graph<Edge = E>,
    {
        let mut c = CC {
            ids: vec![0; g.v_size()],
            sizes: Vec::new(),
        };
        c.init(g);
        c
    }
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
impl CC {
    fn init<G, E: Undirected>(&mut self, g: &G)
    where
        G: Graph<Edge = E>,
    {
        let mut marked = vec![false; g.v_size()];
        for v in 0..g.v_size() {
            if !marked[v] {
                self.sizes.push(0);
                self.dfs(g, v, &mut marked);
            }
        }
    }

    fn dfs<G, E: Undirected>(&mut self, g: &G, v: usize, marked: &mut Vec<bool>)
    where
        G: Graph<Edge = E>,
    {
        marked[v] = true;
        let id = self.count() - 1;
        self.ids[v] = id;
        self.sizes[id] += 1;

        for w in g.adj(v).map(|e| e.other(v)) {
            if !marked[w] {
                self.dfs(g, w, marked)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::edge::WeightedUndirectedEdge as WE;
    use super::super::EdgeWeightedUndirectedGraph as EWG;
    use super::super::UndirectedGraph;
    use super::*;

    #[test]
    fn empty_graph() {
        let g = UndirectedGraph::new(3);
        let c = g.cc();
        assert_eq!(3, c.count());
        assert_eq!(0, c.id(0));
        assert_eq!(1, c.id(1));
        assert_eq!(2, c.id(2));

        assert_eq!(1, c.size(0));
        assert_eq!(1, c.size(1));
        assert_eq!(1, c.size(2));
    }

    #[test]
    fn empty_ewg() {
        let g = EWG::new(3);
        let c = g.cc();
        assert_eq!(3, c.count());
        assert_eq!(0, c.id(0));
        assert_eq!(1, c.id(1));
        assert_eq!(2, c.id(2));

        assert_eq!(1, c.size(0));
        assert_eq!(1, c.size(1));
        assert_eq!(1, c.size(2));
    }

    #[test]
    fn connected_graph() {
        let mut g = UndirectedGraph::new(5);
        g.add_edge(0, 1);
        g.add_edge(1, 2);

        g.add_edge(4, 3);

        let c = g.cc();
        assert_eq!(2, c.count());

        assert_eq!(0, c.id(0));
        assert_eq!(0, c.id(1));
        assert_eq!(0, c.id(2));
        assert_eq!(3, c.size(0));
        assert_eq!(3, c.size(2));

        assert_eq!(1, c.id(3));
        assert_eq!(1, c.id(4));
        assert_eq!(2, c.size(4));

        assert!(c.connected(1, 0));
        assert!(c.connected(1, 2));
        assert!(!c.connected(1, 3));
        assert!(!c.connected(1, 4));
        assert!(c.connected(3, 4));
    }

    #[test]
    fn connected_ewg() {
        let mut g = EWG::new(5);
        g.add_edge(&WE::new(0, 1, 0.5));
        g.add_edge(&WE::new(1, 2, 0.6));

        g.add_edge(&WE::new(4, 3, 0.7));

        let c = g.cc();
        assert_eq!(2, c.count());

        assert_eq!(0, c.id(0));
        assert_eq!(0, c.id(1));
        assert_eq!(0, c.id(2));
        assert_eq!(3, c.size(0));
        assert_eq!(3, c.size(2));

        assert_eq!(1, c.id(3));
        assert_eq!(1, c.id(4));
        assert_eq!(2, c.size(4));

        assert!(c.connected(1, 0));
        assert!(c.connected(1, 2));
        assert!(!c.connected(1, 3));
        assert!(!c.connected(1, 4));
        assert!(c.connected(3, 4));
    }
}
