/*
Given an array of positive integers arr[] of size n, the task is to find second largest distinct element in the array.
*/
use std::i64::MIN;

pub fn second_largest_in_array(data: Vec<i64>) -> i64 {
    let mut sorted = data.clone();
    sorted.sort_by(|a, b| b.cmp(a));

    sorted[1]
}

pub fn second_largest_in_array2(data: Vec<i64>) -> i64 {
    let mut first = MIN;
    let mut second = MIN;

    for &item in &data {
        if item > first {
            second = first;
            first = item;
        } else if item > second{
            second = item;
        }
    }

    second
}

#[cfg(test)]
mod tests {
    use crate::second_largest_in_array::solution::second_largest_in_array;
    use crate::second_largest_in_array::solution::second_largest_in_array2;

    #[test]
    fn test_second_largest_in_array() {
        let data = vec![1,3,9,6,2,4,0];
        assert_eq!(second_largest_in_array(data), 6)
    }

    #[test]
    fn test_second_largest_in_array2() {
        let data = vec![1,3,9,6,2,4,0];
        assert_eq!(second_largest_in_array2(data), 6)
    }
}