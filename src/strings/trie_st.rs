const R: usize = 256;

struct Node<T> {
	val: Option<T>,
	next: [NodePtr<T>; R],
}
type NodePtr<T> = Option<Box<Node<T>>>;

// TODO: find a better way to init `next` array
macro_rules! make_array {
	($n:expr, $constructor:expr, $ty:ty) => {{
		let mut items: [std::mem::MaybeUninit<$ty>; $n] =
			unsafe { std::mem::MaybeUninit::uninit().assume_init() };
		for (i, elem) in items[..].iter_mut().enumerate() {
			unsafe {
				std::ptr::write(elem.as_mut_ptr(), $constructor(i));
				}
			}

		unsafe { std::mem::transmute::<_, [$ty; $n]>(items) }
		}};
}

impl<T> Node<T> {
	fn new(val: Option<T>) -> Self {
		Self {
			val,
			next: make_array!(R, |_| None, NodePtr<T>),
		}
	}
}

pub struct TrieST<T> {
	root: NodePtr<T>,
	n: usize,
}
impl<T> Default for TrieST<T> {
	fn default() -> Self {
		Self { root: None, n: 0 }
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
		Self::_get_node(&self.root, key, 0)
			.as_ref()
			.and_then(|node| node.val.as_ref())
	}
	pub fn put(&mut self, key: &[u8], val: T) {
		let p = self.root.take();
		self.root = self._put(p, key, val, 0);
	}
	pub fn delete(&mut self, key: &[u8]) {
		let p = self.root.take();
		self.root = self._delete(p, key, 0);
	}

	pub fn keys_with_prefix(&self, prefix: &[u8]) -> impl Iterator<Item = Vec<u8>> {
		let mut results = Vec::new();
		let p = Self::_get_node(&self.root, prefix, 0);
		let mut cv = prefix.to_vec();
		Self::_collect(p, &mut cv, &mut results);

		results.into_iter()
	}
	pub fn longest_key_of(&self, prefix: &[u8]) -> Option<usize> {
		let mut max_length = 0;
		Self::_longest_key(&self.root, prefix, 0, &mut max_length);
		if max_length == 0 {
			None
		} else {
			Some(max_length)
		}
	}
	pub fn keys_match_pattern(&self, pattern: &[u8]) -> impl Iterator<Item = Vec<u8>> {
		let mut results = Vec::new();
		let mut cv = Vec::new();
		Self::_collect_match_pattern(&self.root, pattern, &mut cv, &mut results);

		results.into_iter()
	}
}

