use std::cmp::Ordering::{Equal, Greater, Less};

#[derive(Clone)]
enum Color {
    Black,
    Red,
}
use Color::*;

type List<K, V> = Option<Box<Node<K, V>>>;
struct Node<K, V> {
    color: Color,
    key: K,
    value: V,
    left: List<K, V>,
    right: List<K, V>,
    size: usize,
}
fn is_red<K, V>(list: &List<K, V>) -> bool {
    list.as_ref().map_or(false, |n| match n.color {
        Red => true,
        _ => false,
    })
}

pub struct RBTree<K: Ord, V> {
    root: List<K, V>,
}

impl<K: Ord, V> RBTree<K, V> {
    pub fn new() -> Self {
        RBTree { root: None }
    }

    pub fn len(&self) -> usize {
        self.root.as_ref().map_or(0, |p| p.size)
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn min(&self) -> Option<&K> {
        self.root.as_ref().map(|p| Self::_min(p))
    }
    pub fn max(&self) -> Option<&K> {
        self.root.as_ref().map(|p| Self::_max(p))
    }
    pub fn floor(&self, key: &K) -> Option<&K> {
        Self::_floor(&self.root, key)
    }
    pub fn ceiling(&self, key: &K) -> Option<&K> {
        Self::_ceiling(&self.root, key)
    }
    pub fn select(&self, i: usize) -> Option<&K> {
        Self::_select(&self.root, i)
    }
    pub fn rank(&self, key: &K) -> usize {
        Self::_rank(&self.root, key)
    }

    pub fn check(&self) -> bool {
        self.is_bst() && self.is_23() && self.is_balanced()
    }
    pub fn is_bst(&self) -> bool {
        Self::_is_bst(&self.root, None, None)
    }
    pub fn is_23(&self) -> bool {
        Self::_is_23(&self.root)
    }
    pub fn is_balanced(&self) -> bool {
        let mut height = 0;
        let mut x = &self.root;
        while let Some(ref b) = x {
            if let Black = b.color {
                height += 1;
            }
            x = &b.left;
        }
        Self::_is_balanced(&self.root, height)
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.root.as_ref().and_then(|p| self._get(p, key))
    }
    pub fn put(&mut self, key: K, value: V) {
        let a = self.root.take();
        let mut n = self._put(a, key, value);
        n.color = Black;
        self.root = Some(n);
    }

    pub fn contains(&self, key: &K) -> bool {
        self.get(key).is_some()
    }
}

// private methods
impl<K: Ord, V> RBTree<K, V> {
    fn size(list: &List<K, V>) -> usize {
        list.as_ref().map_or(0, |p| p.size)
    }

