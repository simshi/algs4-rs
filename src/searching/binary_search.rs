use std::cmp::{PartialEq, PartialOrd};

pub fn binary_search<T>(item: &T, arr: &[T]) -> Option<usize>
where
    T: PartialEq + PartialOrd,
{
    let mut b = 0;
    let mut e = arr.len();

    while b < e {
        let m = b + (e - b) / 2;

        if item == &arr[m] {
            return Some(m);
        } else if item < &arr[m] {
            e = m;
        } else {
            b = m + 1;
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let index = binary_search(&"a", &vec![]);
        assert_eq!(index, None);
    }

    #[test]
    fn search_strings() {
        let index = binary_search(&"a", &vec!["a"]);
        assert_eq!(index, Some(0));

        assert_eq!(Some(0), binary_search(&"a", &vec!["a", "b"]));
        assert_eq!(Some(1), binary_search(&"b", &vec!["a", "b"]));

        assert_eq!(Some(0), binary_search(&"a", &vec!["a", "b", "c"]));
        assert_eq!(Some(1), binary_search(&"b", &vec!["a", "b", "c"]));
        assert_eq!(Some(2), binary_search(&"c", &vec!["a", "b", "c"]));

        assert_eq!(Some(3), binary_search(&"foo", &vec!["a", "b", "c", "foo"]));
    }

    #[test]
    fn search_ints() {
        let index = binary_search(&4, &vec![1, 2, 3, 4]);
        assert_eq!(index, Some(3));

        let index = binary_search(&3, &vec![1, 2, 3, 4]);
        assert_eq!(index, Some(2));

        let index = binary_search(&2, &vec![1, 2, 3, 4]);
        assert_eq!(index, Some(1));

        let index = binary_search(&1, &vec![1, 2, 3, 4]);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn not_found() {
        let index = binary_search(&5, &vec![1, 2, 3, 4]);
        assert_eq!(index, None);
    }
}
