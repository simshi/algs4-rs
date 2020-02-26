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
    // valid if and only if it's a DAG
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
        if self.stack.is_empty() {
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

// post_order helper functions
fn post_order(order: &mut Vec<usize>, g: &Digraph) {
    let mut marked = vec![false; g.v_size()];
    for v in 0..g.v_size() {
        post_order_dfs(order, g, &mut marked, v);
    }
}
fn post_order_dfs(order: &mut Vec<usize>, g: &Digraph, marked: &mut Vec<bool>, v: usize) {
    if marked[v] {
        return;
    }

    marked[v] = true;
    for &w in g.adj(v) {
        post_order_dfs(order, g, marked, w);
    }
    order.push(v);
}

pub struct PostOrderIter {
    order: Vec<usize>,
    i: usize,
}
impl PostOrderIter {
    pub fn new(g: &Digraph) -> Self {
        let mut order = Vec::new();
        post_order(&mut order, g);

        PostOrderIter { order, i: 0 }
    }
}
impl Iterator for PostOrderIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.order.len() {
            self.i += 1;
            Some(self.order[self.i - 1])
        } else {
            None
        }
    }
}

pub struct TopoOrderIter {
    order: Vec<usize>,
}
impl TopoOrderIter {
    pub fn new(g: &Digraph) -> Self {
        let mut order = Vec::new();
        post_order(&mut order, g);

        TopoOrderIter { order }
    }
}
impl Iterator for TopoOrderIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.order.pop()
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
        g.add_edge(1, 4);
        g.add_edge(3, 2);
        g.add_edge(2, 5);

        g.add_edge(6, 5);

        g.add_edge(8, 7);

        let r = g.post_order().collect::<Vec<_>>();
        assert_eq!(9, r.len());
        if r[0] == 5 {
            if r[2] == 3 {
                assert_eq!(vec![5, 2, 3, 4, 1, 0, 6, 7, 8], r);
            } else {
                assert_eq!(vec![5, 2, 4, 3, 1, 0, 6, 7, 8], r);
            }
        } else {
            assert_eq!(vec![4, 5, 2, 3, 1, 0, 6, 7, 8], r);
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
        if r[6] == 2 {
            assert_eq!(vec![8, 5, 6, 7, 0, 1, 2, 3, 4], r);
        } else {
            assert_eq!(vec![8, 5, 6, 7, 0, 1, 3, 4, 2], r);
        }
    }
}
