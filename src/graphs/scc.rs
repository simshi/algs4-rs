use super::{BaseGraph, Digraph};

// KosarajuShrir strongly-connected components
pub struct SCC {
    count_: usize,
    marked: Vec<bool>,
    ids: Vec<usize>,
    sizes: Vec<usize>,
}
impl SCC {
    pub fn new<'a>(g: &'a Digraph) -> Self {
        let mut c = SCC {
            count_: 0,
            marked: vec![false; g.v_size()],
            ids: vec![0; g.v_size()],
            sizes: vec![0; g.v_size()],
        };
        c.init(g);
        c
    }
    pub fn count(&self) -> usize {
        self.count_
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
    fn init<'a>(&mut self, g: &'a Digraph) {
        let gr = g.reversed();
        for v in gr.topo_order() {
            if !self.marked[v] {
                self.dfs(g, v);
                self.count_ += 1;
            }
        }
    }

    fn dfs<'a>(&mut self, g: &'a Digraph, v: usize) {
        self.marked[v] = true;
        self.ids[v] = self.count_;
        self.sizes[self.count_] += 1;
        for &w in g.adj(v) {
            if !self.marked[w] {
                self.dfs(g, w)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let g = Digraph::new(3);
        let c = SCC::new(&g);
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
        let mut g = Digraph::new(8);
        // 0, (1,2,3) -> 4 -> (5,6) -> 7
        g.add_edge(1, 2);
        g.add_edge(2, 3);
        g.add_edge(3, 1);

        g.add_edge(1, 4);
        g.add_edge(4, 5);
        g.add_edge(5, 6);
        g.add_edge(6, 5);

        g.add_edge(5, 7);

        let c = SCC::new(&g);
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
