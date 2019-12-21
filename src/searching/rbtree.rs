use std::cmp::Ordering::{Equal, Greater, Less};
use std::fmt;

#[derive(Clone, PartialEq, Eq)]
enum Color {
    Black,
    Red,
}
use Color::*;
impl Color {
    fn is_red(&self) -> bool {
        self == &Red
    }
    fn flip(&mut self) {
        *self = if self.is_red() { Black } else { Red };
    }
    fn flip_red(&mut self) -> bool {
        if self.is_red() {
            *self = Black;
            true
        } else {
            false
        }
    }
}

type NodePtr<K, V> = Box<Node<K, V>>;
type List<K, V> = Option<NodePtr<K, V>>;
pub struct Node<K, V> {
    color: Color,
    key: K,
    value: V,
    left: List<K, V>,
    right: List<K, V>,
    size: usize,
}
impl<K: fmt::Debug, V> fmt::Display for Node<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}({:?}):left={:?},right={:?}",
            if let Red = self.color { "Red" } else { "Black" },
            &self.key,
            self.left.as_ref().map(|n| &n.key),
            self.right.as_ref().map(|n| &n.key),
        )
    }
}

fn is_red<K, V>(list: &List<K, V>) -> bool {
    list.as_ref().map_or(false, |n| n.color.is_red())
}
fn is_red_left_child<K, V>(list: &List<K, V>) -> bool {
    list.as_ref().map_or(false, |n| is_red(&n.left))
}
fn is_red_right_child<K, V>(list: &List<K, V>) -> bool {
    list.as_ref().map_or(false, |n| is_red(&n.right))
}
fn flip_red<K, V>(list: &mut List<K, V>) -> bool {
    list.as_mut().map_or(false, |n| n.color.flip_red())
}

