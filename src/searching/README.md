# Searching

## TODO
  - [ ] impl `Drop` to avoid stack overflow

## Binary Search(ordered array) Symbol Table
- no big deal, `Vector` has `insert` and `remove` shift elements for us
- separated `keys[]` and `values[]` may lead to better cache-friendly, due to most operations using keys only

## Binary Search Tree
- idiomatic list definition: `type List<K, V> = Option<Box<Node<K, V>>>`
- be careful with lifetime, but looks quite straightforward after figure out.
- `delete` is the most complicated piece, requires rotating two elements in sub-tree, find by `min` and `delete_min` can't do the trick(due to ownership issue), so I wrote a `pop_min` which combine them together.
- bonus fun: implement `Iter` and `IntoIter`, `Iter` is implemented by iterating (inorder tree traversal) with a stack and a current pointer, not by a preallocated queue, which is more fun~.

## Red Black Tree
- It's left leaning 2-3 tree in the book, but I implement as 2-3-4 tree, more complex, more fun!
