use std::str::Chars;

/// Source: https://leetcode.com/problems/regular-expression-matching/
///
/// Date: 2023-08-21
///
/// Problem: Given an input string s and a pattern p, implement regular expression matching with support for '.' and '*' where:
/// - '.' Matches any single character.​​​​
/// - '*' Matches zero or more of the preceding element.
///
/// The matching should cover the entire input string (not partial).
///
/// Example 1:
/// ```no_run
/// Input: s = "aa", p = "a"
/// Output: false
/// Explanation: "a" does not match the entire string "aa".
/// ```
///
/// Example 2:
/// ```no_run
/// Input: s = "aa", p = "a*"
/// Output: true
/// Explanation: '*' means zero or more of the preceding element, 'a'. Therefore, by repeating 'a' once, it becomes "aa".
/// ```
///
/// Example 3:
/// ```no_run
/// Input: s = "ab", p = ".*"
/// Output: true
/// Explanation: ".*" means "zero or more (*) of any character (.)".
/// ```

pub fn is_match(s: String, p: String) -> bool {
    let mut next_c = s.chars();
    let mut next_p = p.chars();
    is_match_char(next_c.next(), next_c, next_p.next(), next_p, None)
}

fn is_match_char(
    c: Option<char>,
    mut next_c: Chars,
    p: Option<char>,
    mut next_p: Chars,
    last_p: Option<char>,
) -> bool {
    if c.is_none() && p.is_none() {
        return true;
    } else if c.is_none() || p.is_none() {
        return false;
    }

    let c = c.unwrap();
    let p = p.unwrap();

    return match p {
        '.' => is_match_char(next_c.next(), next_c, next_p.next(), next_p, Some(p)),
        '*' => is_match_char(Some(c), next_c, last_p, next_p, last_p),
        _ => {
            if c == p {
                let mut new_p = next_p.next();
                let mut new_c = next_c.next();
                if new_p == Some('*') {
                    // consume repeat c
                    while new_c == Some(c) || (new_c.is_some() && last_p == Some('.')) {
                        new_c = next_c.next();
                    }
                    new_p = next_p.next();
                }
                is_match_char(new_c, next_c, new_p, next_p, Some(p))
            } else {
                let new_p = next_p.next();
                if new_p == Some('*') {
                    is_match_char(next_c.next(), next_c, next_p.next(), next_p, new_p)
                } else {
                    false
                }
            }
        }
    };
}

#[test]
fn is_match_test() {
    assert!(is_match("ab".to_string(), ".*".to_string()));
    assert!(!is_match("aa".to_string(), "a".to_string()));
    assert!(is_match("aa".to_string(), "a*".to_string()));
    assert!(is_match("aab".to_string(), "c*a*b*".to_string()));
    assert!(is_match("aaa".to_string(), "a*a".to_string()));
}

fn main() {}
