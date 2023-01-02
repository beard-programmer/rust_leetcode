mod brute_force;

struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        brute_force::run(s)
    }
}
