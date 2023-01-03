mod binary_search;
mod brute_force;

struct Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        binary_search::run(x)
    }
}
