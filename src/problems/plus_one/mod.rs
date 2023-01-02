mod brute_force;
struct Solution;

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        brute_force::run(digits)
    }
}
