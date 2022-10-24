use std::cmp::Ordering;

type Tree<K, V> = Option<Box<Node<K, V>>>;
struct Node<K, V> {
    key: K,
    value: V,
    left: Tree<K, V>,
    right: Tree<K, V>,
    size: usize,
}
impl<K, V> Node<K, V> {
    fn new(key: K, value: V) -> Self {
        Node {
            key,
            value,
            left: None,
            right: None,
            size: 1,
        }
    }
}

/// Binary Search Tree
///
/// A binary search tree is O(logN) in get/put/delete
#[derive(Default)]
pub struct BSTree<K, V> {
    root: Tree<K, V>,
}

impl<K: Ord, V> BSTree<K, V> {
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
        self.root.as_ref().map(|p| node_min(p))
    }
    pub fn max(&self) -> Option<&K> {
        self.root.as_ref().map(|p| node_max(p))
    }
    pub fn floor(&self, key: &K) -> Option<&K> {
        tree_floor(&self.root, key)
    }
    pub fn ceiling(&self, key: &K) -> Option<&K> {
        tree_ceiling(&self.root, key)
    }
    pub fn select(&self, i: usize) -> Option<&K> {
        tree_select(&self.root, i)
    }
    pub fn rank(&self, key: &K) -> usize {
        tree_rank(&self.root, key)
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.root.as_ref().and_then(|p| node_get(p, key))
    }
    pub fn put(&mut self, key: K, value: V) {
        let a = self.root.take();
        self.root = Some(node_put(a, key, value));
    }

    pub fn contains(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    pub fn delete_min(&mut self) {
        let r = self.root.take();
        self.root = tree_delete_min(r);
    }
    pub fn delete_max(&mut self) {
        let r = self.root.take();
        self.root = tree_delete_max(r);
    }
    pub fn delete(&mut self, key: &K) {
        let r = self.root.take();
        self.root = tree_delete(r, key);
    }
    pub fn pop_min(&mut self) -> Option<(K, V)> {
        self.root.take().map(|b| {
            let (r, x) = node_pop_min(b);
            self.root = r;
            (x.key, x.value)
        })
    }

    pub fn keys(&self) -> Vec<&K> {
        let mut q: Vec<&K> = Vec::new();
        tree_keys(&self.root, &mut q);
        q
    }
    pub fn keys_range(&self, lo: &K, hi: &K) -> Vec<&K> {
        let mut q: Vec<&K> = Vec::new();
        tree_keys_range(&self.root, &mut q, lo, hi);
        q
    }
    pub fn iter(&self) -> Iter<'_, K, V> {
        Iter {
            stack: Vec::new(),
            current: self.root.as_deref(),
        }
    }

    pub fn check(&self) -> bool {
        self.keys().windows(2).all(|w| w[0] <= w[1])
    }
}

// impl<K,V> Tree<K, V>
fn tree_size<K: Ord, V>(t: &Tree<K, V>) -> usize {
    t.as_ref().map_or(0, |p| p.size)
}

fn tree_floor<'a, K: Ord, V>(t: &'a Tree<K, V>, key: &K) -> Option<&'a K> {
    t.as_ref().and_then(|p| match key.cmp(&p.key) {
        Ordering::Equal => Some(&p.key),
        Ordering::Less => tree_floor(&p.left, key),
        Ordering::Greater => tree_floor(&p.right, key).or(Some(&p.key)),
    })
}
fn tree_ceiling<'a, K: Ord, V>(t: &'a Tree<K, V>, key: &K) -> Option<&'a K> {
    t.as_ref().and_then(|p| match key.cmp(&p.key) {
        Ordering::Equal => Some(&p.key),
        Ordering::Less => tree_ceiling(&p.left, key).or(Some(&p.key)),
        Ordering::Greater => tree_ceiling(&p.right, key),
    })
}
fn tree_select<K: Ord, V>(t: &Tree<K, V>, i: usize) -> Option<&K> {
    t.as_ref().and_then(|b| {
        let ls = b.left.as_ref().map_or(0, |b| b.size);
        match i.cmp(&ls) {
            Ordering::Equal => Some(&b.key),
            Ordering::Less => tree_select(&b.left, i),
            Ordering::Greater => tree_select(&b.right, i - ls - 1),
        }
    })
}
fn tree_rank<K: Ord, V>(t: &Tree<K, V>, key: &K) -> usize {
    t.as_ref().map_or(0, |b| match key.cmp(&b.key) {
        Ordering::Equal => b.left.as_ref().map_or(0, |b| b.size),
        Ordering::Less => tree_rank(&b.left, key),
        Ordering::Greater => tree_rank(&b.left, key) + 1 + tree_rank(&b.right, key),
    })
}

