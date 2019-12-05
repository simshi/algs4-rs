pub fn shell_sort<T>(arr: &mut [T])
where
    T: PartialOrd,
{
    let n = arr.len();
    let mut h = 1;
    while h < n / 3 {
        h = h * 3 + 1;
    }

    while h > 0 {
        for i in h..n {
            let mut j = i;
            while j >= h && arr[j] < arr[j - h] {
                arr.swap(j, j - h);
                j -= h;
            }
        }
        h /= 3;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut arr: [i32; 0] = [];
        shell_sort(&mut arr);
    }

    #[test]
    fn already_sorted() {
        let mut arr = [1, 2, 3];
        shell_sort(&mut arr);
        assert_eq!(1, arr[0]);
        assert_eq!(2, arr[1]);
        assert_eq!(3, arr[2]);
    }

    #[test]
    fn two_elements() {
        let mut arr = [3, 2];
        shell_sort(&mut arr);
        assert_eq!(2, arr[0]);
        assert_eq!(3, arr[1]);
    }

    #[test]
    fn several_elements() {
        let mut arr = [3, 2, 5, 8, 1];
        shell_sort(&mut arr);
        assert_eq!(1, arr[0]);
        assert_eq!(2, arr[1]);
        assert_eq!(3, arr[2]);
        assert_eq!(5, arr[3]);
        assert_eq!(8, arr[4]);
    }
}
