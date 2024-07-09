use std::collections::HashMap;
use crate::Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();

        for (index, &value) in nums.iter().enumerate() {
            let remain = target - value;
            match map.get(&remain) {
                None => {
                    map.insert(value, index as i32);
                }
                Some(&remain_index) => {
                    return vec![index as i32, remain_index];
                }
            }
        }

        vec![]
    }
}