use crate::Solution;

impl Solution {
    //https://leetcode.cn/problems/3sum/solutions/2795635/15-san-shu-zhi-he-sortpai-xu-shuang-zhi-2kjcq/?envType=study-plan-v2&envId=top-100-liked
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        let mut nums = nums;

        // ！！！全文的精髓就是对数组进行了排序。这样就能根据和的正负去调整指针移动！！！
        nums.sort();

        let length = nums.len();

        for i in 0..length - 2 {
            if i > 0 && nums[i] == nums[i - 1] {
                // 跳过重复元素
                continue;
            }

            let (mut left, mut right) = (i + 1, length - 1);
            while left < right {
                let sum = nums[i] + nums[left] + nums[right];
                if sum == 0 {
                    ans.push(vec![nums[i], nums[left], nums[right]]);

                    while left < right && nums[left] == nums[left + 1] {
                        left += 1;
                    }

                    while left < right && nums[right] == nums[right - 1] {
                        right -= 1;
                    }

                    left += 1;
                    right -= 1;
                } else if sum < 0 {
                    left += 1;
                } else if sum > 0 {
                    right -= 1;
                }
            }
        }

        ans
    }
}