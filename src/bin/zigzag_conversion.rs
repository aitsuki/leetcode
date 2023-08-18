/// Source: https://leetcode.com/problems/zigzag-conversion/
///
/// Date: 2023-08-18
///
/// Problem: The string "PAYPALISHIRING" is written in a zigzag pattern
/// on a given number of rows like this: (you may want to display this
/// pattern in a fixed font for better legibility)
///
/// ```no_run
/// P   A   H   N
/// A P L S I I G
/// Y   I   R
/// ```
///
/// And then read line by line: "PAHNAPLSIIGYIR"
///
/// Write the code that will take a string and make this conversion given a number of rows:
///
/// ```no_run
/// string convert(string s, int numRows);
/// ```
///
/// Example 1:
/// ```no_run
/// Input: s = "PAYPALISHIRING", numRows = 3
/// Output: "PAHNAPLSIIGYIR"
/// ```
///
/// Example 2:
/// ```no_run
/// Input: s = "PAYPALISHIRING", numRows = 4
/// Output: "PINALSIGYAHRPI"
/// Explanation:
/// P     I    N
/// A   L S  I G
/// Y A   H R
/// P     I
/// ```
///
/// Example 3:
/// ```no_run
/// Input: s = "A", numRows = 1
/// Output: "A"
/// ```

pub fn convert(s: String, num_rows: i32) -> String {
    let num_rows = num_rows as usize;
    if num_rows == 1 {
        return s;
    }

    let mut zigzag = Vec::with_capacity(num_rows);
    for _ in 0..num_rows {
        zigzag.push(Vec::new());
    }

    let mut top_to_bottom = true;
    let mut curr_row: usize = 0;
    let s = s.as_bytes();
    for c in s {
        zigzag[curr_row].push(*c);
        if top_to_bottom {
            if curr_row == num_rows - 1 {
                curr_row -= 1;
                top_to_bottom = false;
            } else {
                curr_row += 1;
            }
        } else {
            if curr_row == 0 {
                curr_row += 1;
                top_to_bottom = true;
            } else {
                curr_row -= 1;
            }
        }
    }

    let mut result = Vec::with_capacity(s.len());
    for ele in &mut zigzag {
        result.append(ele)
    }
    String::from_utf8(result).unwrap()
}

#[test]
fn convert_test() {
    assert_eq!(
        "PAHNAPLSIIGYIR".to_string(),
        convert("PAYPALISHIRING".to_string(), 3)
    );

    assert_eq!(
        "PINALSIGYAHRPI".to_string(),
        convert("PAYPALISHIRING".to_string(), 4)
    );

    assert_eq!("A".to_string(), convert("A".to_string(), 1));
}

fn main() {}
