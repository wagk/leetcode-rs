#![allow(dead_code)]

//! https://leetcode.com/problems/reverse-linked-list/
//! Given the head of a singly linked list, reverse the list, and
//! return the reversed list.
//! Constraints:
//! The number of nodes in the list is the range [0, 5000].
//! -5000 <= Node.val <= 5000

//! Follow up: A linked list can be reversed either iteratively or
//! recursively. Could you implement both?

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

struct Solution;

impl Solution {
    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        use std::collections::VecDeque;

        let mut stack = VecDeque::<i32>::new();

        while let Some(node) = head {
            stack.push_back(node.val);
            head = node.next;
        }

        let mut ret = None;
        let mut tail = &mut ret;
        while let Some(i) = stack.pop_back() {
            *tail = Some(Box::new(ListNode::new(i)));
            tail = &mut (tail.as_mut().unwrap().next);
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    fn from_vec(v: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut tail = &mut head;

        for i in v {
            let node = Some(Box::new(ListNode::new(i)));

            *tail = node;
            tail = &mut (tail.as_mut().unwrap().next);
        }

        head
    }

    fn to_vec(mut n: Option<Box<ListNode>>) -> Vec<i32> {
        let mut v = Vec::<i32>::new();
        while let Some(node) = n {
            v.push(node.val);
            n = node.next;
        }
        v
    }

    #[test]
    fn from_to_vec() {
        let input = vec![1, 2, 3, 4, 5];
        let output = to_vec(from_vec(input.clone()));
        assert_eq!(input, output);
    }

    #[test_case(vec![1, 2, 3, 4, 5] => vec![5, 4, 3, 2, 1])]
    #[test_case(vec![1, 2] => vec![2, 1])]
    fn test(input: Vec<i32>) -> Vec<i32> {
        let input = from_vec(input);
        let output = Solution::reverse_list(input);
        to_vec(output)
    }
}
