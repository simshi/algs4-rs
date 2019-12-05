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

## Quick Sort
> Quicksort is a divide-and-conquer method for sorting. It works by partitioning an array into two parts, then sorting the parts independently.

[Wiki with animation](https://en.wikipedia.org/wiki/Quicksort)

### optimization
- select pivot by median-of-three
- tail recursion optimization
- 3-way comparison

## Heap Sort
> The Heapsort algorithm involves preparing the list by first turning it into a max heap. The algorithm then repeatedly swaps the first value of the list with the last value, decreasing the range of values considered in the heap operation by one, and sifting the new first value into its position in the heap. This repeats until the range of considered values is one value in length.

[Wiki with animation](https://en.wikipedia.org/wiki/Heapsort)

### optimization
- Floyd's heap construction: building subheaps backwards.
- Bounce heuristic: in second phase, exchange a[0] with a[end], while a[end] is one of the smallest elements, comparison with siblings is ineffient, can't get 50-50 probability, so use fast sink by comparing only two siblings.