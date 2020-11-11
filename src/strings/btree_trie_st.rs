use std::collections::BTreeMap;

struct Node<T> {
	val: Option<T>,
	children: Children<T>,
}
type Children<T> = BTreeMap<u8, Node<T>>;

impl<T> Node<T> {
	fn new(val: Option<T>) -> Self {
		Self {
			val,
			children: BTreeMap::new(),
		}
	}
}

pub struct TrieST<T> {
	root: BTreeMap<u8, Node<T>>,
	n: usize,
}
impl<T> Default for TrieST<T> {
	fn default() -> Self {
		Self {
			root: BTreeMap::new(),
			n: 0,
		}
	}
}
impl<T> TrieST<T> {
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
			return None;
		}
		Self::get_node(&self.root, key, 0).and_then(|node| node.val.as_ref())
	}
	pub fn put(&mut self, key: &[u8], val: T) {
		let mut d = 0;
		let mut p = &mut self.root;
		while d < key.len() {
			if p.get(&key[d]).is_none() {
				p.insert(key[d], Node::new(None));
			}
			let mut node = p.get_mut(&key[d]).unwrap();

			d += 1;
			if d == key.len() {
				if node.val.is_none() {
					self.n += 1;
				}
				node.val = Some(val);
				break;
			}
			p = &mut node.children;
		}
	}
	pub fn delete(&mut self, key: &[u8]) {
		if Self::_delete(&mut self.root, key, 0) {
			self.n -= 1;
		}
	}

	pub fn keys_with_prefix(&self, prefix: &[u8]) -> impl Iterator<Item = Vec<u8>> {
		let mut results = Vec::new();
		let mut cv = prefix.to_vec();

		if prefix.is_empty() {
			Self::collect(&self.root, &mut cv, &mut results);
		} else if let Some(node) = Self::get_node(&self.root, prefix, 0) {
			if node.val.is_some() {
				// a key equals to prefix exists
				results.push(cv.clone());
			}
			Self::collect(&node.children, &mut cv, &mut results);
		};

		results.into_iter()
	}
	pub fn longest_match(&self, query: &[u8]) -> Option<(usize, &T)> {
		let mut max_length = 0;
		let mut p = &self.root;
		let mut vv = None;
		for (d, b) in query.iter().enumerate() {
			if let Some(node) = p.get(b) {
				if node.val.is_some() {
					max_length = d + 1;
					vv = node.val.as_ref();
				}
				p = &node.children;
			} else {
				break;
			}
		}
		vv.map(|v| (max_length, v))
	}
	// pub fn keys_match_pattern(&self, pattern: &[u8]) -> impl Iterator<Item = Vec<u8>> {
	// 	let mut results = Vec::new();
	// 	let mut cv = Vec::new();
	// 	Self::_collect_match_pattern(&self.root, pattern, &mut cv, &mut results);

	// 	results.into_iter()
	// }
}

