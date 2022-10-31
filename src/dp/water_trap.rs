/// Rain water trap
///
/// Given N non-negative integers representing an elevation map where the width
/// of each bar is 1, compute how much water it can trap after raining.
///
/// Example:
/// ```
///   ^
///   |
/// 4 |
///   |               |
/// 2 |       | # # # | | # |
///   |   | # | | # | | | | | |
///   + - - - - - - - - - - - - - - ->
/// 0     1   3   5   7   9
/// Input: height = [0,1,0,2,1,0,1,3,2,1,2,1]
/// Output: 6
/// ```
///
/// 算法实现：
/// ```
///                   |
///     |     #       |
///     |     #       |
///     |     |       |
///   - - - - - - - - - - - - - - - ->
///     ^     ^       ^
///     |     |       |
/// left_max  i   right_max
/// ```
///
/// 对于每个柱子i，和两侧最高的柱子相比，不管其他柱子如何:
///   - 如果left_max < right_max，`left_max - height[i]`部分能被柱子i容纳
///   - 如果left_max >= right_max，`right_max - height[i]`部分能被柱子i容纳
///
/// 1. 对于下标i，下雨后水能到达的最大高度等于下标i两边的最大高度的最小值，下标i处能接的雨水量
/// 等于下标i处的水能到达的最大高度减去height[i]
/// 2. 朴素的做法是对于数组 height 中的每个元素，分别向左和向右扫描并记录左边和右边的最大高度，
/// 然后计算每个下标位置能接的雨水量。假设数组 height 的长度为 n，该做法需要对每个下标位置
/// 使用 O(n) 的时间向两边扫描并得到最大高度，因此总时间复杂度是 O(n2)。
/// 3. 上述做法的时间复杂度较高是因为需要对每个下标位置都向两边扫描。如果已经知道每个位置两边的
/// 最大高度，则可以在 O(n) 的时间内得到能接的雨水总量。使用动态规划的方法，可以在 O(n)
/// 的时间内预处理得到每个位置两边的最大高度。 创建两个长度为 n 的数组 leftMax 和
/// rightMax。对于 0≤i<n，leftMax[i] 表示下标 i 及其左边的位置中，height 的最大高度，
/// rightMax[i]表示下标 i 及其右边的位置中，height 的最大高度。
pub fn water_trap(heights: Vec<i32>) -> i32 {
    if heights.len() <= 2 {
        return 0;
    }

    // left_max和right_max可以从数组压缩到两个变量
    let (mut left_max, mut right_max) = (0, 0);
    let (mut left, mut right) = (0, heights.len() - 1);
    let mut ans = 0;
    while left < right {
        if heights[left] > left_max {
            left_max = heights[left];
        }
        if heights[right] > right_max {
            right_max = heights[right];
        }
        if left_max < right_max {
            ans += left_max - heights[left];
            left += 1;
        } else {
            ans += right_max - heights[right];
            right -= 1;
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(0, water_trap(vec![]));
        assert_eq!(0, water_trap(vec![0]));
        assert_eq!(0, water_trap(vec![0, 0, 0]));
        assert_eq!(0, water_trap(vec![0, 1, 0]));
        assert_eq!(0, water_trap(vec![1, 2, 1]));
    }

    #[test]
    fn test_basic() {
        assert_eq!(1, water_trap(vec![1, 0, 2]));
        assert_eq!(6, water_trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]));
        assert_eq!(9, water_trap(vec![4, 2, 0, 3, 2, 5]));
    }
}
