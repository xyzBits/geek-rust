use std::collections::HashSet;

use crate::Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let set = nums.iter().collect::<HashSet<_>>();

        let mut longest_streak = 0;
        for &&num in &set {
            if !set.contains(&(num - 1)) {
                let mut current_num = num;
                let mut current_streak = 1;

                while set.contains(&(current_num + 1)) {
                    current_num += 1;
                    current_streak += 1;
                }

                longest_streak = longest_streak.max(current_streak);
            }
        }

        longest_streak
    }
}

#[test]
fn test_compile() {}