## Merge Sort README

### Introduction

Merge Sort is a popular sorting algorithm that uses the Divide and Conquer approach to sort a list of elements. It is a efficient and stable sorting algorithm with a time complexity of O(n log n).

### How Merge Sort Works

Merge Sort works by dividing the input list into smaller sublists, sorting each sublist, and then merging the sorted sublists back together to form the final sorted list.

### Step-by-Step Breakdown

1. **Divide**: Divide the input list into two halves, each containing roughly half the number of elements as the original list.
2. **Conquer**: Recursively apply the Merge Sort algorithm to each half of the list, until each sublist contains only one element.
3. **Merge**: Merge the sorted sublists back together, comparing elements from each sublist and placing them in the correct order in the final sorted list.

### Time Complexity: O(n log n)

The time complexity of Merge Sort is O(n log n), where n is the number of elements in the input list.

#### Where does n come from?

The `n` in O(n log n) comes from the fact that we need to process each element in the input list exactly once. In the worst case, we need to compare each element with every other element, which takes O(n) time.

#### Where does log n come from?

The `log n` in O(n log n) comes from the recursive nature of the Merge Sort algorithm. Each time we divide the list in half, we reduce the size of the problem by a factor of 2. This means that we need to recursively apply the algorithm log(n) times, where log is the logarithm to the base 2.

### Code Implementation

Yesterday, we implemented the Merge Sort algorithm in code. The implementation involved creating a recursive function that divides the input list into smaller sublists, sorts each sublist, and then merges the sorted sublists back together. The final sorted list is then returned.

If you'd like to review the code, I can resend it to you. Just let me know!

### Conclusion

Merge Sort is a efficient and stable sorting algorithm that uses the Divide and Conquer approach to sort a list of elements. Its time complexity of O(n log n) makes it suitable for large datasets, and its recursive nature makes it easy to implement.
