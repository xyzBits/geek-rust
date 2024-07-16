use crate::Solution;

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let s_length = s.len();
        let p_length = p.len();
        if s_length < p_length {
            return vec![];
        }

        let mut ans = vec![];
        let mut s_count = [0; 26];
        let mut p_count = [0; 26];

        for i in 0..p_length {
            s_count[s.chars().nth(i).unwrap().to_digit(10).unwrap() as usize - 'a'.to_digit(10).unwrap() as usize] += 1;
            p_count[p.chars().nth(i).unwrap().to_digit(10).unwrap() as usize - 'a'.to_digit(10).unwrap() as usize] += 1;
        }

        if s_count.eq(&p_count) {
            ans.push(0);
        }
        for i in 0..s_length - p_length {
            s_count[s.chars().nth(i).unwrap().to_digit(10).unwrap() as usize - 'a'.to_digit(10).unwrap() as usize] -= 1;
            s_count[s.chars().nth(i + p_length).unwrap().to_digit(10).unwrap() as usize - 'a'.to_digit(10).unwrap() as usize] -= 1;

            if s_count.eq(&p_count) {
                ans.push(i as i32 + 1);
            }
        }

        ans
    }
}

#[test]
fn test_compile() {

}