fn tree_delete_min<K: Ord, V>(t: Tree<K, V>) -> Tree<K, V> {
    t.and_then(|mut b| match b.left {
        None => b.right,
        Some(l) => {
            b.left = tree_delete_min(Some(l));
            b.size = 1 + tree_size(&b.left) + tree_size(&b.right);
            Some(b)
        }
    })
}
fn tree_delete_max<K: Ord, V>(t: Tree<K, V>) -> Tree<K, V> {
    t.and_then(|mut b| match b.right {
        None => b.left,
        Some(r) => {
            b.right = tree_delete_max(Some(r));
            b.size = 1 + tree_size(&b.left) + tree_size(&b.right);
            Some(b)
        }
    })
}
fn tree_delete<K: Ord, V>(t: Tree<K, V>, key: &K) -> Tree<K, V> {
    t.and_then(|mut b| {
        match key.cmp(&b.key) {
            Ordering::Less => b.left = tree_delete(b.left, key),
            Ordering::Greater => b.right = tree_delete(b.right, key),
            _ => {
                if b.right.is_none() {
                    return b.left;
                }
                if b.left.is_none() {
                    return b.right;
                }

                // use min of right sub-tree as the new node
                let t = b.left.take();
                let (child, x) = node_pop_min(b.right.unwrap());
                b = x;
                b.right = child;
                b.left = t;
            }
        }
        b.size = 1 + tree_size(&b.left) + tree_size(&b.right);
        Some(b)
    })
}

fn tree_keys<'a, K: Ord, V>(t: &'a Tree<K, V>, q: &mut Vec<&'a K>) {
    if let Some(ref b) = t {
        tree_keys(&b.left, q);
        q.push(&b.key);
        tree_keys(&b.right, q);
    }
}
fn tree_keys_range<'a, K: Ord, V>(t: &'a Tree<K, V>, q: &mut Vec<&'a K>, lo: &K, hi: &K) {
    if let Some(ref b) = t {
        if *lo < b.key {
            tree_keys_range(&b.left, q, lo, hi);
        }
        if *lo <= b.key && b.key <= *hi {
            q.push(&b.key);
        }
        if *hi > b.key {
            tree_keys_range(&b.right, q, lo, hi);
        }
    }
}

// impl<K, V> Node<K, V>
fn node_min<K: Ord, V>(node: &Node<K, V>) -> &K {
    node.left.as_ref().map_or(&node.key, |v| node_min(v))
}
fn node_max<K: Ord, V>(node: &Node<K, V>) -> &K {
    node.right.as_ref().map_or(&node.key, |v| node_max(v))
}

fn node_get<'a, K: Ord, V>(node: &'a Node<K, V>, key: &K) -> Option<&'a V> {
    match key.cmp(&node.key) {
        Ordering::Equal => Some(&node.value),
        Ordering::Less => node.left.as_ref().and_then(|n| node_get(n, key)),
        Ordering::Greater => node.right.as_ref().and_then(|n| node_get(n, key)),
    }
}
fn node_put<K: Ord, V>(t: Tree<K, V>, key: K, value: V) -> Box<Node<K, V>> {
    match t {
        None => Box::new(Node::new(key, value)),
        Some(mut b) => {
            match key.cmp(&b.key) {
                Ordering::Equal => b.value = value,
                Ordering::Less => b.left = Some(node_put(b.left, key, value)),
                Ordering::Greater => b.right = Some(node_put(b.right, key, value)),
            };
            b.size = 1 + tree_size(&b.left) + tree_size(&b.right);
            b
        }
    }
}
fn node_pop_min<K: Ord, V>(mut b: Box<Node<K, V>>) -> (Tree<K, V>, Box<Node<K, V>>) {
    match b.left {
        None => (b.right.take(), b),
        Some(l) => {
            let (child, min) = node_pop_min(l);
            b.left = child;
            b.size = 1 + tree_size(&b.left) + tree_size(&b.right);
            (Some(b), min)
        }
    }
}

impl<K: Ord, V> IntoIterator for BSTree<K, V> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        // take self
        IntoIter(self)
    }
}
/// own the tree
pub struct IntoIter<K, V>(BSTree<K, V>);

impl<K: Ord, V> Iterator for IntoIter<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_min()
    }
}

