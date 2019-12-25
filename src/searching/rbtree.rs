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
#[derive(Default)]
struct Tree<K, V>(Option<NodePtr<K, V>>);
pub struct Node<K, V> {
    color: Color,
    key: K,
    value: V,
    left: Tree<K, V>,
    right: Tree<K, V>,
    size: usize,
}
impl<K: fmt::Debug, V> fmt::Display for Node<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}({:?}):left={:?},right={:?}",
            if let Red = self.color { "Red" } else { "Black" },
            &self.key,
            self.left.0.as_ref().map(|n| &n.key),
            self.right.0.as_ref().map(|n| &n.key),
        )
    }
}
impl<K: Ord, V> Node<K, V> {
    fn new(key: K, value: V) -> Self {
        Node {
            color: Red,
            key: key,
            value: value,
            left: Tree::new(),
            right: Tree::new(),
            size: 1,
        }
    }
    fn flip_colors(&mut self) {
        // n must have different color with its children
        self.color.flip();
        self.left.flip_color();
        self.right.flip_color();
    }
}

#[derive(Default)]
pub struct RBTree<K: Ord, V> {
    root: Tree<K, V>,
}

impl<K: Ord, V> RBTree<K, V> {
    pub fn new() -> Self {
        RBTree { root: Tree::new() }
    }

    pub fn len(&self) -> usize {
        self.root.as_ref().map_or(0, |p| p.size)
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn min(&self) -> Option<&K> {
        self.root.min()
    }
    pub fn max(&self) -> Option<&K> {
        self.root.max()
    }
    pub fn floor(&self, key: &K) -> Option<&K> {
        self.root.floor(key)
    }
    pub fn ceiling(&self, key: &K) -> Option<&K> {
        self.root.ceiling(key)
    }
    pub fn select(&self, i: usize) -> Option<&K> {
        self.root.select(i)
    }
    pub fn rank(&self, key: &K) -> usize {
        self.root.rank(key)
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
        self.root.is_bst(None, None)
    }
    pub fn is_234(&self) -> bool {
        self.root.is_234()
    }
    pub fn is_balanced(&self) -> bool {
        let height = Self::black_height(&self.root, 0);
        self.root.is_balanced(height)
    }
    fn black_height(tree: &Tree<K, V>, height: isize) -> isize {
        tree.as_ref().map_or(height, |b| {
            if b.color.is_red() {
                Self::black_height(&b.left, height)
            } else {
                Self::black_height(&b.left, height + 1)
            }
        })
    }

    pub fn contains(&self, key: &K) -> bool {
        self.get(key).is_some()
    }
    pub fn get(&self, key: &K) -> Option<&V> {
        self.root.get(key)
    }
    pub fn put(&mut self, key: K, value: V) {
        let mut n = self.root.take().put(key, value);
        n.color = Black;
        self.root = Tree::from(n);
    }

    pub fn delete_min(&mut self) {
        let (mut n, _) = self.root.take().delete_min();
        if let Some(r) = n.as_mut() {
            r.color = Black;
        }
        self.root = n;
    }
    pub fn delete_max(&mut self) {
        let (mut n, _) = self.root.take().delete_max();
        if let Some(r) = n.as_mut() {
            r.color = Black;
        }
        self.root = n;
    }
    pub fn delete(&mut self, key: &K) {
        let (mut n, _) = self.root.take().delete(key);
        if let Some(r) = n.as_mut() {
            r.color = Black;
        }
        self.root = n;
    }
}

impl<K, V> From<NodePtr<K, V>> for Tree<K, V> {
    fn from(item: NodePtr<K, V>) -> Self {
        Tree(Some(item))
    }
}
impl<K: Ord, V> Tree<K, V> {
    fn new() -> Self {
        Tree(None)
    }

    // Option<T> methods
    fn as_ref(&self) -> Option<&NodePtr<K, V>> {
        self.0.as_ref()
    }
    fn as_mut(&mut self) -> Option<&mut NodePtr<K, V>> {
        self.0.as_mut()
    }
    fn take(&mut self) -> Self {
        Tree(self.0.take())
    }
    // fn map<U, F: FnOnce(NodePtr<K, V>) -> U>(self, f: F) -> Option<U> {
    //     self.0.map(f)
    // }
    fn map_or<U, F: FnOnce(NodePtr<K, V>) -> U>(self, default: U, f: F) -> U {
        self.0.map_or(default, f)
    }
    fn is_none(&self) -> bool {
        self.0.is_none()
    }
    fn unwrap(self) -> NodePtr<K, V> {
        self.0.unwrap()
    }

