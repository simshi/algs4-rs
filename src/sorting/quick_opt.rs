pub fn quick_sort_opt<T>(arr: &mut [T])
where
    T: PartialOrd,
{
    let n = arr.len();
    if n <= 1 {
        return;
    }

    sort(arr, 0, n - 1);
}

fn sort<T: PartialOrd>(arr: &mut [T], lo: usize, hi: usize) {
    if lo >= hi {
        return;
    }

    if lo + 15 > hi {
        super::insertion_sort(&mut arr[lo..=hi]);
        return;
    }

    let p = partition(arr, lo, hi);
    sort(arr, lo, p);
    sort(arr, p + 1, hi);
    //lo = p + 1;
}

fn partition<T: PartialOrd>(arr: &mut [T], lo: usize, hi: usize) -> usize {
    // select pivot by median-of-three
    let mid = lo + (hi - lo) / 2;
    if arr[lo] > arr[hi] {
        arr.swap(lo, hi);
    } // lo <= hi
    if arr[mid] > arr[hi] {
        arr.swap(mid, hi);
    } // mid <= hi, lo <= hi
    if arr[lo] > arr[mid] {
        arr.swap(lo, mid);
    } // lo <= mid <= hi

    // Hoare partition schema
    let mut i = lo;
    let mut j = hi + 1;
    loop {
        // do-while in Rust...
        while {
            i += 1;
            arr[i - 1] < arr[mid]
        } {}
        while {
            j -= 1;
            arr[j] > arr[mid]
        } {}

        if i - 1 >= j {
            break;
        }

        arr.swap(i - 1, j);
    }

    j
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sorting::is_sorted;

    #[test]
    fn empty() {
        let mut arr: [i32; 0] = [];
        quick_sort_opt(&mut arr);
    }

    #[test]
    fn already_sorted() {
        let mut arr = [1, 2, 3];
        quick_sort_opt(&mut arr);
        assert_eq!(1, arr[0]);
        assert_eq!(2, arr[1]);
        assert_eq!(3, arr[2]);
    }

    #[test]
    fn two_elements() {
        let mut arr = [3, 2];
        quick_sort_opt(&mut arr);
        assert_eq!(2, arr[0]);
        assert_eq!(3, arr[1]);
    }

    #[test]
    fn several_elements() {
        let mut arr = [3, 2, 5, 8, 1];
        quick_sort_opt(&mut arr);
        assert!(is_sorted(&arr));
    }

    #[test]
    fn with_equal_keys() {
        let mut arr = [3, 2, 1, 2, 1];
        quick_sort_opt(&mut arr);
        assert!(is_sorted(&arr));
    }

    #[test]
    fn all_identical() {
        let mut arr = [1, 1, 1, 1, 1];
        quick_sort_opt(&mut arr);
        assert!(is_sorted(&arr));
    }
}
