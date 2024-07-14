use crate::Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let length = height.len();
        if length <= 0 {
            return 0;
        }

        let mut left_max = vec![0; length];
        left_max[0] = height[0];
        for i in 1..length {
            left_max[i] = left_max[i - 1].max(height[i]);
        }

        let mut right_max = vec![0; length];
        right_max[length - 1] = height[length - 1];
        for i in (0..length - 1).rev() {
            right_max[i] = right_max[i + 1].max(height[i]);
        }

        let mut ans = 0;
        for i in 0..length {
            ans += left_max[i].min(right_max[i]) - height[i];
        }

        ans
    }
}

#[test]
fn test_compile() {
    for i in (0..=10).rev() {
        println!("{}", i);
    }
}