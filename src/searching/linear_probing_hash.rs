use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct LinearProbingHashST<K, V> {
    keys: Vec<Option<K>>,
    values: Vec<Option<V>>,
    m: usize,
    n: usize,
}

impl<K, V> LinearProbingHashST<K, V>
where
    K: Hash + Clone + Eq,
    V: Clone,
{
    pub fn new(size: usize) -> Self {
        LinearProbingHashST {
            keys: vec![None; size],
            values: vec![None; size],
            m: size,
            n: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.n
    }
    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    pub fn contains(&self, key: &K) -> bool {
        self.get(key).is_some()
    }
    pub fn get(&self, key: &K) -> Option<&V> {
        let mut i = self.hash(key);
        while let Some(ref k) = self.keys[i] {
            if k == key {
                return self.values[i].as_ref();
            }
            i = (i + 1) % self.n;
        }

        None
    }
    pub fn put(&mut self, key: K, value: V) {
        if self.n >= self.m / 2 {
            self.resize(self.m * 2);
        }

        let mut i = self.hash(&key);
        while self.keys[i].is_some() {
            if self.keys[i].as_ref().map_or(false, |k| k == &key) {
                self.values[i] = Some(value);
                return;
            }
            i = (i + 1) % self.m;
        }

        self.keys[i] = Some(key);
        self.values[i] = Some(value);
        self.n += 1;
    }
    pub fn delete(&mut self, key: &K) {
        let mut i = self.hash(key);
        let mut found = false;
        while let Some(ref k) = self.keys[i] {
            if k == key {
                self.keys[i].take();
                self.values[i].take();
                self.n -= 1;
                found = true;
                break;
            }
            i = (i + 1) % self.m;
        }
        if !found {
            return;
        }

        // rehash all keys in same cluster
        i = (i + 1) % self.m;
        while let (Some(k), Some(v)) = (self.keys[i].take(), self.values[i].take()) {
            self.n -= 1;
            self.put(k, v);
            i = (i + 1) % self.m;
        }

        // shrink if neccessary
        if self.n > 0 && self.n <= self.m / 8 {
            self.resize(self.m / 2);
        }
    }
    pub fn pop(&mut self) -> Option<(K, V)> {
        if self.n == 0 {
            return None;
        }

        for i in 0..self.m {
            if let (Some(k), Some(v)) = (self.keys[i].take(), self.values[i].take()) {
                return Some((k, v));
            }
        }
        None
    }
}

// private methods
impl<K, V> LinearProbingHashST<K, V>
where
    K: Hash + Clone + Eq,
    V: Clone,
{
    fn hash(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish() as usize % self.m
    }

    fn resize(&mut self, size: usize) {
        let mut keys = vec![None; size];
        let mut values = vec![None; size];

        std::mem::swap(&mut keys, &mut self.keys);
        std::mem::swap(&mut values, &mut self.values);
        self.m = size;
        self.n = 0;

        for (key, value) in keys.into_iter().zip(values.into_iter()) {
            if let (Some(key), Some(value)) = (key, value) {
                self.put(key, value);
            }
        }
    }
}

impl<K, V> IntoIterator for LinearProbingHashST<K, V>
where
    K: Hash + Clone + Eq,
    V: Clone,
{
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        // take self
        IntoIter(self)
    }
}
pub struct IntoIter<K, V>(LinearProbingHashST<K, V>);
impl<K, V> Iterator for IntoIter<K, V>
where
    K: Hash + Clone + Eq,
    V: Clone,
{
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{thread_rng, Rng};

    #[test]
    fn empty() {
        let st = LinearProbingHashST::<String, usize>::new(8);
        assert_eq!(0, st.len());
    }

    #[test]
    fn put_get() {
        let mut st = LinearProbingHashST::<String, usize>::new(8);
        st.put("iphone".into(), 500);
        st.put("android".into(), 250);
        assert_eq!(2, st.len());
        assert_eq!(Some(&500), st.get(&"iphone".into()));
        assert_eq!(Some(&250), st.get(&"android".into()));
    }

    #[test]
    fn update() {
        let mut st = LinearProbingHashST::<String, usize>::new(4);
        st.put("iphone".into(), 500);
        st.put("iphone".into(), 250);
        assert_eq!(1, st.len());
        assert_eq!(Some(&250), st.get(&"iphone".into()));
        assert!(st.contains(&"iphone".into()));
    }

    #[test]
    fn extend() {
        let mut st = LinearProbingHashST::<usize, usize>::new(8);
        let size = 8 + 3;
        for i in 0..size {
            st.put(i, i);
        }
        assert_eq!(size, st.len());
        for i in 0..size {
            assert_eq!(Some(&i), st.get(&i));
        }
    }

    #[test]
    fn delete() {
        let mut st = LinearProbingHashST::<usize, usize>::new(16);
        for i in 0..8 {
            st.put(i, i);
        }
        assert_eq!(8, st.len());
        // delete first 2
        st.delete(&0);
        st.delete(&1);
        assert_eq!(6, st.len());
        // delete none exist
        st.delete(&8);
        assert_eq!(6, st.len());
        // delete last one
        st.delete(&7);
        assert_eq!(5, st.len());
        // delete unordered
        st.delete(&5);
        st.delete(&4);
        st.delete(&2);
        st.delete(&3);
        assert_eq!(1, st.len());
        // delete last one
        st.delete(&6);
        assert_eq!(0, st.len());
    }

    #[test]
    fn random_100() {
        let mut st = LinearProbingHashST::<u32, u32>::new(100);
        for (i, k) in thread_rng().gen_iter::<u32>().enumerate().take(100) {
            match (i % 5, i % 7) {
                (0, 0) => {
                    st.put(k, k);
                    st.delete(&k);
                }
                (0, _) => st.delete(&k),
                _ => st.put(k, k),
            }
        }
        assert_eq!(80, st.len());

        assert!(st.into_iter().all(|(k, v)| k == v));
    }
}
