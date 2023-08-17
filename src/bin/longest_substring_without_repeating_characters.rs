use std::{collections::HashMap, cmp::max};

/// Source: https://leetcode.com/problems/longest-substring-without-repeating-characters/
///
/// Date: 2023-08-18
///
/// Problem:
/// Given a string s, find the length of the longest substring without repeating characters.
///
/// Example 1:
/// ```no_run
/// Input: s = "abcabcbb"
/// Output: 3
/// Explanation: The answer is "abc", with the length of 3.
/// ```
///
/// Example 2:
/// ```no_run
/// Input: s = "bbbbb"
/// Output: 1
/// Explanation: The answer is "b", with the length of 1.
/// ```
///
/// Example 3:
/// ```no_run
/// Input: s = "pwwkew"
/// Output: 3
/// Explanation: The answer is "wke", with the length of 3.
/// Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.
/// ```

pub fn length_of_longest_substring(s: String) -> i32 {
    if s.len() < 2 {
        return s.len() as i32;
    }
    let mut map = HashMap::with_capacity(s.len());
    let mut max_len = 0;
    let mut start = 0;
    for (i, c) in s.char_indices() {
        max_len = max(max_len, i - start);
        if let Some(last_idx) = map.get(&c) {
            if *last_idx >= start {
                start = last_idx + 1;
            }
        }
        map.insert(c, i);
    }
    max_len = max(max_len, s.len() - start);
    max_len as i32
}

#[test]
fn length_of_longest_substring_test() {
    assert_eq!(0, length_of_longest_substring("".to_string()));
    assert_eq!(5, length_of_longest_substring("tmmzuxt".to_string()));
    assert_eq!(3, length_of_longest_substring("abcabcbb".to_string()));
    assert_eq!(3, length_of_longest_substring("abc".to_string()));
    assert_eq!(1, length_of_longest_substring(" ".to_string()));
    assert_eq!(1, length_of_longest_substring("bbbbb".to_string()));
    assert_eq!(3, length_of_longest_substring("pwwkew".to_string()));
    assert_eq!(2, length_of_longest_substring("aab".to_string()));
    assert_eq!(3, length_of_longest_substring("dvdf".to_string()));
}

fn main() {}
