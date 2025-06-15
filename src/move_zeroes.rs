use crate::Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let nums_len = nums.len();
        let mut nums_checked = 0usize;
        let mut index = 0usize;
        loop {
            if nums_checked >= nums_len {
                break;
            }
            if nums[index] == 0 {
                nums.remove(index);
                nums.push(0);
                // 不增加 index，因为移除后当前 index 位置的元素变了
            } else {
                index += 1;
            }
            nums_checked += 1;
        }
    }
}
