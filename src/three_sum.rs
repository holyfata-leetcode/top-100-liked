use crate::Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable(); // Sort the input to handle duplicates easily
        let n = nums.len();
        if n < 3 {
            return Vec::new(); // If there are less than 3 numbers, return empty
        }

        let mut result: Vec<Vec<i32>> = Vec::new();
        for i in 0..nums.len() {
            if nums[i] > 0 {
                break;
            }
            let (mut l, mut r) = (i + 1, n - 1);
            while l < r {
                let sum = nums[i] + nums[l] + nums[r];
                if sum == 0 {
                    let turple = vec![nums[i], nums[l], nums[r]];
                    if !result.contains(&turple) {
                        result.push(vec![nums[i], nums[l], nums[r]]);
                    }
                    while l < r && nums[l] == nums[l + 1] {
                        l += 1;
                    }
                    while l < r && nums[r] == nums[r - 1] {
                        r -= 1;
                    }
                    l += 1;
                    r -= 1;
                } else if sum < 0 {
                    l += 1;
                } else {
                    r -= 1;
                }
            }
        }
        result
    }
}
