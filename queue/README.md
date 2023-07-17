# Queue

A queue is a fundamental data structure that follows the First-In-First-Out (FIFO) principle. It represents a collection of elements in which an element can be inserted at the rear and removed from the front. The element that is inserted first is the first one to be removed.

Queues are commonly used in scenarios where elements are processed in the order they arrive. They can be implemented using arrays, linked lists, or other underlying data structures.

## Operations

A queue typically supports the following operations:

- **Enqueue:** Inserts an element at the rear of the queue.
- **Dequeue:** Removes and returns the element from the front of the queue.
- **Peek/Front:** Returns the element at the front of the queue without removing it.
- **isEmpty:** Checks if the queue is empty.
- **Size:** Returns the number of elements in the queue.

## Time Complexities

The time complexities of queue operations depend on the underlying implementation:

| Operation   | Time Complexity |
|-------------|-----------------|
| Enqueue     | O(1)            |
| Dequeue     | O(1)            |
| Peek/Front  | O(1)            |
| isEmpty     | O(1)            |
| Size        | O(1)            |

Queue operations typically have constant time complexity (O(1)). Enqueue, dequeue, peek, isEmpty, and size operations all involve accessing and modifying the front and rear of the queue, which can be done in constant time.

## Use Cases

Queues have various practical applications, including:

- Process scheduling in operating systems
- Task management in concurrent programming
- Breadth-first search in graph algorithms
- Printer job scheduling
- Event-driven programming

Understanding the properties and time complexities of the queue data structure can help in designing efficient algorithms and solving problems that require the FIFO ordering of elements.

For detailed implementation of queues in rust languages, please refer to the queue.rs file within this directory.