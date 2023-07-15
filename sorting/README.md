# Sorting Algorithms

This repository contains implementations of various sorting algorithms in different programming languages. The following sorting algorithms are included:

- Bubble Sort
- Insertion Sort
- Heap Sort
- Merge Sort
- Quick Sort

## Bubble Sort

Bubble sort is a simple comparison-based sorting algorithm. It repeatedly steps through the list, compares adjacent elements, and swaps them if they are in the wrong order. This process is repeated until the list is sorted.

**Steps:**
1. Start from the first element and compare it with the next element.
2. If the next element is smaller, swap them.
3. Move to the next pair of adjacent elements and repeat steps 1 and 2.
4. Continue this process until the end of the list.
5. Repeat steps 1-4 for the remaining unsorted portion of the list.
6. The list is now sorted.

## Insertion Sort

Insertion sort is another comparison-based sorting algorithm. It builds the final sorted array one item at a time, comparing each item with those in the sorted portion of the array and shifting elements to the right until the correct position is found.

**Steps:**
1. Start from the second element and compare it with all previous elements.
2. If the current element is smaller, shift the larger elements to the right.
3. Repeat step 2 until the correct position for the current element is found.
4. Move to the next element and repeat steps 1-3.
5. The array is now sorted.

## Heap Sort

Heap sort is a comparison-based sorting algorithm that uses a binary heap data structure. It first builds a max-heap from the input array and then repeatedly extracts the maximum element from the heap, resulting in a sorted array.

**Steps:**
1. Build a max-heap from the input array.
2. Swap the root (maximum element) with the last element in the heap.
3. Reduce the heap size by 1.
4. Heapify the root element to maintain the heap property.
5. Repeat steps 2-4 until the heap is empty.
6. The sorted array is obtained by extracting elements from the heap.

## Merge Sort

Merge sort is a divide-and-conquer sorting algorithm. It divides the input array into two halves, recursively sorts them, and then merges the two sorted halves to produce a sorted array.

**Steps:**
1. Divide the unsorted array into two halves.
2. Recursively sort each half.
3. Merge the sorted halves by comparing elements and building a sorted array.
4. Repeat steps 1-3 until the array is fully sorted.

## Quick Sort

Quick sort is another divide-and-conquer sorting algorithm. It selects a pivot element from the array and partitions the other elements into two sub-arrays, according to whether they are less than or greater than the pivot. It then recursively sorts the sub-arrays.

**Steps:**
1. Choose a pivot element from the array.
2. Partition the array into two sub-arrays, one with elements less than the pivot and the other with elements greater than the pivot.
3. Recursively apply steps 1-2 to the sub-arrays.
4. Concatenate the sorted sub-arrays with the pivot element in the correct position.

## Time Complexity
---

| Algorithm     | Best Time Complexity | Average Time Complexity | Worst Time Complexity |
|---------------|----------------------|-------------------------|-----------------------|
| Bubble Sort   | O(n)                 | O(n^2)                  | O(n^2)                |
| Insertion Sort| O(n)                 | O(n^2)                  | O(n^2)                |
| Heap Sort     | O(n log n)           | O(n log n)              | O(n log n)            |
| Merge Sort    | O(n log n)           | O(n log n)              | O(n log n)            |
| Quick Sort    | O(n log n)           | O(n log n)              | O(n^2)                |

Each sorting algorithm has its own strengths and weaknesses. The choice of algorithm depends on various factors, such as the size of the input, the expected data distribution, and the available memory. Understanding the time complexity of each algorithm can help in selecting the most appropriate sorting algorithm for a given scenario.

For detailed implementation examples of each sorting algorithm, please refer to the corresponding code files within current directory.
