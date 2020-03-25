use super::base::*;

#[derive(Default, PartialEq, Debug)]
pub struct Cycle {
    path: Vec<usize>,
}
impl Cycle {
    fn new() -> Self {
        Cycle::default()
    }
    fn is_empty(&self) -> bool {
        self.path.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &usize> {
        self.path.iter()
    }
}

// TODO: follow up https://github.com/rust-lang/rust/issues/20400
// pub trait CycleDetection {
//     fn cycle(&self) -> Option<Cycle>;
// }
// impl<G, E> CycleDetection for G
// where
//     E: Undirected,
//     G: Graph<Edge = E>,
// {
//     fn cycle(&self) -> Option<Cycle> {
//         let mut c = Cycle::new();
//         detect_undirected(self, &mut c);
//         if c.path.len() > 0 {
//             Some(c)
//         } else {
//             None
//         }
//     }
// }
// impl<G, E> CycleDetection for G
// where
//     E: Directed,
//     G: Graph<Edge = E>,
// {
//     fn cycle(&self) -> Option<Cycle> {
//         let mut c = Cycle::new();
//         detect_directed(self, &mut c);
//         if c.path.len() > 0 {
//             Some(c)
//         } else {
//             None
//         }
//     }
// }
pub struct CycleDetection<'a, G: Graph> {
    g: &'a G,
    marked: Vec<bool>,
    edge_to: Vec<usize>,
}
impl<'a, G: Graph> CycleDetection<'a, G> {
    pub fn detect_undirected<E: Undirected>(g: &'a G) -> Option<Cycle>
    where
        G: Graph<Edge = E>,
    {
        let mut s = Self::new(g);
        let mut c = Cycle::new();
        s.undirected(&mut c);
        if c.is_empty() {
            None
        } else {
            Some(c)
        }
    }
    pub fn detect_directed<E: Directed>(g: &'a G) -> Option<Cycle>
    where
        G: Graph<Edge = E>,
    {
        let mut s = Self::new(g);
        let mut c = Cycle::new();
        s.directed(&mut c);
        if c.is_empty() {
            None
        } else {
            Some(c)
        }
    }

    fn new(g: &'a G) -> Self {
        let v = g.v_size();
        CycleDetection {
            g,
            marked: vec![false; v],
            edge_to: vec![v; v],
        }
    }

    fn undirected(&mut self, c: &mut Cycle) {
        for v in 0..self.g.v_size() {
            if !self.marked[v] {
                self.dfs_undirected(c, self.g.v_size(), v);
            }
        }
    }
    fn dfs_undirected(&mut self, c: &mut Cycle, prev: usize, v: usize) {
        self.marked[v] = true;
        for e in self.g.adj(v) {
            if !c.is_empty() {
                return;
            }

            let w = e.other(v);
            if !self.marked[w] {
                self.edge_to[w] = v;
                self.dfs_undirected(c, v, w);
            } else if w != prev {
                c.path.push(w);
                let mut x = v;
                while x != w {
                    c.path.push(x);
                    x = self.edge_to[x];
                }
                c.path.push(w);
            }
        }
    }

    fn directed(&mut self, c: &mut Cycle) {
        let mut on_stack = vec![false; self.g.v_size()];
        for v in 0..self.g.v_size() {
            if !self.marked[v] {
                self.dfs_directed(c, v, &mut on_stack);
            }
        }
    }
    fn dfs_directed(&mut self, c: &mut Cycle, v: usize, on_stack: &mut Vec<bool>) {
        self.marked[v] = true;
        on_stack[v] = true;
        for e in self.g.adj(v) {
            if !c.is_empty() {
                return;
            }
            let w = e.other(v);
            if !self.marked[w] {
                self.edge_to[w] = v;
                self.dfs_directed(c, w, on_stack);
            } else if on_stack[w] {
                c.path.push(w);
                let mut x = v;
                while x != w {
                    c.path.push(x);
                    x = self.edge_to[x];
                }
                c.path.push(w);
                c.path.reverse();
            }
        }
        on_stack[v] = false;
    }
}

#[cfg(test)]
mod tests {
    use super::super::DirectedGraph;
    use super::super::UndirectedGraph;
    use super::*;

    #[test]
    fn empty() {
        let g = UndirectedGraph::new(3);
        assert_eq!(None, CycleDetection::detect_undirected(&g));

        let g = DirectedGraph::new(3);
        assert_eq!(None, CycleDetection::detect_directed(&g));
    }

    #[test]
    fn cycled_undirected() {
        let mut g = UndirectedGraph::new(5);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(3, 2);
        g.add_edge(3, 1);

        let c = CycleDetection::detect_undirected(&g);
        assert!(c.is_some());
        let c = c.unwrap();
        let a = c.iter().collect::<Vec<_>>();
        if a[1] == &3 {
            assert_eq!([&1, &3, &2, &1], &a[..]);
        } else {
            assert_eq!([&1, &2, &3, &1], &a[..]);
        }
    }

    #[test]
    fn cycled_digraph() {
        let mut g = DirectedGraph::new(6);
        g.add_edge(0, 1);
        g.add_edge(2, 0);
        g.add_edge(2, 1);

        g.add_edge(3, 4);
        g.add_edge(4, 5);
        g.add_edge(5, 3);

        let c = CycleDetection::detect_directed(&g);
        assert!(c.is_some());
        let c = c.unwrap();
        let a = c.iter().collect::<Vec<_>>();
        assert_eq!(vec![&3, &4, &5, &3], a);
    }

    #[test]
    fn self_loop_undirected() {
        let mut g = UndirectedGraph::new(1);
        g.add_edge(0, 0);

        let c = CycleDetection::detect_undirected(&g);
        assert!(c.is_some());
        let c = c.unwrap();
        let a = c.iter().collect::<Vec<_>>();
        assert_eq!([&0, &0], &a[..]);
    }

    #[test]
    fn self_loop_directed() {
        let mut g = DirectedGraph::new(1);
        g.add_edge(0, 0);

        let c = CycleDetection::detect_directed(&g);
        assert!(c.is_some());
        let c = c.unwrap();
        let a = c.iter().collect::<Vec<_>>();
        assert_eq!([&0, &0], &a[..]);
    }

    #[test]
    #[ignore] // not considerated!
    fn parallel_edge() {
        let mut g = UndirectedGraph::new(2);
        g.add_edge(0, 1);
        g.add_edge(1, 0);

        let c = CycleDetection::detect_undirected(&g);
        assert!(c.is_some());
        let c = c.unwrap();
        let a = c.iter().collect::<Vec<_>>();
        assert_eq!([&0, &1, &0], &a[..]);
    }
}
