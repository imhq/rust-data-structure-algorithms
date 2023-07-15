/*!
 * bubble_sort.rs
 *
 * Description: Bubble sort implementation in Rust
 *
 * Author: Int Main
 * Created: <Date created>
 *
 * Modified By: <Name of person modifying the file>
 * Last Modified: <Date of last modification>
 */

// Rest of the code goes here...

fn bubble_sort(arr: &mut [i32]) 
{
    let len = arr.len();
    let mut swapped;

    for i in 0..len 
    {
        swapped = false;

        for j in 0..len - i - 1 
        {
            if arr[j] > arr[j + 1] 
            {
                arr.swap(j, j + 1);
                swapped = true;
            }
        }

        if !swapped 
        {
            break;
        }
    }
}

//Driver function
fn main() 
{
    let mut arr = [4, 2, 9, 6, 23, 12, 34, 0, 1];

    println!("Original array: {:?}", arr);

    bubble_sort(&mut arr);
    
    println!("Sorted array is: {:?}", arr);
}
