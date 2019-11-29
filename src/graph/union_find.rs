struct Node {
    parent: usize,
    rank: usize,
}
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
        if i > self.ids.capacity() {
            None
        } else {
            Some(self._find(i))
        }
    }

    fn _find(&self, i: usize) -> usize {
        let mut j = i;
        while j != self.ids[j].parent {
            j = self.ids[j].parent;
        }

        j
    }

    pub fn connected(&self, p: usize, q: usize) -> Option<bool> {
        if p > self.ids.capacity() || q > self.ids.capacity() {
            None
        } else {
            Some(self.find(p) == self.find(q))
        }
    }

    pub fn union(&mut self, p: usize, q: usize) {
        if p > self.ids.capacity() || q > self.ids.capacity() {
            return;
        }

        let p_root = self._find(p);
        let q_root = self._find(q);
        if p_root == q_root {
            return;
        }

        if self.ids[p_root].rank < self.ids[q_root].rank {
            self.ids[p_root].parent = q_root;
        } else if self.ids[p_root].rank > self.ids[q_root].rank {
            self.ids[q_root].parent = p_root;
        } else {
            // path compression
            self.ids[p_root].parent = q_root;
            self.ids[q_root].rank += 1;
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
