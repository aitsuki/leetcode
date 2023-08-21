/// Source: https://leetcode.com/problems/palindrome-number/
///
/// Date: 2023-08-21
///
/// Problem: Given an integer x, return true if x is a palindrome, and false otherwise.
///
/// Example 1:
/// ```no_run
/// Input: x = 121
/// Output: true
/// Explanation: 121 reads as 121 from left to right and from right to left.
/// ```
///
/// Example 2:
/// ```no_run
/// Input: x = -121
/// Output: false
/// Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.
/// ```
///
/// Example 3:
/// ```no_run
/// Input: x = 10
/// Output: false
/// Explanation: Reads 01 from right to left. Therefore it is not a palindrome.
/// ```

pub fn is_palindrome(x: i32) -> bool {
    let mut xx = x;
    let mut rx = 0;
    while xx > 0 {
        rx = rx * 10 + xx % 10;
        xx /= 10;
    }
    x == rx
}

#[test]
fn is_palindrome_test() {
    assert!(is_palindrome(121));
    assert!(!is_palindrome(-121));
    assert!(!is_palindrome(10));
}

fn main() {}
