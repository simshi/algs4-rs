use std::cmp::Ordering;

type List<K, V> = Option<Box<Node<K, V>>>;
struct Node<K, V> {
    key: K,
    value: V,
    left: List<K, V>,
    right: List<K, V>,
    size: usize,
}

pub struct BSTree<K, V> {
    root: List<K, V>,
}

impl<K: Eq + Ord, V> BSTree<K, V> {
    pub fn new() -> Self {
        BSTree { root: None }
    }

    pub fn len(&self) -> usize {
        self.root.as_ref().map_or(0, |p| p.size)
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn min(&self) -> Option<&K> {
        self.root.as_ref().map(|p| self._min(p))
    }
    pub fn max(&self) -> Option<&K> {
        self.root.as_ref().map(|p| self._max(p))
    }
    pub fn select(&self, i: usize) -> Option<&K> {
        self.root.as_ref().map(|p| &p.key)
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.root.as_ref().and_then(|p| self._get(p, key))
    }
    pub fn put(&mut self, key: K, value: V) {
        let a = self.root.take();
        self.root = Some(self._put(a, key, value));
    }

    pub fn contains(&self, key: &K) -> bool {
        false
    }
    pub fn rank(&self, key: &K) -> usize {
        0
    }

    pub fn floor(&self, key: &K) -> Option<&K> {
        self.root.as_ref().map(|p| &p.key)
    }
    pub fn ceiling(&self, key: &K) -> Option<&K> {
        self.root.as_ref().map(|p| &p.key)
    }

    pub fn delete(&mut self, key: &K) {}
    pub fn delete_min(&mut self) {}
    pub fn delete_max(&mut self) {}

    // TODO: not sure whether it was the right way to implement in
    // pub fn keys(&self) -> impl Iterator<Item = &K> {
    //     self.keys.iter()
    // }
    // pub fn keys_range<'a>(&'a self, lo: &K, hi: &K) -> impl Iterator<Item = &K> + 'a {
    //     let lo = self.lower_bound(lo);
    //     let hi = self.upper_bound(hi);
    //     self.keys.iter().take(hi).skip(lo)
    // }

    // pub fn check(&self) -> bool {
    //     self.keys.windows(2).all(|w| w[0] <= w[1])
    // }
}

// private methods
impl<K: Eq + Ord, V> BSTree<K, V> {
    fn _len(&self, list: &List<K, V>) -> usize {
        list.as_ref().map_or(0, |p| p.size)
    }

    fn _min<'a>(&self, node: &'a Node<K, V>) -> &'a K {
        node.left.as_ref().map_or(&node.key, |v| self._min(&v))
    }
    fn _max<'a>(&self, node: &'a Node<K, V>) -> &'a K {
        node.right.as_ref().map_or(&node.key, |v| self._max(&v))
    }

    fn _get<'a>(&self, node: &'a Node<K, V>, key: &K) -> Option<&'a V> {
        match key.cmp(&node.key) {
            Ordering::Equal => Some(&node.value),
            Ordering::Less => node.left.as_ref().and_then(|n| self._get(&n, key)),
            Ordering::Greater => node.right.as_ref().and_then(|n| self._get(&n, key)),
        }
    }
    fn _put(&mut self, list: List<K, V>, key: K, value: V) -> Box<Node<K, V>> {
        match list {
            None => Box::new(Node {
                key: key,
                value: value,
                left: None,
                right: None,
                size: 1,
            }),
            Some(mut b) => {
                match key.cmp(&b.key) {
                    Ordering::Equal => b.value = value,
                    Ordering::Less => b.left = Some(self._put(b.left, key, value)),
                    Ordering::Greater => b.right = Some(self._put(b.right, key, value)),
                };
                b.size = 1 + self._len(&b.left) + self._len(&b.right);
                b
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{thread_rng, Rng};

    #[test]
    fn empty() {
        let st = BSTree::<String, usize>::new();
        assert_eq!(0, st.len());
        assert_eq!(None, st.min());
        assert_eq!(None, st.max());
        assert_eq!(None, st.get(&"any".into()));
    }

    #[test]
    fn put_get() {
        let mut st = BSTree::<String, usize>::new();
        st.put("iphone".into(), 500);
        assert_eq!(1, st.len());
        assert_eq!(Some(&"iphone".into()), st.min());
        assert_eq!(Some(&"iphone".into()), st.max());
        assert_eq!(Some(&500), st.get(&"iphone".into()));
    }

    #[test]
    fn update() {
        let mut st = BSTree::<String, usize>::new();
        st.put("iphone".into(), 500);
        st.put("iphone".into(), 250);
        assert_eq!(1, st.len());
        assert_eq!(Some(&250), st.get(&"iphone".into()));
        assert!(st.contains(&"iphone".into()));
    }

    #[test]
    fn get_one_of_three() {
        let mut st = BSTree::<String, usize>::new();
        st.put("iphone".into(), 600);
        st.put("android".into(), 500);
        st.put("blackberry".into(), 800);
        assert_eq!(3, st.len());
        assert_eq!(Some(&"android".into()), st.min());
        assert_eq!(Some(&"iphone".into()), st.max());
        assert_eq!(Some(&800), st.get(&"blackberry".into()));
        // assert!(st.check());

        {
            // mutuable borrow
            // let mut it = st.keys();
            // assert_eq!(Some(&"android".into()), it.next());
            // assert_eq!(Some(&"blackberry".into()), it.next());
            // assert_eq!(Some(&"iphone".into()), it.next());
        }

        st.delete_min();
        assert_eq!(2, st.len());
        assert_eq!(Some(&"blackberry".into()), st.min());

        st.delete_max();
        assert_eq!(1, st.len());
        assert_eq!(Some(&"blackberry".into()), st.min());
        assert_eq!(Some(&"blackberry".into()), st.max());
    }

    // #[test]
    // fn keys_range() {
    //     let mut st = BSTree::<u32, u32>::new();
    //     for i in 1..10 {
    //         st.put(i, i * 2);
    //     }
    //     let v = st.keys_range(&1, &3).collect::<Vec<_>>();
    //     assert_eq!([&1, &2, &3], &v[..]);
    //     let v = st.keys_range(&0, &3).collect::<Vec<_>>();
    //     assert_eq!([&1, &2, &3], &v[..]);
    //     let v = st.keys_range(&8, &9).collect::<Vec<_>>();
    //     assert_eq!([&8, &9], &v[..]);
    //     let v = st.keys_range(&8, &20).collect::<Vec<_>>();
    //     assert_eq!([&8, &9], &v[..]);
    // }

    #[test]
    fn random_100() {
        let mut st = BSTree::<u32, u32>::new();
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
