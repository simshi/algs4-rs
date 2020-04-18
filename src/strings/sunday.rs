pub fn sunday_search(text: &str, pattern: &str) -> Option<usize> {
	if text.is_empty() || pattern.is_empty() {
		return None;
	}

	if pattern.len() == 1 {
		let pc = pattern.chars().next().unwrap();
		for (i, c) in text.chars().enumerate() {
			if c == pc {
				return Some(i);
			}
		}
		return None;
	}

	let rmt = build_right_most_table(pattern);
	let m = pattern.len();
	let mut i = 0;
	while i + m <= text.len() {
		let mut skip = 0;
		for j in 0..m {
			if text.chars().nth(i + j).unwrap() != pattern.chars().nth(j).unwrap() {
				if i + m < text.len() {
					let c = text.chars().nth(i + m).unwrap();
					skip = rmt[c as usize];
					break;
				} else {
					return None;
				}
			}
		}
		if skip == 0 {
			return Some(i as usize);
		}
		i += (m as isize - skip) as usize;
	}

	None
}
fn build_right_most_table(s: &str) -> Vec<isize> {
	let mut rmt: Vec<isize> = vec![-1; 256];
	for (i, c) in s.chars().enumerate() {
		rmt[c as usize] = i as isize;
	}

	rmt
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn empty() {
		assert_eq!(None, sunday_search("", ""));
		assert_eq!(None, sunday_search("abc", ""));
		assert_eq!(None, sunday_search("", "abc"));
	}

	#[test]
	fn rmt() {
		let t = build_right_most_table("");
		assert!(t.iter().all(|&p| p == -1));

		let t = build_right_most_table("a");
		assert_eq!(0, t['a' as usize]);
		assert!(t
			.iter()
			.enumerate()
			.all(|(i, &p)| (i == 'a' as usize && p == 0) || p == -1));

		let t = build_right_most_table("abcd");
		assert_eq!(
			vec![0, 1, 2, 3],
			vec![
				t['a' as usize],
				t['b' as usize],
				t['c' as usize],
				t['d' as usize]
			]
		);

		let t = build_right_most_table("abcab");
		assert_eq!(
			vec![3, 4, 2],
			vec![t['a' as usize], t['b' as usize], t['c' as usize]]
		);
	}

	#[test]
	fn basic() {
		assert_eq!(Some(0), sunday_search("abc", "a"));
		assert_eq!(Some(1), sunday_search("abc", "b"));
		assert_eq!(Some(2), sunday_search("abc", "c"));

		assert_eq!(Some(0), sunday_search("abc", "ab"));
		assert_eq!(Some(1), sunday_search("abc", "bc"));
		assert_eq!(Some(0), sunday_search("abc", "abc"));
	}

	#[test]
	fn examples() {
		assert_eq!(
			Some(17),
			sunday_search("here is a simple example", "example")
		);

		assert_eq!(Some(9), sunday_search("BCBAABACAABABACAA", "ABABAC"));
		assert_eq!(Some(8), sunday_search("ccaabaabaabaaabaab", "aabaaaba"));
		assert_eq!(None, sunday_search("ccaabaabaabaaabaab", "aabaaabb"));
	}

	#[test]
	fn last_char_test() {
		assert_eq!(Some(1), sunday_search("aabc", "abc"));
		assert_eq!(None, sunday_search("aabd", "abc"));
		assert_eq!(None, sunday_search("abd", "abc"));
		assert_eq!(None, sunday_search("abcabd", "abcabc"));
	}

	#[test]
	fn skip_then_match() {
		assert_eq!(Some(1), sunday_search("aabab", "abab"));
		assert_eq!(Some(2), sunday_search("ababac", "abac"));
		assert_eq!(Some(4), sunday_search("abacabab", "abab"));

		assert_eq!(Some(4), sunday_search("abcdabcdabce", "abcdabce"));
		assert_eq!(Some(3), sunday_search("abcabcabadef", "abcabad"));
		assert_eq!(Some(5), sunday_search("abcababcabad", "abcabad"));
	}
}
