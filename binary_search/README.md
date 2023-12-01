# Binary Search
Binary search takes a sorted array and an item to search for.  If the item is in the array is in the array, the function returns its position.

```
 Binary search compares the target value to the middle element of the array. If they are not equal, the half in which the target cannot lie is eliminated and the search continues on the remaining half, again taking the middle element to compare to the target value, and repeating this until the target value is found. If the search ends with the remaining half being empty, the target is not in the array.
```

O(log n) run time