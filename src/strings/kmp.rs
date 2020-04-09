pub fn kmp_search(text: &str, pattern: &str) -> Option<usize> {
	if text.len() == 0 || pattern.len() == 0 {
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

	let next = build_pmt(pattern);
	let m = pattern.len();
	let mut j: isize = 0;
	for (i, c) in text.chars().enumerate() {
		while j >= 0 && c != pattern.chars().nth(j as usize).unwrap() {
			j = next[j as usize];
		}
		j += 1;
		if j as usize >= m {
			return Some(i + 1 - m);
		}
	}

	None
}
fn build_pmt(pattern: &str) -> Vec<isize> {
	let mut pmt: Vec<isize> = vec![0; pattern.len()];
	let mut j: isize = -1; // matched prefix length
	let mut i = 0;
	// -1 requires both text and pattern advance, 0 means set pattern to begin and then compare with current char of text
	pmt[0] = -1;
	while i + 1 < pattern.len() {
		if j < 0 || str_char_equal(pattern, i, j as usize) {
			i += 1;
			j += 1;
			pmt[i] = if str_char_equal(pattern, i, j as usize) {
				pmt[j as usize] // equals to last matched prefix length
			} else {
				j // as normal
			};
		} else {
			j = pmt[j as usize];
		}
	}

	pmt
}
fn str_char_equal(s: &str, i: usize, j: usize) -> bool {
	s.chars().nth(i).unwrap() == s.chars().nth(j).unwrap()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn empty() {
		assert_eq!(None, kmp_search("", ""));
		assert_eq!(None, kmp_search("abc", ""));
		assert_eq!(None, kmp_search("", "abc"));
	}

	#[test]
	fn basic() {
		assert_eq!(Some(0), kmp_search("abc", "a"));
		assert_eq!(Some(1), kmp_search("abc", "b"));
		assert_eq!(Some(2), kmp_search("abc", "c"));

		assert_eq!(vec![-1, 0], build_pmt("ab"));
		assert_eq!(Some(0), kmp_search("abc", "ab"));
		assert_eq!(vec![-1, 0], build_pmt("bc"));
		assert_eq!(Some(1), kmp_search("abc", "bc"));
		assert_eq!(vec![-1, 0, 0], build_pmt("abc"));
		assert_eq!(Some(0), kmp_search("abc", "abc"));
	}

	#[test]
	fn examples() {
		assert_eq!(Some(9), kmp_search("BCBAABACAABABACAA", "ABABAC"));
		assert_eq!(Some(8), kmp_search("ccaabaabaabaaabaab", "aabaaaba"));
		assert_eq!(None, kmp_search("ccaabaabaabaaabaab", "aabaaabb"));
	}

	#[test]
	fn optimization() {
		// without optimization, table of "abab" is [-1, 0, 0, 1], that means
		// if second 'a' mismatch, reset to pattern[0], but since we know
		// pattern[0] is 'a' too, we can further set it to next[0]
		assert_eq!(vec![-1, 0, -1, 0], build_pmt("abab"));
		assert_eq!(vec![-1, 0, 0, -1, 0, 0], build_pmt("abcabc"));
		assert_eq!(vec![-1, 0, 0, 0, -1, 0, 0, 3], build_pmt("abcdabce"));
		// without optimization, table of "abcabad" is [-1, 0, 0, 0, 1, 2, 1],
		// that means if second 'b' mismatch, reset to pattern[1], but since we
		// know pattern[1] is 'b' too, we can set to next[1]
		assert_eq!(vec![-1, 0, 0, -1, 0, 2, 1], build_pmt("abcabad"));

		assert_eq!(None, kmp_search("abcbab", "abab"));
		assert_eq!(Some(4), kmp_search("abacabab", "abab"));
		assert_eq!(None, kmp_search("abcabd", "abcabc"));

		assert_eq!(Some(4), kmp_search("abcdabcdabce", "abcdabce"));

		assert_eq!(Some(3), kmp_search("abcabcabadef", "abcabad"));
		assert_eq!(Some(5), kmp_search("abcababcabad", "abcabad"));
	}
}
