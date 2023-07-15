/*!
 * heap_sort.rs
 *
 * Description: Heap sort implementation in Rust
 *
 * Author: Int Main
 * Created: <Date created>
 *
 * Modified By: <Name of person modifying the file>
 * Last Modified: <Date of last modification>
 */

// Rest of the code goes here...

fn heapify(arr: &mut [i32], n: usize, i: usize) 
{
    let mut largest = i;
    let left = 2 * i + 1;
    let right = 2 * i + 2;

    if left < n && arr[left] > arr[largest] 
    {
        largest = left;
    }

    if right < n && arr[right] > arr[largest] 
    {
        largest = right;
    }

    if largest != i 
    {
        arr.swap(i, largest);
        heapify(arr, n, largest);
    }
}

fn heap_sort(arr: &mut [i32]) 
{
    let n = arr.len();

    // Build max heap
    for i in (0..n / 2).rev() 
    {
        heapify(arr, n, i);
    }

    // Extract elements from the heap in descending order
    for i in (1..n).rev() 
    {
        arr.swap(0, i);
        heapify(arr, i, 0);
    }
}

//Driver function
fn main() 
{
    let mut arr = [64, 25, 12, 22, 11];

    println!("Original array: {:?}", arr);

    heap_sort(&mut arr);

    println!("Sorted array: {:?}", arr);
}
