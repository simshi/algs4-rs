//! Minimum Edit Distance
//!
//! Given two strings word1 and word2, return the minimum number of operations
//! required to convert word1 to word2.
//!
//! You have the following three operations permitted on a word:
//!   - Insert a character
//!   - Delete a character
//!   - Replace a character
//!
//! Example 1:
//! <pre>
//!  Input: word1 = "horse", word2 = "ros"
//!  Output: 3
//!  Explanation:
//!  horse -> rorse (replace 'h' with 'r')
//!  rorse -> rose (remove 'r')
//!  rose -> ros (remove 'e')
//! </pre>
macro_rules! xmin {
    ($x:expr) => ($x);
    ($x:expr, $($y:expr),+) => (std::cmp::min($x, xmin!($($y),*)));
}
pub fn min_edit_distance(s1: &str, s2: &str) -> usize {
    let (m, n) = (s1.len(), s2.len());
    if m + n <= 1 {
        return m + n;
    }

    // avoid `s.chars().nth(n).unwrap()`
    let (s1, s2) = (s1.as_bytes(), s2.as_bytes());
    let mut pre = vec![0; n + 1];
    let mut cur = vec![0; n + 1];

    for i in 0..n {
        pre[i + 1] = i + 1;
    }
    for (i, &c1) in s1.iter().enumerate() {
        cur[0] = i + 1;
        for (j, &c2) in s2.iter().enumerate() {
            if c1 == c2 {
                cur[j + 1] = pre[j];
            } else {
                // cur[j + 1] = 1 + [pre[j], pre[j + 1], cur[j]].iter().min().unwrap();
                // min(replace, insert, delete)
                cur[j + 1] = 1 + xmin!(pre[j], pre[j + 1], cur[j]);
            }
        }
        //(pre, cur) = (cur, pre)
        std::mem::swap(&mut pre, &mut cur);
    }

    pre[n]
}

pub fn min_edit_distance_recursive(s1: &str, s2: &str) -> usize {
    let (m, n) = (s1.len(), s2.len());
    // avoid `s.chars().nth(n).unwrap()`
    let (s1, s2) = (s1.as_bytes(), s2.as_bytes());
    do_recursive(s1, m, s2, n)
}
fn do_recursive(s1: &[u8], m: usize, s2: &[u8], n: usize) -> usize {
    if m == 0 || n == 0 {
        return m + n;
    }

    if s1[m - 1] == s2[n - 1] {
        do_recursive(s1, m - 1, s2, n - 1)
    } else {
        1 + xmin!(
            do_recursive(s1, m - 1, s2, n),     // remove
            do_recursive(s1, m, s2, n - 1),     // insert
            do_recursive(s1, m - 1, s2, n - 1)  // replace
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recursive() {
        assert_eq!(0, min_edit_distance_recursive("h", "h"));
        assert_eq!(4, min_edit_distance_recursive("hhhhh", "h"));
        assert_eq!(1, min_edit_distance_recursive("ab", "a"));
        assert_eq!(3, min_edit_distance_recursive("horse", "roe"));
        assert_eq!(5, min_edit_distance_recursive("intention", "execution"));
    }

    #[test]
    fn tests() {
        assert_eq!(0, min_edit_distance("h", "h"));
        assert_eq!(4, min_edit_distance("hhhhh", "h"));
        assert_eq!(1, min_edit_distance("ab", "a"));
        assert_eq!(3, min_edit_distance("horse", "roe"));
        assert_eq!(5, min_edit_distance("intention", "execution"));
    }
}
