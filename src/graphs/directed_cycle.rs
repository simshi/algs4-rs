use super::{BaseGraph, Digraph};

// connected component
pub struct DirectedCycle {
    marked: Vec<bool>,
    edge_to: Vec<usize>,
    cycle: Vec<usize>,
    on_stack: Vec<bool>,
}
impl DirectedCycle {
    pub fn new(g: &Digraph) -> Self {
        let mut c = DirectedCycle {
            marked: vec![false; g.v_size()],
            edge_to: vec![g.v_size(); g.v_size()],
            cycle: Vec::new(),
            on_stack: vec![false; g.v_size()],
        };
        c.init(g);
        c
    }

    pub fn has_cycle(&self) -> bool {
        !self.cycle.is_empty()
    }
    pub fn cycle(&self) -> impl Iterator<Item = &usize> {
        self.cycle.iter()
    }
}

// private methods
impl DirectedCycle {
    fn init(&mut self, g: &Digraph) {
        for v in 0..g.v_size() {
            if !self.marked[v] {
                self.dfs(g, v);
            }
        }
    }

    fn dfs(&mut self, g: &Digraph, v: usize) {
        self.marked[v] = true;
        self.on_stack[v] = true;
        for &w in g.adj(v) {
            if self.has_cycle() {
                return;
            } else if !self.marked[w] {
                self.edge_to[w] = v;
                self.dfs(g, w)
            } else if self.on_stack[w] {
                self.cycle.push(w);
                let mut x = v;
                while x != w {
                    self.cycle.push(x);
                    x = self.edge_to[x];
                }
                self.cycle.push(w);
                self.cycle.reverse();
            }
        }
        self.on_stack[v] = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let g = Digraph::new(3);
        let c = DirectedCycle::new(&g);

        assert!(!c.has_cycle());
        let mut it = c.cycle();
        assert_eq!(None, it.next());
    }

    #[test]
    fn cycled() {
        let mut g = Digraph::new(5);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(2, 3);
        g.add_edge(3, 1);

        let c = DirectedCycle::new(&g);
        assert!(c.has_cycle());
        let a = c.cycle().collect::<Vec<_>>();
        assert_eq!([&1, &2, &3, &1], &a[..]);
    }

    #[test]
    fn self_loop() {
        let mut g = Digraph::new(1);
        g.add_edge(0, 0);

        let c = DirectedCycle::new(&g);
        assert!(c.has_cycle());
        let a = c.cycle().collect::<Vec<_>>();
        assert_eq!([&0, &0], &a[..]);
    }

    #[test]
    fn parallel_edge() {
        let mut g = Digraph::new(2);
        g.add_edge(0, 1);
        g.add_edge(1, 0);

        let c = DirectedCycle::new(&g);
        assert!(c.has_cycle());
        let a = c.cycle().collect::<Vec<_>>();
        assert_eq!([&0, &1, &0], &a[..]);
    }
}
