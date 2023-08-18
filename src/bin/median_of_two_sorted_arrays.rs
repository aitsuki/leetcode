/// Source: https://leetcode.com/problems/median-of-two-sorted-arrays/
///
/// Date: 2023-08-18
///
/// Problem: Given two sorted arrays nums1 and nums2 of size m and n respectively, return the median of the two sorted arrays.
/// The overall run time complexity should be `O(log (m+n))`.
///
/// Example 1:
/// ```no_run
/// Input: nums1 = [1,3], nums2 = [2]
/// Output: 2.00000
/// Explanation: merged array = [1,2,3] and median is 2.
/// ```
///
/// Example 2:
/// ```no_run
/// Input: nums1 = [1,2], nums2 = [3,4]
/// Output: 2.50000
/// Explanation: merged array = [1,2,3,4] and median is (2 + 3) / 2 = 2.5.
/// ```

pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let virtual_median_idx =  (nums1.len() + nums2.len()) / 2;

    let mut index1 = 0;
    let mut index2 = 0;
    let mut median = 0;
    let mut last_median = 0;

    while (index1 + index2) <= virtual_median_idx {
        last_median = median;
        let v1 = nums1.get(index1);
        let v2 = nums2.get(index2);
        if v1.is_some() && v2.is_some() {
            let v1 = v1.unwrap();
            let v2 = v2.unwrap();
            if v1 < v2 {
                median = *v1;
                index1 += 1;
            } else {
                median = *v2;
                index2 += 1;
            }
        } else if v1.is_some() {
            median = *v1.unwrap();
            index1 += 1;
        } else {
            median = *v2.unwrap();
            index2 += 1;
        }
    }

    if (nums1.len() + nums2.len()) % 2 == 0 {
        f64::from(median + last_median) / 2.0
    } else {
        f64::from(median)
    }
}

#[test]
fn find_median_sorted_arrays_test() {
    let nums1 = vec![1, 3];
    let nums2 = vec![2];
    assert_eq!(2.0, find_median_sorted_arrays(nums1, nums2));

    let nums1 = vec![1, 3];
    let nums2 = vec![2, 4];
    assert_eq!(2.5, find_median_sorted_arrays(nums1, nums2));

    let nums1 = vec![];
    let nums2 = vec![1];
    assert_eq!(1.0, find_median_sorted_arrays(nums1, nums2));
}

fn main() {}
