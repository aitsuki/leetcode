/// Source: https://leetcode.com/problems/reverse-integer/
///
/// Date: 2023-08-19
///
/// Problem: Given a signed 32-bit integer x, return x with its digits reversed.
/// If reversing x causes the value to go outside the signed 32-bit integer
/// range [-2^31, 2^31 - 1], then return 0.
///
/// Assume the environment does not allow you to store 64-bit integers (signed or unsigned).
///
/// Example 1:
/// ```no_run
/// Input: x = 123
/// Output: 321
/// ```
///
/// Example 2:
/// ```no_run
/// Input: x = -123
/// Output: -321
/// ```
///
/// Example 3:
/// ```no_run
/// Input: x = 120
/// Output: 21
/// ```

pub fn reverse(x: i32) -> i32 {
    let mut vec = vec![];
    let mut y = x;
    while y != 0 {
        let v = y % 10;
        vec.push(v);
        y /= 10;
    }

    // Min = -2147483648,
    // Max = 2147483647,
    if vec.len() == 10 {
        if x > 0 {
            let max_vec = vec![2, 1, 4, 7, 4, 8, 3, 6, 4, 7];
            let mut i = 0;
            while i < 10 {
                if vec[i] > max_vec[i] {
                    return 0;
                } else if vec[i] < max_vec[i] {
                    break;
                }
                i += 1;
            }
        } else {
            let min_vec = vec![-2, -1, -4, -7, -4, -8, -3, -6, -4, -8];
            let mut i = 0;
            while i < 10 {
                if vec[i] < min_vec[i] {
                    return 0;
                } else if vec[i] > min_vec[i] {
                    break;
                }
                i += 1;
            }
        }
    }

    let mut result = 0;
    let mut b = 1;
    let mut i = vec.len();
    while i > 0 {
        i -= 1;
        result += vec[i] * b;
        if i > 0 {
            b *= 10;            
        }
    }
    result
}

#[test]
fn reverse_test() {
    assert_eq!(-2143847412, reverse(-2147483412));
    assert_eq!(0, reverse(2147483647));
    assert_eq!(0, reverse(1534236469));
    assert_eq!(321, reverse(123));
    assert_eq!(-321, reverse(-123));
    assert_eq!(21, reverse(120));
}

fn main() {}
