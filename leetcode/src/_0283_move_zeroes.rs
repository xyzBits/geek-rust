use crate::Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut offset  = 0_usize;

        for i in 0..nums.len() {
            if nums[i] == 0 {
                offset += 1;
            } else if offset != 0 {
                nums[i - offset] = nums[i];
                nums[i] = 0;
            }
        }

    }

}

#[test]
fn test_move_zeroes() {

}