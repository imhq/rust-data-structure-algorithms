/*!
 * merge_sort.rs
 *
 * Description: Merge sort implementation in Rust
 *
 * Author: Int Main
 * Created: <Date created>
 *
 * Modified By: <Name of person modifying the file>
 * Last Modified: <Date of last modification>
 */

// Rest of the code goes here...

fn merge_sort(arr: &mut [i32], left: usize, right: usize) 
{
    if left >= right 
    {
        return;
    }

    let mid = left + (right - left) / 2;

    merge_sort(arr, left, mid);
    merge_sort(arr, mid + 1, right);

    merge(arr, left, mid, right);
}

fn merge(arr: &mut [i32], left: usize, mid: usize, right: usize) 
{
    let n1 = mid - left + 1;
    let n2 = right - mid;

    let mut left_arr = Vec::with_capacity(n1);
    let mut right_arr = Vec::with_capacity(n2);

    for i in 0..n1 
    {
        left_arr.push(arr[left + i]);
    }

    for j in 0..n2 
    {
        right_arr.push(arr[mid + 1 + j]);
    }

    let mut i = 0;
    let mut j = 0;
    let mut k = left;

    while i < n1 && j < n2 
    {
        if left_arr[i] <= right_arr[j] 
        {
            arr[k] = left_arr[i];
            i += 1;
        } 
        else 
        {
            arr[k] = right_arr[j];
            j += 1;
        }
        k += 1;
    }

    while i < n1 
    {
        arr[k] = left_arr[i];
        i += 1;
        k += 1;
    }

    while j < n2 
    {
        arr[k] = right_arr[j];
        j += 1;
        k += 1;
    }
}

//Driver function
fn main() 
{
    let mut arr = [64, 25, 12, 22, 11];

    println!("Original array: {:?}", arr);

    let arr_len = arr.len();
    merge_sort(&mut arr, 0, arr_len - 1);

    println!("Sorted aarray: {:?}", arr);
}
