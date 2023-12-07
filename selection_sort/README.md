# Selection Sort

Selection sort is a sorting algorithm which is not very fast (O(n^2)). Quicksort is faster which only takes O(n log n).

Selection sort works by repeatedly selecting the smallest (or largest) element from the unsorted portion of the list and moving it to the sorted portion of the list.

I implemented selection sort both in-place and out-of-place.  The in-place algorithm only needs O(1) extra space, whereas the out-of-place algorithm requires O(n) extra space since it needs extra space to put elements in as its sorting them.

In-place visualized:
```
[9, 7, 5, 3, 1]
[1, 7, 5, 3, 9] # after pass 0 - swap index 0 with the smallest element found to the right
[1, 3, 5, 7, 9] # after pass 1 - swap index 1 with the smallest element found to the right
[1, 3, 5, 7, 9] # after pass 2 - nothing smaller found. swaps index 2 with itself.
[1, 3, 5, 7, 9] # after pass 3 - nothing smaller found. swaps index 3 with itself.
[1, 3, 5, 7, 9] # after pass 4 - nothing smaller found. swaps index 4 with itself.
```

## Mathematical Analysis

```
selection sort is an in-place comparison sorting algorithm. It has an O(n^2) time complexity, which makes it inefficient on large lists, and generally performs worse than the similar insertion sort. Selection sort is noted for its simplicity and has performance advantages over more complicated algorithms in certain situations, particularly where auxiliary memory is limited.
```