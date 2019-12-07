pub fn insertion_sort<T>(arr: &mut [T])
where
    T: PartialOrd,
{
    let n = arr.len();
    for i in 1..n {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sorting::is_sorted;
    use rand::{thread_rng, Rng};

    #[test]
    fn empty() {
        let mut arr: [i32; 0] = [];
        insertion_sort(&mut arr);
    }

    #[test]
    fn already_sorted() {
        let mut arr = [1, 2, 3];
        insertion_sort(&mut arr);
        assert_eq!(1, arr[0]);
        assert_eq!(2, arr[1]);
        assert_eq!(3, arr[2]);
    }

    #[test]
    fn two_elements() {
        let mut arr = [3, 2];
        insertion_sort(&mut arr);
        assert_eq!(2, arr[0]);
        assert_eq!(3, arr[1]);
    }

    #[test]
    fn several_elements() {
        let mut arr = [3, 2, 5, 8, 1];
        insertion_sort(&mut arr);
        assert_eq!(1, arr[0]);
        assert_eq!(2, arr[1]);
        assert_eq!(3, arr[2]);
        assert_eq!(5, arr[3]);
        assert_eq!(8, arr[4]);
    }

    #[test]
    fn random_100() {
        let mut arr = thread_rng().gen_iter::<u32>().take(100).collect::<Vec<_>>();
        insertion_sort(&mut arr);
        assert!(is_sorted(&arr));
    }
}
