#![allow(dead_code)]

//! https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/
//!
//! Suppose an array of length n sorted in ascending order is rotated
//! between 1 and n times. For example, the array nums =
//! [0,1,2,4,5,6,7] might become:
//!
//!     [4,5,6,7,0,1,2] if it was rotated 4 times.
//!     [0,1,2,4,5,6,7] if it was rotated 7 times.
//!
//! Notice that rotating an array [a[0], a[1], a[2], ..., a[n-1]] 1
//! time results in the array [a[n-1], a[0], a[1], a[2], ..., a[n-2]].
//!
//! Given the sorted rotated array nums of unique elements, return the
//! minimum element of this array.
//!
//! *You must write an algorithm that runs in O(log n) time.*
//!
//! Example 1:
//!
//! Input: nums = [3,4,5,1,2]
//! Output: 1
//! Explanation: The original array was [1,2,3,4,5] rotated 3 times.
//!
//! Example 2:
//!
//! Input: nums = [4,5,6,7,0,1,2]
//! Output: 0
//! Explanation: The original array was [0,1,2,4,5,6,7] and it was
//! rotated 4 times.
//!
//! Example 3:
//!
//! Input: nums = [11,13,15,17]
//! Output: 11
//! Explanation: The original array was [11,13,15,17] and it was
//! rotated 4 times.
//!
//! Constraints:
//!
//!     n == nums.length
//!     1 <= n <= 5000
//!     -5000 <= nums[i] <= 5000
//!     All the integers of nums are unique.
//!     nums is sorted and rotated between 1 and n times.
//!

// - we need it to be O(log n) time
// - A linear search would solve the issue but not the time complexity
//
// There's a discontinuity somewhere and we need to find it, because
// that's where the minimum is.
//
// - Take the first element
// - Use it as the pivot and binary search through the array
//   - If the index is smaller than the pivot, then the
//   discontinuity is somewhere to the left
//   - If the index is larger than the pivot, then the
//   discontinuity is somewhere to the right
//
// Terminating condition would be when the indices are the same? (or
// the size of the range hits 1ish)

struct Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        assert!(!nums.is_empty());

        let mut beg = 0;
        let mut end = nums.len();
        let mut mid = (end - beg) / 2;

        while beg != end {
            let (new_beg, new_end) = if nums[beg] < nums[mid] {
                (mid, end)
            } else {
                (beg, mid)
            };

            beg = new_beg;
            end = new_end;
            mid = beg + ((end - beg) / 2);

            assert!(beg <= end);
        }

        nums[(beg + 1) % nums.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case([3, 4, 5, 1, 2]       => 1)]
    #[test_case([4, 5, 6, 7, 0, 1, 2] => 0)]
    #[test_case([11, 13, 15, 17]      => 11)]
    #[test_case([7, 1, 2, 3, 4, 5, 6] => 1)]
    fn test<const N: usize>(list: [i32; N]) -> i32 {
        Solution::find_min(list.to_vec())
    }
}
