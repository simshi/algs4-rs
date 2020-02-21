use std::collections::HashSet;

use super::BaseGraph;

pub struct Graph {
    v: usize,
    e: usize,
    adj: Vec<HashSet<usize>>,
}
impl Graph {
    pub fn new(v: usize) -> Self {
        Graph {
            v,
            e: 0,
            adj: vec![HashSet::new(); v],
        }
    }
}

impl<'a> BaseGraph<'a> for Graph {
    type Iter = std::collections::hash_set::Iter<'a, usize>;

    fn v_size(&self) -> usize {
        self.v
    }
    fn e_size(&self) -> usize {
        self.e
    }
    fn degree(&self, v: usize) -> usize {
        self.adj[v].len()
    }

    fn add_edge(&mut self, v: usize, w: usize) {
        self.adj[v].insert(w);
        self.adj[w].insert(v);
        self.e += 1;
    }

    fn adj(&'a self, v: usize) -> Self::Iter {
        self.adj[v].iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let g = Graph::new(1);
        assert_eq!(1, g.v_size());
    }

    #[test]
    fn add_get_one() {
        let mut g = Graph::new(3);
        g.add_edge(0, 1);
        assert_eq!(1, g.e_size());

        let mut it = g.adj(0);
        assert_eq!(&1, it.next().unwrap());
        assert_eq!(1, g.degree(0));
        let mut it = g.adj(1);
        assert_eq!(&0, it.next().unwrap());
        assert_eq!(1, g.degree(1));
    }

    #[test]
    fn multiple_edges() {
        let mut g = Graph::new(6);
        g.add_edge(0, 1);
        g.add_edge(2, 0);

        g.add_edge(3, 4);
        g.add_edge(5, 3);

        assert_eq!(4, g.e_size());

        let mut a0 = g.adj(0).collect::<Vec<_>>();
        a0.sort_unstable();
        assert_eq!(vec![&1, &2], a0);
        assert_eq!(2, g.degree(0));
        let mut a1 = g.adj(1).collect::<Vec<_>>();
        a1.sort_unstable();
        assert_eq!(vec![&0], a1);
        let mut a2 = g.adj(2).collect::<Vec<_>>();
        a2.sort_unstable();
        assert_eq!(vec![&0], a2);

        let mut a3 = g.adj(3).collect::<Vec<_>>();
        a3.sort_unstable();
        assert_eq!(vec![&4, &5], a3);
        assert_eq!(2, g.degree(3));

        let mut a4 = g.adj(4).collect::<Vec<_>>();
        a4.sort_unstable();
        assert_eq!(vec![&3], a4);
        assert_eq!(1, g.degree(4));
    }
}