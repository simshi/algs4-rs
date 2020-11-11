use std::cmp::Ordering::{Equal, Greater, Less};

struct Node<T> {
	b: u8,
	val: Option<T>,
	left: NodePtr<T>,
	middle: NodePtr<T>,
	right: NodePtr<T>,
}
type NodePtr<T> = Option<Box<Node<T>>>;

impl<T> Node<T> {
	fn new(b: u8, val: Option<T>) -> Self {
		Self {
			b,
			val,
			left: None,
			middle: None,
			right: None,
		}
	}
}

pub struct TST<T: Copy> {
	root: NodePtr<T>,
	n: usize,
}
impl<T: Copy> Default for TST<T> {
	fn default() -> Self {
		Self { root: None, n: 0 }
	}
}
impl<T: Copy> TST<T> {
	pub fn new() -> Self {
		Default::default()
	}
	pub fn size(&self) -> usize {
		self.n
	}
	pub fn is_empty(&self) -> bool {
		self.size() == 0
	}

	pub fn get(&self, key: &[u8]) -> Option<&T> {
		if key.is_empty() {
			None
		} else {
			Self::_get_node(&self.root, key, 0)
				.as_ref()
				.and_then(|node| node.val.as_ref())
		}
	}
	pub fn put(&mut self, key: &[u8], val: T) {
		if !key.is_empty() {
			let p = self.root.take();
			self.root = self._put(p, key, val, 0);
		}
	}
	pub fn delete(&mut self, key: &[u8]) {
		if !key.is_empty() {
			let p = self.root.take();
			self.root = self._delete(p, key, 0);
		}
	}

	pub fn keys_with_prefix(&self, prefix: &[u8]) -> impl Iterator<Item = Vec<u8>> {
		let mut cv = prefix.to_vec();
		let mut results = Vec::new();

		if prefix.is_empty() {
			Self::_collect(&self.root, &mut cv, &mut results);
		} else {
			let p = Self::_get_node(&self.root, prefix, 0);

			if let Some(node) = p {
				if node.val.is_some() {
					// a key equals to prefix exists
					results.push(cv.clone());
				}
				Self::_collect(&node.middle, &mut cv, &mut results);
			}
		};

		results.into_iter()
	}
	pub fn longest_match(&self, query: &[u8]) -> Option<(usize, &T)> {
		if query.is_empty() {
			return None;
		}

		let mut max_length = 0;
		let mut vv = None;
		let mut d = 0;
		let mut p = &self.root;
		while let Some(node) = p {
			match query[d].cmp(&node.b) {
				Less => p = &node.left,
				Greater => p = &node.right,
				Equal => {
					d += 1;
					if node.val.is_some() {
						max_length = d;
						vv = node.val.as_ref();
					}
					if d == query.len() {
						break;
					}
					p = &node.middle;
				}
			}
		}

		vv.map(|v| (max_length, v))
	}

	pub fn iter(&self) -> Iter<'_, T> {
		Iter::new(&self.root)
	}
}

