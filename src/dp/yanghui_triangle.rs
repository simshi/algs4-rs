//! Yanghui(Pascal)'s Triangle
//!
//! `C(i, j) = C(i-1, j-1) + C(i-1, j)`

/// Yanghui(Pascal)'s Triangle
pub fn yanghui_triangle_generate(num_rows: i32) -> Vec<Vec<i32>> {
    let n = num_rows as usize;

    let mut ans = vec![Vec::<i32>::new(); n];
    ans[0] = vec![1];

    for i in 1..n {
        ans[i] = vec![1; i + 1];
        for j in 1..i {
            ans[i][j] = ans[i - 1][j - 1] + ans[i - 1][j];
        }
    }

    ans
}

/// ith Yanghui(Pascal)'s Triangle
///
/// <pre>
/// i: 0-indexed
/// C(n, m)   = n*(n-1)*(n-2)*...*(n-m+1)/m!
/// C(n, m-1) = n*(n-1)*(n-2)*...*(n-m+2)/(m-1)!
/// C(n, m)   = n*(n-1)*(n-2)*...*(n-m+2)/(m-1)! * (n-m+1) / m
/// 可知，递推吧
/// i |         |  i: C(n, m)
/// --+---------+-----------
/// 0 |    1    |  0: C(0, 0)
/// 1 |   1 1   |  0: C(1, 0), 1: C(1, 1)
/// 2 |  1 2 1  |  0: C(2, 0), 1: C(2, 1)
/// 3 | 1 3 3 1 |  0: C(3, 0), 1: C(3, 1), 2: C(3, 2)
/// 可知: n=row_index, m=i
/// </pre>
pub fn yanghui_triangle_ith_generate(row: i32) -> Vec<i32> {
    let row = row as usize;
    let n = row;

    let mut ans = vec![1i32; row + 1];
    for i in 1..row {
        let m = i;
        ans[i] = (ans[i - 1] as usize * (n - m + 1) / m) as i32;
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![vec![1]], yanghui_triangle_generate(1));
        assert_eq!(vec![vec![1], vec![1, 1]], yanghui_triangle_generate(2));
        assert_eq!(
            vec![vec![1], vec![1, 1], vec![1, 2, 1]],
            yanghui_triangle_generate(3)
        );
        assert_eq!(
            vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1]
            ],
            yanghui_triangle_generate(5)
        );
    }

    #[test]
    fn test_ith() {
        assert_eq!(vec![1], yanghui_triangle_ith_generate(0));
        assert_eq!(vec![1, 1], yanghui_triangle_ith_generate(1));
        assert_eq!(vec![1, 2, 1], yanghui_triangle_ith_generate(2));
        assert_eq!(vec![1, 3, 3, 1], yanghui_triangle_ith_generate(3));
        assert_eq!(vec![1, 4, 6, 4, 1], yanghui_triangle_ith_generate(4));
        assert_eq!(vec![1, 5, 10, 10, 5, 1], yanghui_triangle_ith_generate(5));
    }
}
