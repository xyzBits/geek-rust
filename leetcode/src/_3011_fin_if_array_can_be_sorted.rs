use crate::Solution;

impl Solution {
    pub fn can_sort_array(nums: Vec<i32>) -> bool {
        let mut last_cnt = 0;
        let mut last_group_max = 0;
        let mut cur_group_max = 0;

        for num in nums {
            let cur_cnt = num.count_ones();
            if cur_cnt == last_cnt {
                cur_group_max = cur_group_max.max(num);
            } else {
                last_cnt = cur_cnt;
                last_group_max = cur_group_max;
                cur_group_max = num;
            }


            if num < last_group_max {
                return false;
            }
        }

        true
    }
}