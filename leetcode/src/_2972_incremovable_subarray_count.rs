use crate::Solution;

impl Solution {
    pub fn incremovable_subarray_count(nums: Vec<i32>) -> i64 {
        let mut ans = 0_i64;
        let length = nums.len();
        let mut left = 0;

        while left < length - 1 {
            if nums[left] >= nums[left + 1] {
                break;
            }
            left += 1;
        }

        if left == length - 1 {
            return length as i64 * (length as i64 + 1) / 2;
        }

        ans += left as i64 + 2;

        for right in (1..length).rev() {
            if right < length - 1 && nums[right] >= nums[right + 1] {
                break;
            }

            while left as i32 >= 0 && nums[left] >= nums[right] {
                left -= 1;
            }

            ans += left as i64 + 2;
        }


        ans
    }
}

#[test]
fn test_compile() {}