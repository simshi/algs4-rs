# Dynamic Planning

## Longest Palindrome
  - Recursive is straightforward, good for start;
  - State: `dp[i][j]` whether range `[i, j]` is a palindrome or not;
  - Formula: `dp[i][j] = dp[i+1][j-1] && s[i] == s[j]`;
  - O(N^2) obviously;
  - Manacher's algorithm is O(N) but not implemented yet (I'm stupid).

## Rain Water Trap

## Minimum Edit Distance
