# Dynamic Planning

## Longest Palindrome
  - Recursive is straightforward, good for start;
  - State: `dp[i][j]` whether range `[i, j]` is a palindrome or not;
  - Formula: `dp[i][j] = dp[i+1][j-1] && s[i] == s[j]`;
  - O(N^2) obviously;
  - Manacher's algorithm is O(N) but not implemented yet (I'm stupid).

## Rain Water Trap
  - Given N non-negative integers representing an elevation map where the width of each bar is 1, compute how much water it can trap after raining.

## Minimum Edit Distance
  - Given two strings word1 and word2, return the minimum number of operations required to convert word1 to word2.
