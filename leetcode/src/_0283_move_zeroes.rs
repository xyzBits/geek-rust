use crate::Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut offset  = 0_usize;

        for i in 0..nums.len() {
            // 碰到 为 0 的元素，就将 offset ++
            if nums[i] == 0 {
                offset += 1;
            } else if offset != 0 {
                // offset 不为 0 时，进行替换，并将 i 位置上的元素置为 0
                nums[i - offset] = nums[i];
                nums[i] = 0;
            }
        }

    }

}

#[test]
fn test_move_zeroes() {

}