/// Iterating as inorder traversal
pub struct Iter<'a, K, V> {
    stack: Vec<&'a Node<K, V>>,
    current: Option<&'a Node<K, V>>,
}
impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        // push left childen for visiting (travel to the min)
        while let Some(l) = self.current {
            self.stack.push(l);
            self.current = l.left.as_deref();
        }

        // process the top in stack and point current to the right child
        self.stack.pop().map(|n| {
            if let Some(ref r) = n.right {
                self.current = Some(r);
            }
            (&n.key, &n.value)
        })
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

        st.delete_min();
        assert_eq!(2, st.len());
        assert_eq!(Some(&"blackberry".into()), st.min());

        st.delete_max();
        assert_eq!(1, st.len());
        assert_eq!(Some(&"blackberry".into()), st.min());
        assert_eq!(Some(&"blackberry".into()), st.max());
    }

    #[test]
    fn order_methods() {
        let mut st = BSTree::<String, usize>::new();
        for c in "SEACRHMX".chars() {
            st.put(c.to_string(), 1);
        }
        assert_eq!(8, st.len());

        assert_eq!(None, st.floor(&"0".into()));
        assert_eq!(Some(&"E".into()), st.floor(&"E".into()));
        assert_eq!(Some(&"E".into()), st.floor(&"G".into()));
        assert_eq!(Some(&"C".into()), st.floor(&"C".into()));
        assert_eq!(Some(&"S".into()), st.floor(&"S".into()));
        assert_eq!(Some(&"X".into()), st.floor(&"Z".into()));

        assert_eq!(Some(&"A".into()), st.ceiling(&"0".into()));
        assert_eq!(Some(&"A".into()), st.ceiling(&"A".into()));
        assert_eq!(Some(&"X".into()), st.ceiling(&"X".into()));
        assert_eq!(Some(&"M".into()), st.ceiling(&"M".into()));
        assert_eq!(Some(&"C".into()), st.ceiling(&"B".into()));
        assert_eq!(Some(&"H".into()), st.ceiling(&"G".into()));
        assert_eq!(None, st.ceiling(&"Z".into()));

        assert_eq!(Some(&"A".into()), st.select(0));
        assert_eq!(Some(&"C".into()), st.select(1));
        assert_eq!(Some(&"S".into()), st.select(6));
        assert_eq!(Some(&"X".into()), st.select(7));
        assert_eq!(None, st.select(8));
        assert_eq!(None, st.select(9));

        assert_eq!(0, st.rank(&"0".into()));
        assert_eq!(0, st.rank(&"A".into()));
        assert_eq!(1, st.rank(&"B".into()));
        assert_eq!(1, st.rank(&"C".into()));
        assert_eq!(2, st.rank(&"D".into()));
        assert_eq!(6, st.rank(&"S".into()));
        assert_eq!(7, st.rank(&"X".into()));
        assert_eq!(8, st.rank(&"Z".into()));

        for i in 0..8 {
            assert_eq!(i, st.rank(st.select(i).unwrap()));
        }
    }

    #[test]
    fn deletions() {
        let mut st = BSTree::<String, usize>::new();
        for c in "SEACRHMX".chars() {
            st.put(c.to_string(), 1);
        }
        assert_eq!(8, st.len());

        st.delete(&"A".into());
        assert_eq!(7, st.len());
        assert_eq!(Some(&"C".into()), st.min());
        assert_eq!(Some(&"X".into()), st.max());

        st.delete(&"M".into());
        assert_eq!(6, st.len());

        st.delete(&"S".into());
        assert_eq!(5, st.len());
        assert_eq!(Some(&"C".into()), st.min());
        assert_eq!(Some(&"X".into()), st.max());
    }

    #[test]
    fn keys() {
        let mut st = BSTree::<String, usize>::new();
        for c in "SEACRHMX".chars() {
            st.put(c.to_string(), 1);
        }

        let keys = st.keys().iter().fold(String::new(), |acc, v| acc + v);
        assert_eq!("ACEHMRSX", keys);

        assert!(st.check());
    }

    #[test]
    fn keys_range() {
        let mut st = BSTree::<u32, u32>::new();
        for i in 1..10 {
            st.put(i, i * 2);
        }
        assert!(st.check());

        let v = st.keys_range(&1, &3);
        assert_eq!([&1, &2, &3], v.as_slice());
        let v = st.keys_range(&0, &3);
        assert_eq!([&1, &2, &3], &v[..]);
        let v = st.keys_range(&8, &9);
        assert_eq!([&8, &9], &v[..]);
        let v = st.keys_range(&8, &20);
        assert_eq!([&8, &9], &v[..]);
    }

    #[test]
    fn keys_iter() {
        let mut st = BSTree::<String, usize>::new();
        for c in "SEACRHMX".chars() {
            st.put(c.to_string(), 1);
        }

        let mut it = st.iter();
        assert_eq!(Some((&"A".into(), &1)), it.next());
        assert_eq!(Some((&"C".into(), &1)), it.next());
        assert_eq!(Some((&"E".into(), &1)), it.next());
        assert_eq!(Some((&"H".into(), &1)), it.next());
        assert_eq!(Some((&"M".into(), &1)), it.next());
        assert_eq!(Some((&"R".into(), &1)), it.next());
        assert_eq!(Some((&"S".into(), &1)), it.next());
        assert_eq!(Some((&"X".into(), &1)), it.next());
        assert_eq!(None, it.next());
    }

    #[test]
    fn random_100() {
        let mut st = BSTree::<u32, u32>::new();
        for i in thread_rng().gen_iter::<u32>().take(100) {
            st.put(i, i);
        }
        let (mut last_k, last_v) = st.pop_min().unwrap();
        assert_eq!(last_k, last_v);
        for _i in 1..100 {
            let (k, v) = st.pop_min().unwrap();
            assert_eq!(k, v);
            assert!(last_k <= k);
            last_k = k;
        }
    }

    #[test]
    fn random_100_into_iter() {
        let mut st = BSTree::<u32, u32>::new();
        for i in thread_rng().gen_iter::<u32>().take(100) {
            st.put(i, i);
        }
        st.into_iter()
            .collect::<Vec<_>>()
            .windows(2)
            .all(|w| w[0].0 <= w[1].0);
    }
}
