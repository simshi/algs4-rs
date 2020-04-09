# Strings

## Sorting
  - MSD
    - Since string is Unicoded in Rust, use `chars()` instead.
    - it copies result to an `aux` array and then copy back in the book. Here I use an index array `iv` to rearrange, it's linear.
	- it allocates a `count` array in every recursive sorting step in the book, I use only one by using an working stack to storing sorting range, which also avoids recursion.

## Trie Tree
  - similar with BST, pass ownship of `NodePtr` to function and return back it.
  - delete current node (by returning `None` to upper layer) when current node has no value stored and `next` is empty: `node.next.iter().all(|p| p.is_none())`

## Ternary Searching Trie
  - deletion node has complexity.
  - one todo: when deleting a node with no middle pointer, better to collapse it with the `left` or `right`.

## KMP Substring Searching
  1. e.g. find **pattern** "abcabd" in **text** "xyzabcabcabd":
  ```
       012345678901
  i=8: xyzabcabcabd // begin with 3
  j=5:    abcabd
               ^
  ```
  while 'c' mismatch with 'd', reset comparison from `i=4` and `j=0` if we use brutal force solution:
  ```
       012345678901
  i=4: xyzabcabcabd // begin with 3
  j=0:     abcabd
           ^
  ```
  but since we have partial match, i.e. `pattern[0..j]` is "abcab", which means before mismatch, there is "ab" (as `text[i-2..i]`) already matched with `pattern[0..2]`, we can skip this "ab" and continue to compare at `text[i..]` with `pattern[2..]` i.e. keep `i` no change but reset `j` to `2` only:
  ```
       012345678901
  i=8: xyzabcabcabd // begin with 6
  j=2:       abcabd
               ^
  ```
  2. So the problem is left with building a state-machine like table `next` on event **mismatch**, e.g. "abab"'s `next` array is `[-1, 0, 0, 1]`, while mismatch occurs at `[3]`, we need to reset `j=1`, `-1` means `pattern[0]` would not match `text[i]` either, so advance `i`:
  ```
       0123456789
  i=6: xyzabaabab
  j=3:    abab
             ^
  j=1:      abab // compare text[6] with pattern[1]
             ^
  ```
  the building algorithm is matching prefix with suffix:
  ```
       0123456789
        v
  j=1: abab
  i=3: abab
          ^
  ```
  on above example, `pattern[1] == pattern[3]`, so `j` cover the matched prefix, while `i` tracks the matched suffix, we cant set `next[3] = 1`, here `1` means while `pattern[3]` mismatch target text, how many chars (suffix) matched can be treated as prefix, so we can skip them in further comparisons.

  3. Further more, while mismatch on `pattern[3]`, we reset `j=next[3]`, i.e. `1`, which means:
  ```
       0123456789
  i=6: xyzabaabab
  j=1:      abab // compare text[6] with pattern[1]
             ^
  ```
  but since `pattern[1]==pattern[3]=='b'`, if mismatch `pattern[3]`, it would also mismatch with `pattern[1]`, because `pattern[j]==pattern[next[j]]`. To optimized, we can set `next[i]=next[j]` to advance one more step, when build the `next` table, we should consider this scenario as:
  ```rust
	next[i] = if pattern[i] == pattern[j] {
		next[j] // equals to previous matched prefix length
	} else {
		j // as normal
	};
  ```
  so the final `next` is `[-1, 0, -1, 0]`. Be noticed `next[2]` is `-1`, which means if `pattern[2]` mismatched, the `text[i]` isn't 'a', so it wouldn't match `pattern[0]=='a'`, then we should skip `text[i]`.
  ```
       0123456789
  i=2: abcab
  j=2: abab // 'c' mismatch 'a' at [2], set j=-1
         ^
  // align j=-1 to i=2, as below:
  i=2: abcab
  j=-1:   abab // continue to loop
         ^
  // i.e. align j=0 to i=3, as below:
  i=3: abcab
  j=0:    abab // compare text[3] with pattern[0]
          ^

  ```

## Boyer-Moore Substring Searching
