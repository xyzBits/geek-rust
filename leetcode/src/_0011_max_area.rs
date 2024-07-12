use crate::Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut max_volume = 0;

        while left < right {
            let volume = (right - left) as i32
                * (height[left].min(height[right]));
            max_volume = max_volume.max(volume);

            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }

        max_volume
    }
}

#[test]
fn test_compile() {

}