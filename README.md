# Rust Data Structure And Algorithm

This repository contains explanations and implementations of various data structures and algorithms in Rust. It serves as a comprehensive resource for developers looking to deepen their understanding of fundamental concepts and enhance their problem-solving skills.

## Contents

The repository is organized into directories, each focusing on a specific data structure or algorithm. Each directory includes:

- **Explanation:** A detailed explanation of the data structure or algorithm, including its concepts, properties, and use cases.
- **Implementation:** Rust source code implementing the data structure or algorithm, accompanied by inline comments for better understanding.

## Data Structures

The following data structures are included in this repository:

- [Stack](./stack/): Last-In-First-Out (LIFO) data structure.
- [Queue](./queue/): First-In-First-Out (FIFO) data structure.
- [Linked List](./linked-list/): Linear collection of elements.
- [Binary Search Tree](./binary-search-tree/): Binary tree structure with sorted properties.
- [Heap](./heap/): Tree-based data structure with specific ordering properties.
- [Graph](./graph/): Representation of connections between elements.

## Algorithms

The following algorithms are included in this repository:

- [Sorting Algorithms](./sorting/): Bubble Sort, Insertion Sort, Merge Sort, Quick Sort, etc.
- [Searching Algorithms](./searching/): Linear Search, Binary Search, etc.
- [Graph Algorithms](./graph/): Breadth-First Search, Depth-First Search, Dijkstra's Algorithm, etc.


## Data Structures and Algorithms Time Complexity Comparison

Understanding the time complexities of different algorithms can help in selecting the most efficient solution for a given problem.

### Data Structures

| Data Structure    | Access   | Search   | Insertion   | Deletion   |
|-------------------|----------|----------|-------------|------------|
| Array             | O(1)     | O(n)     | O(n)        | O(n)       |
| Linked List       | O(n)     | O(n)     | O(1)        | O(1)       |
| Stack             | O(n)     | O(n)     | O(1)        | O(1)       |
| Queue             | O(n)     | O(n)     | O(1)        | O(1)       |
| Binary Search Tree| O(log n) | O(log n) | O(log n)    | O(log n)   |
| Heap              | O(1)     | O(n)     | O(log n)    | O(log n)   |
| Hash Table        | -        | O(1)     | O(1)        | O(1)       |


### Algorithms

| Algorithm         | Best Case    | Average Case     | Worst Case     |
|-------------------|--------------|------------------|----------------|
| Bubble Sort       | O(n)         | O(n^2)           | O(n^2)         |
| Insertion Sort    | O(n)         | O(n^2)           | O(n^2)         |
| Selection Sort    | O(n^2)       | O(n^2)           | O(n^2)         |
| Merge Sort        | O(n log n)   | O(n log n)       | O(n log n)     |
| Quick Sort        | O(n log n)   | O(n log n)       | O(n^2)         |
| Binary Search     | O(1)         | O(log n)         | O(log n)       |
| Breadth-First Search  | O(V + E)  | O(V + E)         | O(V + E)       |
| Depth-First Search    | O(V + E)  | O(V + E)         | O(V + E)       |
| Dijkstra's Algorithm  | O((V + E) log V) | O((V + E) log V) | O((V + E) log V) |


## Contributing

Contributions to this repository are welcome! If you have additional explanations, implementations, or optimizations for the existing data structures and algorithms, feel free to open a pull request. Please follow the contribution guidelines outlined in the repository.

## License

This repository is licensed under the MIT License. You are free to use the code and explanations for personal or commercial purposes, subject to the terms and conditions of the license.

## Acknowledgements

We extend our gratitude to the authors and contributors of textbooks, online resources, and open-source projects that have helped in expanding our understanding of data structures and algorithms.

---

By providing explanations and implementations of various data structures and algorithms in Rust, this repository aims to support developers in mastering fundamental concepts and applying them effectively in software development.

Explore the explanations and implementations, and happy coding!
