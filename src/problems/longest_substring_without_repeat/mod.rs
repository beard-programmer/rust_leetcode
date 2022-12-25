mod brute_force;
mod with_hash_map;
struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        with_hash_map::run(s)
    }
}
