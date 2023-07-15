/*!
 * quick_sort.rs
 *
 * Description: Quick sort implementation in Rust
 *
 * Author: Int Main
 * Created: <Date created>
 *
 * Modified By: <Name of person modifying the file>
 * Last Modified: <Date of last modification>
 */

// Rest of the code goes here...

fn quick_sort(arr: &mut [i32]) 
{
    let len = arr.len();
    if len <= 1 
    {
        return;
    }

    let pivot_index = partition(arr);

    quick_sort(&mut arr[0..pivot_index]);
    quick_sort(&mut arr[pivot_index + 1..len]);
}

fn partition(arr: &mut [i32]) -> usize 
{
    let len = arr.len();
    let pivot_index = len - 1;
    let mut i = 0;

    for j in 0..len 
    {
        if arr[j] < arr[pivot_index] 
        {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, pivot_index);
    
    return i;
}

//Driver function
fn main() 
{
    let mut arr = [64, 25, 12, 22, 11];

    println!("Original array: {:?}", arr);

    quick_sort(&mut arr);

    println!("Sorted array: {:?}", arr);
}
