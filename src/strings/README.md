# Strings

## Sorting
  - MSD
    - Since string is Unicoded in Rust, use `chars()` instead.
    - it copies result to an `aux` array and then copy back in the book. Here I use an index array `iv` to rearrange, it's linear.
	- it allocates a `count` array in every recursive sorting step in the book, I use only one by using an working stack to storing sorting range, which also avoids recursion.
