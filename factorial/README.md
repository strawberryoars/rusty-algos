# Factorial

```
In mathematics, the factorial of a non-negative integer n, denoted by n!, is the product of all positive integers less than or equal to n.The value of 0! is 1, according to the convention for an empty product.
```

Example:
3! = 3 * 2! = 3 * (2 * 1!) = 3 * (2 * 1 * 1) = 6

| Code      | Call Stack | Notes |
| ----------- | ----------- |----------- |
| factorial(3)  | <table>  <thead>  <tr>  <th colspan="2"> factorial</th> </tr>  </thead>  <tbody>  <tr> <td> x </td> <td>3</td> </tr>   </tbody>  </table>      | first call to factorial x is 3 |
|  if x == 1 \|\| x == 0 {  |   <table>  <thead>  <tr>  <th colspan="2"> factorial</th> </tr>  </thead>  <tbody>  <tr> <td> x </td> <td>3</td> </tr>   </tbody>  </table>    | |
|return x * factorial(x-1);|<div><table>  <thead>  <tr>  <th colspan="2"> factorial</th> </tr>  </thead>  <tbody>  <tr> <td> x </td> <td>2</td> </tr>   </tbody>  </table><table>  <thead>  <tr>  <th colspan="2"> factorial</th> </tr>  </thead> <tbody>  <tr> <td> x </td> <td>3</td> </tr>   </tbody>  </table></div> ||
|  if x == 1 \|\| x == 0 {  |   <div><table>  <thead>  <tr>  <th colspan="2"> factorial</th> </tr>  </thead>  <tbody>  <tr> <td> x </td> <td>2</td> </tr>   </tbody>  </table><table>  <thead>  <tr>  <th colspan="2"> factorial</th> </tr>  </thead> <tbody>  <tr> <td> x </td> <td>3</td> </tr>   </tbody>  </table></div>    | The topmost function call is the call we are currently in|
| return x * factorial(x-1); |   <div><table>  <thead>  <tr>  <th colspan="2"> factorial</th> </tr>  </thead> <tbody>  <tr> <td> x </td> <td>1</td> </tr>   </tbody>  </table><table>  <thead>  <tr>  <th colspan="2"> factorial</th> </tr>  </thead>  <tbody>  <tr> <td> x </td> <td>2</td> </tr>   </tbody>  </table><table>  <thead>  <tr>  <th colspan="2"> factorial</th> </tr>  </thead> <tbody>  <tr> <td> x </td> <td>3</td> </tr>   </tbody>  </table></div>    | both function calls have a variable named x and the value of x is different in both |
| if x == 1 \|\| x == 0 { |   <div><table>  <thead>  <tr>  <th colspan="2"> factorial</th> </tr>  </thead> <tbody>  <tr> <td> x </td> <td>1</td> </tr>   </tbody>  </table><table>  <thead>  <tr>  <th colspan="2"> factorial</th> </tr>  </thead>  <tbody>  <tr> <td> x </td> <td>2</td> </tr>   </tbody>  </table><table>  <thead>  <tr>  <th colspan="2"> factorial</th> </tr>  </thead> <tbody>  <tr> <td> x </td> <td>3</td> </tr>   </tbody>  </table></div>    |  |
| return 1; | <div><table>  <thead>  <tr>  <th colspan="2"> factorial</th> </tr>  </thead> <tbody>  <tr> <td> x </td> <td>1</td> </tr>   </tbody>  </table><table>  <thead>  <tr>  <th colspan="2"> factorial</th> </tr>  </thead>  <tbody>  <tr> <td> x </td> <td>2</td> </tr>   </tbody>  </table><table>  <thead>  <tr>  <th colspan="2"> factorial</th> </tr>  </thead> <tbody>  <tr> <td> x </td> <td>3</td> </tr>   </tbody>  </table></div>    |This is the first box to get popped of the stack, which means its the first call we return from. returns 1|
| return x * factorial(x-1); | <div><table>  <thead>  <tr>  <th colspan="2"> factorial</th> </tr>  </thead>  <tbody>  <tr> <td> x </td> <td>2</td> </tr>   </tbody>  </table><table>  <thead>  <tr>  <th colspan="2"> factorial</th> </tr>  </thead> <tbody>  <tr> <td> x </td> <td>3</td> </tr>   </tbody>  </table></div>    | returns 2|
| return x * factorial(x-1); | <div><table>  <thead>  <tr>  <th colspan="2"> factorial</th> </tr>  </thead> <tbody>  <tr> <td> x </td> <td>3</td> </tr>   </tbody>  </table></div>    | returns 6|

Notice that each call to factorial has its own copy of x. You can't access a different functions copy of x. 