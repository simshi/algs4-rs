pub fn msd_sort(a: &mut Vec<&str>) {
	let mut msd = MSD::new(a.len());
	msd.sort(a);
}

#[derive(Default, Clone, Copy)]
struct WorkSpace {
	lo: usize,
	hi: usize,
	d: usize,
}
struct MSD {
	iv: Vec<usize>,
	count: Vec<usize>,
	stack: Vec<WorkSpace>,
}
impl MSD {
	fn new(len: usize) -> Self {
		Self {
			iv: vec![0; len],
			count: vec![0; 256 + 2],
			stack: Vec::new(),
		}
	}

	fn sort(&mut self, a: &mut Vec<&str>) {
		if a.len() <= 1 {
			return;
		}

		self.stack.push(WorkSpace {
			lo: 0,
			hi: a.len(),
			d: 0,
		});
		while let Some(WorkSpace { lo, hi, d }) = self.stack.pop() {
			self.sort_range(a, lo, hi, d);
		}
	}

	fn sort_range(&mut self, a: &mut Vec<&str>, lo: usize, hi: usize, d: usize) {
		self.count.clear();
		self.count.resize(self.count.capacity(), 0);

		// frequency counts of char
		for s in a.iter().take(hi).skip(lo) {
			self.count[Self::str_char_at(s, d)] += 1;
		}

		// accumulate counts to indicies
		for r in 1..self.count.len() {
			self.count[r] += self.count[r - 1];
		}

		// return earlier while all strings are end
		if *self.count.last().unwrap() == self.count[1] {
			return;
		}

		// target index of each element
		for i in lo..hi {
			let idx = Self::str_char_at(a[i], d);
			self.iv[i] = lo + self.count[idx - 1];
			self.count[idx - 1] += 1;
		}

		// reorder `a` according to `iv`
		// each swap moves at least one element into correct position, so it's linear
		for i in lo..hi {
			while self.iv[i] != i {
				let j = self.iv[i];
				a.swap(i, j);
				self.iv.swap(i, j);
			}
		}

		// count[1] is count of strings whose length < d
		for r in 2..self.count.len() {
			if self.count[r - 1] + 2 <= self.count[r] {
				self.stack.push(WorkSpace {
					lo: lo + self.count[r - 1],
					hi: lo + self.count[r],
					d: d + 1,
				});
			}
		}
	}

	fn str_char_at(s: &str, i: usize) -> usize {
		// TODO: optimization?
		s.chars().nth(i).map_or(1, |c| (c as usize) + 2)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn empty() {
		let mut a: Vec<&str> = vec![];
		msd_sort(&mut a);
	}

	#[test]
	fn three_elements() {
		let mut a = vec!["bca", "ab", "bc"];
		msd_sort(&mut a);
		assert_eq!(vec!["ab", "bc", "bca"], a);
	}

	#[test]
	fn tiny() {
		let mut a = vec![
			"she",
			"sells",
			"seashells",
			"by",
			"the",
			"sea",
			"shore",
			"the",
			"shells",
			"she",
			"sells",
			"are",
			"surely",
			"seashells",
		];
		msd_sort(&mut a);
		assert_eq!(
			vec![
				"are",
				"by",
				"sea",
				"seashells",
				"seashells",
				"sells",
				"sells",
				"she",
				"she",
				"shells",
				"shore",
				"surely",
				"the",
				"the"
			],
			a
		);
	}
}
