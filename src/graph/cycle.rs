use super::Graph;

// connected component
pub struct Cycle {
    marked: Vec<bool>,
    edge_to: Vec<usize>,
    cycle: Vec<usize>,
}
impl Cycle {
    pub fn new(g: &Graph) -> Self {
        let mut c = Cycle {
            marked: vec![false; g.v_size()],
            edge_to: vec![g.v_size() + 1; g.v_size()],
            cycle: Vec::new(),
        };
        c.init(g);
        c
    }

    pub fn has_cycle(&self) -> bool {
        self.cycle.len() > 0
    }
    pub fn cycle(&self) -> impl Iterator<Item = &usize> {
        self.cycle.iter()
    }
}

// private methods
impl Cycle {
    fn init(&mut self, g: &Graph) {
        for v in 0..g.v_size() {
            if !self.marked[v] {
                self.dfs(g, g.v_size() + 1, v);
            }
        }
    }

    fn dfs(&mut self, g: &Graph, p: usize, v: usize) {
        self.marked[v] = true;
        for &w in g.adj(v) {
            if self.has_cycle() {
                return;
            }

            if !self.marked[w] {
                self.edge_to[w] = v;
                self.dfs(g, v, w)
            } else if w != p {
                self.cycle.push(w);
                let mut x = v;
                while x != w {
                    self.cycle.push(x);
                    x = self.edge_to[x];
                }
                self.cycle.push(w);
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
        let c = Cycle::new(&g);

        assert!(!c.has_cycle());
        let mut it = c.cycle();
        assert_eq!(None, it.next());
    }

    #[test]
    fn cycled() {
        let mut g = Graph::new(5);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(3, 2);
        g.add_edge(3, 1);

        let c = Cycle::new(&g);
        assert!(c.has_cycle());
        let a = c.cycle().collect::<Vec<_>>();
        if a[1] == &3 {
            assert_eq!([&1, &3, &2, &1], &a[..]);
        } else {
            assert_eq!([&1, &2, &3, &1], &a[..]);
        }
    }

    #[test]
    fn self_loop() {
        let mut g = Graph::new(1);
        g.add_edge(0, 0);

        let c = Cycle::new(&g);
        assert!(c.has_cycle());
        let a = c.cycle().collect::<Vec<_>>();
        assert_eq!([&0, &0], &a[..]);
    }

    #[test]
    #[ignore] // not considerated!
    fn parallel_edge() {
        let mut g = Graph::new(2);
        g.add_edge(0, 1);
        g.add_edge(1, 0);

        let c = Cycle::new(&g);
        assert!(c.has_cycle());
        let a = c.cycle().collect::<Vec<_>>();
        assert_eq!([&0, &1, &0], &a[..]);
    }
}
