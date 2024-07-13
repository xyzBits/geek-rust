use crate::Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;

        nums.sort();

        let mut ans = Vec::new();

        let length = nums.len();

        for k in 0..length - 2 {
            if nums[k] > 0 {
                break;
            }

            if k > 0 && nums[k] == nums[k - 1] {
                continue;
            }

            let (mut i, mut j) = (k + 1, length - 1);
            while i < j {
                let sum = nums[k] + nums[i] + nums[j];
                if sum < 0 {
                    i += 1;
                    while i < j && nums[i] == nums[i - 1] {
                        i += 1
                    }
                } else if sum > 0 {
                    j -= 1;
                    while i < j && nums[j] == nums[j + 1] {
                        j -= 1;
                    }
                } else {
                    ans.push(vec![nums[k], nums[i], nums[j]]);
                    i += 1;
                    j -= 1;
                    while i < j && nums[i] == nums[i - 1] {
                        i += 1;
                    }
                    while i < j && nums[j] == nums[j + 1] {
                        j -= 1;
                    }
                }
            }
        }

        ans
    }
}