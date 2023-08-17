/// Source: https://leetcode.com/problems/add-two-numbers/
/// 
/// Date: 2023-08-18
/// 
/// Problem:
/// You are given two non-empty linked lists representing two non-negative integers. The digits are stored 
/// in reverse order, and each of their nodes contains a single digit. Add the two numbers and return the 
/// sum as a linked list. You may assume the two numbers do not contain any leading zero, except the 
/// number 0 itself.
/// 
/// Example 1:
/// 
/// ![](https://assets.leetcode.com/uploads/2020/10/02/addtwonumber1.jpg)
/// 
/// ```no_run
/// Input: l1 = [2,4,3], l2 = [5,6,4]
/// Output: [7,0,8]
/// Explanation: 342 + 465 = 807.
/// ```
/// 
/// Example 2:
/// ```no_run
/// Input: l1 = [0], l2 = [0]
/// Output: [0]
/// ```
/// 
/// Example 3:
/// ```no_run
/// Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
/// Output: [8,9,9,9,0,0,0,1]
/// ```



// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut l3_head = None;
    let mut current_node = &mut l3_head;
    let mut carry_out = 0;
    let mut l1 = &l1;
    let mut l2 = &l2;
    while l1.is_some() || l2.is_some() || carry_out > 0 {
        let mut val3 = carry_out;
        if let Some(node) = &l1 {
            val3 += node.val;
            l1 = &node.next;
        }

        if let Some(node) = &l2 {
            val3 += node.val;
            l2 = &node.next;
        }

        carry_out = val3 / 10;
        val3 = val3 % 10;

        let new_node = Some(Box::new(ListNode::new(val3)));
        *current_node = new_node;
        current_node = &mut current_node.as_mut().unwrap().next;
    }
    l3_head
}

#[test]
fn add_two_numbers_test() {
    let l1 = vec_to_list_node(vec![2, 4, 3]);
    let l2 = vec_to_list_node(vec![5, 6, 4]);
    println!("{:?}", l1);
    println!("{:?}", l2);
    let l3 = add_two_numbers(l1, l2);
    println!("{:?}", l3);
    assert_eq!(vec![7,0,8], list_node_to_vec(l3));
}

#[cfg(test)]
fn vec_to_list_node(v: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = ListNode::new(0);
    let mut current = &mut head;
    for val in v {
        let new_node = Some(Box::new(ListNode::new(val)));
        current.next = new_node;
        current = current.next.as_mut().unwrap();
    }
    head.next
}

#[cfg(test)]
fn list_node_to_vec(node: Option<Box<ListNode>>) -> Vec<i32> {
    let mut node = &node;
    let mut vec = vec![];

    while let Some(p) = node {
        vec.push(p.val);
        node = &p.next;
    }
    vec
}

fn main() {}
