# LeetCode 热题 100

<!-- update_progress -->

![](https://img.shields.io/badge/编程语言-Rust-dea584)
![](https://img.shields.io/badge/进度-4%25-blue)

<!-- update_progress -->

## 两数之和

> 给定一个整数数组 nums 和一个整数目标值 target，请你在该数组中找出 和为目标值 target  的那 两个 整数，并返回它们的数组下标。
> 
> 你可以假设每种输入只会对应一个答案，并且你不能使用两次相同的元素。
> 
> 你可以按任意顺序返回答案。

<!-- insert_source_code src=./src/two_sum.rs -->
```rs
use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for (i, num) in nums.iter().enumerate() {
            let complement = target - num;
            if let Some(&idx) = map.get(&complement) {
                return vec![idx as i32, i as i32];
            }
            map.insert(num, i);
        }
        vec![]
    }
}

```
<!-- insert_source_code -->

使用哈希表存储已遍历的数字及其下标：
遍历数组时，用哈希表（HashMap）记录每个数字及其对应的下标，方便后续查找。

查找目标值的补数：
对于当前遍历到的数字 num，计算它与目标值 target 的差值 complement（即 complement = target - num）。
检查 complement 是否已经存在于哈希表中。

找到答案立即返回：
如果补数存在于哈希表中，说明找到了两个数，它们的和等于 target，直接返回它们的下标。

未找到则继续遍历：
如果补数不存在，则将当前数字和下标加入哈希表，继续遍历下一个数字。

时间复杂度 O(n)：
由于哈希表的查找和插入都是常数时间，所以整体时间复杂度为 O(n)

## 字母异位词分组

> 给你一个字符串数组，请你将 字母异位词 组合在一起。可以按任意顺序返回结果列表。
> 
> 字母异位词 是由重新排列源单词的所有字母得到的一个新单词。

<!-- insert_source_code src=./src/group_anagrams.rs -->
```rs
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

```
<!-- insert_source_code -->

利用哈希表分组：
创建一个哈希表（HashMap），用于存储排序后的字符串作为 key，所有属于该 key 的原始字符串作为 value 的列表。

对每个字符串排序：
遍历输入字符串数组，对每个字符串进行字符排序，排序后的字符串作为哈希表的 key。
这样，所有字母异位词排序后都相同，会被分到同一个分组。

将原始字符串加入对应分组：
将排序前的原始字符串加入哈希表对应 key 的 value 列表中。

收集所有分组结果：
遍历哈希表，将所有 value（即分组后的字符串列表）收集起来，作为最终结果返回。

## 最长连续序列

> 给定一个未排序的整数数组 nums ，找出数字连续的最长序列（不要求序列元素在原数组中连续）的长度。
> 
> 请你设计并实现时间复杂度为 O(n) 的算法解决此问题。

<!-- insert_source_code src=./src/longest_consecutive.rs -->
```rs
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

```
<!-- insert_source_code -->

去重存储：
首先用哈希集合（HashSet）对输入数组去重，方便后续 O(1) 查询。

只从连续序列的起点出发：
遍历集合中的每个数字 num，仅当 num-1 不在集合中时，才认为 num 是某个连续序列的起点。

向右扩展序列：
从起点 num 开始，不断检查 num+1、num+2... 是否在集合中，统计当前连续序列的长度。

更新最大长度：
每找到一个连续序列，就更新最大长度。

时间复杂度 O(n)：
每个数字最多只被访问两次，整体效率高。

## 移动零

> 给定一个数组 nums，编写一个函数将所有 0 移动到数组的末尾，同时保持非零元素的相对顺序。
> 
> 请注意 ，必须在不复制数组的情况下原地对数组进行操作。

<!-- insert_source_code src=./src/move_zeroes.rs -->
```rs
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

```
<!-- insert_source_code -->

双指针法：
使用一个指针 last_non_zero 指向下一个应该放非零元素的位置，另一个指针遍历整个数组。

遍历并移动非零元素：
遍历数组时，如果遇到非零元素，就把它赋值到 last_non_zero 指向的位置，并将 last_non_zero 向后移动一位。

填充零：
遍历结束后，从 last_non_zero 开始到数组末尾，将剩余的位置全部赋值为 0。

原地操作：
全程只用常数额外空间，满足原地操作的要求。