    fn _min<'a>(node: &'a Node<K, V>) -> &'a K {
        node.left.as_ref().map_or(&node.key, |v| Self::_min(&v))
    }
    fn _max<'a>(node: &'a Node<K, V>) -> &'a K {
        node.right.as_ref().map_or(&node.key, |v| Self::_max(&v))
    }

    fn _floor<'a>(list: &'a List<K, V>, key: &K) -> Option<&'a K> {
        list.as_ref().and_then(|p| match key.cmp(&p.key) {
            Equal => Some(&p.key),
            Less => Self::_floor(&p.left, key),
            Greater => Self::_floor(&p.right, key).or(Some(&p.key)),
        })
    }
    fn _ceiling<'a>(list: &'a List<K, V>, key: &K) -> Option<&'a K> {
        list.as_ref().and_then(|p| match key.cmp(&p.key) {
            Equal => Some(&p.key),
            Less => Self::_ceiling(&p.left, key).or(Some(&p.key)),
            Greater => Self::_ceiling(&p.right, key),
        })
    }
    fn _select<'a>(list: &'a List<K, V>, i: usize) -> Option<&'a K> {
        list.as_ref().and_then(|b| {
            let ls = b.left.as_ref().map_or(0, |b| b.size);
            match i.cmp(&ls) {
                Equal => Some(&b.key),
                Less => Self::_select(&b.left, i),
                Greater => Self::_select(&b.right, i - ls - 1),
            }
        })
    }
    fn _rank(list: &List<K, V>, key: &K) -> usize {
        list.as_ref().map_or(0, |b| match key.cmp(&b.key) {
            Equal => b.left.as_ref().map_or(0, |b| b.size),
            Less => Self::_rank(&b.left, key),
            Greater => Self::_rank(&b.left, key) + 1 + Self::_rank(&b.right, key),
        })
    }

    fn _is_bst(list: &List<K, V>, min: Option<&K>, max: Option<&K>) -> bool {
        match list {
            None => true,
            Some(ref node) => {
                min.map_or(true, |v| &node.key > v)
                    && max.map_or(true, |v| &node.key < v)
                    && Self::_is_bst(&node.left, min, Some(&node.key))
                    && Self::_is_bst(&node.right, Some(&node.key), max)
            }
        }
    }
    fn _is_23(list: &List<K, V>) -> bool {
        match list {
            None => true,
            Some(ref node) => match (&node.color, is_red(&node.left) || is_red(&node.right)) {
                (Red, true) => false,
                _ => Self::_is_23(&node.left) && Self::_is_23(&node.right),
            },
        }
    }
    fn _is_balanced(list: &List<K, V>, mut height: usize) -> bool {
        match list {
            None => height == 0,
            Some(ref b) => {
                if let Black = b.color {
                    height -= 1;
                }
                Self::_is_balanced(&b.left, height) && Self::_is_balanced(&b.right, height)
            }
        }
    }

    fn _get<'a>(&self, node: &'a Node<K, V>, key: &K) -> Option<&'a V> {
        match key.cmp(&node.key) {
            Equal => Some(&node.value),
            Less => node.left.as_ref().and_then(|n| self._get(&n, key)),
            Greater => node.right.as_ref().and_then(|n| self._get(&n, key)),
        }
    }
    fn _put(&mut self, list: List<K, V>, key: K, value: V) -> Box<Node<K, V>> {
        match list {
            None => Box::new(Node {
                color: Red,
                key: key,
                value: value,
                left: None,
                right: None,
                size: 1,
            }),
            Some(mut b) => {
                match key.cmp(&b.key) {
                    Equal => b.value = value,
                    Less => b.left = Some(self._put(b.left, key, value)),
                    Greater => b.right = Some(self._put(b.right, key, value)),
                };

                // re-balance
                if !is_red(&b.left) && is_red(&b.right) {
                    b = Self::rotate_left(b);
                }
                if is_red(&b.left) && b.left.as_ref().map_or(false, |l| is_red(&l.left)) {
                    b = Self::rotate_right(b);
                }
                if is_red(&b.left) && is_red(&b.right) {
                    Self::flip_colors(&mut b);
                }

                b.size = 1 + Self::size(&b.left) + Self::size(&b.right);
                b
            }
        }
    }

    fn rotate_left(mut node: Box<Node<K, V>>) -> Box<Node<K, V>> {
        let mut x = node.right.unwrap();
        node.right = x.left.take();
        x.color = node.color.clone();
        node.color = Red;
        x.size = node.size;
        node.size = Self::size(&node.left) + 1 + Self::size(&node.right);
        x.left = Some(node);
        x
    }
    fn rotate_right(mut node: Box<Node<K, V>>) -> Box<Node<K, V>> {
        let mut x = node.left.unwrap();
        node.left = x.right.take();
        x.color = node.color.clone();
        node.color = Red;
        x.size = node.size;
        node.size = Self::size(&node.left) + 1 + Self::size(&node.right);
        x.right = Some(node);
        x
    }
    fn flip_colors(mut node: &mut Box<Node<K, V>>) {
        node.color = Red;
        if let Some(n) = node.left.as_mut() {
            n.color = Black;
        }
        if let Some(n) = node.right.as_mut() {
            n.color = Black;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use rand::{thread_rng, Rng};

    #[test]
    fn empty() {
        let st = RBTree::<String, usize>::new();
        assert_eq!(0, st.len());
        assert_eq!(None, st.min());
        assert_eq!(None, st.max());
        assert_eq!(None, st.get(&"any".into()));
        assert!(st.check());
    }

    #[test]
    fn put_get() {
        let mut st = RBTree::<String, usize>::new();
        st.put("iphone".into(), 500);
        assert_eq!(1, st.len());
        assert_eq!(Some(&"iphone".into()), st.min());
        assert_eq!(Some(&"iphone".into()), st.max());
        assert_eq!(Some(&500), st.get(&"iphone".into()));
        assert!(st.check());
    }

    #[test]
    fn update() {
        let mut st = RBTree::<String, usize>::new();
        st.put("iphone".into(), 500);
        st.put("iphone".into(), 250);
        assert_eq!(1, st.len());
        assert_eq!(Some(&250), st.get(&"iphone".into()));
        assert!(st.contains(&"iphone".into()));
        assert!(st.check());
    }

    #[test]
    fn get_one_of_three() {
        let mut st = RBTree::<String, usize>::new();
        st.put("iphone".into(), 600);
        st.put("android".into(), 500);
        st.put("blackberry".into(), 800);
        assert_eq!(3, st.len());
        assert_eq!(Some(&"android".into()), st.min());
        assert_eq!(Some(&"iphone".into()), st.max());
        assert_eq!(Some(&800), st.get(&"blackberry".into()));
        assert!(st.check());
    }

    #[test]
    fn order_methods() {
        let mut st = RBTree::<String, usize>::new();
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

        assert!(st.check());
    }
}