#[derive(Default)]
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

    pub fn iter(&self) -> Iter<'_, K, V> {
        Iter {
            stack: Vec::new(),
            current: self.root.as_ref().map(|r| &**r),
        }
    }

    pub fn check(&self) -> bool {
        self.is_bst() && self.is_234() && self.is_balanced()
    }
    pub fn check_error(&self) -> Option<String> {
        if !self.is_bst() {
            return Some("Not a BST".into());
        }
        if !self.is_234() {
            return Some("Not a 234 tree".into());
        }
        if !self.is_balanced() {
            return Some("Not balanced".into());
        }
        None
    }
    pub fn is_bst(&self) -> bool {
        Self::_is_bst(&self.root, None, None)
    }
    pub fn is_234(&self) -> bool {
        Self::_is_234(&self.root)
    }
    pub fn is_balanced(&self) -> bool {
        let mut height: isize = 0;
        let mut x = &self.root;
        while let Some(ref b) = x {
            if !b.color.is_red() {
                height += 1;
            }
            x = &b.left;
        }
        Self::_is_balanced(&self.root, height)
    }

    pub fn contains(&self, key: &K) -> bool {
        self.get(key).is_some()
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

    pub fn delete_min(&mut self) {
        let (mut n, _) = Self::_delete_min(self.root.take());
        if let Some(r) = n.as_mut() {
            r.color = Black;
        }
        self.root = n;
    }
    pub fn delete_max(&mut self) {
        let (mut n, _) = Self::_delete_max(self.root.take());
        if let Some(r) = n.as_mut() {
            r.color = Black;
        }
        self.root = n;
    }
    pub fn delete(&mut self, key: &K) {
        let (mut n, _) = Self::_delete(self.root.take(), key);
        if let Some(r) = n.as_mut() {
            r.color = Black;
        }
        self.root = n;
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
    fn _is_234(list: &List<K, V>) -> bool {
        match list {
            None => true,
            Some(ref node) => match (&node.color, is_red(&node.left) || is_red(&node.right)) {
                (Red, true) => false,
                _ => Self::_is_234(&node.left) && Self::_is_234(&node.right),
            },
        }
    }
    fn _is_balanced(list: &List<K, V>, mut height: isize) -> bool {
        match list {
            None => height == 0,
            Some(ref b) => {
                if !b.color.is_red() {
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
    fn _put(&mut self, list: List<K, V>, key: K, value: V) -> NodePtr<K, V> {
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
                // elimate the case where P's Sibling is RED
                if is_red(&b.left) && is_red(&b.right) {
                    Self::flip_colors(&mut b);
                }

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

                b.size = 1 + Self::size(&b.left) + Self::size(&b.right);
                b
            }
        }
    }

    fn _delete_min(list: List<K, V>) -> (List<K, V>, bool) {
        list.map_or((None, true), |mut b| match b.left {
            None => Self::fix_self_with_right_child(&mut b),
            Some(_) => {
                let (child, balanced) = Self::_delete_min(b.left);
                b.left = child;
                b.size = 1 + Self::size(&b.left) + Self::size(&b.right);
                if balanced {
                    return (Some(b), true);
                }
                Self::fix_left_with_sibling(b)
            }
        })
    }
    fn _delete_max(list: List<K, V>) -> (List<K, V>, bool) {
        list.map_or((None, true), |mut b| match b.right {
            None => Self::fix_self_with_left_child(&mut b),
            Some(_) => {
                let (child, balanced) = Self::_delete_max(b.right);
                b.right = child;
                b.size = 1 + Self::size(&b.left) + Self::size(&b.right);
                if balanced {
                    return (Some(b), true);
                }
                Self::fix_right_with_sibling(b)
            }
        })
    }
    fn _delete(list: List<K, V>, key: &K) -> (List<K, V>, bool) {
        list.map_or((None, true), |mut b| {
            let balanced: bool;
            let mut is_left = false;
            match key.cmp(&b.key) {
                Less => {
                    let (child, sub_b) = Self::_delete(b.left, key);
                    b.left = child;
                    balanced = sub_b;
                    is_left = true;
                }
                Greater => {
                    let (child, sub_b) = Self::_delete(b.right, key);
                    b.right = child;
                    balanced = sub_b;
                }
                Equal => {
                    if b.left.is_none() {
                        return Self::fix_self_with_right_child(&mut b);
                    }
                    if b.right.is_none() {
                        return Self::fix_self_with_left_child(&mut b);
                    }

                    // replace with b's successor (min of right sub-tree)
                    let (x, mut s, sub_b) = Self::_pop_min(b.right.unwrap());
                    b.right = x;
                    std::mem::swap(&mut b.key, &mut s.key);
                    std::mem::swap(&mut b.value, &mut s.value);
                    balanced = sub_b;
                }
            }

            b.size = 1 + Self::size(&b.left) + Self::size(&b.right);
            if balanced {
                (Some(b), balanced)
            } else if is_left {
                Self::fix_left_with_sibling(b)
            } else {
                Self::fix_right_with_sibling(b)
            }
        })
    }
    fn _pop_min(mut b: NodePtr<K, V>) -> (List<K, V>, NodePtr<K, V>, bool) {
        match b.left {
            None => {
                let (x, balanced) = Self::fix_self_with_right_child(&mut b);
                (x, b, balanced)
            }
            Some(left) => {
                let (x, min, sub_b) = Self::_pop_min(left);
                b.left = x;
                b.size = 1 + Self::size(&b.left) + Self::size(&b.right);

                if sub_b {
                    (Some(b), min, true)
                } else {
                    let (x, sub_b) = Self::fix_left_with_sibling(b);
                    (x, min, sub_b)
                }
            }
        }
    }

    fn rotate_left(mut node: NodePtr<K, V>) -> NodePtr<K, V> {
        let mut x = node.right.unwrap();
        node.right = x.left.take();
        x.color = node.color.clone();
        node.color = Red;
        x.size = node.size;
        node.size = Self::size(&node.left) + 1 + Self::size(&node.right);
        x.left = Some(node);
        x
    }
    fn rotate_right(mut node: NodePtr<K, V>) -> NodePtr<K, V> {
        let mut x = node.left.unwrap();
        node.left = x.right.take();
        x.color = node.color.clone();
        node.color = Red;
        x.size = node.size;
        node.size = Self::size(&node.left) + 1 + Self::size(&node.right);
        x.right = Some(node);
        x
    }
    fn flip_colors(node: &mut NodePtr<K, V>) {
        // n must have different color with its children
        node.color.flip();
        if let Some(n) = node.left.as_mut() {
            n.color.flip();
        }
        if let Some(n) = node.right.as_mut() {
            n.color.flip();
        }
    }

    fn fix_self_with_right_child(b: &mut NodePtr<K, V>) -> (List<K, V>, bool) {
        if b.color.is_red() {
            // no impact on black-height
            (b.right.take(), true)
        } else if flip_red(&mut b.right) {
            // add one to black-height
            (b.right.take(), true)
        } else {
            (b.right.take(), false)
        }
    }
    fn fix_self_with_left_child(b: &mut NodePtr<K, V>) -> (List<K, V>, bool) {
        if b.color.is_red() {
            // no impact on black-height
            (b.left.take(), true)
        } else if flip_red(&mut b.left) {
            // add one to black-height
            (b.left.take(), true)
        } else {
            (b.left.take(), false)
        }
    }

    // fix left sub-tree lost one black-height
    fn fix_left_with_sibling(mut b: NodePtr<K, V>) -> (List<K, V>, bool) {
        if is_red(&b.right) {
            // case: left_S
            b = Self::rotate_left(b);
            // go one layer down to fix
            let (child, balanced) = Self::fix_left_black_s(b.left.unwrap());
            b.left = child;
            if balanced {
                return (Some(b), true);
            }
        }

        Self::fix_left_black_s(b)
    }
    fn fix_left_black_s(mut b: NodePtr<K, V>) -> (List<K, V>, bool) {
        if is_red_left_child(&b.right) && !is_red_right_child(&b.right) {
            // case: left_SL, transfer to left_SR
            b.right = Some(Self::rotate_right(b.right.unwrap()));
        }

        if is_red_right_child(&b.right) {
            // case: left_SR, borrow one black-height from S sub-tree
            b = Self::rotate_left(b);
            if let Some(n) = b.left.as_mut() {
                n.color = Black;
            }
            if let Some(n) = b.right.as_mut() {
                n.color = Black;
            }

            (Some(b), true)
        } else {
            // case: S/SL/SR are all black
            // if red: path P->X add one black-height while P->S stays same
            // if black: sub one black-height from right sub-tree and escalate to up
            let balanced = b.color.is_red();
            b.color = Black;
            if let Some(n) = b.right.as_mut() {
                n.color = Red;
            }

            (Some(b), balanced)
        }
    }

    // fix right sub-tree lost one black-height
    fn fix_right_with_sibling(mut b: NodePtr<K, V>) -> (List<K, V>, bool) {
        if is_red(&b.left) {
            // case: right_S
            b = Self::rotate_right(b);
            // go one layer down to fix
            let (child, balanced) = Self::fix_right_black_s(b.right.unwrap());
            b.right = child;
            if balanced {
                return (Some(b), true);
            }
        }

        Self::fix_right_black_s(b)
    }
    fn fix_right_black_s(mut b: NodePtr<K, V>) -> (List<K, V>, bool) {
        if !is_red_left_child(&b.left) && is_red_right_child(&b.left) {
            // case: red SR, transfer to red SL
            b.left = Some(Self::rotate_left(b.left.unwrap()));
        }

        if is_red_left_child(&b.left) {
            // case: red SL, borrow one black-height from S sub-tree
            b = Self::rotate_right(b);
            if let Some(n) = b.left.as_mut() {
                n.color = Black;
            }
            if let Some(n) = b.right.as_mut() {
                n.color = Black;
            }

            (Some(b), true)
        } else {
            // case: S/SL/SR are all black
            // if red: path P->X add one black-height while P->S stays same
            // if black: sub one black-height from right sub-tree and escalate to up
            let balanced = b.color.is_red();
            b.color = Black;
            if let Some(n) = b.left.as_mut() {
                n.color = Red;
            }

            (Some(b), balanced)
        }
    }
}
/// Iterating as inorder traversal
pub struct Iter<'a, K, V> {
    stack: Vec<&'a Node<K, V>>,
    current: Option<&'a Node<K, V>>,
}
impl<'a, K, V> Iterator for Iter<'a, K, V> {
    // type Item = (&'a K, &'a V);
    type Item = &'a Node<K, V>;

    fn next(&mut self) -> Option<Self::Item> {
        // push left childen for visiting (travel to the min)
        while let Some(ref l) = self.current {
            self.stack.push(l);
            self.current = l.left.as_ref().map(|x| &**x);
        }

        // process the top in stack and point current to the right child
        self.stack.pop().map(|n| {
            if let Some(ref r) = n.right {
                self.current = Some(r);
            }
            // (&n.key, &n.value)
            n
        })
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

    #[test]
    fn ordered_insertion() {
        let mut st = RBTree::<usize, usize>::new();
        for i in 0..10 {
            st.put(i, i)
        }
        assert_eq!(10, st.len());
        assert!(st.check());
    }

    #[test]
    fn delete_min() {
        let mut st = RBTree::<usize, usize>::new();
        for i in 0..10 {
            st.put(i, i);
        }

        st.delete_min();
        assert_eq!(9, st.len());
        assert_eq!(Some(&1), st.min());
        assert_eq!(None, st.check_error());

        for i in 1..9 {
            st.delete_min();
            assert_eq!(9 - i, st.len());
            assert_eq!(&(i + 1), st.min().unwrap());
            assert_eq!(None, st.check_error());
        }

        st.delete_min();
        assert_eq!(0, st.len());
        assert_eq!(None, st.min());
        assert_eq!(None, st.check_error());
    }
    #[test]
    fn delete_max() {
        let mut st = RBTree::<usize, usize>::new();
        for i in 0..10 {
            st.put(i, i);
        }

        for i in (1..10).rev() {
            st.delete_max();
            assert_eq!(i, st.len());
            assert_eq!(&(i - 1), st.max().unwrap());
            assert_eq!(None, st.check_error());
        }

        st.delete_max();
        assert_eq!(0, st.len());
        assert_eq!(None, st.max());
        assert_eq!(None, st.check_error());
    }

    #[test]
    fn delete_basic() {
        let mut st = RBTree::<usize, usize>::new();
        for i in 0..20 {
            st.put(i, i);
        }

        st.delete(&0);
        // println!("====1====");
        // for n in st.iter() {
        //     println!("{}", n);
        // }
        assert_eq!(19, st.len());
        assert_eq!(&1, st.min().unwrap());
        assert_eq!(&19, st.max().unwrap());
        assert_eq!(None, st.check_error());

        st.delete(&5);
        assert_eq!(18, st.len());
        assert_eq!(&1, st.min().unwrap());
        assert_eq!(&19, st.max().unwrap());
        assert_eq!(None, st.check_error());

        st.delete(&15);
        assert_eq!(17, st.len());
        assert_eq!(&1, st.min().unwrap());
        assert_eq!(&19, st.max().unwrap());
        assert_eq!(None, st.check_error());

        st.delete(&1);
        assert_eq!(16, st.len());
        assert_eq!(&2, st.min().unwrap());
        assert_eq!(&19, st.max().unwrap());
        assert_eq!(None, st.check_error());

        st.delete(&6);
        assert_eq!(15, st.len());
        assert_eq!(&2, st.min().unwrap());
        assert_eq!(&19, st.max().unwrap());
        assert_eq!(None, st.check_error());

        st.delete(&16);
        assert_eq!(14, st.len());
        assert_eq!(&2, st.min().unwrap());
        assert_eq!(&19, st.max().unwrap());
        assert_eq!(None, st.check_error());

        st.delete(&17);
        assert_eq!(13, st.len());
        assert_eq!(&2, st.min().unwrap());
        assert_eq!(&19, st.max().unwrap());
        assert_eq!(None, st.check_error());

        st.delete(&9);
        assert_eq!(12, st.len());
        assert_eq!(&2, st.min().unwrap());
        assert_eq!(&19, st.max().unwrap());
        assert_eq!(None, st.check_error());
    }
}
