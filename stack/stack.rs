/*!
 * <Filename>.rs
 *
 * Description: Array implemenation in Rust
 *
 * Author: Int Main
 * Created: <Date created>
 *
 * Modified By: <Name of person modifying the file>
 * Last Modified: <Date of last modification>
 */

// Rest of the code goes here...

struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { data: Vec::new() }
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn push(&mut self, item: T) {
        self.data.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.data.last()
    }
}

//Driver function

fn main() {
    let mut stack: Stack<i32> = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    println!("Top of the stack is: {}", stack.peek().unwrap());
    
    stack.push(4);
    stack.push(5);
    stack.push(6);
    
    while stack.is_empty() != true 
    {
        let item = stack.pop();
        println!("Popped item: {}", item.unwrap());
    }
}
