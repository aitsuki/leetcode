/// Source: https://leetcode.com/problems/longest-palindromic-substring/
/// 
/// Date: 2023-08-18
/// 
/// Problem: Given a string s, return the longest palindromic substring in s.
/// 
/// Example 1:
/// ```no_run
/// Input: s = "babad"
/// Output: "bab"
/// Explanation: "aba" is also a valid answer.
/// ```
/// 
/// Example 2:
/// ```no_run
/// Input: s = "cbbd"
/// Output: "bb"
/// ```
 

pub fn longest_palindrome(s: String) -> String {
    let s = s.as_bytes();
    let mut pld_len = s.len();
    while pld_len > 0 {
        let mut i = 0;
        let mut j = i + pld_len - 1;
        while j < s.len() {
            if is_palindrome(s, i, j) {
                return std::str::from_utf8(&s[i..=j]).unwrap().to_string();
            } else {
                i += 1;
                j += 1;
            }
        }
        pld_len -= 1;
    }
    return "".to_string();
}

fn is_palindrome(s: &[u8], mut i: usize, mut j: usize) -> bool {
    while i < j {
        if s[i] == s[j] {
            i += 1;
            j -= 1;
        } else {
            return false;
        }
    }
    true
}

#[test]
fn longest_palindrome_test() {
    assert_eq!("bab".to_string(), longest_palindrome("babad".to_string()));
    assert_eq!("bb".to_string(), longest_palindrome("cbbd".to_string()));
    assert_eq!("bb".to_string(), longest_palindrome("bb".to_string()));
    assert_eq!("owo".to_string(), longest_palindrome("helloworld!".to_string()));
    assert_eq!("a".to_string(), longest_palindrome("abc".to_string()));
    assert_eq!("a".to_string(), longest_palindrome("a".to_string()));
    assert_eq!("ccc".to_string(), longest_palindrome("ccc".to_string()));
    assert_eq!("anana".to_string(), longest_palindrome("anana".to_string()));
}

fn main() {}