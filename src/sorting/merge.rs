use std::cmp::min;
use std::iter::repeat;

pub fn merge_sort<T>(arr: &mut [T])
where
    T: PartialOrd + Default + Copy,
{
    let n = arr.len();
    let mut aux_vec: Vec<T> = repeat(T::default()).take(n).collect();
    let aux = &mut aux_vec[..];

    let mut width = 1;
    while width < n {
        for i in (0..n).step_by(width * 2) {
            bottom_up_merge(arr, aux, i, min(i + width, n), min(i + width * 2, n));
        }
        arr.copy_from_slice(aux);

        width *= 2;
    }
}

fn bottom_up_merge<T>(arr: &mut [T], aux: &mut [T], left: usize, right: usize, end: usize)
where
    T: PartialOrd + Clone,
{
    let mut i = left;
    let mut j = right;
    for k in left..end {
        if j >= end {
            aux[k] = arr[i].clone();
            i += 1;
        } else if i >= right {
            aux[k] = arr[j].clone();
            j += 1;
        } else if arr[i] < arr[j] {
            aux[k] = arr[i].clone();
            i += 1;
        } else {
            aux[k] = arr[j].clone();
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