// private methods
impl<T: Copy> TST<T> {
	fn _get_node<'a>(p: &'a NodePtr<T>, key: &[u8], d: usize) -> &'a NodePtr<T> {
		if let Some(node) = p {
			return match key[d].cmp(&node.b) {
				Less => Self::_get_node(&node.left, key, d),
				Greater => Self::_get_node(&node.right, key, d),
				Equal if d + 1 == key.len() => p,
				_ => Self::_get_node(&node.middle, key, d + 1),
			};
		}

		p
	}

	fn _put(&mut self, p: NodePtr<T>, key: &[u8], val: T, d: usize) -> NodePtr<T> {
		let b = key[d];
		let mut is_new = false;
		let mut node = match p {
			Some(node) => node,
			_ => {
				let n = Box::new(Node::new(b, None));
				is_new = true;
				n
			}
		};

		match b.cmp(&node.b) {
			Less => node.left = self._put(node.left.take(), key, val, d),
			Greater => node.right = self._put(node.right.take(), key, val, d),
			Equal if d + 1 == key.len() => {
				if is_new {
					self.n += 1;
				}
				node.val = Some(val);
			}
			_ => node.middle = self._put(node.middle.take(), key, val, d + 1),
		}

		Some(node)
	}

	fn _delete(&mut self, p: NodePtr<T>, key: &[u8], d: usize) -> NodePtr<T> {
		p.and_then(|mut node| {
			match key[d].cmp(&node.b) {
				Less => node.left = self._delete(node.left.take(), key, d),
				Greater => node.right = self._delete(node.right.take(), key, d),
				Equal if d + 1 == key.len() => {
					if node.val.is_some() {
						self.n -= 1;
						node.val = None;
					}
				}
				_ => node.middle = self._delete(node.middle.take(), key, d + 1),
			}

			if node.val.is_some() || node.middle.is_some() {
				Some(node)
			} else {
				match (&node.left, &node.right) {
					(_, None) => node.left,
					(None, _) => node.right,
					(Some(_), Some(_)) => Some(node),
				}
			}
		})
	}

	fn _collect(p: &NodePtr<T>, cv: &mut Vec<u8>, results: &mut Vec<Vec<u8>>) {
		if let Some(node) = p.as_ref() {
			if node.val.is_some() {
				cv.push(node.b);
				results.push(cv.clone());
				cv.pop();
			}

			Self::_collect(&node.left, cv, results);
			cv.push(node.b);
			Self::_collect(&node.middle, cv, results);
			cv.pop();
			Self::_collect(&node.right, cv, results);
		}
	}
}

enum Branch {
	Root,
	Left(u8),
	Middle(u8),
	Right(u8),
}
impl std::fmt::Display for Branch {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		let v = match self {
			Branch::Root => String::from("T"),
			Branch::Left(b) => format!("{}->L", *b as char),
			Branch::Middle(b) => format!("{}->", *b as char),
			Branch::Right(b) => format!("{}->R", *b as char),
		};
		write!(f, "{}", v)
	}
}
pub struct Iter<'a, T> {
	stack: Vec<(&'a NodePtr<T>, Branch, usize)>,
}
impl<'a, T> Iter<'a, T> {
	fn new(p: &'a NodePtr<T>) -> Self {
		let mut stack = Vec::new();
		stack.push((p, Branch::Root, 0));
		Self { stack }
	}
}
impl<'a, T> Iterator for Iter<'a, T> {
	type Item = String;

