/*
Given an array arr[] of size n, the task is to find the largest element in the given array.
*/

pub fn largest(array: Vec<i32>) -> i32 {
    let mut largest: i32 = array[0];

    for i in 1..array.len() {
        if array[i] > array[i - 1] {
            largest = array[i];
        }
    }

    largest
}

pub fn largest_recursive(array: Vec<i32>, i: i32) -> i32 {
    // Compare first and last item
    println!("{:?}", array);
    println!("i {:?}", i);

    if i == (array.len() - 1) as i32 {
        println!("last {:?}", i);
        return array[i as usize];
    }

    let mut largest: i32 = largest_recursive(array.clone(), i + 1);

    println!("largest is {:?}", largest);

    if largest < array[i as usize] {
        largest = array[i as usize];
    }

    largest
}

#[cfg(test)]
mod tests {
    use crate::largest_element_in_array::solution::{largest, largest_recursive};

    #[test]
    fn largest_element_in_array() {
        assert_eq!(largest(vec![1,2,3,5]), 5);
    }

    #[test]
    fn largest_recursive_in_array() {
        assert_eq!(largest_recursive(vec![1,2,3,5], 0), 5);
    }
}