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
- handle insertion and deletion bottom-up, one layer by one layer, same as binary search tree. Add a `parent` pointer could be more complex.
- Key point: re-balance after insert/delete, solve by the node itself, or by borrowing from its sibling, otherwise escalate to up layer.
- Insertion, only thing to do is to fix consective red node by rotation:
  - add N as red;
  - check at grand-parent level, eliminate consective red P+N.
- Deletion: delete red node is fine, but delete one black node cause imbalance, solve it by borrow from sub-tree, or sibling, till escalate to up layer.
- Deletion re-balance: X is the sub-tree which lost one black-height, P as parent, G as grand-parent, S as sibling, SL as sibling's left child, SR as sibling's right child:
  - 0. deleted node is red: no further change, it's still in balance
  - 1. deleted node is black, X is left child of P:
    - 1.1 its right child is red: change its color to black, black-height keeps
    - 1.2 S is black, X is left child of P:
      - 1.2.1 SR is red: borrow from right sub-tree while still keep black-height of right sub-tree no change (by change a red node to black)
      - 1.2.2 SR is black, SL is red, right rotate S, goto 1.2.1
      - 1.2.3 SL/SR are both black: set S to red
        - 1.2.3.1 P is red: set P to black, borrow success, balanced!
        - 1.2.3.2 P is black: X and S sub-tree balanced, but whole P tree lost one black-height, escalate to up, P is new X
    - 1.3 S is red, X is left child of P: left rotate P, goto 1.2 with P (SL is new S, which is black).
  - 2. X is right child of P: a mirror problem.
