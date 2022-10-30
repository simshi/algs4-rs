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
    // 假设只有英文数字
    // 避免循环中使用s.chars().nth(i).unwrap()，而且enumerate()也不方便
    let s = s.as_bytes();
    // let s = s.chars().collect::<Vec<_>>();

    // 状态：dp[i][j]表示[i, j]区间是否是回文, 为简化i-1，j+1溢出，上下左右各+1
    let mut dp = vec![vec![false; n + 2]; n + 2];
    let mut ans = (1, 0); // 让"abc"的回文至少返回"a"，所以初始化为1
    for i in 0..n {
        let (row, col) = (i + 1, i + 1);
        dp[row][col] = true;
    }
    // 状态转移方程： 必须先判断字串i..=j是否为回文，然后才能判断i-1..=j+1
    // 按距离开始扫描，即扫描window(d)，然后再是window(d+1)
    for d in 1..n {
        for i in 0..n - d {
            let (row, col) = (i + 1, i + d + 1);
            if s[i] == s[i + d] && (d == 1 || dp[row + 1][col - 1]) {
                dp[row][col] = true;
                if d + 1 > ans.0 {
                    ans = (d + 1, i);
                }
            }
        }
    }

    String::from_utf8(s[ans.1..(ans.1 + ans.0)].to_vec()).unwrap()
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