	fn next(&mut self) -> Option<Self::Item> {
		while let Some((p, br, d)) = self.stack.pop() {
			if p.is_none() {
				continue;
			}
			let node = p.as_ref().unwrap();
			let v = format!("{}:{}:{}", br, d, node.b as char);
			self.stack.push((&node.right, Branch::Right(node.b), d));
			self.stack.push((&node.left, Branch::Left(node.b), d));
			self.stack
				.push((&node.middle, Branch::Middle(node.b), d + 1));
			return Some(v);
		}

		None
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn empty() {
		let t = TST::<usize>::new();
		assert_eq!(0, t.size());
		assert_eq!(true, t.is_empty());
	}

	#[test]
	fn put_get_several() {
		let mut t = TST::<f64>::new();
		t.put("LK".as_bytes(), 6.4);
		t.put("AAPL".as_bytes(), 244.93);
		assert_eq!(2, t.size());

		assert_eq!(None, t.get("MSFT".as_bytes()));
		assert_eq!(Some(&244.93), t.get("AAPL".as_bytes()));
		assert_eq!(Some(&6.4), t.get("LK".as_bytes()));

		assert_eq!(None, t.get("AAP".as_bytes()));
		assert_eq!(None, t.get("AA".as_bytes()));
		assert_eq!(None, t.get("A".as_bytes()));
		assert_eq!(None, t.get("L".as_bytes()));
	}

	#[test]
	fn update() {
		let mut t = TST::<f64>::new();
		t.put("LK".as_bytes(), 6.4);
		t.put("AAPL".as_bytes(), 244.93);
		t.put("AAPL".as_bytes(), 250.13);

		assert_eq!(Some(&6.4), t.get("LK".as_bytes()));
		assert_eq!(Some(&250.13), t.get("AAPL".as_bytes()));
	}

	#[test]
	fn delete() {
		let mut t = TST::<f64>::new();
		t.put("LK".as_bytes(), 6.4);
		t.put("sea".as_bytes(), 120.93);
		t.put("seafood".as_bytes(), 150.13);
		t.put("seashell".as_bytes(), 250.13);
		t.put("seashells".as_bytes(), 50.13);

		// delete middle
		t.delete("seashell".as_bytes());
		assert_eq!(None, t.get("seashell".as_bytes()));
		assert_eq!(Some(&120.93), t.get("sea".as_bytes()));
		assert_eq!(Some(&150.13), t.get("seafood".as_bytes()));
		assert_eq!(Some(&50.13), t.get("seashells".as_bytes()));
		assert_eq!(Some(&6.4), t.get("LK".as_bytes()));

		// delete leaf which has parent
		t.delete("seafood".as_bytes());
		assert_eq!(None, t.get("seafood".as_bytes()));
		assert_eq!(Some(&50.13), t.get("seashells".as_bytes()));
		assert_eq!(Some(&120.93), t.get("sea".as_bytes()));
		assert_eq!(Some(&6.4), t.get("LK".as_bytes()));

		// delete parent
		t.delete("sea".as_bytes());
		assert_eq!(None, t.get("sea".as_bytes()));
		assert_eq!(None, t.get("seafood".as_bytes()));
		assert_eq!(Some(&50.13), t.get("seashells".as_bytes()));
		assert_eq!(Some(&6.4), t.get("LK".as_bytes()));

		// delete leaf without parent
		t.delete("seashells".as_bytes());
		assert_eq!(None, t.get("seashell".as_bytes()));
		assert_eq!(None, t.get("seashells".as_bytes()));
		assert_eq!(Some(&6.4), t.get("LK".as_bytes()));
	}

	#[test]
	fn keys_match_prefix() {
		let a = vec![
			"she",
			"seashells",
			"by",
			"sea",
			"shore",
			"the",
			"shells",
			"are",
			"surely",
		];
		let mut t = TST::<usize>::new();
		for (i, s) in a.iter().enumerate() {
			t.put(s.as_bytes(), i);
		}

		let mut k = t
			.keys_with_prefix("".as_bytes())
			.map(|bv| String::from_utf8(bv).unwrap())
			.collect::<Vec<_>>();
		k.sort_unstable();
		let mut ac = a.clone();
		ac.sort_unstable();
		assert_eq!(ac, k);

		let mut k = t
			.keys_with_prefix("a".as_bytes())
			.map(|bv| String::from_utf8(bv).unwrap())
			.collect::<Vec<_>>();
		k.sort_unstable();
		assert_eq!(vec!["are"], k);

		let mut k = t
			.keys_with_prefix("s".as_bytes())
			.map(|bv| String::from_utf8(bv).unwrap())
			.collect::<Vec<_>>();
		k.sort_unstable();
		assert_eq!(
			vec!["sea", "seashells", "she", "shells", "shore", "surely"],
			k
		);

		let mut k = t
			.keys_with_prefix("sh".as_bytes())
			.map(|bv| String::from_utf8(bv).unwrap())
			.collect::<Vec<_>>();
		k.sort_unstable();
		assert_eq!(vec!["she", "shells", "shore"], k);

		let mut k = t
			.keys_with_prefix("she".as_bytes())
			.map(|bv| String::from_utf8(bv).unwrap())
			.collect::<Vec<_>>();
		k.sort_unstable();
		assert_eq!(vec!["she", "shells"], k);

		let mut k = t
			.keys_with_prefix("shel".as_bytes())
			.map(|bv| String::from_utf8(bv).unwrap())
			.collect::<Vec<_>>();
		k.sort_unstable();
		assert_eq!(vec!["shells"], k);

		let mut k = t
			.keys_with_prefix("shells".as_bytes())
			.map(|bv| String::from_utf8(bv).unwrap())
			.collect::<Vec<_>>();
		k.sort_unstable();
		assert_eq!(vec!["shells"], k);

		let k = t
			.keys_with_prefix("shellsxxx".as_bytes())
			.map(|bv| String::from_utf8(bv).unwrap())
			.collect::<Vec<_>>();
		assert_eq!(0, k.len());
	}

	#[test]
	fn longest_key_of_query() {
		let a = vec![
			"she",
			"seashells",
			"by",
			"sea",
			"shore",
			"the",
			"shells",
			"are",
			"surely",
		];
		let mut t = TST::<usize>::new();
		for (i, s) in a.iter().enumerate() {
			t.put(s.as_bytes(), i);
		}

		assert_eq!(None, t.longest_match("".as_bytes()));
		assert_eq!(None, t.longest_match("a".as_bytes()));
		assert_eq!(Some((3, &7)), t.longest_match("are".as_bytes()));

		assert_eq!(None, t.longest_match("s".as_bytes()));

		assert_eq!(Some((3, &3)), t.longest_match("sea".as_bytes()));
		assert_eq!(Some((3, &3)), t.longest_match("seafood".as_bytes()));
		assert_eq!(Some((9, &1)), t.longest_match("seashellsabc".as_bytes()));
	}

	#[test]
	fn delete_with_one_child() {
		let mut t = TST::<usize>::new();
		let a = t.iter().collect::<Vec<_>>();
		assert_eq!(0, a.len());

		t.put("shell".as_bytes(), 95);
		let a = t.iter().collect::<Vec<_>>();
		assert_eq!(vec!["T:0:s", "s->:1:h", "h->:2:e", "e->:3:l", "l->:4:l"], a);

		t.put("sea".as_bytes(), 990);
		let a = t.iter().collect::<Vec<_>>();
		assert_eq!(
			vec!["T:0:s", "s->:1:h", "h->:2:e", "e->:3:l", "l->:4:l", "h->L:1:e", "e->:2:a"],
			a
		);

		t.put("shore".as_bytes(), 123);
		//     s
		// e - h
		// a   e - o
		//     l   r
		//     l   e
		let a = t.iter().collect::<Vec<_>>();
		assert_eq!(
			vec![
				"T:0:s", "s->:1:h", "h->:2:e", "e->:3:l", "l->:4:l", "e->R:2:o", "o->:3:r",
				"r->:4:e", "h->L:1:e", "e->:2:a"
			],
			a
		);

		t.delete("shell".as_bytes());
		// collapse 'e' in "she" by right tree
		//     s            s
		// e - h        e - h
		// a   e - o => a   o
		//         r        r
		//         e        e
		let a = t.iter().collect::<Vec<_>>();
		assert_eq!(
			vec!["T:0:s", "s->:1:h", "h->:2:o", "o->:3:r", "r->:4:e", "h->L:1:e", "e->:2:a"],
			a
		);

		t.delete("shore".as_bytes());
		// collapse 'h' in "sh" by left tree
		//     s    s
		// e - h => e
		// a        a
		let a = t.iter().collect::<Vec<_>>();
		assert_eq!(vec!["T:0:s", "s->:1:e", "e->:2:a"], a);
	}

	#[test]
	fn delete_with_both_left_and_right_exist() {
		let mut t = TST::<usize>::new();

		t.put("she".as_bytes(), 88);
		t.put("sea".as_bytes(), 1230);
		t.put("sold".as_bytes(), 230);
		//     s
		// e - h - o
		// a   e   l
		//         d
		let a = t.iter().collect::<Vec<_>>();
		assert_eq!(
			vec![
				"T:0:s", "s->:1:h", "h->:2:e", "h->L:1:e", "e->:2:a", "h->R:1:o", "o->:2:l",
				"l->:3:d"
			],
			a
		);

		t.delete("she".as_bytes());
		//     s
		// e - h - o
		// a       l
		//         d
		let a = t.iter().collect::<Vec<_>>();
		assert_eq!(
			vec!["T:0:s", "s->:1:h", "h->L:1:e", "e->:2:a", "h->R:1:o", "o->:2:l", "l->:3:d"],
			a
		);

		t.delete("sold".as_bytes());
		// now, it can collapse
		//     s    s
		// e - h => e
		// a        a
		let a = t.iter().collect::<Vec<_>>();
		assert_eq!(vec!["T:0:s", "s->:1:e", "e->:2:a"], a);
	}
}
