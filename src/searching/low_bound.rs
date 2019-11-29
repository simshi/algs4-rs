use std::cmp::{PartialEq, PartialOrd};

pub fn low_bound<T>(x: &T, arr: &[T]) -> usize
where
    T: PartialEq + PartialOrd,
{
    let mut b = 0;
    let mut e = arr.len();

    while b < e {
        let m = b + (e - b) / 2;

        if x <= &arr[m] {
            e = m;
        } else {
            b = m + 1;
        }
    }

    b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(0, low_bound(&"a", &vec![]));
    }

    #[test]
    fn search_strings() {
        assert_eq!(0, low_bound(&"a", &vec!["a"]));

        assert_eq!(0, low_bound(&"a", &vec!["a", "b"]));
        assert_eq!(1, low_bound(&"b", &vec!["a", "b", "b"]));

        assert_eq!(0, low_bound(&"a", &vec!["a", "b", "b", "c"]));
        assert_eq!(1, low_bound(&"b", &vec!["a", "b", "b", "c"]));
        assert_eq!(3, low_bound(&"c", &vec!["a", "b", "b", "c"]));

        assert_eq!(2, low_bound(&"c", &vec!["a", "b", "c", "c"]));
    }

    #[test]
    fn search_ints() {
        assert_eq!(0, low_bound(&1, &vec![1, 1]));

        assert_eq!(0, low_bound(&1, &vec![1, 1, 2, 3, 4]));
        assert_eq!(2, low_bound(&2, &vec![1, 1, 2, 3, 4]));
        assert_eq!(4, low_bound(&4, &vec![1, 1, 2, 3, 4]));

        assert_eq!(0, low_bound(&0, &vec![1]));
        assert_eq!(0, low_bound(&0, &vec![1, 2]));
    }

    #[test]
    fn bigger() {
        assert_eq!(4, low_bound(&5, &vec![1, 2, 3, 4]));
    }
}
