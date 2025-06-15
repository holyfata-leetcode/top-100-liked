use crate::Solution;
use std::collections::HashSet;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut set = HashSet::new();
        let (mut left, mut max_len) = (0, 0);
        for right in 0..chars.len() {
            while set.contains(&chars[right]) {
                set.remove(&chars[left]);
                left += 1;
            }
            set.insert(chars[right]);
            max_len = max_len.max(right - left + 1);
        }
        max_len as i32
    }
}
