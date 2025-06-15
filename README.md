# LeetCode 热题 100

<!-- update_progress -->

![](https://img.shields.io/badge/编程语言-Rust-dea584)
![](https://img.shields.io/badge/进度-7%25-blue)

<!-- update_progress -->

## [两数之和](https://leetcode.cn/problems/two-sum/description/?envType=study-plan-v2&envId=top-100-liked)

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

## [字母异位词分组](https://leetcode.cn/problems/group-anagrams/description/?envType=study-plan-v2&envId=top-100-liked)

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

## [最长连续序列](https://leetcode.cn/problems/longest-consecutive-sequence/description/?envType=study-plan-v2&envId=top-100-liked)

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

## [移动零](https://leetcode.cn/problems/move-zeroes/description/?envType=study-plan-v2&envId=top-100-liked)

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

## 盛最多水的容器

> 给定一个长度为 n 的整数数组 height 。有 n 条垂线，第 i 条线的两个端点是 (i, 0) 和 (i, height[i]) 。
> 
> 找出其中的两条线，使得它们与 x 轴共同构成的容器可以容纳最多的水。
> 
> 返回容器可以储存的最大水量。
> 
> 说明：你不能倾斜容器。

<!-- insert_source_code src=./src/max_area.rs -->
```rs
use crate::Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, height.len() - 1);
        let mut max_area = 0;
        while left < right {
            let h = height[left].min(height[right]);
            let w = (right - left) as i32;
            max_area = max_area.max(h * w);
            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }
        max_area
    }
}

```
<!-- insert_source_code -->

双指针法：
用两个指针分别指向数组的两端（left 和 right），代表当前选取的两条线。

计算当前面积：
每次计算由 left 和 right 形成的容器面积：高度取两端较小值，宽度为 right - left。

更新最大面积：
用一个变量记录目前为止的最大面积，每次计算后进行更新。

移动较短板指针：
为了尝试获得更大的面积，移动高度较小的那一端指针（如果左端较短则 left++，否则 right--），因为移动较高的那一端不会增加面积。

直到指针相遇：
当 left 和 right 相遇时，遍历结束，返回最大面积。

## 三数之和

> 给你一个整数数组 nums ，判断是否存在三元组 [nums[i], nums[j], nums[k]] 满足 i != j、i != k 且 j != k ，同时还满足 nums[i] + nums[j] + nums[k] == 0 。请你返回所有和为 0 且不重复的三元组。
> 
> 注意：答案中不可以包含重复的三元组。

<!-- insert_source_code src=./src/three_sum.rs -->
```rs
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

```
<!-- insert_source_code -->

排序
首先对输入数组进行排序，方便后续去重和双指针查找。

遍历每个元素作为三元组的第一个数
用下标 i 遍历数组，每次以 nums[i] 作为三元组的第一个数。

双指针查找剩余两个数
对于每个 i，用两个指针 l（左指针，初始为 i+1）和 r（右指针，初始为数组末尾）查找，使得 nums[i] + nums[l] + nums[r] == 0。

跳过重复元素

外层循环时，如果当前元素和前一个元素相同，则跳过，避免重复三元组。
内层查找时，找到一个三元组后，分别跳过左右指针指向的重复元素。
移动指针

如果三数之和等于 0，记录结果，并左右指针分别跳过重复元素后继续移动。
如果和小于 0，左指针右移；如果和大于 0，右指针左移。
收集所有不重复的三元组
最终返回所有满足条件且不重复的三元组。

## 接雨水

> 给定 n 个非负整数表示每个宽度为 1 的柱子的高度图，计算按此排列的柱子，下雨之后能接多少雨水。

<!-- insert_source_code src=./src/trap.rs -->
```rs
use crate::Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let n = height.len();
        if n == 0 {
            return 0;
        }
        let mut left_max = vec![0; n];
        let mut right_max = vec![0; n];
        left_max[0] = height[0];
        for i in 1..n {
            left_max[i] = left_max[i - 1].max(height[i]);
        }
        right_max[n - 1] = height[n - 1];
        for i in (0..n - 1).rev() {
            right_max[i] = right_max[i + 1].max(height[i]);
        }
        let mut res = 0;
        for i in 0..n {
            res += left_max[i].min(right_max[i]) - height[i];
        }
        res
    }
}

```
<!-- insert_source_code -->

预处理左右最大高度：
对于每个位置，分别计算其左侧（包括自身）和右侧（包括自身）的最大高度，分别存入 left_max 和 right_max 数组。

遍历每个位置计算可接水量：
对于每个柱子，能接的水量等于该位置左右最大高度的较小值减去当前高度，即 min(left_max[i], right_max[i]) - height[i]。

累加所有位置的水量：
将每个位置能接的水量累加，得到总的接雨水量。

时间复杂度 O(n)：
只需三次遍历数组，效率高，空间复杂度 O(n)。
