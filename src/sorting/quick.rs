pub fn quick_sort<T>(arr: &mut [T])
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

    let p = partition(arr, lo, hi);
    if p > 0 {
        sort(arr, lo, p - 1);
    }
    sort(arr, p + 1, hi);
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
    if arr[lo] < arr[mid] {
        arr.swap(lo, mid);
    } // mid <= lo <= hi

    let mut i = lo + 1;
    let mut j = hi;
    loop {
        while i <= hi && arr[i] < arr[lo] {
            i += 1;
        }
        while j >= lo && arr[j] > arr[lo] {
            j -= 1;
        }
        if i >= j {
            break;
        }

        arr.swap(i, j);
    }
    arr.swap(lo, j);

    j
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut arr: [i32; 0] = [];
        quick_sort(&mut arr);
    }

    #[test]
    fn already_sorted() {
        let mut arr = [1, 2, 3];
        quick_sort(&mut arr);
        assert_eq!(1, arr[0]);
        assert_eq!(2, arr[1]);
        assert_eq!(3, arr[2]);
    }

    #[test]
    fn two_elements() {
        let mut arr = [3, 2];
        quick_sort(&mut arr);
        assert_eq!(2, arr[0]);
        assert_eq!(3, arr[1]);
    }

    #[test]
    fn several_elements() {
        let mut arr = [3, 2, 5, 8, 1];
        quick_sort(&mut arr);
        assert_eq!(1, arr[0]);
        assert_eq!(2, arr[1]);
        assert_eq!(3, arr[2]);
        assert_eq!(5, arr[3]);
        assert_eq!(8, arr[4]);
    }

    #[test]
    fn with_equal_keys() {
        let mut arr = [3, 2, 1, 2, 1];
        quick_sort(&mut arr);
        assert_eq!(1, arr[0]);
        assert_eq!(1, arr[1]);
        assert_eq!(2, arr[2]);
        assert_eq!(2, arr[3]);
        assert_eq!(3, arr[4]);
    }
}
