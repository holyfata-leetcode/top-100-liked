use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map = HashMap::new();
        for s in strs {
            let mut chars: Vec<char> = s.chars().collect();
            chars.sort_unstable();
            let sorted: String = chars.into_iter().collect();
            map.entry(sorted).or_insert_with(Vec::new).push(s);
        }
        map.into_values().collect()
    }
}
