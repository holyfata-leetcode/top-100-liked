use crate::Solution;

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let mut p_str = p.chars().collect::<Vec<char>>();
        p_str.sort();
        let mut result: Vec<i32> = Vec::new();
        if s.len() < p.len() {
            return result;
        }
        let p_len = p.len();
        for i in 0..s.len() + 1 - p_len {
            let sub = &s[i..i + p_len];
            let mut sub_str = sub.chars().collect::<Vec<char>>();
            sub_str.sort();
            if sub_str == p_str {
                result.push(i as i32);
            }
        }
        result
    }
}
