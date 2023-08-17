/// Source: https://leetcode.com/problems/two-sum/
/// 
/// Date: 2023-08-17
/// 
/// Problem:
/// Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
/// You may assume that each input would have exactly one solution, and you may not use the same element twice.
/// You can return the answer in any order.
/// 
/// Example 1:
/// ```no_run
/// Input: nums = [2,7,11,15], target = 9
/// Output: [0,1]
/// Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
/// ```
/// 
/// Example 2:
/// ```no_run
/// Input: nums = [3,2,4], target = 6
/// Output: [1,2]
/// ```
/// 
/// Example 3:
/// ```no_run
/// Input: nums = [3,3], target = 6
/// Output: [0,1]
/// ```

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for left in 0..nums.len()-1 {
        for right in (left + 1)..nums.len() {
            let sum = nums[left] + nums[right];
            if sum == target {
                return vec![left as i32, right as i32];
            }
        }
    }
    vec![]
}

#[test]
fn two_sum_test() {
    let nums = vec![2,7,11,15];
    let target = 9;
    let output: Vec<i32> = vec![0, 1];
    assert_eq!(output, two_sum(nums, target));

    let nums = vec![3,2,4];
    let target = 6;
    let output: Vec<i32> = vec![1, 2];
    assert_eq!(output, two_sum(nums, target));

    let nums = vec![3,3];
    let target = 6;
    let output: Vec<i32> = vec![0, 1];
    assert_eq!(output, two_sum(nums, target));
}

fn main() {
}