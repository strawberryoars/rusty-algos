# Quicksort

As the name implies, quicksort is a sorting algorithm.  Quicksort uses Divide & Conquer (D&C) which is a well known recursive technique for solving problems.

```
Quicksort is a divide-and-conquer algorithm. It works by selecting a 'pivot' element from the array and partitioning the other elements into two sub-arrays, according to whether they are less than or greater than the pivot. For this reason, it is sometimes called partition-exchange sort.
Quicksort is a comparison sort, meaning that it can sort items of any type for which a "less-than" relation (formally, a total order) is defined.
```

## D&C

D&C is not a simple algorithm you can apply to a problem, instead its a way to think about a problem.
Heres how D&C works:
1. Figure out a simple case as the base case
2. Figure out how to reduce your problem and get to the base case

```
You should think of a divide-and-conquer algorithm as having three parts:
Divide the problem into a number of subproblems that are smaller instances of the same problem.
Conquer the subproblems by solving them recursively. If they are small enough, solve the subproblems as base cases.
Combine the solutions to the subproblems into the solution for the original problem.
```

In quicksort the base case is an array of size less than 2. Meaning an empty array or an array with one element is sorted.
Now we need to reduce our problem and get to this base case.

1. Pick a pivot
2. Partition the array into two sub-arrays: elements less than the pivot and element greater than the pivot
3. Call quicksort recursively on the two sub-arrays.
```
[33,10,15,7]
[10,15,7] <33> []
[7] <10> [15]
```

### Mathematical Analysis

Quicksort is unique because the speed depends on the pivot you choose.
In the worst case quicksort takes O(n^2)
In the average case quicksort takes O(n log n)

#### Worse Case
In the worst case, you will have an unbalanced partition where one of the parititions or subarray will contain no elements and the oother partition will contain n-1 elements.
You can see this occur if you attempted to sort an already sorted array and always picked the 1st element as the pivot. For example:
```
[1,2,3,4,5,6,7,8]
[] <1> [2,3,4,5,6,7,8]
[] <2> [3,4,5,6,7,8]
[] <3> [4,5,6,7,8]
[] <4> [5,6,7,8]
[] <5> [6,7,8]
[] <6> [7,8]
[] <7> [8]
```
The problem size will only be reduced by 1 (the size of the pivot), because the recursive calls to quick sort are made on the elements on either side of the partition (which will have n-1 elements on 1 side and 0 on the other).
Since partitioning is O(n) and the problem size reduces by 1 each partition it will be: O( n + (n-1) + (n-2) + ... + 3 + 2 + 1) = O(n^2)
put another way the stack size is O(n) so the algorithm will take O(n) * O(n) = O(n^2) time.

#### Best Case
The best case occurs when the parititions are evenly balanced as possible: their sizes are either equal or are within 1 of each other.
If you picked the middle element for the pivot everytime, you won't need to make as many recursive calls. For example:
```
[1,2,3,4,5,6,7,8]
[1,2,3] <4> [5,6,7,8]
[1] <2> [3]    [5] <6> [7,8]
                    [] <7> [8]
```

This illustrates a call stack size of 4 compared to 8 in the worst case example above.  This means best case has O(log n) levels.
In this best case example, each level takes O(n).  This yields a time complexity of O(n) * O(log n) = O(n log n)


#### Average case
The best case is the same as the average case.