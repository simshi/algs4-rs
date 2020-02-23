use super::BaseGraph;
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

    pub fn reversed(&self) -> Self {
        let mut g = Digraph::new(self.v_size());
        for v in 0..self.v_size() {
            for w in self.adj(v) {
                g.add_edge(*w, v);
            }
        }

        g
    }

    pub fn pre_order(&self) -> PreOrderIter {
        PreOrderIter::new(&self)
    }
    pub fn post_order(&self) -> PostOrderIter {
        PostOrderIter::new(&self)
    }
    pub fn topo_order(&self) -> TopoOrderIter {
        TopoOrderIter::new(&self)
    }
}

impl<'a> BaseGraph<'a> for Digraph {
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
        self.e += 1;
    }

    fn adj(&'a self, v: usize) -> Self::Iter {
        self.adj[v].iter()
    }
}

pub struct PreOrderIter<'a> {
    g: &'a Digraph,
    v: usize,
    marked: Vec<bool>,
    stack: Vec<usize>,
}
impl<'a> PreOrderIter<'a> {
    pub fn new(g: &'a Digraph) -> Self {
        PreOrderIter {
            g,
            v: 0,
            marked: vec![false; g.v_size()],
            stack: Vec::new(),
        }
    }
}
impl Iterator for PreOrderIter<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stack.len() == 0 {
            while self.v < self.g.v_size() {
                if !self.marked[self.v] {
                    self.stack.push(self.v);
                    break;
                }
                self.v += 1;
            }
        }

        self.stack.pop().map(|v| {
            self.marked[v] = true;
            for w in self.g.adj(v) {
                if !self.marked[*w] {
                    self.stack.push(*w);
                }
            }

            v
        })
    }
}

pub struct PostOrderIter<'a> {
    g: &'a Digraph,
    v: usize,
    marked: Vec<bool>,
    stack: Vec<usize>,
}
impl<'a> PostOrderIter<'a> {
    pub fn new(g: &'a Digraph) -> Self {
        PostOrderIter {
            g,
            v: 0,
            marked: vec![false; g.v_size()],
            stack: Vec::new(),
        }
    }

    fn dfs(&mut self, v: usize) {
        self.marked[v] = true;
        self.stack.push(v);
        for &w in self.g.adj(v) {
            if !self.marked[w] {
                self.dfs(w);
            }
        }
    }
}
impl Iterator for PostOrderIter<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stack.len() == 0 {
            while self.v < self.g.v_size() {
                if !self.marked[self.v] {
                    self.dfs(self.v);
                    break;
                }
                self.v += 1;
            }
        }

        self.stack.pop()
    }
}

pub struct TopoOrderIter<'a> {
    g: &'a Digraph,
    v: usize,
    marked: Vec<bool>,
    stack: Vec<usize>,
}
impl<'a> TopoOrderIter<'a> {
    pub fn new(g: &'a Digraph) -> Self {
        TopoOrderIter {
            g,
            v: 0,
            marked: vec![false; g.v_size()],
            stack: Vec::new(),
        }
    }

    fn dfs(&mut self, v: usize) {
        self.marked[v] = true;
        self.stack.push(v);
        for &w in self.g.adj(v) {
            if !self.marked[w] {
                self.dfs(w);
            }
        }
    }
}
impl Iterator for TopoOrderIter<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stack.len() == 0 {
            while self.v < self.g.v_size() {
                if !self.marked[self.v] {
                    self.dfs(self.v);
                    self.stack.reverse();
                    break;
                }
                self.v += 1;
            }
        }

        self.stack.pop()
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

    #[test]
    fn pre_order() {
        let mut g = Digraph::new(6);
        g.add_edge(0, 4);
        g.add_edge(4, 5);
        g.add_edge(4, 3);
        g.add_edge(3, 1);

        let r = g.pre_order().collect::<Vec<_>>();
        assert_eq!(6, r.len());
        if r[2] == 5 {
            assert_eq!(vec![0, 4, 5, 3, 1, 2], r);
        } else {
            assert_eq!(vec![0, 4, 3, 1, 5, 2], r);
        }
    }

    #[test]
    fn post_order() {
        let mut g = Digraph::new(9);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(1, 3);
        g.add_edge(3, 4);

        g.add_edge(5, 6);
        g.add_edge(6, 7);

        let r = g.post_order().collect::<Vec<_>>();
        assert_eq!(9, r.len());
        if r[0] == 2 {
            assert_eq!(vec![2, 4, 3, 1, 0, 7, 6, 5, 8], r);
        } else {
            assert_eq!(vec![4, 3, 2, 1, 0, 7, 6, 5, 8], r);
        }
    }

    #[test]
    fn topo_order() {
        let mut g = Digraph::new(9);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(1, 3);
        g.add_edge(3, 4);

        g.add_edge(5, 6);
        g.add_edge(6, 7);

        let r = g.topo_order().collect::<Vec<_>>();
        assert_eq!(9, r.len());
        if r[2] == 2 {
            assert_eq!(vec![0, 1, 2, 3, 4, 5, 6, 7, 8], r);
        } else {
            assert_eq!(vec![0, 1, 3, 4, 2, 5, 6, 7, 8], r);
        }
    }
}
