use std::cmp::Ordering::{Greater, Less};

struct Node {
    parent: usize,
    rank: usize,
}
/// Union Find
///
/// ids[]: pointers to parent node id, so we can track recursively until root
pub struct UnionFind {
    ids: Vec<Node>,
    count: usize,
}

impl UnionFind {
    pub fn new(cap: usize) -> Self {
        UnionFind {
            ids: (0..cap).map(|i| Node { parent: i, rank: 0 }).collect(),
            count: cap,
        }
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn find(&self, i: usize) -> Option<usize> {
        if i >= self.ids.len() {
            None
        } else {
            Some(self._find(i))
        }
    }

    fn _find(&self, i: usize) -> usize {
        let mut i = i;
        while i != self.ids[i].parent {
            i = self.ids[i].parent;
        }

        i
    }

    fn _find_mut(&mut self, i: usize) -> usize {
        let mut i = i;
        while i != self.ids[i].parent {
            self.ids[i].parent = self.ids[self.ids[i].parent].parent; // path compression
            i = self.ids[i].parent;
        }

        i
    }

    pub fn connected(&self, p: usize, q: usize) -> Option<bool> {
        match (self.find(p), self.find(q)) {
            (Some(p_root), Some(q_root)) => Some(p_root == q_root),
            _ => None,
        }
    }

    pub fn union(&mut self, p: usize, q: usize) {
        if p > self.ids.capacity() || q > self.ids.capacity() {
            return;
        }

        let p_root = self._find_mut(p);
        let q_root = self._find_mut(q);
        if p_root == q_root {
            return;
        }

        match self.ids[p_root].rank.cmp(&self.ids[q_root].rank) {
            Less => self.ids[p_root].parent = q_root,
            Greater => self.ids[q_root].parent = p_root,
            _ => {
                self.ids[p_root].parent = q_root;
                self.ids[q_root].rank += 1;
            }
        }
        self.count -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let uf = UnionFind::new(1);
        assert_eq!(1, uf.count());
    }

    #[test]
    fn none() {
        let uf = UnionFind::new(1);
        assert_eq!(None, uf.find(1));
        assert_eq!(None, uf.connected(1, 0));
    }

    #[test]
    fn non_connected() {
        let uf = UnionFind::new(3);
        assert_eq!(Some(0), uf.find(0));
        assert_eq!(Some(1), uf.find(1));
        assert_eq!(Some(2), uf.find(2));

        assert_eq!(3, uf.count());
    }

    #[test]
    fn one_connected() {
        let mut uf = UnionFind::new(3);
        uf.union(0, 1);
        assert_eq!(2, uf.count());
        assert_eq!(uf.find(0), uf.find(1));
        assert_eq!(Some(2), uf.find(2));

        assert_eq!(Some(true), uf.connected(0, 1));
        assert_eq!(Some(false), uf.connected(0, 2));
        assert_eq!(Some(false), uf.connected(1, 2));
    }

    #[test]
    fn linear_connected() {
        let mut uf = UnionFind::new(6);
        uf.union(0, 1);
        uf.union(1, 2);
        uf.union(2, 3);
        uf.union(3, 4);
        uf.union(4, 5);

        assert_eq!(1, uf.count());

        assert_eq!(Some(true), uf.connected(0, 1));
        assert_eq!(Some(true), uf.connected(0, 3));
        assert_eq!(Some(true), uf.connected(2, 3));
        assert_eq!(Some(true), uf.connected(4, 1));
        assert_eq!(Some(true), uf.connected(2, 5));

        let root = uf.find(0);
        assert_eq!(root, uf.find(1));
        assert_eq!(root, uf.find(2));
        assert_eq!(root, uf.find(3));
        assert_eq!(root, uf.find(4));
        assert_eq!(root, uf.find(5));
    }
}
