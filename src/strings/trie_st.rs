use std::iter::FromIterator;

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
impl<T> TrieST<T> {
	pub fn new() -> Self {
		Self { root: None, n: 0 }
	}
	pub fn size(&self) -> usize {
		self.n
	}
	pub fn is_empty(&self) -> bool {
		self.size() == 0
	}

	pub fn get(&self, key: &str) -> Option<&T> {
		Self::_get_node(&self.root, key, 0)
			.as_ref()
			.and_then(|node| node.val.as_ref())
	}
	pub fn put(&mut self, key: &str, val: T) {
		let p = self.root.take();
		self.root = self._put(p, key, val, 0);
	}
	pub fn delete(&mut self, key: &str) {
		let p = self.root.take();
		self.root = self._delete(p, key, 0);
	}

	pub fn keys_with_prefix(&self, prefix: &str) -> impl Iterator<Item = String> {
		let mut results = Vec::new();
		let p = Self::_get_node(&self.root, prefix, 0);
		let mut cv = prefix.chars().collect::<Vec<_>>();
		Self::_collect(p, &mut cv, &mut results);

		results.into_iter()
	}
	pub fn longest_key_of(&self, prefix: &str) -> Option<String> {
		let max_length = Self::_longest_key(&self.root, prefix, 0, 0);
		if max_length == 0 {
			None
		} else {
			Some(String::from(&prefix[..max_length]))
		}
	}
	pub fn keys_match_pattern(&self, pattern: &str) -> impl Iterator<Item = String> {
		let mut results = Vec::new();
		let mut cv: Vec<char> = Vec::new();
		Self::_collect_match_pattern(&self.root, pattern, &mut cv, &mut results);

		results.into_iter()
	}
}

