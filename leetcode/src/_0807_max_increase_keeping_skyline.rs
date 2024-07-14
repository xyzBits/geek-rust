use crate::Solution;

impl Solution {
    pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
        let length = grid.len();
        let mut row_max = vec![0; length];
        let mut col_max = vec![0; length];

        for i in 0..length {
            for j in 0..length {
                row_max[i] = row_max[i].max(grid[i][j]);
                col_max[j] = col_max[j].max(grid[i][j]);
            }
        }
        let mut ans = 0;

        for i in 0..length {
            for j in 0..length {
                ans += row_max[i].min(col_max[j]) - grid[i][j];
            }
        }

        ans
    }
}

#[test]
fn test_compile() {}