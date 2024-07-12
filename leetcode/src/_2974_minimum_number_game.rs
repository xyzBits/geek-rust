use crate::Solution;

impl Solution {
    pub fn number_game(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort();

        for i in (0..nums.len()).step_by(2) {
            let tmp = nums[i];
            nums[i] = nums[i + 1];
            nums[i + 1] = tmp;
        }

        nums
    }
}