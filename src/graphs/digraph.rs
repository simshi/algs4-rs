use std::collections::HashSet;

pub struct Digraph {
    v: usize,
    e: usize,
    adj: Vec<HashSet<usize>>,
}
impl Digraph {
    pub fn new(v: usize) -> Self {
        Digraph {
            v,
            e: 0,
            adj: vec![HashSet::new(); v],
        }
    }

    pub fn v_size(&self) -> usize {
        self.v
    }
    pub fn e_size(&self) -> usize {
        self.e
    }
    pub fn degree(&self, v: usize) -> usize {
        self.adj[v].len()
    }

    pub fn add_edge(&mut self, v: usize, w: usize) {
        self.adj[v].insert(w);
        self.e += 1;
    }

    pub fn adj(&self, v: usize) -> impl Iterator<Item = &usize> {
        self.adj[v].iter()
    }

    pub fn reversed(&self) -> Self {
        let mut g = Digraph::new(self.v_size());
        for v in 0..self.v_size() {
            for w in self.adj(v) {
                g.add_edge(*w, v);
            }
        }

        g
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let g = Digraph::new(1);
        assert_eq!(1, g.v_size());
    }

    #[test]
    fn add_get_one() {
        let mut g = Digraph::new(3);
        g.add_edge(0, 1);
        assert_eq!(1, g.e_size());

        let mut it = g.adj(0);
        assert_eq!(&1, it.next().unwrap());
        assert_eq!(1, g.degree(0));
        let mut it = g.adj(1);
        assert_eq!(None, it.next());
        assert_eq!(0, g.degree(1));
    }

    #[test]
    fn multiple_edges() {
        let mut g = Digraph::new(6);
        g.add_edge(0, 1);
        g.add_edge(2, 0);

        g.add_edge(3, 4);
        g.add_edge(5, 3);

        assert_eq!(4, g.e_size());

        assert_eq!(1, g.degree(0));
        let mut a0 = g.adj(0).collect::<Vec<_>>();
        a0.sort_unstable();
        assert_eq!(vec![&1], a0);

        assert_eq!(0, g.degree(1));
        assert_eq!(None, g.adj(1).next());

        assert_eq!(1, g.degree(2));
        let mut a2 = g.adj(2).collect::<Vec<_>>();
        a2.sort_unstable();
        assert_eq!(vec![&0], a2);

        assert_eq!(1, g.degree(3));
        let mut a3 = g.adj(3).collect::<Vec<_>>();
        a3.sort_unstable();
        assert_eq!(vec![&4], a3);

        assert_eq!(0, g.degree(4));
        assert_eq!(None, g.adj(4).next());
    }

    #[test]
    fn reversed() {
        let mut g = Digraph::new(5);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(1, 3);
        g.add_edge(2, 3);
        g.add_edge(3, 4);

        let g = g.reversed();

        assert_eq!(None, g.adj(0).next());

        let mut a = g.adj(1).cloned().collect::<Vec<_>>();
        a.sort_unstable();
        assert_eq!(vec![0], a);

        let mut a = g.adj(2).cloned().collect::<Vec<_>>();
        a.sort_unstable();
        assert_eq!(vec![1], a);

        let mut a = g.adj(3).cloned().collect::<Vec<_>>();
        a.sort_unstable();
        assert_eq!(vec![1, 2], a);

        let mut a = g.adj(4).cloned().collect::<Vec<_>>();
        a.sort_unstable();
        assert_eq!(vec![3], a);
    }
}
