#![allow(dead_code)]

/// https://leetcode.com/problems/longest-substring-without-repeating-characters/
/// Given a string s, find the length of the longest substring without
/// repeating characters.
///
/// Example 1:
///
/// Input: s = "abcabcbb"
/// Output: 3
/// Explanation: The answer is "abc", with the length of 3.
///
/// Example 2:
///
/// Input: s = "bbbbb"
/// Output: 1
/// Explanation: The answer is "b", with the length of 1.
///
/// Example 3:
///
/// Input: s = "pwwkew"
/// Output: 3
/// Explanation: The answer is "wke", with the length of 3.
/// Notice that the answer must be a substring, "pwke" is a
/// subsequence and not a substring.
///
/// Constraints:
///
///     0 <= s.length <= 5 * 104
///     s consists of English letters, digits, symbols and spaces.
///

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::collections::VecDeque;

        let mut vec = VecDeque::<char>::new();
        let mut longest_length = 0;

        for c in s.chars() {
            while vec.contains(&c) {
                vec.pop_front();
            }

            vec.push_back(c);

            if vec.len() > longest_length {
                longest_length = vec.len();
            }
        }

        longest_length as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(""         => 0)] // ""
    #[test_case("aab"      => 2)] // "ab"
    #[test_case("abcabcbb" => 3)] // "abc"
    #[test_case("bbbbb"    => 1)] // "b"
    #[test_case("bbtablud" => 6)] // "tablud"
    #[test_case("dvdf"     => 3)] // "vdf"
    #[test_case("ohvhjdml" => 6)] // "vhjdml"
    #[test_case("pwwkew"   => 3)] // "wke"
    #[test_case("tmmzuxt"  => 5)] // "mzuxt"
    fn test(input: &'static str) -> i32 {
        Solution::length_of_longest_substring(input.to_string())
    }
}
