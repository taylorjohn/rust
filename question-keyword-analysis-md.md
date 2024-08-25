# Analyzing Question Keywords for Problem-Solving Strategies

Understanding the keywords and phrases in coding interview questions can often provide valuable hints about the expected solution or algorithm. Here's a guide to common terms and the strategies they might suggest:

## Common Keywords and Their Implications

1. **"Greatest" or "Largest"**
   - Often implies sorting or using a max-heap
   - Example: "Find the k largest elements in an array"
   - Possible approach: Use a min-heap of size k

2. **"Least" or "Smallest"**
   - Similar to "greatest", but might use sorting or a min-heap
   - Example: "Find the k smallest elements in an array"
   - Possible approach: Use a max-heap of size k

3. **"Common" (e.g., "Greatest Common Divisor")**
   - Often involves mathematical algorithms
   - Example: "Find the GCD of two numbers"
   - Possible approach: Use Euclidean algorithm

4. **"Reverse"**
   - Usually involves array or string manipulation
   - Example: "Reverse a linked list"
   - Possible approach: Two-pointer technique or stack

5. **"Product of"**
   - May involve dynamic programming or prefix/suffix products
   - Example: "Product of Array Except Self"
   - Possible approach: Calculate prefix and suffix products

6. **"Increasing" or "Decreasing"**
   - Often relates to subsequence or subarray problems
   - Example: "Longest Increasing Subsequence"
   - Possible approach: Dynamic Programming or Binary Search

7. **"String"**
   - Can involve various string algorithms or data structures
   - Example: "Longest Palindromic Substring"
   - Possible approaches: Two-pointer, Dynamic Programming, or Manacher's Algorithm

8. **"Move"**
   - Often involves array manipulation or pointer movement
   - Example: "Move Zeroes to the End"
   - Possible approach: Two-pointer technique

9. **"Subsequence" or "Subarray"**
   - Frequently involves dynamic programming or sliding window
   - Example: "Longest Common Subsequence"
   - Possible approach: 2D Dynamic Programming

10. **"Most" or "Majority"**
    - May involve counting or hash tables
    - Example: "Majority Element"
    - Possible approach: Boyer-Moore Voting Algorithm

11. **"Max number" or "Maximum"**
    - Could involve dynamic programming, greedy algorithms, or data structures like heaps
    - Example: "Maximum Subarray"
    - Possible approach: Kadane's Algorithm

12. **"Given length" or "Fixed size"**
    - Often hints at sliding window technique
    - Example: "Longest Substring with At Most K Distinct Characters"
    - Possible approach: Sliding Window with HashMap

13. **"Longest" or "Shortest"**
    - Frequently involves dynamic programming or two-pointer technique
    - Example: "Longest Palindromic Substring"
    - Possible approach: Expand Around Center or Dynamic Programming

14. **"Find" or "Search"**
    - May involve various search algorithms
    - Example: "Find Minimum in Rotated Sorted Array"
    - Possible approach: Modified Binary Search

15. **"Determine" or "Check"**
    - Often requires a boolean return and might involve traversal or checking conditions
    - Example: "Determine if a Binary Tree is Balanced"
    - Possible approach: Recursive depth calculation

16. **"Decode" or "Encode"**
    - Usually involves string manipulation or bit operations
    - Example: "Decode String"
    - Possible approach: Stack or Recursion

17. **"Remove" or "Delete"**
    - Involves modifying data structures, often arrays or linked lists
    - Example: "Remove Nth Node From End of List"
    - Possible approach: Two-pointer technique

18. **"Odd" or "Even"**
    - May involve bit manipulation or simple math
    - Example: "Single Number" (find the number that appears once in an array where every other number appears twice)
    - Possible approach: XOR operation

19. **"Binary Tree" or "Binary Search Tree"**
    - Involves tree traversal algorithms or recursion
    - Example: "Invert Binary Tree"
    - Possible approach: Recursive tree traversal

20. **"Sum"**
    - Could involve prefix sums, hash tables, or two-pointer technique
    - Example: "Two Sum"
    - Possible approach: Hash Table

## Example Analysis

Let's analyze a few questions to demonstrate this approach:

1. **Question**: "Find the longest palindromic substring in a given string"
   - Keywords: "longest", "palindromic", "substring"
   - Implication: This likely involves dynamic programming or a two-pointer expanding technique
   - Possible Approach: Expand around center for each character

2. **Question**: "Given an array of integers, find two numbers such that they add up to a specific target number"
   - Keywords: "find", "add up to", "target"
   - Implication: This suggests using a hash table for efficient lookup
   - Possible Approach: Single pass with a hash map

3. **Question**: "Merge k sorted linked lists"
   - Keywords: "merge", "sorted", "k"
   - Implication: This involves both merging and handling multiple (k) items efficiently
   - Possible Approach: Use a min-heap to efficiently select the smallest element among k lists

Remember, while these keywords can provide hints, they're not definitive. Always consider the full context of the problem and be prepared to combine multiple approaches for more complex questions.
