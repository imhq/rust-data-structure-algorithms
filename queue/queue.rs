/*!
 * queue.rs
 *
 * Description: Queue implemenation in Rust
 *
 * Author: Int Main
 * Created: <Date created>
 *
 * Modified By: <Name of person modifying the file>
 * Last Modified: <Date of last modification>
 */

// Rest of the code goes here...

struct Queue {
    queue: Vec<i32>,
}

impl Queue {
    fn new() -> Queue {
        Queue { queue: Vec::new() }
    }

    fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    fn enqueue(&mut self, item: i32) {
        self.queue.push(item);
    }

    fn dequeue(&mut self) -> Option<i32> {
        if self.is_empty() {
            return None;
        }
        Some(self.queue.remove(0))
    }

    fn peek(&self) -> Option<i32> {
        self.queue.first().cloned()
    }

    fn size(&self) -> usize {
        self.queue.len()
    }
}

fn main() 
{
    let mut queue = Queue::new();

    queue.enqueue(10);
    queue.enqueue(20);
    queue.enqueue(30);
    
    println!("Size of queue: {}", queue.size());
    println!("Front of the queue is: {}", queue.peek().unwrap());
    
    queue.enqueue(40);
    queue.enqueue(50);
    queue.enqueue(60);

    println!("Size of queue: {}", queue.size());

    while let Some(item) = queue.dequeue() 
    {
        println!("Dequeued item: {}", item);
    }

    println!("Is queue empty: {}", queue.is_empty());
}
