use crate::Solution;
use std::collections::HashSet;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let set: HashSet<i32> = nums.into_iter().collect();
        let mut max_len = 0;
        for &num in &set {
            if !set.contains(&(num - 1)) {
                let mut current = num;
                let mut len = 1;
                while set.contains(&(current + 1)) {
                    current += 1;
                    len += 1;
                }
                max_len = max_len.max(len);
            }
        }
        max_len
    }
}
