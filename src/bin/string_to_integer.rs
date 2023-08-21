/// Source: https://leetcode.com/problems/string-to-integer-atoi/
/// 
/// Date: 2023-08-21
/// 
/// Problem: Implement the myAtoi(string s) function, which converts a string to a 32-bit signed 
/// integer (similar to C/C++'s atoi function). The algorithm for myAtoi(string s) is as follows:
/// 1. Read in and ignore any leading whitespace.
/// 2. Check if the next character (if not already at the end of the string) is '-' or '+'. Read 
/// this character in if it is either. This determines if the final result is negative or positive 
/// respectively. Assume the result is positive if neither is present.
/// 3. Read in next the characters until the next non-digit character or the end of the input is
///  reached. The rest of the string is ignored.
/// 4. Convert these digits into an integer (i.e. "123" -> 123, "0032" -> 32). If no digits were
/// read, then the integer is 0. Change the sign as necessary (from step 2).
/// 5. If the integer is out of the 32-bit signed integer range [-231, 231 - 1], then clamp the
/// integer so that it remains in the range. Specifically, integers less than -231 should be
/// clamped to -231, and integers greater than 231 - 1 should be clamped to 231 - 1.
/// 6. Return the integer as the final result.

pub fn my_atoi(s: String) -> i32 {
    let mut result = 0i32;

    let mut sign = None;
    for c in s.chars() {
        if sign == None {
            match c {
                ' ' => continue,
                '-' => {
                    sign = Some(-1);
                    continue;
                },
                '+' => {
                    sign = Some(1);
                    continue;
                },
                _ => {
                    if c.is_digit(10) {
                        sign = Some(1);
                    }
                }
            }
        }
        if let Some(digit) = c.to_digit(10) {
            // check overflow (7 is the last digit of i32::MAX)
            let remind = i32::MAX / 10 + 7 - digit as i32;
            if sign.unwrap() == 1 {
                if remind <= result {
                    return i32::MAX;
                }
            } else {
                if remind + 1 <= result {
                    return i32::MIN;
                }
            }
            result *= 10;
            result += digit as i32;
        } else {
            break;
        }
    }
    result * sign.unwrap_or(0)
}

#[test]
fn my_atoi_test() {
    assert_eq!(42, my_atoi("42".to_string()));
    assert_eq!(-42, my_atoi("   -42".to_string()));
    assert_eq!(4193, my_atoi("4193 with words".to_string()));
    assert_eq!(0, my_atoi(" ".to_string()));
    // Min = -2147483648
    // Max = 2147483647
    assert_eq!(2147483647, my_atoi("2147483647".to_string()));
    assert_eq!(-2147483648, my_atoi("-2147483649".to_string()));
    assert_eq!(-2147483648, my_atoi("-21474836482".to_string()));
    assert_eq!(2147483647, my_atoi("2147483648".to_string()));
    assert_eq!(2147483646, my_atoi("2147483646".to_string()));
}

fn main() {}