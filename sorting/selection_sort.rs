/*!
 * selection_sort.rs
 *
 * Description: Selection sort implementation in Rust
 *
 * Author: Int Main
 * Created: <Date created>
 *
 * Modified By: <Name of person modifying the file>
 * Last Modified: <Date of last modification>
 */

// Rest of the code goes here...

fn selection_sort(arr: &mut [i32]) 
{
    let len = arr.len();
    
    for i in 0..len 
    {
        let mut min_index = i;
        
        for j in (i + 1)..len 
        {
            if arr[j] < arr[min_index] 
            {
                min_index = j;
            }
        }
        
        if min_index != i 
        {
            arr.swap(i, min_index);
        }
    }
}

//Driver function
fn main() 
{
    let mut arr = [64, 25, 12, 22, 11];
    
    println!("Original array: {:?}", arr);
    
    selection_sort(&mut arr);
    
    println!("Sorted array: {:?}", arr);
}
