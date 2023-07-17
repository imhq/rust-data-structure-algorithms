# Stack

A stack is a fundamental data structure that follows the Last-In-First-Out (LIFO) principle. It represents a collection of elements in which an element can be inserted or removed only from the top of the stack. The element that is inserted last is the first one to be removed.

Stacks are commonly used to solve problems that require a temporary or sequential ordering of data. They can be implemented using arrays or linked lists, where the top of the stack represents the last element added.

## Operations

A stack typically supports the following operations:

- **Push:** Inserts an element onto the top of the stack.
- **Pop:** Removes and returns the element from the top of the stack.
- **Peek/Top:** Returns the element at the top of the stack without removing it.
- **isEmpty:** Checks if the stack is empty.
- **Size:** Returns the number of elements in the stack.

## Time Complexities

The time complexities of stack operations depend on the underlying implementation:

| Operation    | Time Complexity |
|--------------|-----------------|
| Push         | O(1)            |
| Pop          | O(1)            |
| Peek/Top     | O(1)            |
| isEmpty      | O(1)            |
| Size         | O(1)            |

Stack operations have constant time complexity (O(1)) because they involve accessing and modifying only the topmost element of the stack. These operations do not depend on the size of the stack.

## Use Cases

Stacks have various practical applications, including:

- Expression evaluation and syntax parsing
- Function call management in programming languages
- Backtracking and undo operations
- Browser history management
- Depth-first search in graph algorithms

Understanding the properties and time complexities of the stack data structure can help in designing efficient algorithms and solving problems that require the LIFO ordering of elements.

For detailed implementation of stack in rust, please refer to the stack.rs file in the current directory.