pub fn heap_sort<T>(arr: &mut [T])
where
    T: PartialOrd,
{
    let mut n = arr.len();
    if n <= 1 {
        return;
    }

    // construct heap, by building subheaps bottom-up
    // use zero-based heap index, k is root of subheap
    let mut k = (n - 1) / 2;
    loop {
        sink(arr, k, n);
        if k == 0 {
            break;
        }
        k -= 1;
    }

    while n > 0 {
        arr.swap(0, n - 1);
        sink(arr, 0, n - 1);
        n -= 1;
    }
}

fn sink<T: PartialOrd>(arr: &mut [T], mut i: usize, n: usize) {
    while 2 * i + 1 < n {
        let mut j = 2 * i + 1;
        if j + 1 < n && arr[j] < arr[j + 1] {
            j += 1;
        }
        if arr[i] >= arr[j] {
            break;
        }

        arr.swap(i, j);
        i = j;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sorting::is_sorted;

    #[test]
    fn empty() {
        let mut arr: [i32; 0] = [];
        heap_sort(&mut arr);
    }

    #[test]
    fn already_sorted() {
        let mut arr = [1, 2, 3];
        heap_sort(&mut arr);
        assert!(is_sorted(&arr));
    }

    #[test]
    fn two_elements() {
        let mut arr = [3, 2];
        heap_sort(&mut arr);
        assert!(is_sorted(&arr));
    }

    #[test]
    fn several_elements() {
        let mut arr = [3, 2, 5, 8, 1];
        heap_sort(&mut arr);
        assert!(is_sorted(&arr));
    }

    #[test]
    fn with_equal_keys() {
        let mut arr = [3, 2, 1, 2, 1];
        heap_sort(&mut arr);
        assert!(is_sorted(&arr));
    }
}
