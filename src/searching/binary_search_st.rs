pub struct BinarySearchST<K, V> {
    keys: Vec<K>,
    values: Vec<V>,
}

impl<K: Eq + Ord, V> BinarySearchST<K, V> {
    pub fn new() -> Self {
        BinarySearchST {
            keys: Vec::new(),
            values: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.keys.len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn min(&self) -> Option<&K> {
        self.keys.first()
    }
    pub fn max(&self) -> Option<&K> {
        self.keys.last()
    }
    pub fn select(&self, i: usize) -> Option<&K> {
        self.keys.get(i)
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        if self.keys.len() == 0 {
            return None;
        }

        let lo = self.lower_bound(key);
        if &self.keys[lo] == key {
            Some(&self.values[lo])
        } else {
            None
        }
    }
    pub fn put(&mut self, key: K, value: V) {
        let up = self.upper_bound(&key);
        if up > 0 && self.keys[up - 1] == key {
            self.values[up - 1] = value;
        } else if up >= self.keys.len() {
            self.keys.push(key);
            self.values.push(value);
        } else {
            self.keys.insert(up, key);
            self.values.insert(up, value);
        }
    }

    pub fn contains(&self, key: &K) -> bool {
        // contain method is O(n)
        //self.keys.contains(key)
        let lo = self.lower_bound(key);
        lo < self.len() && &self.keys[lo] == key
    }
    pub fn rank(&self, key: &K) -> usize {
        self.lower_bound(key)
    }

    pub fn floor(&self, key: &K) -> Option<&K> {
        //self.keys.get(self.lower_bound(key))
        let lo = self.lower_bound(key);
        self.keys.get(lo).and_then(|k| {
            if k == key {
                Some(k)
            } else if lo == 0 {
                None
            } else {
                self.keys.get(lo - 1)
            }
        })
    }
    pub fn ceiling(&self, key: &K) -> Option<&K> {
        self.keys.get(self.upper_bound(key) - 1)
    }

    pub fn delete(&mut self, key: &K) {
        let lo = self.lower_bound(key);
        if lo < self.len() && &self.keys[lo] == key {
            self.keys.remove(lo);
        }
    }
    pub fn delete_min(&mut self) {
        if self.len() > 0 {
            self.keys.remove(0);
            self.values.remove(0);
        }
    }
    pub fn delete_max(&mut self) {
        let n = self.len();
        if n > 0 {
            self.keys.remove(n - 1);
            self.values.remove(n - 1);
        }
    }

    // TODO: not sure whether it was the right way to implement in
    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.keys.iter()
    }
    pub fn keys_range<'a>(&'a self, lo: &K, hi: &K) -> impl Iterator<Item = &K> + 'a {
        let lo = self.lower_bound(lo);
        let hi = self.upper_bound(hi);
        self.keys.iter().take(hi).skip(lo)
    }

    pub fn check(&self) -> bool {
        self.keys.windows(2).all(|w| w[0] <= w[1])
    }

    /// [low_bound, upper_bound)
    fn lower_bound(&self, key: &K) -> usize {
        let mut b = 0;
        let mut e = self.keys.len();

        while b < e {
            let m = b + (e - b) / 2;

            if key <= &self.keys[m] {
                e = m;
            } else {
                b = m + 1;
            }
        }

        b
    }
    fn upper_bound(&self, key: &K) -> usize {
        let mut b = 0;
        let mut e = self.keys.len();

        while b < e {
            let m = b + (e - b) / 2;

            if &self.keys[m] <= key {
                b = m + 1;
            } else {
                e = m;
            }
        }

        b
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{thread_rng, Rng};

    #[test]
    fn empty() {
        let st = BinarySearchST::<String, usize>::new();
        assert_eq!(0, st.len());
        assert_eq!(None, st.min());
        assert_eq!(None, st.max());
        assert_eq!(None, st.get(&"any".into()));
    }

    #[test]
    fn put_get() {
        let mut st = BinarySearchST::<String, usize>::new();
        st.put("iphone".into(), 500);
        assert_eq!(1, st.len());
        assert_eq!(Some(&"iphone".into()), st.min());
        assert_eq!(Some(&"iphone".into()), st.max());
        assert_eq!(Some(&500), st.get(&"iphone".into()));
    }

    #[test]
    fn update() {
        let mut st = BinarySearchST::<String, usize>::new();
        st.put("iphone".into(), 500);
        st.put("iphone".into(), 250);
        assert_eq!(1, st.len());
        assert_eq!(Some(&250), st.get(&"iphone".into()));
        assert!(st.contains(&"iphone".into()));
    }

    #[test]
    fn get_one_of_three() {
        let mut st = BinarySearchST::<String, usize>::new();
        st.put("iphone".into(), 600);
        st.put("android".into(), 500);
        st.put("blackberry".into(), 800);
        assert_eq!(3, st.len());
        assert_eq!(Some(&"android".into()), st.min());
        assert_eq!(Some(&"iphone".into()), st.max());
        assert_eq!(Some(&800), st.get(&"blackberry".into()));
        assert!(st.check());

        {
            // mutuable borrow
            let mut it = st.keys();
            assert_eq!(Some(&"android".into()), it.next());
            assert_eq!(Some(&"blackberry".into()), it.next());
            assert_eq!(Some(&"iphone".into()), it.next());
        }

        st.delete_min();
        assert_eq!(2, st.len());
        assert_eq!(Some(&"blackberry".into()), st.min());

        st.delete_max();
        assert_eq!(1, st.len());
        assert_eq!(Some(&"blackberry".into()), st.min());
        assert_eq!(Some(&"blackberry".into()), st.max());
    }

    #[test]
    fn keys_range() {
        let mut st = BinarySearchST::<u32, u32>::new();
        for i in 1..10 {
            st.put(i, i * 2);
        }
        let v = st.keys_range(&1, &3).collect::<Vec<_>>();
        assert_eq!([&1, &2, &3], &v[..]);
        let v = st.keys_range(&0, &3).collect::<Vec<_>>();
        assert_eq!([&1, &2, &3], &v[..]);
        let v = st.keys_range(&8, &9).collect::<Vec<_>>();
        assert_eq!([&8, &9], &v[..]);
        let v = st.keys_range(&8, &20).collect::<Vec<_>>();
        assert_eq!([&8, &9], &v[..]);
    }

    #[test]
    fn random_100() {
        let mut st = BinarySearchST::<u32, u32>::new();
        for i in thread_rng().gen_iter::<u32>().take(100) {
            st.put(i, i);
        }
        let mut last_k = st.min().unwrap().clone();
        assert_eq!(Some(&last_k), st.get(&last_k));
        for _i in 1..100 {
            st.delete_min();
            let k = st.min().unwrap();
            assert!(&last_k <= k);
            assert_eq!(Some(k), st.get(&k));
            last_k = *k;
        }
    }
}
