#![allow(dead_code)]

//! https://leetcode.com/problems/container-with-most-water/
//!
//! You are given an integer array height of length n. There are n
//! vertical lines drawn such that the two endpoints of the ith line
//! are (i, 0) and (i, height[i]).
//!
//! Find two lines that together with the x-axis form a container,
//! such that the container contains the most water.
//!
//! Return the maximum amount of water a container can store.
//!
//! Notice that you may not slant the container.
//!
//! Input: height = [1,8,6,2,5,4,8,3,7]
//! Output: 49
//! Explanation: The above vertical lines are represented by array
//! [1,8,6,2,5,4,8,3,7]. In this case, the max area of water (blue
//! section) the container can contain is 49.
//!
//! Example 2:
//!
//! Input: height = [1,1]
//! Output: 1
//!
//! Constraints:
//!
//!     n == height.length
//!     2 <= n <= 105
//!     0 <= height[i] <= 104

//! The brute force method is:
//!
//! ```rust
//! for i in 0..height.len() {
//!     for j in 0..height.len() {
//!         let area = Self::find_area(&height, i, j);
//!         if area > max_area {
//!             max_area = area;
//!         }
//!     }
//! }
//! ```
//!
//! Which is too slow for us. The trick leetcode suggests is the following:
//!
//! Approach 2: Two Pointer Approach
//!
//! Algorithm
//!
//! The intuition behind this approach is that the area formed between
//! the lines will always be limited by the height of the shorter
//! line. Further, the farther the lines, the more will be the area
//! obtained.
//!
//! We take two pointers, one at the beginning and one at the end of
//! the array constituting the length of the lines. Futher, we
//! maintain a variable `maxarea` to store the maximum area obtained
//! till now. At every step, we find out the area formed between them,
//! update `maxarea` and move the pointer pointing to the shorter line
//! towards the other end by one step.
//!
//! Initially we consider the area constituting the exterior most
//! lines. Now, to maximize the area, we need to consider the area
//! between the lines of larger lengths. If we try to move the pointer
//! at the longer line inwards, we won't gain any increase in area,
//! since it is limited by the shorter line. But moving the shorter
//! line's pointer could turn out to be beneficial, as per the same
//! argument, despite the reduction in the width. This is done since a
//! relatively longer line obtained by moving the shorter line's
//! pointer might overcome the reduction in area caused by the width
//! reduction.
//!
//! https://leetcode.com/problems/container-with-most-water/solution/

struct Solution;

impl Solution {
    fn find_area(vec: &[i32], i: usize, j: usize) -> i32 {
        let (i, j) = if j > i { (i, j) } else { (j, i) };

        let height = vec[i].min(vec[j]);

        height * (j - i) as i32
    }

    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max_area = 0;

        let mut back = 0;
        let mut front = height.len() - 1;

        while back < front {
            let area = Self::find_area(&height, back, front);
            max_area = max_area.max(area);

            if height[back] > height[front] {
                front = front.saturating_sub(1);
            } else {
                back = back.saturating_add(1);
            }
        }

        max_area
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case([1, 1]              => 1)]
    #[test_case([1,8,6,2,5,4,8,3,7] => 49)]
    fn test<const N: usize>(list: [i32; N]) -> i32 {
        Solution::max_area(list.to_vec())
    }
}
