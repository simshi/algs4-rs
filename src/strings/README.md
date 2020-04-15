# Strings

## TODO
  - replace `.chars()` with `.as_bytes()` to make some algorithms work on non-ASCII characters.
  - should return `Result` in some interfaces.

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
  - after deletion, if a node has no value and no middle child, it should collapse with its left or right sub-tree, but I collapse if there was only one of them, i.e. if both left and right child exist, keep it no change. Ideally, to avoid all unneccessary layer, we must implement it like a Red Black Tree, but it's too complicated. Or maybe we can flattern when inserting, to make sure left child has no right child (and right child has no left child).

## KMP Substring Searching
  1. e.g. find **pattern** "abcabd" in **text** "xyzabcabcabd":
  ```
       012345678901
  i=8: xyzabcabcabd // begin with 3
  j=5:    abcabd
               ^
  ```
  while 'c' mismatch with 'd', reset comparison from `i=4` and `j=0` if we use a brutal forced solution:
  ```
       012345678901
  i=4: xyzabcabcabd // begin with 4
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
  2. So the problem is left with building a state-machine like table `next` on event **mismatch**, e.g. "abcabd"'s `next` array must be `[-1, 0, 0, 0, 1, 2]`, while mismatch occurs at `[5]`, we need to reset `j=2`, if mismatch at `[2]`, reset `j=0`:
  ```
       01234567890123
  i=8: xyzabcababcabd
  j=5:    abcabd  // mismatch at pattern[5], set j=next[5] (i.e. 2)
               ^
  j=2:       abcabd // compare text[8] with pattern[2], mismatch, set j=next[2], (i.e. 0)
               ^
  j=0:         abcabd // compare text[8] with pattern[0]
               ^
  ```
  essentially, the building algorithm is counting prefix matched with suffix, because it equals to how many chars we've already matched (when mismatch at next char), thus we can skip in further comparisons:
  ```
       0123456789
         v
  j=2: abcabd // prefix "ab..."
  i=5: abcabd // suffix "...ab"
            ^
  ```
  on above example, `pattern[0..2] == pattern[3..5] == "ab"`, so `j==2` cover the matched prefix length ("ab".len()==2), so we know `next[5]=2`, which means while `pattern[3]` mismatch target text, the text must be "..abcab?" ('?' is not a 'd'), so next we can try to compare "ab?..." with the pattern, i.e. comapre begins with `i=2`.

  3. Further more, while mismatch on `pattern[4]`, if we reset `j=next[4]`, i.e. `1`, which means:
  ```
  let next = [-1, 0, 0, 0, 1, 2];
       01234567890123
  i=8: abcaabcabd
  j=4: abcabd  // mismatch at pattern[4], 'b', set j=next[4] (i.e. 1)
           ^
  j=1:    abcabd // compare text[4] with pattern[1], also 'b', set j=next[1] (i.e 0)
           ^
  j=0:     abcabd // compare text[4] with pattern[0]
           ^
  ```
  but since `pattern[4]==pattern[1]=='b'`, it would not match with `pattern[1]` either. To optimize, we can set `next[i]=next[j]` to advance one more step when build the `next` table, express this scenario as:
  ```rust
	next[i] = if pattern[i] == pattern[j] {
		next[j] // equals to previous matched prefix length
	} else {
		j // as normal
	};
  ```
  so the final `next` is `[-1, 0, 0, 0, 0, 2]`, skip comparing the text with `pattern[1]`.
  ```
  let next = [-1, 0, 0, 0, 0, 2];
       01234567890123
  i=8: xyzabcababcabd
  j=5:    abcabd  // mismatch at pattern[4], 'b', set j=next[4] (now it's 0)
              ^
  j=0:        abcabd // compare text[8] with pattern[0]
              ^
  ```

  4. Some words on the essence of `-1`: Indicating to align the `pattern[-1]` with curretn `text[i]`. Take "abab" as an example, while `pattern[2]` mismatched, `pattern[2]==pattern[0]=='a'`, which means set `j=0` wouldn't match either, so we should skip the whole pattern:
  ```
  let next = [-1, 0, -1, 0];
       0123456789
  i=2: abcab
  j=2: abab // 'c' mismatch 'a' at [2], set j=-1
         ^
  i=2: abcab
  j=-1:   abab // align j=-1 to i=2, skip the whole pattern
         ^
  // j+=1, i+=1 i.e. j=0, i=3, as below:
  i=3: abcab
  j=0:    abab // compare text[3] with pattern[0]
          ^
  ```
  then we can simplify the logic as:
  ```c++
  for (i=0, j=0; i<n && j<m; ++i, ++j) {
    while (j>=0 && text[i] != pattern[j]) {
      j = next[j];
    }
    // now j=-1 or text[i]==pattern[j], either way, next step must be
    // ++i and ++j moving to the next position to compare (or end loop)
    // ...
  }
  ```

## Boyer-Moore Substring Searching
  1. in short, it compares backwards so it can skip more chars on mismatch. e.g. find "example" in "here is a simple example":
  ```
       012345678901234567890123
  i=0: here is a simple example // comparing pattern[j] with text[i+j]
  j=6: example // compare backwards, not match, and 's' in not in pattern
             ^ // skip = j - (-1), i.e. 7

  i=7: here is a simple example // i=0+7
  j=6:        example // not match, and 'p' == pattern[4]
                    ^ // skip = j - 4, i.e. 2

  i=9: here is a simple example // i=7+2
  j=6:          example // align 'p', compare backwards from j=6
                      ^

  i=9: here is a simple example
  j=2:          example // not match at j=2, and 'i' is not in pattern
                  ^     // skip = j - (-1), i.e. 3

  i=12:here is a simple example // i=9+3
  j=6:             example // not match, 'x' == pattern[1]
                         ^ // skip = j - 1, i.e. 5

  i=17:here is a simple example // i=12+5
  j=6:                  example // comparing till find!
                              ^
  ```
  2. `skip` is based on the mismatched char occurs at the **right most position** in the pattern, so we build the right most table as:
  ``` rust
  let mut rmt:Vec<isize> = vec![-1; 256];
  for (i, c) in pattern.chars().enumerate() {
      rmt[c as usize] = i as isize;
  }
  ```
  3. **Good-char Rule**: above example is only applied bad-char rule, but we can see while 'i' mismatched with 'a', there are last 4 chars matched between "simple" and "example", the suffix list is `["mple", "ple", "le", "e"]`, and "e" is a prefix of the pattern, so we can take advantage of it:
  ```
       012345678901234567890123
  i=9: here is a simple example
  j=2:          example // not match at j=2, "e" is a prefix, good-char!
                  ^     // skip = m-0, i.e. 6

  i=15:here is a simple example // i=9+6, align 'e' with prefix 'e'
  j=6:                example // mismatch, 'p' = pattern[4]
                            ^ // skip = j - 4, i.e. 2

  i=17:here is a simple example // i=15+2
  j=6:                  example // comparing pattern[j] with text[i+j]
                              ^
  ```

## Sunday Substring Searching
  1. it's a variant of Boyer-Moore algorithm, but it compare forewards, and checks the next char when mismatch, thus skip faster:
  ```
  let right[R]:Vec<isize> = ... // last position of a char occurs in the pattern
  let m = pattern.len(); // 7
       012345678901234567890123
  i=0: here is a simple example
  j=0: example // mismatch on [0], check text[0+7], right[' ']==-1
             ^ // skip = m-(-1), i.e. 7+1

  i=8: here is a simple example // i=0+8
  j=0:         example // mismatch, check text[8+7], right['e']==6
               ^ // skip = m-6, i.e. 7-6

  i=9: here is a simple example // i=8+1
  j=0:          example // mismatch, check text[9+7], rigth[' ']==-1
                ^ // skip = m-(-1), i.e. 7+1

  i=17:here is a simple example // i=9+8
  j=0:                  example // will match
                        ^
  ```
  2. should be a little bit faster and simpler than BM algorithm.

## Huffman Coding Compression
  - theory: decoding without ambiguity by **Prefix Rule** (only leaf nodes have coding points).
  - coding: use `enum Kind` to express node clearly.
  - coding: (TODO) `prefix:Vec<u8>` expressing 1 bit per element, can be compressed.

## LZW
  - `char` is 32-bit wide, so we must use `u8` instead to compress and decompress.
  - `TST::longest_key_of` makes code simple, comparing with extending the key one byte by one byte and then search in a dictionary.
  - specical case `symbol == st.len()` in decompression, consider a pattern `x.*x.*x` (two `.*` substrings are same), after compress `x.*`, `x.*x` would be added to the dictionary, then the following `x.*x` would use this symbol immediately. But while decompressing met the symbol, `x.*x` is not in the dictionary yet, that's where the special case comes from, or in short, decompressing is one step slower than compressing.
