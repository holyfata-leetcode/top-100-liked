# LeetCode 热题 100

<!-- update_progress -->

![](https://img.shields.io/badge/编程语言-Rust-dea584)
![](https://img.shields.io/badge/进度-2%25-blue)

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
