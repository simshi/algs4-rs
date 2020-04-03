const R: usize = 256;

pub struct Node<T> {
	val: Option<T>,
	next: [NodePtr<T>; R],
}
type NodePtr<T> = Option<Box<Node<T>>>;

macro_rules! make_array {
	($n:expr, $constructor:expr, $ty:ty) => {
		unsafe {
			let mut items: std::mem::MaybeUninit<[_; $n]> = std::mem::MaybeUninit::uninit();
			// for (i, place) in items.iter_mut().enumerate() {
			// std::ptr::write(place, $constructor(i));
			let arr_ptr = items.as_mut_ptr() as *mut Option<$ty>;
			for i in 0..$n {
				arr_ptr.add(i).write($constructor(i));
				}
			items.assume_init()
			}
	};
}

impl<T> Node<T> {
	fn new(val: Option<T>) -> Self {
		Self {
			val,
			next: make_array!(R, |_| None, T),
		}
	}
}

pub struct TrieST<T: Copy> {
	root: NodePtr<T>,
	n: usize,
}
impl<T: Copy> TrieST<T> {
	pub fn new() -> Self {
		Self { root: None, n: 0 }
	}
	pub fn size(&self) -> usize {
		self.n
	}
	pub fn is_empty(&self) -> bool {
		self.size() == 0
	}

	pub fn put(&mut self, key: &str, val: T) {
		let p = self.root.take();
		self.root = self._put(p, key, val, 0);
	}
	pub fn get(&mut self, key: &str) -> Option<&T> {
		None
	}
}

// private methods
impl<T: Copy> TrieST<T> {
	fn _put(&mut self, p: NodePtr<T>, key: &str, val: T, d: usize) -> NodePtr<T> {
		let mut is_new = false;
		let mut node = match p {
			Some(node) => node,
			_ => {
				let n = Box::new(Node::new(Some(val)));
				is_new = true;
				n
			}
		};

		if d == key.len() {
			if is_new {
				self.n += 1;
			} else {
				node.val = Some(val); // update
			}
			return Some(node);
		}

		let c = key.chars().nth(d).unwrap();
		let pc = node.next[c as usize].take();
		node.next[c as usize] = self._put(pc, key, val, d + 1);

		Some(node)
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
	}
}
