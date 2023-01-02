mod brute_force;

struct Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        brute_force::run(x)
    }
}
