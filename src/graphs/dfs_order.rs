use super::base::*;

/// DFS order traversal
///
/// DFS traversal can be in pre-order or (reversed) post-order.
pub trait DFSOrder<G, E: Directed>
where
    G: Graph<Edge = E>,
{
    fn pre_order(&self) -> PreOrderIter<'_, G>;
    fn post_order(&self) -> PostOrderIter<'_, G, E>;
    fn reversed_post_order(&self) -> std::vec::IntoIter<Vertex>;
}

// DFS order applied only to all directed graphs, i.e. graphs with `Edge: Directed`
impl<G, E: Directed> DFSOrder<G, E> for G
where
    G: Graph<Edge = E>,
{
    fn pre_order(&self) -> PreOrderIter<'_, G> {
        PreOrderIter {
            g: self,
            v: 0,
            marked: vec![false; self.v_size()],
            stack: Vec::new(),
        }
    }

    fn post_order(&self) -> PostOrderIter<G, E> {
        PostOrderIter {
            g: self,
            v: 0,
            marked: vec![false; self.v_size()],
            stack: Vec::new(),
        }
    }

    fn reversed_post_order(&self) -> std::vec::IntoIter<Vertex> {
        reversed_post_order(self)
    }
}

/// Pre-order iterator for visiting vertices in pre-order
///
/// Hold a reference to G, implement by a marked vector and a stack for
/// a recursive process.
pub struct PreOrderIter<'a, G> {
    g: &'a G,
    v: usize,
    marked: Vec<bool>,
    stack: Vec<usize>,
}
impl<'a, G, E> Iterator for PreOrderIter<'a, G>
where
    E: Directed,
    G: Graph<Edge = E>,
{
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        // handle forest
        if self.stack.is_empty() {
            while self.v < self.g.v_size() {
                if !self.marked[self.v] {
                    self.stack.push(self.v);
                    break;
                }
                self.v += 1;
            }
        }

        // visit the top one, if none, end due to all vertices were visited
        self.stack.pop().map(|v| {
            // visit the v first as `pre` order
            self.marked[v] = true;
            for e in self.g.adj(v) {
                if !self.marked[e.to()] {
                    self.stack.push(e.to());
                }
            }

            v
        })
    }
}

/// Post-order iterator for visiting vertices in post-order
///
/// Hold a reference to G, implement by a marked vector and a stack for
/// a recursive process.
pub struct PostOrderIter<'a, G, E>
where
    E: Directed,
    G: Graph<Edge = E>,
{
    g: &'a G,
    v: usize,
    marked: Vec<bool>,
    stack: Vec<(Vertex, Box<dyn Iterator<Item = E> + 'a>)>,
}
impl<'a, G, E> Iterator for PostOrderIter<'a, G, E>
where
    E: Directed,
    G: Graph<Edge = E>,
{
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        // handle forest
        if self.stack.is_empty() {
            while self.v < self.g.v_size() {
                if !self.marked[self.v] {
                    self.marked[self.v] = true;
                    self.stack.push((self.v, self.g.adj(self.v)));
                    break;
                }
                self.v += 1;
            }
        }

        // traversal trees
        while let Some((v, mut iter)) = self.stack.pop() {
            // find the first un-visited adjacent
            let mut adj: Option<Vertex> = None;
            for e in iter.by_ref() {
                let w = e.to();
                if !self.marked[w] {
                    self.marked[w] = true;
                    adj = Some(w);
                    break;
                }
            }
            // push back current vertex
            self.stack.push((v, iter));

            // if there is un-visited adjacent, recursively visits it first,
            // otherwise all adjacent vertices were visited, break and pop the
            // vertex on stack top for visiting.
            if let Some(w) = adj {
                self.stack.push((w, self.g.adj(w)));
            } else {
                break;
            }
        }

        self.stack.pop().map(|(v, _)| v)
    }
}
fn reversed_post_order<G, E: Directed>(g: &G) -> std::vec::IntoIter<Vertex>
where
    G: Graph<Edge = E>,
{
    let mut order = vec![g.v_size(); g.v_size()];
    let mut i = g.v_size();
    let mut marked = vec![false; g.v_size()];
    for v in 0..g.v_size() {
        reversed_post_order_dfs(&mut order, v, &mut i, &mut marked, g);
    }
    order.into_iter()
}
fn reversed_post_order_dfs<G, E: Directed>(
    order: &mut Vec<Vertex>,
    v: Vertex,
    i: &mut usize,
    marked: &mut Vec<bool>,
    g: &G,
) where
    G: Graph<Edge = E>,
{
    if marked[v] {
        return;
    }
    marked[v] = true;

    for w in g.adj(v).map(|e| e.to()) {
        reversed_post_order_dfs(order, w, i, marked, g);
    }

    *i -= 1;
    order[*i] = v;
}

#[cfg(test)]
mod tests {
    use super::super::DirectedGraph;
    use super::*;

    #[test]
    fn empty() {
        let g = DirectedGraph::new(1);
        assert_eq!(vec![0], g.pre_order().collect::<Vec<_>>());
        assert_eq!(vec![0], g.post_order().collect::<Vec<_>>());
        assert_eq!(vec![0], g.reversed_post_order().collect::<Vec<_>>());
    }

    #[test]
    fn simple() {
        let mut g = DirectedGraph::new(6);
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

        let r = g.post_order().collect::<Vec<_>>();
        assert_eq!(6, r.len());
        if r[0] == 1 {
            assert_eq!(vec![1, 3, 5, 4, 0, 2], r);
        } else {
            assert_eq!(vec![5, 1, 3, 4, 0, 2], r);
        }

        let r = g.reversed_post_order().collect::<Vec<_>>();
        assert_eq!(6, r.len());
        if r[3] == 3 {
            assert_eq!(vec![2, 0, 4, 3, 1, 5], r);
        } else {
            assert_eq!(vec![2, 0, 4, 5, 3, 1], r);
        }
    }
}
