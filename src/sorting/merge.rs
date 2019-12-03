use std::cmp::min;
use std::mem;

pub fn merge_sort<T>(arr: &mut [T])
where
    T: PartialOrd + Default + Clone,
{
    let n = arr.len();
    let mut aux_vec: Vec<T> = vec![T::default(); n];
    let aux = &mut aux_vec[..];

    // bottom-up
    let mut width = 1;
    while width < n {
        for i in (0..n).step_by(width * 2) {
            merge(arr, aux, i, min(i + width, n), min(i + width * 2, n));
        }
        arr.swap_with_slice(aux);

        width *= 2;
    }
}

fn merge<T>(arr: &mut [T], aux: &mut [T], left: usize, right: usize, end: usize)
where
    T: PartialOrd,
{
    let mut i = left;
    let mut j = right;
    for k in left..end {
        if j >= end {
            mem::swap(&mut aux[k], &mut arr[i]);
            i += 1;
        } else if i >= right {
            mem::swap(&mut aux[k], &mut arr[j]);
            j += 1;
        } else if arr[i] < arr[j] {
            mem::swap(&mut aux[k], &mut arr[i]);
            i += 1;
        } else {
            mem::swap(&mut aux[k], &mut arr[j]);
            j += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut arr: [i32; 0] = [];
        merge_sort(&mut arr);
    }

    #[test]
    fn already_sorted() {
        let mut arr = [1, 2, 3];
        merge_sort(&mut arr);
        assert_eq!(1, arr[0]);
        assert_eq!(2, arr[1]);
        assert_eq!(3, arr[2]);
    }

    #[test]
    fn two_elements() {
        let mut arr = [3, 2];
        merge_sort(&mut arr);
        assert_eq!(2, arr[0]);
        assert_eq!(3, arr[1]);
    }

    #[test]
    fn several_elements() {
        let mut arr = [3, 2, 5, 8, 1];
        merge_sort(&mut arr);
        assert_eq!(1, arr[0]);
        assert_eq!(2, arr[1]);
        assert_eq!(3, arr[2]);
        assert_eq!(5, arr[3]);
        assert_eq!(8, arr[4]);
    }
}
