/*
**Find the Missing Number
** Given an array arr[] of size n-1 with integers in the range of [1, n],
** the task is to find the missing number from the first N integers.
*/

// Using sum formula N * (N + 1) / 2
// When N is the length of the range

fn sum_first_n_nums(n: i32) -> i32 {
    (n * (n + 1)) / 2
}
pub fn find_the_missing_number(nums: Vec<i32>) -> i32 {
    // length -> real size of range [1...n]
    let length = nums.len() + 1;

    sum_first_n_nums(length as i32) - nums.iter().sum::<i32>()
}

pub fn find_the_missing_number_2(nums: Vec<i32>) -> i32 {
    let length = nums.len() + 1;
    let mut range = 1..length;

    println!("length {},range {:?}",length,  range);
    let rsp = range.find(|num| !nums.contains(&(*num as i32)));

    // let mut sum = 0;
    // for value in range {
    //     if !nums.contains(&(value as i32)) {
    //         return  value as i32
    //     }
    // }

    rsp.unwrap() as i32
    // sum
}

#[cfg(test)]
mod find_the_missing_number_tests {
    use crate::find_the_missing_number::find_the_missing_number;
    use crate::find_the_missing_number::find_the_missing_number_2;
    #[test]
    fn find_the_missing_number_2_tests() {
        assert_eq!(find_the_missing_number_2(vec![1,2,3,5]), 4);
    }

    #[test]
    fn find_the_missing_number_tests() {
        assert_eq!(find_the_missing_number(vec![1,2,3,5]), 4);
    }
}