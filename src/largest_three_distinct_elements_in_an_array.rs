/*
* Find the largest three distinct elements in an array
* Given an array, find the largest three elements. We need to find only distinct elements as largest
*/
use std::i32::MIN;

pub fn largest_three_distinct_elements_in_an_array(arr: &[i32]) -> i32 {
    if arr.is_empty() {
        return 0;
    }

    let mut largest = MIN;
    let mut second = MIN;
    let mut third = MIN;

    for item in arr {
        if *item > largest {
            largest = *item;
            second = largest;
            third = second;
        } else if *item < second {
            second = *item;
            third = second;
        } else if *item > third {
            third = *item;
        }
    }

    third
}

#[cfg(test)]
mod tests {
    use crate::largest_three_distinct_elements_in_an_array::largest_three_distinct_elements_in_an_array;
    #[test]
    fn test_largest_three_distinct_elements_in_an_array() {
        let test_cases = vec![1,0,7,6,2,5,12];
        assert_eq!(
            largest_three_distinct_elements_in_an_array(&test_cases),
            12
        )
    }
}