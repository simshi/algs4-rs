use std::cmp::min;
use std::mem;

pub fn merge_sort_opt<T>(arr: &mut [T])
where
    T: PartialOrd + Default + Clone,
{
    let n = arr.len();
    let mut aux_vec: Vec<T> = vec![T::default(); n];
    let aux = &mut aux_vec[..];

    // bottom-up
    let mut width = 15;
    for i in (0..n).step_by(width) {
        super::insertion_sort(&mut arr[i..min(i + width, n)]);
    }

    let mut flipped = false;
    while width < n {
        for i in (0..n).step_by(width * 2) {
            if flipped {
                merge(aux, arr, i, min(i + width, n), min(i + width * 2, n));
            } else {
                merge(arr, aux, i, min(i + width, n), min(i + width * 2, n));
            }
        }
        flipped = !flipped;

        width *= 2;
    }
    if flipped {
        arr.swap_with_slice(aux);
    }
}

fn merge<T>(arr: &mut [T], aux: &mut [T], left: usize, right: usize, end: usize)
where
    T: PartialOrd,
{
    let mut i = left;
    let mut j = right;
    for item in aux.iter_mut().take(end).skip(left) {
        if j >= end {
            mem::swap(item, &mut arr[i]);
            i += 1;
        } else if i >= right {
            mem::swap(item, &mut arr[j]);
            j += 1;
        } else if arr[i] < arr[j] {
            mem::swap(item, &mut arr[i]);
            i += 1;
        } else {
            mem::swap(item, &mut arr[j]);
            j += 1;
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
        merge_sort_opt(&mut arr);
    }

    #[test]
    fn already_sorted() {
        let mut arr = [1, 2, 3];
        merge_sort_opt(&mut arr);
        assert_eq!(1, arr[0]);
        assert_eq!(2, arr[1]);
        assert_eq!(3, arr[2]);
    }

    #[test]
    fn two_elements() {
        let mut arr = [3, 2];
        merge_sort_opt(&mut arr);
        assert_eq!(2, arr[0]);
        assert_eq!(3, arr[1]);
    }

    #[test]
    fn several_elements() {
        let mut arr = [3, 2, 5, 8, 1];
        merge_sort_opt(&mut arr);
        assert_eq!(1, arr[0]);
        assert_eq!(2, arr[1]);
        assert_eq!(3, arr[2]);
        assert_eq!(5, arr[3]);
        assert_eq!(8, arr[4]);
    }

    #[test]
    fn random_100() {
        let mut arr = thread_rng().gen_iter::<u32>().take(100).collect::<Vec<_>>();
        merge_sort_opt(&mut arr);
        assert!(is_sorted(&arr));
    }
}
