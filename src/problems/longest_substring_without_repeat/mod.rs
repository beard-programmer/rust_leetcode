mod brute_force;

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        brute_force::run(s)
    }
}
