# Selection Sort

Selection sort is a sorting algorithm which is not very fast (O(n^2)). Quicksort is faster which only takes O(n log n).

Selection sort works by repeatedly selecting the smallest (or largest) element from the unsorted portion of the list and moving it to the sorted portion of the list.

I implemented selection sort both in-place and out-of-place.  The in-place algorithm only needs O(1) extra space, whereas the out-of-place algorithm requires O(n) extra space since it needs extra space to put elements in as its sorting them.

## Mathematical Analysis

```
selection sort is an in-place comparison sorting algorithm. It has an O(n^2) time complexity, which makes it inefficient on large lists, and generally performs worse than the similar insertion sort. Selection sort is noted for its simplicity and has performance advantages over more complicated algorithms in certain situations, particularly where auxiliary memory is limited.
```