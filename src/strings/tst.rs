use std::cmp::Ordering::{Equal, Greater, Less};
use std::iter::FromIterator;

struct Node<T> {
	c: char,
	val: Option<T>,
	left: NodePtr<T>,
	middle: NodePtr<T>,
	right: NodePtr<T>,
}
type NodePtr<T> = Option<Box<Node<T>>>;

impl<T> Node<T> {
	fn new(c: char, val: Option<T>) -> Self {
		Self {
			c,
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
impl<T: Copy> TST<T> {
	pub fn new() -> Self {
		TST { root: None, n: 0 }
	}
	pub fn size(&self) -> usize {
		self.n
	}
	pub fn is_empty(&self) -> bool {
		self.size() == 0
	}

	pub fn get(&self, key: &str) -> Option<&T> {
		if key.is_empty() {
			None
		} else {
			Self::_get_node(&self.root, key, 0)
				.as_ref()
				.and_then(|node| node.val.as_ref())
		}
	}
	pub fn put(&mut self, key: &str, val: T) {
		if !key.is_empty() {
			let p = self.root.take();
			self.root = self._put(p, key, val, 0);
		}
	}
	pub fn delete(&mut self, key: &str) {
		if !key.is_empty() {
			let p = self.root.take();
			self.root = self._delete(p, key, 0);
		}
	}

	pub fn keys_with_prefix(&self, prefix: &str) -> impl Iterator<Item = String> {
		let mut cv = prefix.chars().collect::<Vec<_>>();
		let mut results = Vec::new();

		if prefix.is_empty() {
			Self::_collect(&self.root, &mut cv, &mut results);
		} else {
			let p = Self::_get_node(&self.root, prefix, 0);

			if let Some(node) = p {
				if node.val.is_some() {
					// a key equals to prefix exists
					results.push(String::from(prefix));
				}
				Self::_collect(&node.middle, &mut cv, &mut results);
			}
		};

		results.into_iter()
	}
	pub fn longest_key_of(&self, prefix: &str) -> Option<String> {
		if prefix.is_empty() {
			return None;
		}

		let mut max_length = 0;
		Self::_longest_key(&self.root, prefix, 0, &mut max_length);
		if max_length == 0 {
			None
		} else {
			Some(String::from(&prefix[..max_length]))
		}
	}
	pub fn keys_match_pattern(&self, pattern: &str) -> impl Iterator<Item = String> {
		let mut results = Vec::new();
		if !pattern.is_empty() {
			let mut cv: Vec<char> = Vec::new();
			Self::_collect_match_pattern(&self.root, pattern, &mut cv, &mut results);
		}
		results.into_iter()
	}
}

// private methods
impl<T: Copy> TST<T> {
	fn _get_node<'a>(p: &'a NodePtr<T>, key: &str, d: usize) -> &'a NodePtr<T> {
		if let Some(node) = p {
			let c = key.chars().nth(d).unwrap();
			return match c.cmp(&node.c) {
				Less => Self::_get_node(&node.left, key, d),
				Greater => Self::_get_node(&node.right, key, d),
				Equal if d + 1 == key.len() => p,
				_ => Self::_get_node(&node.middle, key, d + 1),
			};
		}

		p
	}

	fn _put(&mut self, p: NodePtr<T>, key: &str, val: T, d: usize) -> NodePtr<T> {
		let c = key.chars().nth(d).unwrap();
		let mut is_new = false;
		let mut node = match p {
			Some(node) => node,
			_ => {
				let n = Box::new(Node::new(c, None));
				is_new = true;
				n
			}
		};

		match c.cmp(&node.c) {
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

	fn _delete(&mut self, p: NodePtr<T>, key: &str, d: usize) -> NodePtr<T> {
		p.and_then(|mut node| {
			let c = key.chars().nth(d).unwrap();
			match c.cmp(&node.c) {
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

			if node.val.is_some()
				|| node.left.is_some()
				|| node.middle.is_some()
				|| node.right.is_some()
			{
				Some(node)
			} else {
				None
			}
		})
	}

	fn _collect(p: &NodePtr<T>, cv: &mut Vec<char>, results: &mut Vec<String>) {
		if let Some(node) = p.as_ref() {
			if node.val.is_some() {
				cv.push(node.c);
				results.push(String::from_iter(cv.iter().cloned()));
				cv.pop();
			}

			Self::_collect(&node.left, cv, results);
			cv.push(node.c);
			Self::_collect(&node.middle, cv, results);
			cv.pop();
			Self::_collect(&node.right, cv, results);
		}
	}

	fn _longest_key(p: &NodePtr<T>, query: &str, d: usize, length: &mut usize) {
		if let Some(node) = p {
			if node.val.is_some() {
				*length = d + 1;
			}

			let c = query.chars().nth(d).unwrap();
			match c.cmp(&node.c) {
				Less => Self::_longest_key(&node.left, query, d, length),
				Greater => Self::_longest_key(&node.right, query, d, length),
				Equal if d + 1 == query.len() => (),
				_ => Self::_longest_key(&node.middle, query, d + 1, length),
			}
		}
	}

	fn _collect_match_pattern(
		p: &NodePtr<T>,
		pattern: &str,
		cv: &mut Vec<char>,
		results: &mut Vec<String>,
	) {
		if let Some(node) = p.as_ref() {
			let d = cv.len();
			let c = pattern.chars().nth(d).unwrap();

			if c == '.' || c < node.c {
				Self::_collect_match_pattern(&node.left, pattern, cv, results);
			}

			if c == '.' || c == node.c {
				cv.push(node.c);
				if d + 1 == pattern.len() {
					if node.val.is_some() {
						results.push(String::from_iter(cv.iter().cloned()));
					}
				} else {
					Self::_collect_match_pattern(&node.middle, pattern, cv, results);
				}
				cv.pop();
			}

			if c == '.' || c > node.c {
				Self::_collect_match_pattern(&node.right, pattern, cv, results);
			}
		}
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
		let mut t = TST::<f64>::new();
		t.put("LK", 6.4);
		t.put("AAPL", 244.93);
		t.put("AAPL", 250.13);

		assert_eq!(Some(&6.4), t.get("LK"));
		assert_eq!(Some(&250.13), t.get("AAPL"));
	}

	#[test]
	fn delete() {
		let mut t = TST::<f64>::new();
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
		let mut t = TST::<usize>::new();
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
		let mut t = TST::<usize>::new();
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
		let mut t = TST::<usize>::new();
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