// private methods
impl<T> TrieST<T> {
	fn get_node<'a>(p: &'a Children<T>, key: &[u8], d: usize) -> Option<&'a Node<T>> {
		p.get(&key[d]).and_then(|node| {
			if d + 1 == key.len() {
				Some(node)
			} else {
				Self::get_node(&node.children, key, d + 1)
			}
		})
	}
	fn _delete(c: &mut Children<T>, key: &[u8], d: usize) -> bool {
		let mut changed = false;
		if let Some(node) = c.get_mut(&key[d]) {
			if d + 1 == key.len() {
				if node.val.is_some() {
					changed = true;
					node.val = None;
				}
			} else {
				changed = Self::_delete(&mut node.children, key, d + 1);
			}

			if node.val.is_none() && node.children.is_empty() {
				c.remove(&key[d]);
			}
		}

		changed
	}

	fn collect(c: &Children<T>, cv: &mut Vec<u8>, results: &mut Vec<Vec<u8>>) {
		for (k, node) in c.iter() {
			cv.push(*k);
			if node.val.is_some() {
				results.push(cv.clone());
			}

			Self::collect(&node.children, cv, results);
			cv.pop();
		}
	}

	// fn _collect_match_pattern(
	// 	p: &NodePtr<T>,
	// 	pattern: &[u8],
	// 	cv: &mut Vec<u8>,
	// 	results: &mut Vec<Vec<u8>>,
	// ) {
	// 	if let Some(node) = p.as_ref() {
	// 		let d = cv.len();
	// 		if d == pattern.len() {
	// 			if node.val.is_some() {
	// 				results.push(cv.clone());
	// 			}
	// 			return;
	// 		}

	// 		let c = pattern[d];
	// 		if c == b'.' {
	// 			for (i, p) in node.next.iter().enumerate() {
	// 				cv.push(i as u8);
	// 				Self::_collect_match_pattern(p, pattern, cv, results);
	// 				cv.pop();
	// 			}
	// 		} else {
	// 			cv.push(c);
	// 			Self::_collect_match_pattern(&node.next[c as usize], pattern, cv, results);
	// 			cv.pop();
	// 		}
	// 	}
	// }
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn empty() {
		let t = TrieST::<usize>::new();
		assert_eq!(0, t.size());
		assert_eq!(true, t.is_empty());
	}

	#[test]
	fn put_get_several() {
		let mut t = TrieST::<f64>::new();
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
		let mut t = TrieST::<f64>::new();
		t.put("LK".as_bytes(), 6.4);
		t.put("AAPL".as_bytes(), 244.93);
		t.put("AAPL".as_bytes(), 250.13);

		assert_eq!(Some(&6.4), t.get("LK".as_bytes()));
		assert_eq!(Some(&250.13), t.get("AAPL".as_bytes()));
	}

	#[test]
	fn unicode() {
		let mut t = TrieST::<f64>::new();
		t.put("LK瑞幸".as_bytes(), 6.4);
		t.put("AAPL苹果".as_bytes(), 244.93);

		assert_eq!(Some(&6.4), t.get("LK瑞幸".as_bytes()));
		assert_eq!(Some(&244.93), t.get("AAPL苹果".as_bytes()));
	}

	#[test]
	fn delete() {
		let mut t = TrieST::<f64>::new();
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
		let mut t = TrieST::<usize>::new();
		for (i, s) in a.iter().enumerate() {
			t.put(s.as_bytes(), i);
		}

		let mut k = t
			.keys_with_prefix("".as_bytes())
			.map(|k| String::from_utf8(k).unwrap())
			.collect::<Vec<_>>();
		k.sort_unstable();
		let mut ac = a.clone();
		ac.sort_unstable();
		assert_eq!(ac, k);

		let mut k = t
			.keys_with_prefix("a".as_bytes())
			.map(|k| String::from_utf8(k).unwrap())
			.collect::<Vec<_>>();
		k.sort_unstable();
		assert_eq!(vec!["are"], k);

		let mut k = t
			.keys_with_prefix("s".as_bytes())
			.map(|k| String::from_utf8(k).unwrap())
			.collect::<Vec<_>>();
		k.sort_unstable();
		assert_eq!(
			vec!["sea", "seashells", "she", "shells", "shore", "surely"],
			k
		);

		let mut k = t
			.keys_with_prefix("sh".as_bytes())
			.map(|k| String::from_utf8(k).unwrap())
			.collect::<Vec<_>>();
		k.sort_unstable();
		assert_eq!(vec!["she", "shells", "shore"], k);

		let mut k = t
			.keys_with_prefix("she".as_bytes())
			.map(|k| String::from_utf8(k).unwrap())
			.collect::<Vec<_>>();
		k.sort_unstable();
		assert_eq!(vec!["she", "shells"], k);

		let mut k = t
			.keys_with_prefix("shel".as_bytes())
			.map(|k| String::from_utf8(k).unwrap())
			.collect::<Vec<_>>();
		k.sort_unstable();
		assert_eq!(vec!["shells"], k);

		let mut k = t
			.keys_with_prefix("shells".as_bytes())
			.map(|k| String::from_utf8(k).unwrap())
			.collect::<Vec<_>>();
		k.sort_unstable();
		assert_eq!(vec!["shells"], k);

		let k = t
			.keys_with_prefix("shellsxxx".as_bytes())
			.collect::<Vec<_>>();
		assert_eq!(0, k.len());
	}

	// #[test]
	// fn keys_match_pattern() {
	// 	let a = vec![
	// 		"she",
	// 		"seashells",
	// 		"by",
	// 		"sea",
	// 		"shore",
	// 		"the",
	// 		"shells",
	// 		"are",
	// 		"surely",
	// 	];
	// 	let mut t = TrieST::<usize>::new();
	// 	for (i, s) in a.iter().enumerate() {
	// 		t.put(s.as_bytes(), i);
	// 	}

	// 	let k = t.keys_match_pattern("".as_bytes()).collect::<Vec<_>>();
	// 	assert_eq!(0, k.len());

	// 	let mut k = t
	// 		.keys_match_pattern("a..".as_bytes())
	// 		.map(|k| String::from_utf8(k).unwrap())
	// 		.collect::<Vec<_>>();
	// 	k.sort_unstable();
	// 	assert_eq!(vec!["are"], k);

	// 	let mut k = t
	// 		.keys_match_pattern("a.e".as_bytes())
	// 		.map(|k| String::from_utf8(k).unwrap())
	// 		.collect::<Vec<_>>();
	// 	k.sort_unstable();
	// 	assert_eq!(vec!["are"], k);

	// 	let mut k = t
	// 		.keys_match_pattern("ar.".as_bytes())
	// 		.map(|k| String::from_utf8(k).unwrap())
	// 		.collect::<Vec<_>>();
	// 	k.sort_unstable();
	// 	assert_eq!(vec!["are"], k);

	// 	let mut k = t
	// 		.keys_match_pattern(".re".as_bytes())
	// 		.map(|k| String::from_utf8(k).unwrap())
	// 		.collect::<Vec<_>>();
	// 	k.sort_unstable();
	// 	assert_eq!(vec!["are"], k);

	// 	let mut k = t
	// 		.keys_match_pattern(".r.".as_bytes())
	// 		.map(|k| String::from_utf8(k).unwrap())
	// 		.collect::<Vec<_>>();
	// 	k.sort_unstable();
	// 	assert_eq!(vec!["are"], k);

	// 	let mut k = t
	// 		.keys_match_pattern("..e".as_bytes())
	// 		.map(|k| String::from_utf8(k).unwrap())
	// 		.collect::<Vec<_>>();
	// 	k.sort_unstable();
	// 	assert_eq!(vec!["are", "she", "the"], k);

	// 	let mut k = t
	// 		.keys_match_pattern("......".as_bytes())
	// 		.map(|k| String::from_utf8(k).unwrap())
	// 		.collect::<Vec<_>>();
	// 	k.sort_unstable();
	// 	assert_eq!(vec!["shells", "surely"], k);

	// 	let mut k = t
	// 		.keys_match_pattern("..rel.".as_bytes())
	// 		.map(|k| String::from_utf8(k).unwrap())
	// 		.collect::<Vec<_>>();
	// 	k.sort_unstable();
	// 	assert_eq!(vec!["surely"], k);
	// }

	#[test]
	fn longest_match_of_query() {
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
		let mut t = TrieST::<usize>::new();
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
}
