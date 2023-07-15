/*!
 * insertion_sort.rs
 *
 * Description: Insertion sort implementation in Rust
 *
 * Author: Int Main
 * Created: <Date created>
 *
 * Modified By: <Name of person modifying the file>
 * Last Modified: <Date of last modification>
 */

// Rest of the code goes here...

fn insertion_sort(arr: &mut [i32]) 
{
    let len = arr.len();

    for i in 1..len 
    {
        let key = arr[i];
        let mut j = i as i32 - 1;

        while j >= 0 && arr[j as usize] > key 
        {
            arr[(j + 1) as usize] = arr[j as usize];
            j -= 1;
        }

        arr[(j + 1) as usize] = key;
    }
}

//Driver function
fn main() 
{
    let mut arr = [64, 25, 12, 22, 11];

    println!("Original array: {:?}", arr);

    insertion_sort(&mut arr);

    println!("Sorted array: {:?}", arr);
}
