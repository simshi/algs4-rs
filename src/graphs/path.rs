use super::base::*;

pub struct Path {
    s: usize,
    marked: Vec<bool>,
    edge_to: Vec<usize>,
}

impl Path {
    pub fn new<'a, G, E>(g: &'a G, s: usize) -> Self
    where
        E: Edge,
        G: Graph<Edge = E>,
    {
        let mut f = Path {
            s,
            marked: vec![false; g.v_size()],
            edge_to: vec![g.v_size(); g.v_size()],
        };
        f.dfs(g, s);
        f
    }

    pub fn marked(&self, v: usize) -> bool {
        self.marked[v]
    }

    pub fn has_path_to(&self, v: usize) -> bool {
        self.marked(v)
    }

    pub fn path_to(&self, v: usize) -> impl Iterator<Item = usize> {
        let mut path = Vec::new();
        if self.has_path_to(v) {
            let mut x = v;
            while x < self.marked.len() && x != self.s {
                path.push(x);
                x = self.edge_to[x];
            }
            path.push(self.s);
            path.reverse();
        }

        path.into_iter()
    }
}

impl Path {
    fn dfs<'a, G, E>(&mut self, g: &'a G, v: usize)
    where
        E: Edge,
        G: Graph<Edge = E>,
    {
        self.marked[v] = true;
        for e in g.adj(v) {
            if !self.marked[e.other(v)] {
                self.edge_to[e.other(v)] = v;
                self.dfs(g, e.other(v));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::DiGraph;
    use super::super::UndirectedGraph;
    use super::*;

    #[test]
    fn empty() {
        let g = UndirectedGraph::new(3);
        let f = Path::new(&g, 0);
        assert!(!f.marked(1));
        assert!(!f.marked(2));

        let g = DiGraph::new(3);
        let f = Path::new(&g, 0);
        assert!(!f.marked(1));
        assert!(!f.marked(2));
    }

    #[test]
    fn connected_undirected() {
        let mut g = UndirectedGraph::new(5);
        g.add_edge(0, 1);
        g.add_edge(2, 0);
        g.add_edge(3, 4);

        let f = Path::new(&g, 0);
        assert!(f.marked(0));
        assert!(f.marked(1));
        assert!(f.marked(2));
        assert!(!f.marked(3));
        assert!(!f.marked(4));
    }

    #[test]
    fn connected_digraph() {
        let mut g = DiGraph::new(5);
        g.add_edge(0, 1);
        g.add_edge(2, 0);
        g.add_edge(3, 4);

        let f = Path::new(&g, 0);
        assert!(f.marked(0));
        assert!(f.marked(1));
        assert!(!f.marked(2));
        assert!(!f.marked(3));
        assert!(!f.marked(4));
    }

    #[test]
    fn paths_undirected() {
        let mut g = UndirectedGraph::new(6);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(2, 3);

        g.add_edge(1, 4);

        let f = Path::new(&g, 0);
        let mut it = f.path_to(1);
        assert_eq!(0, it.next().unwrap());
        assert_eq!(1, it.next().unwrap());
        assert_eq!(None, it.next());
        assert_eq!(None, f.path_to(5).next());

        let f = Path::new(&g, 0);
        let mut it = f.path_to(3);
        assert_eq!(0, it.next().unwrap());
        assert_eq!(1, it.next().unwrap());
        assert_eq!(2, it.next().unwrap());
        assert_eq!(3, it.next().unwrap());
        assert_eq!(None, it.next());
        assert_eq!(None, f.path_to(5).next());

        let f = Path::new(&g, 3);
        let mut it = f.path_to(4);
        assert_eq!(3, it.next().unwrap());
        assert_eq!(2, it.next().unwrap());
        assert_eq!(1, it.next().unwrap());
        assert_eq!(4, it.next().unwrap());
        assert_eq!(None, it.next());
        assert_eq!(None, f.path_to(5).next());
    }

    #[test]
    fn paths_digraph() {
        let mut g = DiGraph::new(6);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(2, 3);

        g.add_edge(1, 4);

        let f = Path::new(&g, 0);
        let mut it = f.path_to(1);
        assert_eq!(0, it.next().unwrap());
        assert_eq!(1, it.next().unwrap());
        assert_eq!(None, it.next());
        assert_eq!(None, f.path_to(5).next());

        let f = Path::new(&g, 0);
        let mut it = f.path_to(3);
        assert_eq!(0, it.next().unwrap());
        assert_eq!(1, it.next().unwrap());
        assert_eq!(2, it.next().unwrap());
        assert_eq!(3, it.next().unwrap());
        assert_eq!(None, it.next());
        assert_eq!(None, f.path_to(5).next());

        let f = Path::new(&g, 3);
        let mut it = f.path_to(4);
        assert_eq!(None, it.next());
        assert_eq!(None, f.path_to(5).next());
    }
}
