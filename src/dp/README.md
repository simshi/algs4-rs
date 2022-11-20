# Dynamic Planning

## Rain Water Trap
  - Given N non-negative integers representing an elevation map where the width of each bar is 1, compute how much water it can trap after raining.
  - key：当前i柱能容纳多少水量，取决于其左右两边的最高柱子，两个最高柱中矮的那根

## Yanghui (Pascal)'s Triangle
  - `dp[i][j] = dp[i-1][j-]+dp[i-1][j]`
  - or `C(i, j) = C(i-1, j-1) + C(i-1, j)`

## Longest Palindrome
  - Recursive is straightforward, good for start;
  - State: `dp[i][j]` whether range `[i, j]` is a palindrome or not;
  - Formula: `dp[i][j] = dp[i+1][j-1] && s[i] == s[j]`;
  - O(N^2) obviously;
  - Manacher's algorithm is O(N) but not implemented yet (I'm stupid).

## Minimum Edit Distance
  - Given two strings word1 and word2, return the minimum number of operations required to convert word1 to word2.
  - key: if `src[i]==dst[i], dp[i][j]=dp[i-1][j-1]`, else `dp[i][j]=max(delete src[i], replace src[i] to dst[j], insert dst[j] to src[i])`

## Buy and Sell Stock for Max Profit
  - You are given an array prices where prices[i] is the price of a given stock on the ith day, find the max profit.
  - State & transition: `dp[i] = current profit`,
    - `dp[i][CASH] = max(keep cash, sold) = max(dp[i-1][CASH], dp[i-1][HOLD]+price[i])`,
    - `dp[i][HOLD] = max(keep hold, buy) = max(dp[i-1][HOLD], dp[i-1][HOLD]-price[i])`
  - With limitations