    // color methods
    fn is_red(&self) -> bool {
        self.0.as_ref().map_or(false, |n| n.color.is_red())
    }
    fn is_red_left_child(&self) -> bool {
        self.0.as_ref().map_or(false, |n| n.left.is_red())
    }
    fn is_red_right_child(&self) -> bool {
        self.0.as_ref().map_or(false, |n| n.right.is_red())
    }
    fn flip_red(&mut self) -> bool {
        self.0.as_mut().map_or(false, |n| n.color.flip_red())
    }
    fn flip_color(&mut self) {
        if let Some(b) = self.0.as_mut() {
            b.color.flip();
        }
    }

    fn is_bst(&self, min: Option<&K>, max: Option<&K>) -> bool {
        match self.0 {
            None => true,
            Some(ref node) => {
                min.map_or(true, |v| &node.key > v)
                    && max.map_or(true, |v| &node.key < v)
                    && node.left.is_bst(min, Some(&node.key))
                    && node.right.is_bst(Some(&node.key), max)
            }
        }
    }
    fn is_234(&self) -> bool {
        match self.0 {
            None => true,
            Some(ref node) => match (&node.color, node.left.is_red() || node.right.is_red()) {
                (Red, true) => false,
                _ => node.left.is_234() && node.right.is_234(),
            },
        }
    }
    fn is_balanced(&self, mut height: isize) -> bool {
        match self.0 {
            None => height == 0,
            Some(ref b) => {
                if !b.color.is_red() {
                    height -= 1;
                }
                b.left.is_balanced(height) && b.right.is_balanced(height)
            }
        }
    }

    fn size(&self) -> usize {
        self.as_ref().map_or(0, |p| p.size)
    }

    fn min(&self) -> Option<&K> {
        self.0.as_ref().and_then(|v| v.left.min().or(Some(&v.key)))
    }
    fn max(&self) -> Option<&K> {
        self.0.as_ref().and_then(|v| v.right.max().or(Some(&v.key)))
    }

