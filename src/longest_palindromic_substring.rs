#![allow(dead_code)]

/// https://leetcode.com/problems/longest-palindromic-substring/
/// Given a string s, return the longest palindromic substring in s.
///
/// Example 1:
///
/// Input: s = "babad"
/// Output: "bab"
/// Explanation: "aba" is also a valid answer.
///
/// Example 2:
///
/// Input: s = "cbbd"
/// Output: "bb"
///
/// Constraints:
///
///     1 <= s.length <= 1000
///     s consist of only digits and English letters.

struct Solution;

impl Solution {
    fn longest_palindrome_at(s: &Vec<char>, backwards: usize, forwards: usize) -> Vec<char> {
        use std::collections::VecDeque;

        let mut p = VecDeque::<char>::new();
        let range = 0..s.len();

        let mut backwards = Some(backwards);
        let mut forwards = Some(forwards);

        match (backwards, forwards) {
            (Some(b), Some(f)) if b == f => {
                p.push_back(s[b]);
                backwards = b.checked_sub(1);
                forwards = f.checked_add(1);
            }
            _ => {}
        }

        loop {
            match (backwards, forwards) {
                (Some(b), Some(f)) if range.contains(&b) && range.contains(&f) && s[b] == s[f] => {
                    p.push_front(s[b]);
                    p.push_back(s[f]);

                    backwards = b.checked_sub(1);
                    forwards = f.checked_add(1);
                }
                _ => break,
            }
        }

        p.into_iter().collect::<Vec<char>>()
    }

    pub fn longest_palindrome(s: String) -> String {
        assert!(s.is_ascii());

        let s = s.chars().collect::<Vec<char>>();

        let mut longest = Vec::<char>::new();

        for i in 0..s.len() {
            let p = Self::longest_palindrome_at(&s, i, i);
            if p.len() > longest.len() {
                longest = p;
            }

            let q = Self::longest_palindrome_at(&s, i, i + 1);
            if q.len() > longest.len() {
                longest = q;
            }
        }

        longest.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("babad" => "bab")] // "aba" works too, not sure how to express in test
    #[test_case("cbbd"  => "bb")]
    #[test_case("bb"    => "bb")]
    fn test(input: &'static str) -> String {
        Solution::longest_palindrome(input.to_string())
    }
}
