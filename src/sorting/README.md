# Sorting


## Selection Sort
> One of the simplest sorting algorithms works as follows: First, find the smallest item in the array, and exchange it with the first entry. Then, find the next smallest item and exchange it with the second entry. Continue in this way until the entire array is sorted. This method is called selection sort because it works by repeatedly selecting the smallest remaining item

[Wiki with animation](https://en.wikipedia.org/wiki/Selection_sort)

## Insertion Sort
> The algorithm that people often use to sort bridge hands is to consider the cards one at a time, inserting each into its proper place among those already considered (keeping them sorted). In a computer implementation, we need to make space for the current item by moving larger items one position to the right, before inserting the current item into the vacated position.

[Wiki with animation](https://en.wikipedia.org/wiki/Insertion_sort)

## Shell Sort
> Shellsort is a simple extension of insertion sort that gains speed by allowing exchanges of entries that are far apart, to produce partially sorted arrays that can be efficiently sorted, eventually by insertion sort. The idea is to rearrange the array to give it the property that taking every hth entry (starting anywhere) yields a sorted sequence. Such an array is said to be h-sorted.

[Wiki with animation](https://en.wikipedia.org/wiki/Shellsort)

- hint:
    1. regroup as columns
    2. line by line, sort each columns within every line
    3. decreace gap, loop until gap==0
- example:
    ```
    3 1 7 6
    9 2 8 4
    5 0
    ```

## Merge Sort
> combining two ordered arrays to make one larger ordered array. This operation immediately lends itself to a simple recursive sort method known as mergesort: to sort an array, divide it into two halves, sort the two halves (recursively), and then merge the results.

[Wiki with animation](https://en.wikipedia.org/wiki/Mergesort)

### optimization
- allocate aux array once
- interleaved use original array with aux array
- switch to insertion sort while n is small

## Quick Sort
> Quicksort is a divide-and-conquer method for sorting. It works by partitioning an array into two parts, then sorting the parts independently.

[Wiki with animation](https://en.wikipedia.org/wiki/Quicksort)

### optimization
- select pivot by median-of-three
- Hoare partition schema
- tail recursion optimization (unneccessary if complier evolves)
- 3-way comparison (also perform better with many identical keys)
- switch to insertion sort while n is small

## Heap Sort
> The Heapsort algorithm involves preparing the list by first turning it into a max heap. The algorithm then repeatedly swaps the first value of the list with the last value, decreasing the range of values considered in the heap operation by one, and sifting the new first value into its position in the heap. This repeats until the range of considered values is one value in length.

[Wiki with animation](https://en.wikipedia.org/wiki/Heapsort)

### optimization
- Floyd's heap construction: building subheaps backwards.
- Bounce heuristic: in second phase, exchange a[0] with a[end], while a[end] is one of the smallest elements, comparison with siblings is ineffient, can't get 50-50 probability, so use fast sink by comparing only two siblings.
- **Bounce performs worse on identical keys

# Benchmark

## Methods
1. Insertion: naive implementation
2. Shell: step(i+1) = step(i)/3
3. Merge: naive
4. Merge optimized:
    - insertion sort 15 elements as a group, then merge bottom up
    - interleave original array with aux array
5. Quick: naive
6. Quick 3way:
    - 3-way comparison
    - switch to insertion sort while <=15 elements
    - select pivot by median of three
7. Quick optimized:
    - Hoare partition schema
    - switch to insertion sort while <=15 elements
    - select pivot by median of three
8. Heap: naive
9. Heap optimized: Bounce heuristic

drastical fast for sorted lists, fast for small

## Results

| **Type** | **Count** | **Schema** | Base | Insertion | Shell | Merge | Merge Opt | Quick | Quick 3way |Quick Optimized| Heap | Heap Optimized |
|---|---:|---|---:|----:|---:|---:|---:|---:|---:|---:|---:|---:|
| u32 | 7 | random | 18 | 33 | |161 |65 | 41 | 37 | 37 | 45 | 48 |
| u32 | 15 | random | 18 | 74 |72| 346|135 | 75 | 80 | 77 | 105 | 111 |
| u32 | 100 | random | 25 | 4907|790| 4039|1292 | 658 | 879 |626 | 1181 | 1081 |
| u32 | 100 | identical | 0 | 128 |534| 3781|860 | 631 | 129 | 387 | 346 | 1094 |
| u32 | 100 | sorted | 0 | 128 |635| 3763|883| 3380 | 1340 | 219 | 1361 | 1215 |
| u32 | 1000 | reversed | 90 | 630984 |13266|182921|26542 | 270252 | 31359 | 5085 | 44467 | 45845 |
| u32 | 10000 | random | 95 | 32247954 |840717|20426974|714673 | 551410 | 624298 | 546464 | 777537 | 799576 |

> `Base` is time consumed by random-generating

## My Analysis
1. Insertion/Shell: as expected, fastest for small arrays. Shell gets better on larger arrays.
2. Merge:
    - similar optimization with quick sort, but ~2x slower than quick-optimized, due to doubled swap/copy between 2 arrays, as expected.
    - worse than shell sort if no optimization, too many moves
3. Quick sort: the **WINNER!!!**
    - switching to insertion while partitions become small is a good move
    - **Hoare partition schema** performs good, competitive with 3-way partition on identical arrays, and **surprisingly good on already-sorted and reversed arrays**. It keeps middle pivot at middle, avoid all swaps if partitioning is already done, means:
      1. for sorted arrays, e.g. [1,2,3,4,5] it will keep all elements as-is, i.e. no swap/copy required!
      2. for reversed arrays, e.g. [5,4,3,2,1] would be changed to [1,2,3,4,5], then goes like above situation
      3. for identical arrays, no swap/copy are required, so, NlogN comparisions only!
    - meanwhile, 3-way performs great on identical arrays, but no luck on others, though it's still outperforms other sorting algorithms.
    - comparing 3-way with optimized version, it uses first element as pivot, for already-sorted & reversed arrays, it breaks the original order.
    - basically cache-friendly
4. Heap sort:
    - bounce heuristic helps a little bit only, worse on most-common schemas (**might require more investigations**)
    - worse than quick and merge, moving elements around(not in a cache-friendly way) too many times

## TODO
- Compare with C++ version