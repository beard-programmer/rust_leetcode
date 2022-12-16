// Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.

// You may assume that each input would have exactly one solution, and you may not use the same element twice.

// You can return the answer in any order.

// Example 1:

// Input: nums = [2,7,11,15], target = 9
// Output: [0,1]
// Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
// Example 2:

// Input: nums = [3,2,4], target = 6
// Output: [1,2]
// Example 3:

// Input: nums = [3,3], target = 6
// Output: [0,1]

// Constraints:

// 2 <= nums.length <= 104
// -109 <= nums[i] <= 109
// -109 <= target <= 109
// Only one valid answer exists.

// Follow-up: Can you come up with an algorithm that is less than O(n2) time complexity?

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        broot_force(nums, target)
    }
}
// O(n^2) because of iter inside iter
fn broot_force(nums: Vec<i32>, target: i32) -> Vec<i32> {
    nums.iter()
        .enumerate()
        .filter_map(|(seek_index, seek)| {
            let seek_value = nums
                .iter()
                .enumerate()
                .filter(|(index, _)| index != &seek_index)
                .find(|(_, value)| *value + seek == target);
            match seek_value {
                Some(_) => Some(seek_index as i32),
                None => None,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn broot_force_test_1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let result = broot_force(nums, target);
        let expected_result = vec![0, 1];
        assert_eq!(result, expected_result)
    }

    #[test]
    fn broot_force_test_2() {
        let nums = vec![3, 2, 4];
        let target = 6;
        let result = broot_force(nums, target);
        let expected_result = vec![1, 2];
        assert_eq!(result, expected_result)
    }

    #[test]
    fn broot_force_test_3() {
        let nums = vec![3, 3];
        let target = 6;
        let result = broot_force(nums, target);
        let expected_result = vec![0, 1];
        assert_eq!(result, expected_result)
    }
}