// private methods
impl<T> TrieST<T> {
	fn _get_node<'a>(p: &'a NodePtr<T>, key: &str, d: usize) -> &'a NodePtr<T> {
		if let Some(node) = p {
			if d < key.len() {
				let c = key.chars().nth(d).unwrap();
				return Self::_get_node(&node.next[c as usize], key, d + 1);
			}
		}

		p
	}

	fn _put(&mut self, p: NodePtr<T>, key: &str, val: T, d: usize) -> NodePtr<T> {
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

		let c = key.chars().nth(d).unwrap();
		let pc = node.next[c as usize].take();
		node.next[c as usize] = self._put(pc, key, val, d + 1);

		Some(node)
	}

	fn _delete(&mut self, p: NodePtr<T>, key: &str, d: usize) -> NodePtr<T> {
		p.and_then(|mut node| {
			if d == key.len() {
				if node.val.is_some() {
					self.n -= 1;
					node.val = None;
				}
			} else {
				let c = key.chars().nth(d).unwrap();
				let pc = node.next[c as usize].take();
				node.next[c as usize] = self._delete(pc, key, d + 1);
			}

			if node.val.is_some() || node.next.iter().any(|p| p.is_some()) {
				Some(node)
			} else {
				None
			}
		})
	}

	fn _collect(p: &NodePtr<T>, cv: &mut Vec<char>, results: &mut Vec<String>) {
		if let Some(node) = p.as_ref() {
			if node.val.is_some() {
				results.push(String::from_iter(cv.iter().cloned()));
			}

			for (i, p) in node.next.iter().enumerate() {
				if p.is_some() {
					cv.push(std::char::from_u32(i as u32).unwrap());
					Self::_collect(p, cv, results);
					cv.pop();
				}
			}
		}
	}

	fn _longest_key(p: &NodePtr<T>, query: &str, d: usize, mut length: usize) -> usize {
		if let Some(node) = p {
			if node.val.is_some() {
				length = d;
			}
			if d == query.len() {
				return length;
			}

			let c = query.chars().nth(d).unwrap();
			length = Self::_longest_key(&node.next[c as usize], query, d + 1, length)
		}

		return length;
	}

	fn _collect_match_pattern(
		p: &NodePtr<T>,
		pattern: &str,
		cv: &mut Vec<char>,
		results: &mut Vec<String>,
	) {
		if let Some(node) = p.as_ref() {
			let d = cv.len();
			if d == pattern.len() {
				if node.val.is_some() {
					results.push(String::from_iter(cv.iter().cloned()));
				}
				return;
			}

			let c = pattern.chars().nth(d).unwrap();
			if c == '.' {
				for (i, p) in node.next.iter().enumerate() {
					cv.push(std::char::from_u32(i as u32).unwrap());
					Self::_collect_match_pattern(p, pattern, cv, results);
					cv.pop();
				}
			} else {
				cv.push(std::char::from_u32(c as u32).unwrap());
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
		t.put("LK", 6.4);
		t.put("AAPL", 244.93);
		assert_eq!(2, t.size());

		assert_eq!(None, t.get("MSFT"));
		assert_eq!(Some(&244.93), t.get("AAPL"));
		assert_eq!(Some(&6.4), t.get("LK"));

		assert_eq!(None, t.get("AAP"));
		assert_eq!(None, t.get("AA"));
		assert_eq!(None, t.get("A"));
		assert_eq!(None, t.get("L"));
	}

	#[test]
	fn update() {
		let mut t = TrieST::<f64>::new();
		t.put("LK", 6.4);
		t.put("AAPL", 244.93);
		t.put("AAPL", 250.13);

		assert_eq!(Some(&6.4), t.get("LK"));
		assert_eq!(Some(&250.13), t.get("AAPL"));
	}

	#[test]
	fn delete() {
		let mut t = TrieST::<f64>::new();
		t.put("LK", 6.4);
		t.put("sea", 120.93);
		t.put("seafood", 150.13);
		t.put("seashell", 250.13);
		t.put("seashells", 50.13);

		// delete middle
		t.delete("seashell");
		assert_eq!(None, t.get("seashell"));
		assert_eq!(Some(&120.93), t.get("sea"));
		assert_eq!(Some(&150.13), t.get("seafood"));
		assert_eq!(Some(&50.13), t.get("seashells"));
		assert_eq!(Some(&6.4), t.get("LK"));

		// delete leaf which has parent
		t.delete("seafood");
		assert_eq!(None, t.get("seafood"));
		assert_eq!(Some(&50.13), t.get("seashells"));
		assert_eq!(Some(&120.93), t.get("sea"));
		assert_eq!(Some(&6.4), t.get("LK"));

		// delete parent
		t.delete("sea");
		assert_eq!(None, t.get("sea"));
		assert_eq!(None, t.get("seafood"));
		assert_eq!(Some(&50.13), t.get("seashells"));
		assert_eq!(Some(&6.4), t.get("LK"));

		// delete leaf without parent
		t.delete("seashells");
		assert_eq!(None, t.get("seashell"));
		assert_eq!(None, t.get("seashells"));
		assert_eq!(Some(&6.4), t.get("LK"));
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
			t.put(s, i);
		}

		let mut k = t.keys_with_prefix("").collect::<Vec<_>>();
		k.sort_unstable();
		let mut ac = a.clone();
		ac.sort_unstable();
		assert_eq!(ac, k);

		let mut k = t.keys_with_prefix("a").collect::<Vec<_>>();
		k.sort_unstable();
		assert_eq!(vec!["are"], k);

		let mut k = t.keys_with_prefix("s").collect::<Vec<_>>();
		k.sort_unstable();
		assert_eq!(
			vec!["sea", "seashells", "she", "shells", "shore", "surely"],
			k
		);

		let mut k = t.keys_with_prefix("sh").collect::<Vec<_>>();
		k.sort_unstable();
		assert_eq!(vec!["she", "shells", "shore"], k);

		let mut k = t.keys_with_prefix("she").collect::<Vec<_>>();
		k.sort_unstable();
		assert_eq!(vec!["she", "shells"], k);

		let mut k = t.keys_with_prefix("shel").collect::<Vec<_>>();
		k.sort_unstable();
		assert_eq!(vec!["shells"], k);

		let mut k = t.keys_with_prefix("shells").collect::<Vec<_>>();
		k.sort_unstable();
		assert_eq!(vec!["shells"], k);

		let k = t.keys_with_prefix("shellsxxx").collect::<Vec<_>>();
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
			t.put(s, i);
		}

		let k = t.keys_match_pattern("").collect::<Vec<_>>();
		assert_eq!(0, k.len());

		let mut k = t.keys_match_pattern("a..").collect::<Vec<_>>();
		k.sort_unstable();
		assert_eq!(vec!["are"], k);

		let mut k = t.keys_match_pattern("a.e").collect::<Vec<_>>();
		k.sort_unstable();
		assert_eq!(vec!["are"], k);

		let mut k = t.keys_match_pattern("ar.").collect::<Vec<_>>();
		k.sort_unstable();
		assert_eq!(vec!["are"], k);

		let mut k = t.keys_match_pattern(".re").collect::<Vec<_>>();
		k.sort_unstable();
		assert_eq!(vec!["are"], k);

		let mut k = t.keys_match_pattern(".r.").collect::<Vec<_>>();
		k.sort_unstable();
		assert_eq!(vec!["are"], k);

		let mut k = t.keys_match_pattern("..e").collect::<Vec<_>>();
		k.sort_unstable();
		assert_eq!(vec!["are", "she", "the"], k);

		let mut k = t.keys_match_pattern("......").collect::<Vec<_>>();
		k.sort_unstable();
		assert_eq!(vec!["shells", "surely"], k);

		let mut k = t.keys_match_pattern("..rel.").collect::<Vec<_>>();
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
			t.put(s, i);
		}

		assert_eq!(None, t.longest_key_of(""));
		assert_eq!(None, t.longest_key_of("a"));
		assert_eq!(Some(String::from("are")), t.longest_key_of("are"));

		assert_eq!(None, t.longest_key_of("s"));

		assert_eq!(Some(String::from("sea")), t.longest_key_of("sea"));
		assert_eq!(Some(String::from("sea")), t.longest_key_of("seafood"));
		assert_eq!(
			Some(String::from("seashells")),
			t.longest_key_of("seashellsabc")
		);
	}
}
