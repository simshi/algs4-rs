use super::{BaseGraph, Graph};

// connected component
pub struct CC {
    count_: usize,
    marked: Vec<bool>,
    ids: Vec<usize>,
    sizes: Vec<usize>,
}
impl CC {
    pub fn new<'a>(g: &'a Graph) -> Self {
        let mut c = CC {
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
impl CC {
    fn init<'a>(&mut self, g: &'a Graph) {
        for v in 0..g.v_size() {
            if !self.marked[v] {
                self.dfs(g, v);
                self.count_ += 1;
            }
        }
    }

    fn dfs<'a>(&mut self, g: &'a Graph, v: usize) {
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
        let g = Graph::new(3);
        let c = CC::new(&g);
        assert_eq!(3, c.count());
        assert_eq!(0, c.id(0));
        assert_eq!(1, c.id(1));
        assert_eq!(2, c.id(2));

        assert_eq!(1, c.size(0));
        assert_eq!(1, c.size(1));
        assert_eq!(1, c.size(2));
    }

    #[test]
    fn connected() {
        let mut g = Graph::new(5);
        g.add_edge(0, 1);
        g.add_edge(1, 2);

        g.add_edge(4, 3);

        let c = CC::new(&g);
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
