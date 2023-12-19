#  Exponential Search

Exponential search is a powerful search algorithm that can be usued to find values in large datasets quickly and effieciently.  It works on sorted, unbounded lists. Exponential search is also known as galloping search or Doubling search.
Exponential search will find an item within a list of sorted values.
It works by repeatedly multiplying the search interval by 2 and checking the middle elemnt of the interval.  If the middle element is equal to the target value then the search is successful.  Otherwise, the search continues on half of the interval value.