    fn floor<'a>(&'a self, key: &K) -> Option<&'a K> {
        self.0.as_ref().and_then(|p| match key.cmp(&p.key) {
            Equal => Some(&p.key),
            Less => p.left.floor(key),
            Greater => p.right.floor(key).or(Some(&p.key)),
        })
    }
    fn ceiling<'a>(&'a self, key: &K) -> Option<&'a K> {
        self.0.as_ref().and_then(|p| match key.cmp(&p.key) {
            Equal => Some(&p.key),
            Less => p.left.ceiling(key).or(Some(&p.key)),
            Greater => p.right.ceiling(key),
        })
    }
    fn select<'a>(&'a self, i: usize) -> Option<&'a K> {
        self.0.as_ref().and_then(|b| {
            let ls = b.left.as_ref().map_or(0, |b| b.size);
            match i.cmp(&ls) {
                Equal => Some(&b.key),
                Less => b.left.select(i),
                Greater => b.right.select(i - ls - 1),
            }
        })
    }
    fn rank(&self, key: &K) -> usize {
        self.0.as_ref().map_or(0, |b| match key.cmp(&b.key) {
            Equal => b.left.as_ref().map_or(0, |b| b.size),
            Less => b.left.rank(key),
            Greater => b.left.rank(key) + 1 + b.right.rank(key),
        })
    }

    fn get<'a>(&'a self, key: &K) -> Option<&'a V> {
        self.0.as_ref().and_then(|node| match key.cmp(&node.key) {
            Equal => Some(&node.value),
            Less => node.left.get(key),
            Greater => node.right.get(key),
        })
    }
    fn put(self, key: K, value: V) -> NodePtr<K, V> {
        match self.0 {
            None => Box::new(Node::new(key, value)),
            Some(mut b) => {
                // split 4-tree to 2-trees, elimate the case where S is RED
                if b.left.is_red() && b.right.is_red() {
                    b.flip_colors();
                }

                match key.cmp(&b.key) {
                    Equal => b.value = value,
                    Less => {
                        let mut left = b.left.put(key, value);
                        if left.color.is_red() && left.right.is_red() {
                            //      G             G
                            //    /   \         /   \
                            // R(P)    S ->  R(PR)   S
                            //   \           /
                            //   R(PR)      R(P)
                            left = Self::rotate_left(left);
                        }
                        b.left = Tree::from(left);
                        if b.left.is_red() && b.left.is_red_left_child() {
                            //        G              P
                            //      /   \          /   \
                            //    R(P)   S  ->  R(PL)  R(G)
                            //    /  \                /   \
                            // R(PL) (PR)          (PR)    S
                            b = Self::rotate_right(b);
                        }
                    }
                    Greater => {
                        let mut right = b.right.put(key, value);
                        if right.color.is_red() && right.left.is_red() {
                            //      G            G
                            //    /   \        /   \
                            //   S    R(P) -> S    R(PL)
                            //        /              \
                            //      R(PL)            R(P)
                            right = Self::rotate_right(right);
                        }
                        b.right = Tree::from(right);
                        if b.right.is_red() && b.right.is_red_right_child() {
                            //      G                  P
                            //    /   \              /   \
                            //   S    R(P)   ->   R(G)   PR
                            //       /  \        /   \
                            //    (PL)  R(PR)   S    (PL)
                            b = Self::rotate_left(b);
                        }
                    }
                };

                b.size = 1 + b.left.size() + b.right.size();
                b
            }
        }
    }

    fn delete_min(self) -> (Tree<K, V>, bool) {
        self.map_or((Tree::new(), true), |mut b| match b.left.0 {
            None => Self::fix_self_with_right_child(&mut b),
            Some(_) => {
                let (child, balanced) = b.left.delete_min();
                b.left = child;
                b.size = 1 + b.left.size() + b.right.size();
                if balanced {
                    return (Tree::from(b), true);
                }
                Self::fix_left_with_sibling(b)
            }
        })
    }
    fn delete_max(self) -> (Tree<K, V>, bool) {
        self.0.map_or((Tree::new(), true), |mut b| match b.right.0 {
            None => Self::fix_self_with_left_child(&mut b),
            Some(_) => {
                let (child, balanced) = b.right.delete_max();
                b.right = child;
                b.size = 1 + b.left.size() + b.right.size();
                if balanced {
                    return (Tree::from(b), true);
                }
                Self::fix_right_with_sibling(b)
            }
        })
    }
    fn delete(self, key: &K) -> (Tree<K, V>, bool) {
        self.map_or((Tree::new(), true), |mut b| {
            let balanced: bool;
            let mut is_left = false;
            match key.cmp(&b.key) {
                Less => {
                    let (child, sub_b) = b.left.delete(key);
                    b.left = child;
                    balanced = sub_b;
                    is_left = true;
                }
                Greater => {
                    let (child, sub_b) = b.right.delete(key);
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
                    let (x, mut s, sub_b) = Self::pop_min(b.right.unwrap());
                    b.right = x;
                    std::mem::swap(&mut b.key, &mut s.key);
                    std::mem::swap(&mut b.value, &mut s.value);
                    balanced = sub_b;
                }
            }

            b.size = 1 + b.left.size() + b.right.size();
            if balanced {
                (b.into(), balanced)
            } else if is_left {
                Self::fix_left_with_sibling(b)
            } else {
                Self::fix_right_with_sibling(b)
            }
        })
    }
    fn pop_min(mut b: NodePtr<K, V>) -> (Tree<K, V>, NodePtr<K, V>, bool) {
        match b.left.0 {
            None => {
                let (x, balanced) = Self::fix_self_with_right_child(&mut b);
                (x, b, balanced)
            }
            Some(left) => {
                let (x, min, sub_b) = Self::pop_min(left);
                b.left = x;
                b.size = 1 + b.left.size() + b.right.size();

                if sub_b {
                    (b.into(), min, true)
                } else {
                    let (x, sub_b) = Self::fix_left_with_sibling(b);
                    (x, min, sub_b)
                }
            }
        }
    }

    // operations
    fn rotate_left(mut node: NodePtr<K, V>) -> NodePtr<K, V> {
        let mut x = node.right.unwrap();
        node.right = x.left.take();
        x.color = node.color.clone();
        node.color = Red;
        x.size = node.size;
        node.size = node.left.size() + 1 + node.right.size();
        x.left = Tree::from(node);
        x
    }
    fn rotate_right(mut node: NodePtr<K, V>) -> NodePtr<K, V> {
        let mut x = node.left.unwrap();
        node.left = x.right.take();
        x.color = node.color.clone();
        node.color = Red;
        x.size = node.size;
        node.size = node.left.size() + 1 + node.right.size();
        x.right = Tree::from(node);
        x
    }

    fn fix_self_with_right_child(b: &mut NodePtr<K, V>) -> (Tree<K, V>, bool) {
        if b.color.is_red() {
            // no impact on black-height
            (b.right.take(), true)
        } else if b.right.flip_red() {
            // add one to black-height
            (b.right.take(), true)
        } else {
            (b.right.take(), false)
        }
    }
    fn fix_self_with_left_child(b: &mut NodePtr<K, V>) -> (Tree<K, V>, bool) {
        if b.color.is_red() {
            // no impact on black-height
            (b.left.take(), true)
        } else if b.left.flip_red() {
            // add one to black-height
            (b.left.take(), true)
        } else {
            (b.left.take(), false)
        }
    }

    // fix left sub-tree lost one black-height
    fn fix_left_with_sibling(mut p: NodePtr<K, V>) -> (Tree<K, V>, bool) {
        if p.right.is_red() {
            //      P              S
            //    /   \          /   \
            //   X    R(S) ->  R(P)   SR
            //        / \      /  \
            //      SL  SR    X    SL
            p = Self::rotate_left(p);
            // go one layer down to fix R(P) sub-tree
            let (child, balanced) = Self::fix_left_black_s(p.left.unwrap());
            p.left = child;
            if balanced {
                return (Tree::from(p), true);
            }
        }

        Self::fix_left_black_s(p)
    }
    fn fix_left_black_s(mut p: NodePtr<K, V>) -> (Tree<K, V>, bool) {
        if p.right.is_red_left_child() && !p.right.is_red_right_child() {
            //     (P)           (P)
            //    /   \         /   \
            //   X     S   ->  X     SL
            //       /   \             \
            //    R(SL)  SR            R(S)
            //                           \
            //                           SR
            p.right = Tree::from(Self::rotate_right(p.right.unwrap()));
        }

        if p.right.is_red_right_child() {
            //     (P)             (S)
            //    /   \           /   \
            //   X     S   ->    P     SR
            //       /   \      / \
            //    (SL)  R(SR)  X  (SL)
            p = Self::rotate_left(p);
            if let Some(n) = p.left.as_mut() {
                n.color = Black;
            }
            if let Some(n) = p.right.as_mut() {
                n.color = Black;
            }

            (Tree::from(p), true)
        } else {
            //     (P)            P
            //    /   \         /   \
            //   X     S   ->  X    R(S)
            //       /   \         /   \
            //      SL   SR       SL   SR
            // if P was red: path P->X added one black-height while P->S stays same, then we're done
            // if P was black: sub one black-height from right sub-tree and escalate to up
            let balanced = p.color.is_red();
            p.color = Black;
            if let Some(n) = p.right.as_mut() {
                n.color = Red;
            }

            (Tree::from(p), balanced)
        }
    }

    // fix right sub-tree lost one black-height
    fn fix_right_with_sibling(mut p: NodePtr<K, V>) -> (Tree<K, V>, bool) {
        if p.left.is_red() {
            // case: right_S
            p = Self::rotate_right(p);
            // go one layer down to fix
            let (child, balanced) = Self::fix_right_black_s(p.right.unwrap());
            p.right = child;
            if balanced {
                return (Tree::from(p), true);
            }
        }

        Self::fix_right_black_s(p)
    }
    fn fix_right_black_s(mut p: NodePtr<K, V>) -> (Tree<K, V>, bool) {
        if !p.left.is_red_left_child() && p.left.is_red_right_child() {
            // case: red SR, transfer to red SL
            p.left = Tree::from(Self::rotate_left(p.left.unwrap()));
        }

        if p.left.is_red_left_child() {
            // case: red SL, borrow one black-height from S sub-tree
            p = Self::rotate_right(p);
            if let Some(n) = p.left.as_mut() {
                n.color = Black;
            }
            if let Some(n) = p.right.as_mut() {
                n.color = Black;
            }

            (Tree::from(p), true)
        } else {
            // case: S/SL/SR are all black
            // if red: path P->X add one black-height while P->S stays same
            // if black: sub one black-height from right sub-tree and escalate to up
            let balanced = p.color.is_red();
            p.color = Black;
            if let Some(n) = p.left.as_mut() {
                n.color = Red;
            }

            (Tree::from(p), balanced)
        }
    }
}

/// Iterating as inorder traversal
pub struct Iter<'a, K, V> {
    stack: Vec<&'a Node<K, V>>,
    current: Option<&'a Node<K, V>>,
}
impl<'a, K: Ord, V> Iterator for Iter<'a, K, V> {
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
            if let Some(ref r) = n.right.0 {
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
    use rand::{thread_rng, Rng};

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
    fn deletion() {
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
    #[test]
    fn put_fix_right() {
        let mut st = RBTree::<usize, usize>::new();
        st.put(3, 30);
        st.put(1, 10);
        st.put(5, 50);
        st.delete(&1);
        st.put(4, 40);

        assert_eq!(3, st.len());
        assert_eq!(None, st.check_error());
    }

    #[test]
    fn random_100() {
        let mut st = RBTree::<u32, usize>::new();
        for (i, k) in thread_rng().gen_iter::<u32>().enumerate().take(100) {
            match (i % 5, i % 7) {
                (0, 0) => {
                    st.put(k, i);
                    st.delete(&k);
                }
                (0, _) => st.delete(&k),
                _ => st.put(k, i),
            }
            assert_eq!(None, st.check_error());
        }
        assert_eq!(80, st.len());
    }
}
