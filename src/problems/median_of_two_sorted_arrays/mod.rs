mod brute_force;

struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        brute_force::run(nums1, nums2)
    }
}