// private methods
impl<T> TrieST<T> {
	fn _get_node<'a>(p: &'a NodePtr<T>, key: &[u8], d: usize) -> &'a NodePtr<T> {
		if let Some(node) = p {
			if d < key.len() {
				let c = key[d] as usize;
				return Self::_get_node(&node.next[c], key, d + 1);
			}
		}

		p
	}

	fn _put(&mut self, p: NodePtr<T>, key: &[u8], val: T, d: usize) -> NodePtr<T> {
		let mut is_new = false;
		let mut node = match p {
			Some(node) => node,
			_ => {
				let n = Box::new(Node::new(None));
				is_new = true;
				n
			}
		};

		if d == key.len() {
			if is_new {
				self.n += 1;
			}
			node.val = Some(val);
			return Some(node);
		}

		let c = key[d] as usize;
		let pc = node.next[c].take();
		node.next[c] = self._put(pc, key, val, d + 1);

		Some(node)
	}

	fn _delete(&mut self, p: NodePtr<T>, key: &[u8], d: usize) -> NodePtr<T> {
		p.and_then(|mut node| {
			if d == key.len() {
				if node.val.is_some() {
					self.n -= 1;
					node.val = None;
				}
			} else {
				let c = key[d] as usize;
				let pc = node.next[c].take();
				node.next[c] = self._delete(pc, key, d + 1);
			}

			if node.val.is_some() || node.next.iter().any(|p| p.is_some()) {
				Some(node)
			} else {
				None
			}
		})
	}

	fn _collect(p: &NodePtr<T>, cv: &mut Vec<u8>, results: &mut Vec<Vec<u8>>) {
		if let Some(node) = p.as_ref() {
			if node.val.is_some() {
				results.push(cv.clone());
			}

			for (i, p) in node.next.iter().enumerate() {
				if p.is_some() {
					cv.push(i as u8);
					Self::_collect(p, cv, results);
					cv.pop();
				}
			}
		}
	}

	fn _longest_key(p: &NodePtr<T>, query: &[u8], d: usize, length: &mut usize) {
		if let Some(node) = p {
			if node.val.is_some() {
				*length = d;
			}
			if d == query.len() {
				return;
			}

			let c = query[d] as usize;
			Self::_longest_key(&node.next[c], query, d + 1, length);
		}
	}

	fn _collect_match_pattern(
		p: &NodePtr<T>,
		pattern: &[u8],
		cv: &mut Vec<u8>,
		results: &mut Vec<Vec<u8>>,
	) {
		if let Some(node) = p.as_ref() {
			let d = cv.len();
			if d == pattern.len() {
				if node.val.is_some() {
					results.push(cv.clone());
				}
				return;
			}

			let c = pattern[d];
			if c == b'.' {
				for (i, p) in node.next.iter().enumerate() {
					cv.push(i as u8);
					Self::_collect_match_pattern(p, pattern, cv, results);
					cv.pop();
				}
			} else {
				cv.push(c);
				Self::_collect_match_pattern(&node.next[c as usize], pattern, cv, results);
				cv.pop();
			}
		}
	}
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

	#[test]
	fn keys_match_pattern() {
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

		let k = t.keys_match_pattern("".as_bytes()).collect::<Vec<_>>();
		assert_eq!(0, k.len());

		let mut k = t
			.keys_match_pattern("a..".as_bytes())
			.map(|k| String::from_utf8(k).unwrap())
			.collect::<Vec<_>>();
		k.sort_unstable();
		assert_eq!(vec!["are"], k);

		let mut k = t
			.keys_match_pattern("a.e".as_bytes())
			.map(|k| String::from_utf8(k).unwrap())
			.collect::<Vec<_>>();
		k.sort_unstable();
		assert_eq!(vec!["are"], k);

		let mut k = t
			.keys_match_pattern("ar.".as_bytes())
			.map(|k| String::from_utf8(k).unwrap())
			.collect::<Vec<_>>();
		k.sort_unstable();
		assert_eq!(vec!["are"], k);

		let mut k = t
			.keys_match_pattern(".re".as_bytes())
			.map(|k| String::from_utf8(k).unwrap())
			.collect::<Vec<_>>();
		k.sort_unstable();
		assert_eq!(vec!["are"], k);

		let mut k = t
			.keys_match_pattern(".r.".as_bytes())
			.map(|k| String::from_utf8(k).unwrap())
			.collect::<Vec<_>>();
		k.sort_unstable();
		assert_eq!(vec!["are"], k);

		let mut k = t
			.keys_match_pattern("..e".as_bytes())
			.map(|k| String::from_utf8(k).unwrap())
			.collect::<Vec<_>>();
		k.sort_unstable();
		assert_eq!(vec!["are", "she", "the"], k);

		let mut k = t
			.keys_match_pattern("......".as_bytes())
			.map(|k| String::from_utf8(k).unwrap())
			.collect::<Vec<_>>();
		k.sort_unstable();
		assert_eq!(vec!["shells", "surely"], k);

		let mut k = t
			.keys_match_pattern("..rel.".as_bytes())
			.map(|k| String::from_utf8(k).unwrap())
			.collect::<Vec<_>>();
		k.sort_unstable();
		assert_eq!(vec!["surely"], k);
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
		let mut t = TrieST::<usize>::new();
		for (i, s) in a.iter().enumerate() {
			t.put(s.as_bytes(), i);
		}

		assert_eq!(None, t.longest_key_of("".as_bytes()));
		assert_eq!(None, t.longest_key_of("a".as_bytes()));
		assert_eq!(Some(3), t.longest_key_of("are".as_bytes()));

		assert_eq!(None, t.longest_key_of("s".as_bytes()));

		assert_eq!(Some(3), t.longest_key_of("sea".as_bytes()));
		assert_eq!(Some(3), t.longest_key_of("seafood".as_bytes()));
		assert_eq!(Some(9), t.longest_key_of("seashellsabc".as_bytes()));
	}
}
