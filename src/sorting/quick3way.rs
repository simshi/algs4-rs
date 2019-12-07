pub fn quick3way_sort<T>(arr: &mut [T])
where
    T: PartialOrd,
{
    let n = arr.len();
    if n <= 1 {
        return;
    }

    sort3way(arr, 0, n - 1);
}

fn sort3way<T: PartialOrd>(arr: &mut [T], lo: usize, hi: usize) {
    if lo >= hi {
        return;
    }

    if lo + 15 > hi {
        super::insertion_sort(&mut arr[lo..=hi]);
        return;
    }

    let (lt, gt) = partition3way(arr, lo, hi);
    if lt > 0 {
        sort3way(arr, lo, lt - 1);
    }

    sort3way(arr, gt + 1, hi);
}

fn partition3way<T: PartialOrd>(arr: &mut [T], lo: usize, hi: usize) -> (usize, usize) {
    // select pivot by median-of-three
    let mid = lo + (hi - lo) / 2;
    if arr[lo] > arr[hi] {
        arr.swap(lo, hi);
    } // lo <= hi
    if arr[mid] > arr[hi] {
        arr.swap(mid, hi);
    } // mid <= hi, lo <= hi
    if arr[lo] < arr[mid] {
        arr.swap(lo, mid);
    } // mid <= lo <= hi

    let mut lt = lo;
    let mut i = lo + 1;
    let mut gt = hi;
    while i <= gt {
        if arr[i] < arr[lt] {
            arr.swap(i, lt);
            i += 1;
            lt += 1;
        } else if arr[i] > arr[lt] {
            arr.swap(i, gt);
            gt -= 1;
        } else {
            i += 1;
        }
    }

    (lt, gt)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sorting::is_sorted;
    use rand::{thread_rng, Rng};

    #[test]
    fn empty() {
        let mut arr: [i32; 0] = [];
        quick3way_sort(&mut arr);
    }

    #[test]
    fn already_sorted() {
        let mut arr = [1, 2, 3];
        quick3way_sort(&mut arr);
        assert_eq!(1, arr[0]);
        assert_eq!(2, arr[1]);
        assert_eq!(3, arr[2]);
    }

    #[test]
    fn two_elements() {
        let mut arr = [3, 2];
        quick3way_sort(&mut arr);
        assert_eq!(2, arr[0]);
        assert_eq!(3, arr[1]);
    }

    #[test]
    fn several_elements() {
        let mut arr = [3, 2, 5, 8, 1];
        quick3way_sort(&mut arr);
        assert_eq!(1, arr[0]);
        assert_eq!(2, arr[1]);
        assert_eq!(3, arr[2]);
        assert_eq!(5, arr[3]);
        assert_eq!(8, arr[4]);
    }

    #[test]
    fn with_equal_keys() {
        let mut arr = [3, 2, 1, 2, 1];
        quick3way_sort(&mut arr);
        assert_eq!(1, arr[0]);
        assert_eq!(1, arr[1]);
        assert_eq!(2, arr[2]);
        assert_eq!(2, arr[3]);
        assert_eq!(3, arr[4]);
    }

    #[test]
    fn random_100() {
        let mut arr = thread_rng().gen_iter::<u32>().take(100).collect::<Vec<_>>();
        quick3way_sort(&mut arr);
        assert!(is_sorted(&arr));
    }
}
