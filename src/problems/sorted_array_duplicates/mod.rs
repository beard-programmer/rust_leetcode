mod brute_force;

struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        brute_force::run(nums)
    }
}
