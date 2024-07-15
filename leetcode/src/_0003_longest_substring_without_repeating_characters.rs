use std::collections::HashSet;

use crate::Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let (mut left, mut right, mut ans) = (0, 0, 0);

        let mut s_arr = s.chars().collect::<Vec<_>>();
        let mut cache = HashSet::new();

        s_arr.iter().enumerate().for_each(|(i, ch)| {
            while cache.contains(ch) {
                cache.remove(&s_arr[left as usize]);
                left += 1;
            }
            cache.insert(ch);
            ans = ans.max(right - left + 1);
            right += 1;
        });

        ans
    }
}

#[test]
fn test_compile() {}