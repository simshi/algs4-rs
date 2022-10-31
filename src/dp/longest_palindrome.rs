use std::collections::HashMap;

fn recursive(
    s: &str,
    l: usize,
    r: usize,
    memo: &mut HashMap<(usize, usize), (usize, usize)>,
) -> (usize, usize) {
    if l > r {
        return (l, 0);
    }
    if l == r {
        return (l, 1);
    }
    if let Some(r) = memo.get(&(l, r)) {
        return *r;
    }

    let mut rs = [(0, 0); 3];
    if s.chars().nth(l).unwrap() == s.chars().nth(r).unwrap() {
        let (i, n1) = recursive(s, l + 1, r - 1, memo);
        if i == l + 1 && 2 + n1 == r - l + 1 {
            return (l, 1 + n1 + 1);
        } else {
            rs[0] = (i, n1);
        }
    };
    rs[1] = recursive(s, l, r - 1, memo);
    rs[2] = recursive(s, l + 1, r, memo);

    let ans = IntoIterator::into_iter(rs).max_by_key(|r| r.1).unwrap();
    memo.insert((l, r), ans);

    ans
}

pub fn longest_palindrome(s: &str) -> String {
    if s.len() < 2 {
        return s.to_owned();
    }

    let mut memo = HashMap::new();
    let (i, n) = recursive(s, 0, s.len() - 1, &mut memo);
    String::from(&s[i..(i + n)])
}

pub fn longest_palindrome_dp(s: &str) -> String {
    let n = s.len();
    if n < 2 {
        return s.to_owned();
    }

    // 避免循环中使用s.chars().nth(i).unwrap()，而且enumerate()也不方便
    // let s = s.chars().collect::<Vec<_>>();
    let s = s.as_bytes(); // 假设只有英文数字

    // 状态：dp[i][j]表示[i, j]区间是否是回文, 为避免i-1，j+1溢出判断，上下左右各+1
    let mut dp = vec![vec![false; n + 2]; n + 2];
    // ans: (index, length)，让"abc"的回文至少返回"a"，所以length初始化为1
    let mut ans = (0, 1);
    // 先处理单个字符，置为true，避免下面状态转移方程循环内判断
    for i in 0..n {
        let (row, col) = (i + 1, i + 1);
        dp[row][col] = true;
        if i + 1 < n && s[i] == s[i + 1] {
            dp[i + 1][i + 2] = true;
            ans = (i, 2);
        }
    }
    // 状态转移方程： 先判断字串i..=j是否为回文，然后扩大范围i-1..=j+1
    // 按距离开始扫描，即扫描window(d)，然后再是window(d+1)
    for d in 2..n {
        for i in 0..n - d {
            let (row, col) = (i + 1, i + d + 1);
            if s[i] == s[i + d] && dp[row + 1][col - 1] {
                dp[row][col] = true;
                if d + 1 > ans.1 {
                    ans = (i, d + 1);
                }
            }
        }
    }

    String::from_utf8(s[ans.0..(ans.0 + ans.1)].to_vec()).unwrap()
    // s.into_iter().skip(ans.1).take(ans.0).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!("", longest_palindrome(""));
        assert_eq!("a", longest_palindrome("a"));
        assert_eq!("aa", longest_palindrome("aa"));
        assert_eq!(
            true,
            ["a".to_owned(), "b".to_owned()].contains(&longest_palindrome("ab"))
        );
    }

    #[test]
    fn test_basic_dp() {
        assert_eq!("", longest_palindrome_dp(""));
        assert_eq!("a", longest_palindrome_dp("a"));
        assert_eq!("aa", longest_palindrome_dp("aa"));
        assert_eq!(
            true,
            ["a".to_owned(), "b".to_owned()].contains(&longest_palindrome_dp("ab"))
        );
    }

    #[test]
    fn test_contains() {
        assert_eq!("aba", longest_palindrome("aba"));
        assert_eq!("bb", longest_palindrome("cbbd"));
        assert_eq!("12321", longest_palindrome("12321"));
        assert_eq!("cbdbc", longest_palindrome("cbdbc"));
        assert_eq!(
            true,
            ["bab".to_owned(), "aba".to_owned()].contains(&longest_palindrome("babad"))
        );
    }

    #[test]
    fn test_contains_dp() {
        assert_eq!("aba", longest_palindrome_dp("aba"));
        assert_eq!("bb", longest_palindrome_dp("cbbd"));
        assert_eq!("12321", longest_palindrome_dp("12321"));
        assert_eq!("cbdbc", longest_palindrome_dp("cbdbc"));
        assert_eq!(
            true,
            ["bab".to_owned(), "aba".to_owned()].contains(&longest_palindrome_dp("babad"))
        );
    }

    #[test]
    fn test_broken_palindrome_bothside() {
        assert_eq!("12321", longest_palindrome("ax12321a"));
        assert_eq!("aca", longest_palindrome("aacabdkacaa"));
        assert_eq!(
            true,
            ["cbababc".to_owned(), "bbcccbb".to_owned()]
                .contains(&longest_palindrome("abbcccbbbcaaccbababcbcabca"))
        );
    }

    #[test]
    fn test_broken_palindrome_bothside_dp() {
        assert_eq!("12321", longest_palindrome_dp("ax12321a"));
        assert_eq!("aca", longest_palindrome_dp("aacabdkacaa"));
        assert_eq!(
            true,
            ["cbababc".to_owned(), "bbcccbb".to_owned()]
                .contains(&longest_palindrome_dp("abbcccbbbcaaccbababcbcabca"))
        );
    }

    #[test]
    fn test_super_long() {
        assert_eq!("qgjjgq",
    longest_palindrome_dp("rgczcpratwyqxaszbuwwcadruayhasynuxnakpmsyhxzlnxmdtsqqlmwnbxvmgvllafrpmlfuqpbhjddmhmbcgmlyeypkfpreddyencsdmgxysctpubvgeedhurvizgqxclhpfrvxggrowaynrtuwvvvwnqlowdihtrdzjffrgoeqivnprdnpvfjuhycpfydjcpfcnkpyujljiesmuxhtizzvwhvpqylvcirwqsmpptyhcqybstsfgjadicwzycswwmpluvzqdvnhkcofptqrzgjqtbvbdxylrylinspncrkxclykccbwridpqckstxdjawvziucrswpsfmisqiozworibeycuarcidbljslwbalcemgymnsxfziattdylrulwrybzztoxhevsdnvvljfzzrgcmagshucoalfiuapgzpqgjjgqsmcvtdsvehewrvtkeqwgmatqdpwlayjcxcavjmgpdyklrjcqvxjqbjucfubgmgpkfdxznkhcejscymuildfnuxwmuklntnyycdcscioimenaeohgpbcpogyifcsatfxeslstkjclauqmywacizyapxlgtcchlxkvygzeucwalhvhbwkvbceqajstxzzppcxoanhyfkgwaelsfdeeviqogjpresnoacegfeejyychabkhszcokdxpaqrprwfdahjqkfptwpeykgumyemgkccynxuvbdpjlrbgqtcqulxodurugofuwzudnhgxdrbbxtrvdnlodyhsifvyspejenpdckevzqrexplpcqtwtxlimfrsjumiygqeemhihcxyngsemcolrnlyhqlbqbcestadoxtrdvcgucntjnfavylip"));
    }
}
