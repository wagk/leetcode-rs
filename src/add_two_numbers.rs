/// https://leetcode.com/problems/add-two-numbers/

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

#[allow(dead_code)]
struct Solution;

// -----

impl Solution {
    #[allow(dead_code)]
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut carry = false;

        let mut sum = None;
        let mut sum_tail = &mut sum;

        loop {
            let mut l1_next = None;
            let mut l2_next = None;

            let (value_1, value_2) = match (l1, l2) {
                (None, None) => {
                    if carry {
                        (0, 0)
                    } else {
                        return sum;
                    }
                }
                (None, Some(v2)) => {
                    l2_next = v2.next;
                    (0, v2.val)
                }
                (Some(v1), None) => {
                    l1_next = v1.next;
                    (v1.val, 0)
                }
                (Some(v1), Some(v2)) => {
                    l1_next = v1.next;
                    l2_next = v2.next;
                    (v1.val, v2.val)
                }
            };

            l1 = l1_next;
            l2 = l2_next;

            let total = value_1 + value_2 + if carry { 1 } else { 0 };
            carry = false;

            let next_val = match total {
                x @ 0..=9 => x,
                x @ 10.. => {
                    carry = true;
                    x - 10
                }
                x => panic!("x: {:?}", x),
            };

            *sum_tail = Some(Box::new(ListNode::new(next_val)));
            sum_tail = &mut (sum_tail.as_mut().unwrap().next);
        }
    }
}

// -----

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    fn construct_input<const N: usize>(input: [i32; N]) -> Option<Box<ListNode>> {
        let mut list_head = None;
        let mut list_tail = &mut list_head;
        for i in input {
            *list_tail = Some(Box::new(ListNode::new(i)));
            list_tail = &mut (list_tail.as_mut().unwrap().next);
        }
        list_head
    }

    fn export_list(mut list: Option<Box<ListNode>>) -> Vec<i32> {
        let mut values = vec![];
        while let Some(node) = list {
            values.push(node.val);
            list = node.next;
        }
        values
    }

    #[test_case([1, 2, 3])]
    fn test_construct_input<const N: usize>(input: [i32; N]) {
        let values = export_list(construct_input(input.clone()));
        assert_eq!(values, input);
    }

    #[test_case([2, 4, 3], [5, 6, 4]                => vec![7, 0, 8])]
    #[test_case([9, 9, 9, 9, 9, 9, 9], [9, 9, 9, 9] => vec![8, 9, 9, 9, 0, 0, 0, 1])]
    fn test<const N: usize, const M: usize>(list_1: [i32; N], list_2: [i32; M]) -> Vec<i32> {
        let l1 = construct_input(list_1);
        let l2 = construct_input(list_2);

        let sum = Solution::add_two_numbers(l1, l2);

        export_list(sum)
    }
}
