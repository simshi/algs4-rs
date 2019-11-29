pub struct UnionFind {
    ids: Vec<usize>,
}

impl UnionFind {
    pub fn new(cap: usize) -> Self {
        UnionFind {
            ids: (0..cap).map(|i| i).collect()
        }
    }

    pub fn count(&self) -> usize {
        self.ids.capacity()
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
}
