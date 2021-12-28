All the solutions should be unit tested. The problem is fairly simple,
so no need to go crazy with tests. Solutions don't need to be
particularly optimized. Documentation is not necessary.

## Exercise 1

Implement a function that takes a vector of values as argument,
removes all the duplicate values, and returns the corresponding
vector. Note that the order in which the values appear for the first
time must be preserved.

| input | output |
| ----- | ------ |
| `[]` | `[]` |
| `[0, 1, 2]` | `[0, 1, 2]` |
| `[0, 1, 1, 2]` | `[0, 1, 2]` |
| `[0, 1, 2, 1]` | `[0, 1, 2]` |

## Exercise 2

Same as exercise 1, but this time the function should take an iterator
as argument and return an iterator

## Exercise 3

Same as exercise 2, but this time the function must be generic over
the values type. Restricting said type with traits bounds may be
necessary.
