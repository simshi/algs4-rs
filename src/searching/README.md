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
- It's left leaning 2-3 tree in the book, but I implement as 2-3-4 tree, more general, more complex, and more fun!
- handle insertion and deletion bottom-up, one layer by one layer, same as binary search tree, thus we avoid use a `parent` pointer in the node.
- Key point: re-balance after insertion/deletion, fix either by the node itself, or by borrowing from its sibling, till escalate to up layer.
- Insertion, only thing to do is to fix consective red node by rotation:
  - flip colors with two children are both red, i.e. split 4-tree to 2-trees from top-down in search stage;
  - add N as red;
  - check at grand-parent layer, eliminate consective red P+N.
- Deletion: delete red node is fine, but delete one black node cause imbalance.
- Deletion re-balance: X is the sub-tree which lost one black-height, P as parent, G as grand-parent, S as sibling, SL as sibling's left child, SR as sibling's right child:
  - 0. can be fixed by self and/or its child:
    - 0.1 deleted node is red: no further change, it's still in balance
    - 0.2 deleted node is black, but its right child is red: change its color to black, black-height keeps
  - 1. deleted node is black, and its children are black, X is left child of P, as below figure:

  ```text
  Legend:
    - X is the sub-tree which lost one black-height
    - P is the parent of X, S is the sibling of X
    - SL is the left child of S, SR is the right child of S
    - R(P) means P is red, (P) means P can be red or black

   | S is Red  |
   +-----------+
   |    P      | rotate       S
   |  /   \    |  left      /   \
   | X    R(S) |  ===>    R(P)   SR                            escalate to up,
   |      / \  |          /  \                                 P is the new X
   |    SL  SR |         X    SL                                      ^
                           |                                          |
                           v                                          | if P was Black
       (go one level down to fix R(P) sub-tree)                       |
   | SL is red  |        |  SR is Red   |  |SL/SR black |             |
   +------------+        +--------------+  +------------+
   |   (P)      | rotate |   (P)        |  |   (P)      |              P
   |  /   \     | right  |  /   \       |  |  /   \     | P: Black   /   \
   | X     S    |   S    | X     S      |  | X     S    | S: Red    X    R(S)
   |     /   \  |  ===>  |     /   \    |  |     /   \  |   ===>        /   \
   |  R(SL)  SR |        |   (SL) R(SR) |  |    SL   SR |              SL   SR
   |            |        |              |  |            |
                               |                                      |
                               | rotate left P                        |
                               | P: Black                             | if P was Red
                               | SR: Black                            |
                               v                                      |
                                (S)                                   v
                               /   \
                              P    SR         ------------------>   Done
                             / \
                            X (SL)
  ```

    - 1.1 S is red: left rotate P, then goto 1.2 with P sub-tree (SL is the new S, which is black).
    - 1.2 S is black, X is left child of P:
      - 1.2.1 SL is red, SR is black: right rotate S, then S (the new SR) is red, goto 1.2.2
      - 1.2.2 SR is red: left rotate P, set P and SR to black, then path X->P added one black-height, while (SL)->S->(P) is same with (SL)->P->(S), and R(SR)->S->(P) is same with SR->(S), so it's done here
      - 1.2.3 SL/SR are both black: set S to red
        - 1.2.3.1 P is red: set P to black, borrow success, done!
        - 1.2.3.2 P is black: X and S sub-tree balanced, but whole P tree lost one black-height, escalate to up layer, P is the new X
  - 2. X is right child of P: a mirror problem.
